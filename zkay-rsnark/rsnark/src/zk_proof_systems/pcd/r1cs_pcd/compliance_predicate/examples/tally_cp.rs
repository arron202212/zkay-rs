/** @file
*****************************************************************************

Declaration of interfaces for the tally compliance predicate.

The tally compliance predicate has two purposes:
(1) it exemplifies the use of interfaces declared in cp_handler.hpp, and
(2) it enables us to test r1cs_pcd functionalities.

See
- libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/examples/run_r1cs_sp_ppzkpcd.hpp
- libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/examples/run_r1cs_mp_ppzkpcd.hpp
for code that uses the tally compliance predicate.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TALLY_CP_HPP_
// #define TALLY_CP_HPP_
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;

/**
 * Subclasses a R1CS PCD message to the tally compliance predicate.
 */
//
pub struct tally_pcd_message<FieldT> {
    //: public r1cs_pcd_message<FieldT>
    wordsize: usize,

    sum: usize,
    count: usize,
}

//
pub struct tally_pcd_local_data<FieldT> {
    //  : public r1cs_pcd_local_data<FieldT>
    summand: usize,
}

/**
 * Subclass a R1CS compliance predicate handler to the tally compliance predicate handler.
 */
type base_handler = compliance_predicate_handler<FieldT, RcCell<protoboard<FieldT>>>;
pub struct tally_cp_handler<FieldT> {
    // /: public compliance_predicate_handler<FieldT, RcCell<protoboard<FieldT>> >
    incoming_types: pb_variable_array<FieldT>,

    sum_out_packed: pb_variable<FieldT>,
    count_out_packed: pb_variable<FieldT>,
    sum_in_packed: pb_variable_array<FieldT>,
    count_in_packed: pb_variable_array<FieldT>,

    sum_in_packed_aux: pb_variable_array<FieldT>,
    count_in_packed_aux: pb_variable_array<FieldT>,

    unpack_sum_out: RcCell<packing_gadget<FieldT>>,
    unpack_count_out: RcCell<packing_gadget<FieldT>>,
    pack_sum_in: Vec<packing_gadget<FieldT>>,
    pack_count_in: Vec<packing_gadget<FieldT>>,

    type_val_inner_product: pb_variable<FieldT>,
    compute_type_val_inner_product: RcCell<inner_product_gadget<FieldT>>,

    arity_indicators: pb_variable_array<FieldT>,

    wordsize: usize,
    message_length: usize,
}

// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp;

//#endif // TALLY_CP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for the tally compliance predicate.

See tally_cp.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TALLY_CP_TCC_
// #define TALLY_CP_TCC_

// use  <algorithm>
// use  <functional>
use ffec::algebra::field_utils::field_utils;

impl tally_pcd_message<FieldT> {
    pub fn new(types: usize, wordsize: usize, sum: usize, count: usize) -> Self {
        // r1cs_pcd_message<FieldT>(types),
        Self {
            wordsize,
            sum,
            count,
        }
    }

    pub fn payload_as_r1cs_variable_assignment() -> r1cs_variable_assignment<FieldT> {
        fn bit_to_FieldT<FieldT>(bit: bool) {
            if bit { FieldT::one() } else { FieldT::zero() }
        }

        let sum_bits = ffec::convert_field_element_to_bit_vector::<FieldT>(sum, wordsize);
        let count_bits = ffec::convert_field_element_to_bit_vector::<FieldT>(count, wordsize);

        let mut result = r1cs_variable_assignment::<FieldT>::new(2 * wordsize);
        std::transform(
            sum_bits.begin(),
            sum_bits.end(),
            result.begin(),
            bit_to_FieldT,
        );
        std::transform(
            count_bits.begin(),
            count_bits.end(),
            result.begin() + wordsize,
            bit_to_FieldT,
        );

        return result;
    }

    pub fn print() {
        print!("Tally message of.types {}:\n", self.types);
        print!("  wordsize: {}\n", wordsize);
        print!("  sum: {}\n", sum);
        print!("  count: {}\n", count);
    }
}
impl tally_pcd_local_data<FieldT> {
    pub fn new(summand: usize) -> Self {
        Self { summand }
    }

    pub fn as_r1cs_variable_assignment() -> r1cs_variable_assignment<FieldT> {
        let result = r1cs_variable_assignment::<FieldT>([FieldT(summand)]);
        return result;
    }

    pub fn print() {
        print!("Tally PCD local data:\n");
        print!("  summand: {}\n", summand);
    }
}

//
pub struct tally_pcd_message_variable<FieldT> {
    // : public r1cs_pcd_message_variable<FieldT>
    sum_bits: pb_variable_array<FieldT>,
    count_bits: pb_variable_array<FieldT>,
    wordsize: usize,
}
impl tally_pcd_message_variable<FieldT> {
    pub fn new(
        pb: &RcCell<protoboard<FieldT>>,
        wordsize: usize,
        annotation_prefix: &String,
    ) -> Self {
        sum_bits.allocate(&pb, wordsize, FMT(annotation_prefix, " sum_bits"));
        count_bits.allocate(&pb, wordsize, FMT(annotation_prefix, " count_bits"));

        self.update_all_vars();
        //r1cs_pcd_message_variable<FieldT>(&pb, annotation_prefix),
        Self { wordsize }
    }

    pub fn get_message() -> RcCell<r1cs_pcd_message<FieldT>> {
        let type_val = self.pb.borrow().val(&self.types).as_ulong();
        let sum_val = sum_bits.get_field_element_from_bits(self.pb).as_ulong();
        let count_val = count_bits.get_field_element_from_bits(self.pb).as_ulong();

        let mut result = r1cs_pcd_message::<FieldT>::new();
        result = RcCell::new(tally_pcd_message::<FieldT>::new(
            type_val, wordsize, sum_val, count_val,
        ));
        return result;
    }
}

//
pub struct tally_pcd_local_data_variable<FieldT> {
    // : public r1cs_pcd_local_data_variable
    summand: pb_variable<FieldT>,
}
impl tally_pcd_local_data_variable<FieldT> {
    pub fn new(pb: &RcCell<protoboard<FieldT>>, annotation_prefix: &String) -> Self {
        summand.allocate(&pb, FMT(annotation_prefix, " summand"));

        self.update_all_vars();
        // r1cs_pcd_local_data_variable<FieldT>(&pb, annotation_prefix)
    }

    pub fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<FieldT>> {
        let summand_val = self.pb.borrow().val(&summand).as_ulong();

        let mut result = r1cs_pcd_local_data::<FieldT>::new();
        result = RcCell::new(tally_pcd_local_data::<FieldT>::new(summand_val));
        return result;
    }
}

impl tally_cp_handler<FieldT> {
    pub fn new(
        types: usize,
        max_arity: usize,
        wordsize: usize,
        relies_on_same_type_inputs: bool,
        accepted_input_types: BTreeSet<usize>,
    ) -> Self {
        self.outgoing_message = RcCell::new(tally_pcd_message_variable::<FieldT>::new(
            self.pb,
            wordsize,
            "outgoing_message",
        ));
        self.arity.allocate(self.pb, "arity");

        for i in 0..max_arity {
            self.incoming_messages[i] = RcCell::new(tally_pcd_message_variable::<FieldT>::new(
                self.pb,
                wordsize,
                FMT("", "incoming_messages_{}", i),
            ));
        }

        self.local_data = RcCell::new(tally_pcd_local_data_variable::<FieldT>::new(
            self.pb,
            "local_data",
        ));

        sum_out_packed.allocate(self.pb, "sum_out_packed");
        count_out_packed.allocate(self.pb, "count_out_packed");

        sum_in_packed.allocate(self.pb, max_arity, "sum_in_packed");
        count_in_packed.allocate(self.pb, max_arity, "count_in_packed");

        sum_in_packed_aux.allocate(self.pb, max_arity, "sum_in_packed_aux");
        count_in_packed_aux.allocate(self.pb, max_arity, "count_in_packed_aux");

        type_val_inner_product.allocate(self.pb, "type_val_inner_product");
        for msg in &self.incoming_messages {
            incoming_types.push(msg.types);
        }

        compute_type_val_inner_product = RcCell::new(inner_product_gadget::<FieldT>::new(
            self.pb,
            incoming_types,
            sum_in_packed,
            type_val_inner_product,
            "compute_type_val_inner_product",
        ));

        unpack_sum_out = RcCell::new(packing_gadget::<FieldT>::new(
            self.pb,
            tally_pcd_message_variable::<FieldT>(self.outgoing_message).sum_bits,
            sum_out_packed,
            "pack_sum_out",
        ));
        unpack_count_out = RcCell::new(packing_gadget::<FieldT>::new(
            self.pb,
            tally_pcd_message_variable::<FieldT>(self.outgoing_message).count_bits,
            count_out_packed,
            "pack_count_out",
        ));

        for i in 0..max_arity {
            pack_sum_in.push(packing_gadget::<FieldT>(
                self.pb,
                tally_pcd_message_variable::<FieldT>(self.incoming_messages[i]).sum_bits,
                sum_in_packed[i],
                FMT("", "pack_sum_in_{}", i),
            ));
            pack_count_in.push(packing_gadget::<FieldT>(
                self.pb,
                tally_pcd_message_variable::<FieldT>(self.incoming_messages[i]).sum_bits,
                count_in_packed[i],
                FMT("", "pack_count_in_{}", i),
            ));
        }

        arity_indicators.allocate(self.pb, max_arity + 1, "arity_indicators");
        // compliance_predicate_handler::<FieldT, protoboard::<FieldT> >(protoboard::<FieldT>(),
        //                                                          .types*100,
        //                                                          .types,
        //                                                           max_arity,
        //                                                           relies_on_same_type_inputs,
        //                                                           accepted_input_types),
        Self { wordsize }
    }

    pub fn generate_r1cs_constraints() {
        unpack_sum_out.generate_r1cs_constraints(true);
        unpack_count_out.generate_r1cs_constraints(true);

        for i in 0..self.max_arity {
            pack_sum_in[i].generate_r1cs_constraints(true);
            pack_count_in[i].generate_r1cs_constraints(true);
        }

        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT>(
                    incoming_types[i],
                    sum_in_packed_aux[i],
                    sum_in_packed[i],
                ),
                FMT("", "initial_sum_{}_is_zero", i),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT>(
                    incoming_types[i],
                    count_in_packed_aux[i],
                    count_in_packed[i],
                ),
                FMT("", "initial_sum_{}_is_zero", i),
            );
        }

        /* constrain arity indicator variables so that arity_indicators[arity] = 1 and arity_indicators[i] = 0 for any other i */
        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT>(self.arity - FieldT(i), arity_indicators[i], 0),
                FMT("", "arity_indicators_{}", i),
            );
        }

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(1, pb_sum::<FieldT>(arity_indicators), 1),
            "arity_indicators",
        );

        /* require that types of messages that are past arity (i.e. unbound wires) carry 0 */
        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT>(
                    0 + pb_sum::<FieldT>(pb_variable_array::<FieldT>(
                        arity_indicators.begin(),
                        arity_indicators.begin() + i,
                    )),
                    incoming_types[i],
                    0,
                ),
                FMT("", "unbound_types_{}", i),
            );
        }

        /* sum_out = local_data + \sum_i.types[i] * sum_in[i] */
        compute_type_val_inner_product.generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(
                1,
                type_val_inner_product
                    + tally_pcd_local_data_variable::<FieldT>(self.local_data).summand,
                sum_out_packed,
            ),
            "update_sum",
        );

        /* count_out = 1 + \sum_i count_in[i] */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(1, 1 + pb_sum::<FieldT>(count_in_packed), count_out_packed),
            "update_count",
        );
    }

    pub fn generate_r1cs_witness(
        incoming_messages: &Vec<RcCell<r1cs_pcd_message<FieldT>>>,
        local_data: &RcCell<r1cs_pcd_local_data<FieldT>>,
    ) {
        base_handler::generate_r1cs_witness(incoming_messages, local_data);

        for i in 0..self.max_arity {
            pack_sum_in[i].generate_r1cs_witness_from_bits();
            pack_count_in[i].generate_r1cs_witness_from_bits();

            if !self.pb.borrow().val(&incoming_types[i]).is_zero() {
                self.pb.borrow().val(&sum_in_packed_aux[i]) =
                    self.pb.borrow().val(&sum_in_packed[i])
                        * self.pb.borrow().val(&incoming_types[i]).inverse();
                self.pb.borrow().val(&count_in_packed_aux[i]) =
                    self.pb.borrow().val(&count_in_packed[i])
                        * self.pb.borrow().val(&incoming_types[i]).inverse();
            }
        }

        for i in 0..self.max_arity + 1 {
            self.pb.borrow().val(&arity_indicators[i]) = if incoming_messages.len() == i {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }

        compute_type_val_inner_product.generate_r1cs_witness();
        self.pb.borrow().val(&sum_out_packed) = self
            .pb
            .borrow()
            .val(&tally_pcd_local_data_variable::<FieldT>(self.local_data).summand)
            + self.pb.borrow().val(&type_val_inner_product);

        self.pb.borrow().val(&count_out_packed) = FieldT::one();
        for i in 0..self.max_arity {
            self.pb.borrow().val(&count_out_packed) += self.pb.borrow().val(&count_in_packed[i]);
        }

        unpack_sum_out.generate_r1cs_witness_from_packed();
        unpack_count_out.generate_r1cs_witness_from_packed();
    }

    pub fn get_base_case_message() -> RcCell<r1cs_pcd_message<FieldT>> {
        let types = 0;
        let sum = 0;
        let count = 0;

        let result = r1cs_pcd_message::<FieldT>::new();
        result = RcCell::new(tally_pcd_message::<FieldT>::new(
            types, wordsize, sum, count,
        ));

        return result;
    }
}

//#endif // TALLY_CP_TCC_
