//  Declaration of interfaces for the Benes routing gadget.

//  The gadget verifies that the outputs are a permutation of the inputs,
//  by use of a Benes network.
use crate::common::data_structures::integer_permutation::integer_permutation;
use crate::common::routing_algorithms::as_waksman_routing_algorithm::{
    as_waksman_num_columns, as_waksman_topology, generate_as_waksman_topology,
    get_as_waksman_routing,
};
use crate::common::routing_algorithms::benes_routing_algorithm::{
    benes_num_columns, benes_topology, generate_benes_topology, get_benes_routing,
};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, multipacking_gadget, multipacking_gadgets,
};
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig,ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::profiling::print_time;
use ffec::common::utils::{div_ceil, log2};
use rccell::RcCell;
use std::collections::BTreeMap;
#[derive(Clone, Default)]
pub struct benes_routing_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>

    /*
      Indexing conventions:

      routed_packets[column_idx][packet_idx][subpacket_idx]
      pack_inputs/unpack_outputs[packet_idx]
      benes_switch_bits[column_idx][row_idx]

      Where column_idx ranges is in range 0 .. 2*dimension
      (2*dimension-1 for switch bits/topology) and packet_idx is in
      range 0 .. num_packets-1.
    */
    routed_packets: Vec<Vec<pb_variable_array<FieldT, PB>>>,
    unpack_outputs: Vec<multipacking_gadgets<FieldT, PB>>,
    pack_inputs: Vec<multipacking_gadgets<FieldT, PB>>,

    /*
      If #packets = 1 then we can route without explicit routing bits
      (and save half the constraints); in this case benes_switch_bits will
      be unused.

      For benes_switch_bits 0 corresponds to straight edge and 1
      corresponds to cross edge.
    */
    benes_switch_bits: Vec<pb_variable_array<FieldT, PB>>,
    neighbors: benes_topology,

    num_packets: usize,
    num_columns: usize,

    routing_input_bits: Vec<pb_variable_array<FieldT, PB>>,
    routing_output_bits: Vec<pb_variable_array<FieldT, PB>>,
    lines_to_unpack: usize,

    packet_size: usize,
    num_subpackets: usize,
}

pub type benes_routing_gadgets<FieldT, PB> = gadget<FieldT, PB, benes_routing_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> benes_routing_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        num_packets: usize,
        routing_input_bits: Vec<pb_variable_array<FieldT, PB>>,
        routing_output_bits: Vec<pb_variable_array<FieldT, PB>>,
        lines_to_unpack: usize,
        annotation_prefix: String,
    ) -> benes_routing_gadgets<FieldT, PB> {
        let num_columns = benes_num_columns(num_packets);
        let packet_size = routing_input_bits[0].len();
        let num_subpackets = div_ceil(packet_size, FieldT::capacity()).unwrap();
        assert!(lines_to_unpack <= routing_input_bits.len());
        assert!(num_packets == 1usize << log2(num_packets));
        assert!(routing_input_bits.len() == num_packets);

        let mut neighbors = generate_benes_topology(num_packets);

        let mut routed_packets = vec![vec![]; num_columns + 1];

        for column_idx in 0..=num_columns {
            routed_packets[column_idx] =
                vec![pb_variable_array::<FieldT, PB>::default(); num_packets];
            for packet_idx in 0..num_packets {
                routed_packets[column_idx][packet_idx].allocate(
                    &pb,
                    num_subpackets,
                    prefix_format!(
                        annotation_prefix,
                        " routed_packets_{}_{}",
                        column_idx,
                        packet_idx,
                    ),
                );
            }
        }

        let mut pack_inputs = Vec::with_capacity(num_packets);
        let mut unpack_outputs = Vec::with_capacity(num_packets);

        for packet_idx in 0..num_packets {
            pack_inputs.push(multipacking_gadget::<FieldT, PB>::new(
                pb.clone(),
                pb_variable_array::<FieldT, PB>::new(
                    routing_input_bits[packet_idx].contents.clone(),
                )
                .into(),
                routed_packets[0][packet_idx].clone().into(),
                FieldT::capacity(),
                prefix_format!(annotation_prefix, " pack_inputs_{}", packet_idx),
            ));
            if packet_idx < lines_to_unpack {
                unpack_outputs.push(multipacking_gadget::<FieldT, PB>::new(
                    pb.clone(),
                    pb_variable_array::<FieldT, PB>::new(
                        routing_output_bits[packet_idx].contents.clone(),
                    )
                    .into(),
                    routed_packets[num_columns][packet_idx].clone().into(),
                    FieldT::capacity(),
                    prefix_format!(annotation_prefix, " unpack_outputs_{}", packet_idx),
                ));
            }
        }
        let mut benes_switch_bits = vec![];

        if num_subpackets > 1 {
            benes_switch_bits.resize(num_columns, pb_variable_array::<FieldT, PB>::default());
            for column_idx in 0..num_columns {
                benes_switch_bits[column_idx].allocate(
                    &pb,
                    num_packets,
                    prefix_format!(annotation_prefix, " benes_switch_bits_{}", column_idx),
                );
            }
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                routed_packets,
                unpack_outputs,
                pack_inputs,
                benes_switch_bits,
                neighbors,
                num_packets,
                num_columns,
                routing_input_bits,
                routing_output_bits,
                lines_to_unpack,
                packet_size,
                num_subpackets,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> benes_routing_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        /* packing/unpacking */
        for packet_idx in 0..self.t.num_packets {
            self.t.pack_inputs[packet_idx].generate_r1cs_constraints(false);
            if packet_idx < self.t.lines_to_unpack {
                self.t.unpack_outputs[packet_idx].generate_r1cs_constraints(true);
            } else {
                for subpacket_idx in 0..self.t.num_subpackets {
                    self.pb.borrow_mut().add_r1cs_constraint(
                        r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                            FieldT::from(1).into(),
                            self.t.routed_packets[0][packet_idx][subpacket_idx]
                                .clone()
                                .into(),
                            self.t.routed_packets[self.t.num_columns][packet_idx][subpacket_idx]
                                .clone()
                                .into(),
                        ),
                        prefix_format!(
                            self.annotation_prefix,
                            " fix_line_{}_subpacket_{}",
                            packet_idx,
                            subpacket_idx,
                        ),
                    );
                }
            }
        }

        /* actual routing constraints */
        for column_idx in 0..self.t.num_columns {
            for packet_idx in 0..self.t.num_packets {
                let (straight_edge, cross_edge) = self.t.neighbors[column_idx][packet_idx];

                if self.t.num_subpackets == 1 {
                    /* easy case: (cur-next)*(cur-cross) = 0 */
                    self.pb.borrow_mut().add_r1cs_constraint(
                        r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                            (self.t.routed_packets[column_idx][packet_idx][0].clone()
                                - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(self.t.routed_packets[column_idx + 1][straight_edge][0].clone())),
                            (self.t.routed_packets[column_idx][packet_idx][0].clone()
                                - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(self.t.routed_packets[column_idx + 1][cross_edge][0].clone())),
                            FieldT::from(0).into(),
                        ),
                        prefix_format!(
                            self.annotation_prefix,
                            " easy_route_{}_{}",
                            column_idx,
                            packet_idx,
                        ),
                    );
                } else {
                    /* routing bit must be boolean */
                    generate_boolean_r1cs_constraint::<FieldT, PB>(
                        &self.pb,
                        &(self.t.benes_switch_bits[column_idx][packet_idx]
                            .clone()
                            .into()),
                        prefix_format!(
                            self.annotation_prefix,
                            " routing_bit_{}_{}",
                            column_idx,
                            packet_idx,
                        ),
                    );

                    /* route forward according to routing bits */
                    for subpacket_idx in 0..self.t.num_subpackets {
                        /*
                          (1-switch_bit) * (cur-straight_edge) + switch_bit * (cur-cross_edge) = 0
                          switch_bit * (cross_edge-straight_edge) = cur-straight_edge
                        */
                        self.pb.borrow_mut().add_r1cs_constraint(
                            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                                self.t.benes_switch_bits[column_idx][packet_idx]
                                    .clone()
                                    .into(),
                                self.t.routed_packets[column_idx + 1][cross_edge][subpacket_idx]
                                    .clone()
                                    - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(
                                        self.t.routed_packets[column_idx + 1][straight_edge]
                                            [subpacket_idx]
                                            .clone(),
                                    ),
                                self.t.routed_packets[column_idx][packet_idx][subpacket_idx]
                                    .clone()
                                    - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(
                                        self.t.routed_packets[column_idx + 1][straight_edge]
                                            [subpacket_idx]
                                            .clone(),
                                    ),
                            ),
                            prefix_format!(
                                self.annotation_prefix,
                                " route_forward_{}_{}_{}",
                                column_idx,
                                packet_idx,
                                subpacket_idx,
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn generate_r1cs_witness(&self, permutation: &integer_permutation) {
        /* pack inputs */
        for packet_idx in 0..self.t.num_packets {
            self.t.pack_inputs[packet_idx].generate_r1cs_witness_from_bits();
        }

        /* do the routing */
        let routing = get_benes_routing(permutation);

        for column_idx in 0..self.t.num_columns {
            for packet_idx in 0..self.t.num_packets {
                let (straight_edge, cross_edge) = self.t.neighbors[column_idx][packet_idx];

                if self.t.num_subpackets > 1 {
                    *self
                        .pb
                        .borrow_mut()
                        .val_ref(&self.t.benes_switch_bits[column_idx][packet_idx]) =
                        FieldT::from(if routing[column_idx][packet_idx] {
                            1
                        } else {
                            0
                        });
                }

                for subpacket_idx in 0..self.t.num_subpackets {
                    *self
                        .pb
                        .borrow_mut()
                        .val_ref(&if routing[column_idx][packet_idx] {
                            self.t.routed_packets[column_idx + 1][cross_edge][subpacket_idx].clone()
                        } else {
                            self.t.routed_packets[column_idx + 1][straight_edge][subpacket_idx]
                                .clone()
                        }) = self
                        .pb
                        .borrow()
                        .val(&self.t.routed_packets[column_idx][packet_idx][subpacket_idx]);
                }
            }
        }

        /* unpack outputs */
        for packet_idx in 0..self.t.lines_to_unpack {
            self.t.unpack_outputs[packet_idx].generate_r1cs_witness_from_packed();
        }
    }
}

pub fn test_benes_routing_gadget<FieldT: FieldTConfig, PB: PBConfig>(
    num_packets: usize,
    packet_size: usize,
) {
    let dimension = log2(num_packets);
    assert!(num_packets == 1usize << dimension);

    print!(
        "testing benes_routing_gadget by routing 2^{}-entry vector of {} bits (Fp fits all {} bit integers)\n",
        dimension,
        packet_size,
        FieldT::capacity()
    );

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut permutation = integer_permutation::new(num_packets);
    permutation.random_shuffle();
    print_time("generated permutation");

    let (mut randbits, mut outbits) = (
        vec![pb_variable_array::<FieldT, PB>::default(); num_packets],
        vec![pb_variable_array::<FieldT, PB>::default(); num_packets],
    );
    for packet_idx in 0..num_packets {
        randbits[packet_idx].allocate(
            &pb,
            packet_size,
            prefix_format!("", "randbits_{}", packet_idx),
        );
        outbits[packet_idx].allocate(
            &pb,
            packet_size,
            prefix_format!("", "outbits_{}", packet_idx),
        );

        for bit_idx in 0..packet_size {
            *pb.borrow_mut().val_ref(&randbits[packet_idx][bit_idx]) =
                if rand::random::<usize>() % 2 != 0 {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
        }
    }
    print_time("generated bits to be routed");

    let mut r = benes_routing_gadget::<FieldT, PB>::new(
        pb.clone(),
        num_packets,
        randbits.clone(),
        outbits.clone(),
        num_packets,
        "main_routing_gadget".to_owned(),
    );
    r.generate_r1cs_constraints();
    print_time("generated routing constraints");

    r.generate_r1cs_witness(&permutation);
    print_time("generated routing assignment");

    print!("positive test\n");
    assert!(pb.borrow().is_satisfied());
    for packet_idx in 0..num_packets {
        for bit_idx in 0..packet_size {
            assert!(
                pb.borrow()
                    .val(&outbits[permutation.get(packet_idx)][bit_idx])
                    == pb.borrow().val(&randbits[packet_idx][bit_idx])
            );
        }
    }

    print!("negative test\n");
    *pb.borrow_mut()
        .val_ref(&variable::<FieldT, pb_variable>::from(10)) = FieldT::from(12345);
    assert!(!pb.borrow().is_satisfied());

    print!(
        "num_constraints = {}, num_variables = {}\n",
        pb.borrow().num_constraints(),
        pb.borrow().constraint_system.num_variables()
    );
}
