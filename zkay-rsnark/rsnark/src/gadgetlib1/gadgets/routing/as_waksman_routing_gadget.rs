//  Declaration of interfaces for the AS-Waksman routing gadget.

//  The gadget verifies that the outputs are a permutation of the inputs,
//  by use of an AS-Waksman network.
use crate::common::data_structures::integer_permutation::integer_permutation;
use crate::common::routing_algorithms::as_waksman_routing_algorithm::{
    as_waksman_num_columns, as_waksman_topology, generate_as_waksman_topology,
    get_as_waksman_routing,
};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, multipacking_gadget, multipacking_gadgets,
};
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::profiling::print_time;
use ffec::common::utils::div_ceil;
use rccell::RcCell;
use std::collections::BTreeMap;
#[derive(Clone, Default)]
pub struct as_waksman_routing_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>

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
    routed_packets: Vec<Vec<pb_variable_array<FieldT, PB>>>,
    unpack_outputs: Vec<multipacking_gadgets<FieldT, PB>>,
    pack_inputs: Vec<multipacking_gadgets<FieldT, PB>>,

    /*
      If #packets = 1 then we can route without explicit switch bits
      (and save half the constraints); in this case asw_switch_bits will
      be unused.

      For asw_switch_bits 0 corresponds to switch off (straight
      connection), and 1 corresponds to switch on (crossed
      connection).
    */
    asw_switch_bits: Vec<BTreeMap<usize, variable<FieldT, pb_variable>>>,
    neighbors: as_waksman_topology,

    num_packets: usize,
    num_columns: usize,
    routing_input_bits: Vec<pb_variable_array<FieldT, PB>>,
    routing_output_bits: Vec<pb_variable_array<FieldT, PB>>,

    packet_size: usize,
    num_subpackets: usize,
}

pub type as_waksman_routing_gadgets<FieldT, PB> =
    gadget<FieldT, PB, as_waksman_routing_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> as_waksman_routing_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        num_packets: usize,
        routing_input_bits: Vec<pb_variable_array<FieldT, PB>>,
        routing_output_bits: Vec<pb_variable_array<FieldT, PB>>,
        annotation_prefix: String,
    ) -> as_waksman_routing_gadgets<FieldT, PB> {
        let packet_size = routing_input_bits[0].len();
        let num_subpackets = div_ceil(packet_size, FieldT::capacity()).unwrap();
        let num_columns = as_waksman_num_columns(num_packets);
        let mut neighbors = generate_as_waksman_topology(num_packets);
        let mut routed_packets = vec![vec![]; num_columns + 1];

        /* Two pass allocation. First allocate LHS packets, then for every
        switch either copy over the variables from previously allocated
        to allocate target packets */
        routed_packets[0] = vec![pb_variable_array::<FieldT, PB>::default(); num_packets];
        for packet_idx in 0..num_packets {
            routed_packets[0][packet_idx].allocate(
                &pb,
                num_subpackets,
                prefix_format!(annotation_prefix, " routed_packets_0_{}", packet_idx),
            );
        }

        for column_idx in 0..num_columns {
            routed_packets[column_idx + 1] =
                vec![pb_variable_array::<FieldT, PB>::default(); num_packets];

            for row_idx in 0..num_packets {
                if neighbors[column_idx][row_idx].0 == neighbors[column_idx][row_idx].1 {
                    /* This is a straight edge, so just copy over the previously allocated subpackets */
                    routed_packets[column_idx + 1][neighbors[column_idx][row_idx].0] =
                        routed_packets[column_idx][row_idx].clone();
                } else {
                    let straight_edge = neighbors[column_idx][row_idx].0;
                    let cross_edge = neighbors[column_idx][row_idx].1;
                    routed_packets[column_idx + 1][straight_edge].allocate(
                        &pb,
                        num_subpackets,
                        prefix_format!(
                            annotation_prefix,
                            " routed_packets_{}_{}",
                            column_idx + 1,
                            straight_edge,
                        ),
                    );
                    routed_packets[column_idx + 1][cross_edge].allocate(
                        &pb,
                        num_subpackets,
                        prefix_format!(
                            annotation_prefix,
                            " routed_packets_{}_{}",
                            column_idx + 1,
                            cross_edge,
                        ),
                    );
                    // row_idx += 1; /* skip the next idx, as it to refers to the same packets */
                }
            }
        }

        /* create packing/unpacking gadgets */
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
        let mut asw_switch_bits: Vec<BTreeMap<_, _>> = vec![];
        /* allocate switch bits */
        if num_subpackets > 1 {
            asw_switch_bits.resize(
                num_columns,
                BTreeMap::<usize, variable<FieldT, pb_variable>>::default(),
            );

            for column_idx in 0..num_columns {
                for row_idx in 0..num_packets {
                    if neighbors[column_idx][row_idx].0 != neighbors[column_idx][row_idx].1 {
                        asw_switch_bits[column_idx]
                            .get_mut(&row_idx)
                            .unwrap()
                            .allocate(
                                &pb,
                                prefix_format!(
                                    annotation_prefix,
                                    " asw_switch_bits_{}_{}",
                                    column_idx,
                                    row_idx,
                                ),
                            );
                        // row_idx += 1; /* next row_idx corresponds to the same switch, so skip it */
                    }
                }
            }
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                routed_packets,
                unpack_outputs,
                pack_inputs,
                asw_switch_bits,
                neighbors,
                num_packets,
                num_columns,
                routing_input_bits,
                routing_output_bits,
                packet_size,
                num_subpackets,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> as_waksman_routing_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        /* packing/unpacking */
        for packet_idx in 0..self.t.num_packets {
            self.t.pack_inputs[packet_idx].generate_r1cs_constraints(false);
            self.t.unpack_outputs[packet_idx].generate_r1cs_constraints(true);
        }

        /* actual routing constraints */
        for column_idx in 0..self.t.num_columns {
            for row_idx in 0..self.t.num_packets {
                if self.t.neighbors[column_idx][row_idx].0
                    == self.t.neighbors[column_idx][row_idx].1
                {
                    /* if there is no switch at this position, then just continue with next row_idx */
                    continue;
                }

                if self.t.num_subpackets == 1 {
                    /* easy case: require that
                    (cur-straight_edge)*(cur-cross_edge) = 0 for both
                    switch inputs */
                    for switch_input in row_idx..=row_idx + 1 {
                        let straight_edge = self.t.neighbors[column_idx][switch_input].0.clone();
                        let cross_edge = self.t.neighbors[column_idx][switch_input].1.clone();

                        self.pb.borrow_mut().add_r1cs_constraint(
                            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                                (self.t.routed_packets[column_idx][switch_input][0].clone()
                                    - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(
                                        self.t.routed_packets[column_idx + 1][straight_edge][0]
                                            .clone(),
                                    ))
                                .into(),
                                (self.t.routed_packets[column_idx][switch_input][0].clone()
                                    - linear_combination::<
                                        FieldT,
                                        pb_variable,
                                        pb_linear_combination,
                                    >::from(
                                        self.t.routed_packets[column_idx + 1][cross_edge][0]
                                            .clone(),
                                    ))
                                .into(),
                                FieldT::from(0).into(),
                            ),
                            prefix_format!(
                                self.annotation_prefix,
                                " easy_route_{}_{}",
                                column_idx,
                                switch_input,
                            ),
                        );
                    }
                } else {
                    /* require switching bit to be boolean */
                    generate_boolean_r1cs_constraint::<FieldT, PB>(
                        &self.pb,
                        &(self.t.asw_switch_bits[column_idx][&row_idx].clone().into()),
                        prefix_format!(
                            self.annotation_prefix,
                            " asw_switch_bits_{}_{}",
                            column_idx,
                            row_idx,
                        ),
                    );

                    /* route forward according to the switch bit */
                    for subpacket_idx in 0..self.t.num_subpackets {
                        /*
                         (1-switch_bit) * (cur-straight_edge) + switch_bit * (cur-cross_edge) = 0
                         switch_bit * (cross_edge-straight_edge) = cur-straight_edge
                        */
                        for &switch_input in &[row_idx, row_idx + 1] {
                            let (straight_edge, cross_edge) =
                                self.t.neighbors[column_idx][switch_input];

                            self.pb.borrow_mut().add_r1cs_constraint(
                                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                                    self.t.asw_switch_bits[column_idx][&row_idx].clone().into(),
                                    (self.t.routed_packets[column_idx + 1][cross_edge]
                                        [subpacket_idx]
                                        .clone()
                                        - linear_combination::<
                                            FieldT,
                                            pb_variable,
                                            pb_linear_combination,
                                        >::from(
                                            self.t.routed_packets[column_idx + 1][straight_edge]
                                                [subpacket_idx]
                                                .clone(),
                                        ))
                                    .into(),
                                    (self.t.routed_packets[column_idx][switch_input]
                                        [subpacket_idx]
                                        .clone()
                                        - linear_combination::<
                                            FieldT,
                                            pb_variable,
                                            pb_linear_combination,
                                        >::from(
                                            self.t.routed_packets[column_idx + 1][straight_edge]
                                                [subpacket_idx]
                                                .clone(),
                                        ))
                                    .into(),
                                ),
                                prefix_format!(
                                    self.annotation_prefix,
                                    " route_forward_{}_{}_{}",
                                    column_idx,
                                    switch_input,
                                    subpacket_idx,
                                ),
                            );
                        }
                    }
                }

                /* we processed both switch inputs at once, so skip the next iteration */
                // row_idx += 1;
            }
        }
    }

    pub fn generate_r1cs_witness(&self, permutation: &integer_permutation) {
        /* pack inputs */
        for packet_idx in 0..self.t.num_packets {
            self.t.pack_inputs[packet_idx].generate_r1cs_witness_from_bits();
        }

        /* do the routing */
        let routing = get_as_waksman_routing(permutation);

        for column_idx in 0..self.t.num_columns {
            for row_idx in 0..self.t.num_packets {
                if self.t.neighbors[column_idx][row_idx].0
                    == self.t.neighbors[column_idx][row_idx].1
                {
                    /* this is a straight edge, so just pass the values forward */
                    let next = self.t.neighbors[column_idx][row_idx].0;

                    for subpacket_idx in 0..self.t.num_subpackets {
                        *self
                            .pb
                            .borrow_mut()
                            .val_ref(&self.t.routed_packets[column_idx + 1][next][subpacket_idx]) =
                            self.pb
                                .borrow()
                                .val(&self.t.routed_packets[column_idx][row_idx][subpacket_idx]);
                    }
                } else {
                    if self.t.num_subpackets > 1 {
                        /* update the switch bit */
                        *self
                            .pb
                            .borrow_mut()
                            .val_ref(&self.t.asw_switch_bits[column_idx][&row_idx]) =
                            FieldT::from(if routing[column_idx][row_idx] { 1 } else { 0 });
                    }

                    /* route according to the switch bit */
                    let mut switch_val = routing[column_idx][row_idx];

                    for &switch_input in &[row_idx, row_idx + 1] {
                        let straight_edge = self.t.neighbors[column_idx][switch_input].0.clone();
                        let cross_edge = self.t.neighbors[column_idx][switch_input].1.clone();

                        let switched_edge = if switch_val {
                            cross_edge
                        } else {
                            straight_edge
                        };

                        for subpacket_idx in 0..self.t.num_subpackets {
                            *self.pb.borrow_mut().val_ref(
                                &self.t.routed_packets[column_idx + 1][switched_edge]
                                    [subpacket_idx],
                            ) = self.pb.borrow().val(
                                &self.t.routed_packets[column_idx][switch_input][subpacket_idx],
                            );
                        }
                    }

                    /* we processed both switch inputs at once, so skip the next iteration */
                    // row_idx += 1;
                }
            }
        }

        /* unpack outputs */
        for packet_idx in 0..self.t.num_packets {
            self.t.unpack_outputs[packet_idx].generate_r1cs_witness_from_packed();
        }
    }
}

pub fn test_as_waksman_routing_gadget<FieldT: FieldTConfig, PB: PBConfig>(
    num_packets: usize,
    packet_size: usize,
) {
    print!(
        "testing as_waksman_routing_gadget by routing {} element vector of {} bits (Fp fits all {} bit integers)\n",
        num_packets,
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
                if (rand::random::<usize>() % 2 != 0) {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
        }
    }
    print_time("generated bits to be routed");

    let mut r = as_waksman_routing_gadget::<FieldT, PB>::new(
        pb.clone(),
        num_packets,
        randbits.clone(),
        outbits.clone(),
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
