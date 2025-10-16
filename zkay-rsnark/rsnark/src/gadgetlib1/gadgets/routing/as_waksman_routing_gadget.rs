/** @file
 *****************************************************************************

 Declaration of interfaces for the AS-Waksman routing gadget.

 The gadget verifies that the outputs are a permutation of the inputs,
 by use of an AS-Waksman network.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef AS_WAKSMAN_ROUTING_GADGET_HPP_
// #define AS_WAKSMAN_ROUTING_GADGET_HPP_

use crate::common::data_structures::integer_permutation;
use crate::common::routing_algorithms::as_waksman_routing_algorithm;
use crate::gadgetlib1::gadgets/basic_gadgets;
use crate::gadgetlib1::protoboard;



template<typename FieldT>
class as_waksman_routing_gadget : public gadget<FieldT> {
private:
    /*
      Indexing conventions:

      routed_packets[column_idx][packet_idx][subpacket_idx]
      pack_inputs/unpack_outputs[packet_idx]
      asw_switch_bits[column_idx][row_idx]

      Where column_idx ranges is in range 0 .. width and packet_idx is
      in range 0 .. num_packets-1.

      Note that unlike in Bene\v{s} routing networks row_idx are
      *not* necessarily consecutive; similarly for straight edges
      routed_packets[column_idx][packet_idx] will *reuse* previously
      allocated variables.

    */
    std::vector<std::vector<pb_variable_array<FieldT> > > routed_packets;
    std::vector<multipacking_gadget<FieldT> > pack_inputs, unpack_outputs;

    /*
      If #packets = 1 then we can route without explicit switch bits
      (and save half the constraints); in this case asw_switch_bits will
      be unused.

      For asw_switch_bits 0 corresponds to switch off (straight
      connection), and 1 corresponds to switch on (crossed
      connection).
    */
    std::vector<std::map<size_t, pb_variable<FieldT> > > asw_switch_bits;
    as_waksman_topology neighbors;
public:
    const size_t num_packets;
    const size_t num_columns;
    const std::vector<pb_variable_array<FieldT>> routing_input_bits;
    const std::vector<pb_variable_array<FieldT>> routing_output_bits;

    const size_t packet_size, num_subpackets;

    as_waksman_routing_gadget(protoboard<FieldT> &pb,
                              const size_t num_packets,
                              const std::vector<pb_variable_array<FieldT>> &routing_input_bits,
                              const std::vector<pb_variable_array<FieldT>> &routing_output_bits,
                              const std::string& annotation_prefix="");
    void generate_r1cs_constraints();
    void generate_r1cs_witness(const integer_permutation& permutation);
};

template<typename FieldT>
void test_as_waksman_routing_gadget(const size_t num_packets, const size_t packet_size);



use crate::gadgetlib1::gadgets/routing/as_waksman_routing_gadget;

//#endif // AS_WAKSMAN_ROUTING_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the AS-Waksman routing gadget.

 See as_waksman_routing_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef AS_WAKSMAN_ROUTING_GADGET_TCC_
// #define AS_WAKSMAN_ROUTING_GADGET_TCC_

use  <algorithm>

use ffec::common::profiling;

use crate::common::routing_algorithms::as_waksman_routing_algorithm;



template<typename FieldT>
as_waksman_routing_gadget<FieldT>::as_waksman_routing_gadget(protoboard<FieldT> &pb,
                                                             const size_t num_packets,
                                                             const std::vector<pb_variable_array<FieldT> > &routing_input_bits,
                                                             const std::vector<pb_variable_array<FieldT> > &routing_output_bits,
                                                             const std::string& annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    num_packets(num_packets),
    num_columns(as_waksman_num_columns(num_packets)),
    routing_input_bits(routing_input_bits),
    routing_output_bits(routing_output_bits),
    packet_size(routing_input_bits[0].size()),
    num_subpackets(ffec::div_ceil(packet_size, FieldT::capacity()))
{
    neighbors = generate_as_waksman_topology(num_packets);
    routed_packets.resize(num_columns+1);

    /* Two pass allocation. First allocate LHS packets, then for every
       switch either copy over the variables from previously allocated
       to allocate target packets */
    routed_packets[0].resize(num_packets);
    for packet_idx in 0..num_packets
    {
        routed_packets[0][packet_idx].allocate(pb, num_subpackets, FMT(annotation_prefix, " routed_packets_0_{}", packet_idx));
    }

    for column_idx in 0..num_columns
    {
        routed_packets[column_idx+1].resize(num_packets);

        for row_idx in 0..num_packets
        {
            if neighbors[column_idx][row_idx].first == neighbors[column_idx][row_idx].second
            {
                /* This is a straight edge, so just copy over the previously allocated subpackets */
                routed_packets[column_idx+1][neighbors[column_idx][row_idx].first] = routed_packets[column_idx][row_idx];
            }
            else
            {
                const size_t straight_edge = neighbors[column_idx][row_idx].first;
                const size_t cross_edge = neighbors[column_idx][row_idx].second;
                routed_packets[column_idx+1][straight_edge].allocate(pb, num_subpackets, FMT(annotation_prefix, " routed_packets_%zu_{}", column_idx+1, straight_edge));
                routed_packets[column_idx+1][cross_edge].allocate(pb, num_subpackets, FMT(annotation_prefix, " routed_packets_%zu_{}", column_idx+1, cross_edge));
                row_idx+=1; /* skip the next idx, as it to refers to the same packets */
            }
        }
    }

    /* create packing/unpacking gadgets */
    pack_inputs.reserve(num_packets); unpack_outputs.reserve(num_packets);
    for packet_idx in 0..num_packets
    {
        pack_inputs.push(
            multipacking_gadget<FieldT>(pb,
                                        pb_variable_array<FieldT>(routing_input_bits[packet_idx].begin(), routing_input_bits[packet_idx].end()),
                                        routed_packets[0][packet_idx],
                                        FieldT::capacity(),
                                        FMT(self.annotation_prefix, " pack_inputs_{}", packet_idx)));
        unpack_outputs.push(
            multipacking_gadget<FieldT>(pb,
                                        pb_variable_array<FieldT>(routing_output_bits[packet_idx].begin(), routing_output_bits[packet_idx].end()),
                                        routed_packets[num_columns][packet_idx],
                                        FieldT::capacity(),
                                        FMT(self.annotation_prefix, " unpack_outputs_{}", packet_idx)));
    }

    /* allocate switch bits */
    if num_subpackets > 1
    {
        asw_switch_bits.resize(num_columns);

        for column_idx in 0..num_columns
        {
            for row_idx in 0..num_packets
            {
                if neighbors[column_idx][row_idx].first != neighbors[column_idx][row_idx].second
                {
                    asw_switch_bits[column_idx][row_idx].allocate(pb, FMT(annotation_prefix, " asw_switch_bits_%zu_{}", column_idx, row_idx));
                    row_idx+=1; /* next row_idx corresponds to the same switch, so skip it */
                }
            }
        }
    }
}

template<typename FieldT>
void as_waksman_routing_gadget<FieldT>::generate_r1cs_constraints()
{
    /* packing/unpacking */
    for packet_idx in 0..num_packets
    {
        pack_inputs[packet_idx].generate_r1cs_constraints(false);
        unpack_outputs[packet_idx].generate_r1cs_constraints(true);
    }

    /* actual routing constraints */
    for column_idx in 0..num_columns
    {
        for row_idx in 0..num_packets
        {
            if neighbors[column_idx][row_idx].first == neighbors[column_idx][row_idx].second
            {
                /* if there is no switch at this position, then just continue with next row_idx */
                continue;
            }

            if num_subpackets == 1
            {
                /* easy case: require that
                   (cur-straight_edge)*(cur-cross_edge) = 0 for both
                   switch inputs */
                for switch_input in &{ row_idx, row_idx+1 }
                {
                    const size_t straight_edge = neighbors[column_idx][switch_input].first;
                    const size_t cross_edge = neighbors[column_idx][switch_input].second;

                    self.pb.add_r1cs_constraint(
                        r1cs_constraint<FieldT>(routed_packets[column_idx][switch_input][0] - routed_packets[column_idx+1][straight_edge][0],
                                                routed_packets[column_idx][switch_input][0] - routed_packets[column_idx+1][cross_edge][0],
                                                0),
                        FMT(self.annotation_prefix, " easy_route_%zu_{}", column_idx, switch_input));
                }
            }
            else
            {
                /* require switching bit to be boolean */
                generate_boolean_r1cs_constraint<FieldT>(self.pb, asw_switch_bits[column_idx][row_idx],
                                                         FMT(self.annotation_prefix, " asw_switch_bits_%zu_{}", column_idx, row_idx));

                /* route forward according to the switch bit */
                for subpacket_idx in 0..num_subpackets
                {
                    /*
                      (1-switch_bit) * (cur-straight_edge) + switch_bit * (cur-cross_edge) = 0
                      switch_bit * (cross_edge-straight_edge) = cur-straight_edge
                     */
                    for switch_input in &{ row_idx, row_idx+1 }
                    {
                        const size_t straight_edge = neighbors[column_idx][switch_input].first;
                        const size_t cross_edge = neighbors[column_idx][switch_input].second;

                        self.pb.add_r1cs_constraint(
                            r1cs_constraint<FieldT>(
                                asw_switch_bits[column_idx][row_idx],
                                routed_packets[column_idx+1][cross_edge][subpacket_idx] - routed_packets[column_idx+1][straight_edge][subpacket_idx],
                                routed_packets[column_idx][switch_input][subpacket_idx] - routed_packets[column_idx+1][straight_edge][subpacket_idx]),
                            FMT(self.annotation_prefix, " route_forward_%zu_%zu_{}", column_idx, switch_input, subpacket_idx));
                    }
                }
            }

            /* we processed both switch inputs at once, so skip the next iteration */
            row_idx+=1;
        }
    }
}

template<typename FieldT>
void as_waksman_routing_gadget<FieldT>::generate_r1cs_witness(const integer_permutation& permutation)
{
    /* pack inputs */
    for packet_idx in 0..num_packets
    {
        pack_inputs[packet_idx].generate_r1cs_witness_from_bits();
    }

    /* do the routing */
    as_waksman_routing routing = get_as_waksman_routing(permutation);

    for column_idx in 0..num_columns
    {
        for row_idx in 0..num_packets
        {
            if neighbors[column_idx][row_idx].first == neighbors[column_idx][row_idx].second
            {
                /* this is a straight edge, so just pass the values forward */
                const size_t next = neighbors[column_idx][row_idx].first;

                for subpacket_idx in 0..num_subpackets
                {
                    self.pb.val(routed_packets[column_idx+1][next][subpacket_idx]) = self.pb.val(routed_packets[column_idx][row_idx][subpacket_idx]);
                }
            }
            else
            {
                if num_subpackets > 1
                {
                    /* update the switch bit */
                    self.pb.val(asw_switch_bits[column_idx][row_idx]) = FieldT(routing[column_idx][row_idx] ? 1 : 0);
                }

                /* route according to the switch bit */
                const bool switch_val = routing[column_idx][row_idx];

                for switch_input in &{ row_idx, row_idx+1 }
                {
                    const size_t straight_edge = neighbors[column_idx][switch_input].first;
                    const size_t cross_edge = neighbors[column_idx][switch_input].second;

                    const size_t switched_edge = (switch_val ? cross_edge : straight_edge);

                    for subpacket_idx in 0..num_subpackets
                    {
                        self.pb.val(routed_packets[column_idx+1][switched_edge][subpacket_idx]) = self.pb.val(routed_packets[column_idx][switch_input][subpacket_idx]);
                    }
                }

                /* we processed both switch inputs at once, so skip the next iteration */
                row_idx+=1;
            }
        }
    }

    /* unpack outputs */
    for packet_idx in 0..num_packets
    {
        unpack_outputs[packet_idx].generate_r1cs_witness_from_packed();
    }
}

template<typename FieldT>
void test_as_waksman_routing_gadget(const size_t num_packets, const size_t packet_size)
{
    print!("testing as_waksman_routing_gadget by routing {} element vector of {} bits (Fp fits all {} bit integers)\n", num_packets, packet_size, FieldT::capacity());
    protoboard<FieldT> pb;
    integer_permutation permutation(num_packets);
    permutation.random_shuffle();
    ffec::print_time("generated permutation");

    std::vector<pb_variable_array<FieldT> > randbits(num_packets), outbits(num_packets);
    for packet_idx in 0..num_packets
    {
        randbits[packet_idx].allocate(pb, packet_size, FMT("", "randbits_{}", packet_idx));
        outbits[packet_idx].allocate(pb, packet_size, FMT("", "outbits_{}", packet_idx));

        for bit_idx in 0..packet_size
        {
            pb.val(randbits[packet_idx][bit_idx]) = (rand() % 2) ? FieldT::one() : FieldT::zero();
        }
    }
    ffec::print_time("generated bits to be routed");

    as_waksman_routing_gadget<FieldT> r(pb, num_packets, randbits, outbits, "main_routing_gadget");
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
            assert!(pb.val(outbits[permutation.get(packet_idx)][bit_idx]) == pb.val(randbits[packet_idx][bit_idx]));
        }
    }

    print!("negative test\n");
    pb.val(pb_variable<FieldT>(10)) = FieldT(12345);
    assert!(!pb.is_satisfied());

    print!("num_constraints = {}, num_variables = {}\n",
           pb.num_constraints(),
           pb.constraint_system.num_variables);
}



//#endif // AS_WAKSMAN_ROUTING_GADGET_TCC_
