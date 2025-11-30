/** @file
 *****************************************************************************

 Declaration of interfaces for the Benes routing gadget.

 The gadget verifies that the outputs are a permutation of the inputs,
 by use of a Benes network.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BENES_ROUTING_GADGET_HPP_
// #define BENES_ROUTING_GADGET_HPP_

use crate::common::data_structures::integer_permutation;
use crate::common::routing_algorithms::benes_routing_algorithm;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::protoboard;




pub struct benes_routing_gadget<FieldT> {//gadget<FieldT>

    /*
      Indexing conventions:

      routed_packets[column_idx][packet_idx][subpacket_idx]
      pack_inputs/unpack_outputs[packet_idx]
      benes_switch_bits[column_idx][row_idx]

      Where column_idx ranges is in range 0 .. 2*dimension
      (2*dimension-1 for switch bits/topology) and packet_idx is in
      range 0 .. num_packets-1.
    */
routed_packets:    Vec<Vec<pb_variable_array<FieldT> > >,
unpack_outputs:    Vec<multipacking_gadget<FieldT> >, pack_inputs:    Vec<multipacking_gadget<FieldT> >,

    /*
      If #packets = 1 then we can route without explicit routing bits
      (and save half the constraints); in this case benes_switch_bits will
      be unused.

      For benes_switch_bits 0 corresponds to straight edge and 1
      corresponds to cross edge.
    */
benes_switch_bits:    Vec<pb_variable_array<FieldT>>,
neighbors:    benes_topology,

    num_packets:usize,
    num_columns:usize,

    routing_input_bits:Vec<pb_variable_array<FieldT> >,
    routing_output_bits:Vec<pb_variable_array<FieldT> >,
lines_to_unpack:    usize,

    packet_size:usize, num_subpackets:usize,

}




// use crate::gadgetlib1::gadgets::routing::benes_routing_gadget;

//#endif // BENES_ROUTING_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the Benes routing gadget.

 See benes_routing_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BENES_ROUTING_GADGET_TCC_
// #define BENES_ROUTING_GADGET_TCC_

// use  <algorithm>

use ffec::common::profiling;


impl benes_routing_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                   num_packets:usize,
                                                   routing_input_bits:&Vec<pb_variable_array<FieldT> >,
                                                   routing_output_bits:&Vec<pb_variable_array<FieldT> >,
                                                   lines_to_unpack:usize,
                                                   annotation_prefix:&String)->Self
    
{
    assert!(lines_to_unpack <= routing_input_bits.len());
    assert!(num_packets == 1u64<<ffec::log2(num_packets));
    assert!(routing_input_bits.len() == num_packets);

    neighbors = generate_benes_topology(num_packets);

    routed_packets.resize(num_columns+1);
    for column_idx in 0..=num_columns
    {
        routed_packets[column_idx].resize(num_packets);
        for packet_idx in 0..num_packets
        {
            routed_packets[column_idx][packet_idx].allocate(&pb, num_subpackets, FMT(annotation_prefix, " routed_packets_{}_{}", column_idx, packet_idx));
        }
    }

    pack_inputs.reserve(num_packets);
    unpack_outputs.reserve(num_packets);

    for packet_idx in 0..num_packets
    {
        pack_inputs.push(
            multipacking_gadget::<FieldT>(&pb,
                                        pb_variable_array::<FieldT>(routing_input_bits[packet_idx].begin(), routing_input_bits[packet_idx].end()),
                                        routed_packets[0][packet_idx],
                                        FieldT::capacity(),
                                      FMT(self.annotation_prefix, " pack_inputs_{}", packet_idx)));
        if packet_idx < lines_to_unpack
        {
            unpack_outputs.push(
                multipacking_gadget::<FieldT>(&pb,
                                            pb_variable_array::<FieldT>(routing_output_bits[packet_idx].begin(), routing_output_bits[packet_idx].end()),
                                            routed_packets[num_columns][packet_idx],
                                            FieldT::capacity(),
                                          FMT(self.annotation_prefix, " unpack_outputs_{}", packet_idx)));
        }
    }

    if num_subpackets > 1
    {
        benes_switch_bits.resize(num_columns);
        for column_idx in 0..num_columns
        {
            benes_switch_bits[column_idx].allocate(&pb, num_packets, FMT(self.annotation_prefix, " benes_switch_bits_{}", column_idx));
        }
    }
    // gadget<FieldT>(&pb, annotation_prefix),
   Self{num_packets,
    num_columns:benes_num_columns(num_packets),
   routing_input_bits,
   routing_output_bits,
   lines_to_unpack,
    packet_size:routing_input_bits[0].len(),
    num_subpackets:ffec::div_ceil(packet_size, FieldT::capacity())
    }
}


pub fn generate_r1cs_constraints()
{
    /* packing/unpacking */
    for packet_idx in 0..num_packets
    {
        pack_inputs[packet_idx].generate_r1cs_constraints(false);
        if packet_idx < lines_to_unpack
        {
            unpack_outputs[packet_idx].generate_r1cs_constraints(true);
        }
        else
        {
            for subpacket_idx in 0..num_subpackets
            {
                self.pb.borrow_mut().add_r1cs_constraint(
                    r1cs_constraint::<FieldT>(1, routed_packets[0][packet_idx][subpacket_idx], routed_packets[num_columns][packet_idx][subpacket_idx]),
                  FMT(self.annotation_prefix, " fix_line_{}_subpacket_{}", packet_idx, subpacket_idx));
            }
        }
    }

    /* actual routing constraints */
    for column_idx in 0..num_columns
    {
        for packet_idx in 0..num_packets
        {
            let straight_edge = neighbors[column_idx][packet_idx].first;
            let cross_edge = neighbors[column_idx][packet_idx].second;

            if num_subpackets == 1
            {
                /* easy case: (cur-next)*(cur-cross) = 0 */
                self.pb.borrow_mut().add_r1cs_constraint(
                    r1cs_constraint::<FieldT>(
                        routed_packets[column_idx][packet_idx][0] - routed_packets[column_idx+1][straight_edge][0],
                        routed_packets[column_idx][packet_idx][0] - routed_packets[column_idx+1][cross_edge][0],
                        0),
                  FMT(self.annotation_prefix, " easy_route_{}_{}", column_idx, packet_idx));
            }
            else
            {
                /* routing bit must be boolean */
                generate_boolean_r1cs_constraint::<FieldT>(self.pb, benes_switch_bits[column_idx][packet_idx],
                                                       FMT(self.annotation_prefix, " routing_bit_{}_{}", column_idx, packet_idx));

                /* route forward according to routing bits */
                for subpacket_idx in 0..num_subpackets
                {
                    /*
                      (1-switch_bit) * (cur-straight_edge) + switch_bit * (cur-cross_edge) = 0
                      switch_bit * (cross_edge-straight_edge) = cur-straight_edge
                    */
                    self.pb.borrow_mut().add_r1cs_constraint(
                        r1cs_constraint::<FieldT>(
                            benes_switch_bits[column_idx][packet_idx],
                            routed_packets[column_idx+1][cross_edge][subpacket_idx] - routed_packets[column_idx+1][straight_edge][subpacket_idx],
                            routed_packets[column_idx][packet_idx][subpacket_idx] - routed_packets[column_idx+1][straight_edge][subpacket_idx]),
                      FMT(self.annotation_prefix, " route_forward_{}_{}_{}", column_idx, packet_idx, subpacket_idx));
                }
            }
        }
    }
}


pub fn generate_r1cs_witness(permutation:&integer_permutation)
{
    /* pack inputs */
    for packet_idx in 0..num_packets
    {
        pack_inputs[packet_idx].generate_r1cs_witness_from_bits();
    }

    /* do the routing */
    let routing= get_benes_routing(permutation);

    for column_idx in 0..num_columns
    {
        for packet_idx in 0..num_packets
        {
            let straight_edge = neighbors[column_idx][packet_idx].first;
            let cross_edge = neighbors[column_idx][packet_idx].second;

            if num_subpackets > 1
            {
                self.pb.borrow().val(&benes_switch_bits[column_idx][packet_idx]) = FieldT( if routing[column_idx][packet_idx] {1} else {0});
            }

            for subpacket_idx in 0..num_subpackets
            {
                self.pb.borrow().val(& if routing[column_idx][packet_idx] 
                             {routed_packets[column_idx+1][cross_edge][subpacket_idx]} else
                             {routed_packets[column_idx+1][straight_edge][subpacket_idx]}) =
                    self.pb.borrow().val(&routed_packets[column_idx][packet_idx][subpacket_idx]);
            }
        }
    }

    /* unpack outputs */
    for packet_idx in 0..lines_to_unpack
    {
        unpack_outputs[packet_idx].generate_r1cs_witness_from_packed();
    }
}

}

pub fn  test_benes_routing_gadget(num_packets:usize, packet_size:usize)
{
    let dimension = ffec::log2(num_packets);
    assert!(num_packets == 1u64<<dimension);

    print!("testing benes_routing_gadget by routing 2^{}-entry vector of {} bits (Fp fits all {} bit integers)\n", dimension, packet_size, FieldT::capacity());

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  permutation=integer_permutation::new(num_packets);
    permutation.random_shuffle();
    ffec::print_time("generated permutation");

    let (randbits, outbits)=(vec![vec![];num_packets],vec![vec![];num_packets]);
    for packet_idx in 0..num_packets
    {
        randbits[packet_idx].allocate(&pb, packet_size, FMT("", "randbits_{}", packet_idx));
        outbits[packet_idx].allocate(&pb, packet_size, FMT("", "outbits_{}", packet_idx));

        for bit_idx in 0..packet_size
        {
            pb.borrow().val(&randbits[packet_idx][bit_idx])=  if rand() % 2!=0 {FieldT::one()} else{FieldT::zero()};
        }
    }
    ffec::print_time("generated bits to be routed");

    let mut r=benes_routing_gadget::<FieldT>::new(pb, num_packets, randbits, outbits, num_packets, "main_routing_gadget");
    r.generate_r1cs_constraints();
    ffec::print_time("generated routing constraints");

    r.generate_r1cs_witness(permutation);
    ffec::print_time("generated routing assignment");

    print!("positive test\n");
    assert!(pb.is_satisfied());
    for packet_idx in 0..num_packets
    {
        for bit_idx in 0..packet_size
        {
            assert!(pb.borrow().val(&outbits[permutation.get(packet_idx)][bit_idx]) == pb.borrow().val(&randbits[packet_idx][bit_idx]));
        }
    }

    print!("negative test\n");
    pb.borrow().val(&variable::<FieldT,pb_variable>(10)) = FieldT(12345);
    assert!(!pb.is_satisfied());

    print!("num_constraints = {}, num_variables = {}\n",
           pb.num_constraints(),
           pb.constraint_system.num_variables);
}



//#endif // BENES_ROUTING_GADGET_TCC_
