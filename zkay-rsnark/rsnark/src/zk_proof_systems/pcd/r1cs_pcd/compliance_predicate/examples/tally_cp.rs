// Declaration of interfaces for the tally compliance predicate.

// The tally compliance predicate has two purposes:
// (1) it exemplifies the use of interfaces declared in cp_handler.hpp, and
// (2) it enables us to test r1cs_pcd functionalities.

// See
// - libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/examples/run_r1cs_sp_ppzkpcd.hpp
// - libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/examples/run_r1cs_mp_ppzkpcd.hpp
// for code that uses the tally compliance predicate.

// use crate::gadgetlib1::gadgets::basic_gadgets;
// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;
// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;
// use ffec::algebra::field_utils::field_utils;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    inner_product_gadget, inner_product_gadgets, packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::pb_linear_combination;
use crate::gadgetlib1::pb_variable::pb_sum;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::gadgetlib2::variable::sum;
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig, r1cs_pcd_local_data, r1cs_pcd_message,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::LocalDataVariableConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::MessageVariableConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::{
    CPHConfig, compliance_predicate_handler, r1cs_pcd_local_data_variable,
    r1cs_pcd_local_data_variables, r1cs_pcd_message_variable, r1cs_pcd_message_variables,
};
use ffec::FieldTConfig;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector1;
use ffec::{One, Zero};
use rccell::RcCell;
use std::collections::BTreeSet;
use std::marker::PhantomData;

pub trait TallyCPHConfig:
    CPHConfig<
        protoboardT = Self::protoboard_type,
        M = tally_pcd_message<<Self as ppTConfig>::FieldT>,
        LD = tally_pcd_local_data<<Self as ppTConfig>::FieldT>,
        LDV = tally_pcd_local_data_variable<Self>,
    >
{
    type protoboard_type: ProtoboardConfig<FieldT = <Self as ppTConfig>::FieldT, PB = <Self as ppTConfig>::PB>;
}

/**
 * Subclasses a R1CS PCD message to the tally compliance predicate.
 */
#[derive(Default, Clone)]
pub struct tally_pcd_message<FieldT: FieldTConfig> {
    //: public r1cs_pcd_message<FieldT>
    wordsize: usize,

    sum: usize,
    count: usize,
    _t: PhantomData<FieldT>,
}

#[derive(Default, Clone)]
pub struct tally_pcd_local_data<FieldT: FieldTConfig> {
    //  : public r1cs_pcd_local_data<FieldT>
    summand: usize,
    _t: PhantomData<FieldT>,
}

/**
 * Subclass a R1CS compliance predicate handler to the tally compliance predicate handler.
 */
type base_handler<CPH> = compliance_predicate_handler<
    CPH,
    RcCell<protoboard<<CPH as ppTConfig>::FieldT, <CPH as ppTConfig>::PB>>,
>;

#[derive(Default, Clone)]
pub struct tally_cp_handler<CPH: TallyCPHConfig> {
    // /: public compliance_predicate_handler<FieldT, RcCell<protoboard<FieldT>> >
    incoming_types: pb_variable_array<CPH::FieldT, CPH::PB>,

    sum_out_packed: variable<CPH::FieldT, pb_variable>,
    count_out_packed: variable<CPH::FieldT, pb_variable>,
    sum_in_packed: pb_variable_array<CPH::FieldT, CPH::PB>,
    count_in_packed: pb_variable_array<CPH::FieldT, CPH::PB>,

    sum_in_packed_aux: pb_variable_array<CPH::FieldT, CPH::PB>,
    count_in_packed_aux: pb_variable_array<CPH::FieldT, CPH::PB>,

    unpack_sum_out: RcCell<packing_gadgets<CPH::FieldT, CPH::PB>>,
    unpack_count_out: RcCell<packing_gadgets<CPH::FieldT, CPH::PB>>,
    pack_sum_in: Vec<packing_gadgets<CPH::FieldT, CPH::PB>>,
    pack_count_in: Vec<packing_gadgets<CPH::FieldT, CPH::PB>>,

    type_val_inner_product: variable<CPH::FieldT, pb_variable>,
    compute_type_val_inner_product: RcCell<inner_product_gadgets<CPH::FieldT, CPH::PB>>,

    arity_indicators: pb_variable_array<CPH::FieldT, CPH::PB>,

    wordsize: usize,
    message_length: usize,
}

pub type tally_pcd_messages<FieldT> = r1cs_pcd_message<FieldT, tally_pcd_message<FieldT>>;
impl<FieldT: FieldTConfig> tally_pcd_message<FieldT> {
    pub fn new(
        types: usize,
        wordsize: usize,
        sum: usize,
        count: usize,
    ) -> tally_pcd_messages<FieldT> {
        r1cs_pcd_message::<FieldT, Self>::new(
            types,
            Self {
                wordsize,
                sum,
                count,
                _t: PhantomData,
            },
        )
    }
}
impl<FieldT: FieldTConfig> MessageConfig for tally_pcd_message<FieldT> {
    type FieldT = FieldT;
    fn payload_as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<FieldT> {
        let sum_bits =
            convert_field_element_to_bit_vector1::<FieldT>(&FieldT::from(self.sum), self.wordsize);
        let count_bits = convert_field_element_to_bit_vector1::<FieldT>(
            &FieldT::from(self.count),
            self.wordsize,
        );

        let mut result: Vec<_> = sum_bits
            .into_iter()
            .chain(count_bits)
            .map(|bit| if bit { FieldT::one() } else { FieldT::zero() })
            .collect();

        result
    }
}

impl<FieldT: FieldTConfig> tally_pcd_messages<FieldT> {
    pub fn prints(&self) {
        print!("Tally message of.types {}:\n", self.types);
        print!("  wordsize: {}\n", self.t.wordsize);
        print!("  sum: {}\n", self.t.sum);
        print!("  count: {}\n", self.t.count);
    }
}

pub type tally_pcd_local_datas<FieldT> = r1cs_pcd_local_data<FieldT, tally_pcd_local_data<FieldT>>;
impl<FieldT: FieldTConfig> tally_pcd_local_data<FieldT> {
    pub fn new(summand: usize) -> tally_pcd_local_datas<FieldT> {
        r1cs_pcd_local_data::<FieldT, Self>::new(Self {
            summand,
            _t: PhantomData,
        })
    }
}

impl<FieldT: FieldTConfig> LocalDataConfig for tally_pcd_local_data<FieldT> {
    type FieldT = FieldT;
    fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<FieldT> {
        vec![FieldT::from(self.summand)]
    }
}
impl<FieldT: FieldTConfig> tally_pcd_local_data<FieldT> {
    pub fn print(&self) {
        print!("Tally PCD local data:\n");
        print!("  summand: {}\n", self.summand);
    }
}

#[derive(Default, Clone)]
pub struct tally_pcd_message_variable<CPH: TallyCPHConfig> {
    // : public r1cs_pcd_message_variable<FieldT>
    sum_bits: pb_variable_array<CPH::FieldT, CPH::PB>,
    count_bits: pb_variable_array<CPH::FieldT, CPH::PB>,
    wordsize: usize,
}

pub type tally_pcd_message_variables<CPH> =
    r1cs_pcd_message_variables<tally_pcd_message_variable<CPH>>;
impl<CPH: TallyCPHConfig> tally_pcd_message_variable<CPH> {
    pub fn new(
        pb: RcCell<protoboard<CPH::FieldT, CPH::PB>>,
        wordsize: usize,
        annotation_prefix: String,
    ) -> tally_pcd_message_variables<CPH> {
        let mut sum_bits = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        let mut count_bits = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        sum_bits.allocate(
            &pb,
            wordsize,
            prefix_format!(annotation_prefix, " sum_bits"),
        );
        count_bits.allocate(
            &pb,
            wordsize,
            prefix_format!(annotation_prefix, " count_bits"),
        );

        let mut _self = r1cs_pcd_message_variable::<Self>::new(
            pb,
            annotation_prefix,
            Self {
                sum_bits,
                count_bits,
                wordsize,
            },
        );
        _self.update_all_vars();
        _self
    }
}

impl<CPH: TallyCPHConfig> MessageVariableConfig for tally_pcd_message_variable<CPH> {
    type FieldT = CPH::FieldT;
    type PB = CPH::PB;
    type Output = tally_pcd_message<CPH::FieldT>;
    fn get_message(&self) -> RcCell<tally_pcd_messages<CPH::FieldT>> {
        panic!("tally_pcd_message_variable");
    }
}
impl<CPH: TallyCPHConfig> MessageVariableConfig for tally_pcd_message_variables<CPH> {
    type FieldT = CPH::FieldT;
    type PB = CPH::PB;
    type Output = tally_pcd_message<CPH::FieldT>;
    fn get_message(&self) -> RcCell<tally_pcd_messages<CPH::FieldT>> {
        let type_val = self.pb.borrow().val(&self.t.types).as_ulong();
        let sum_val = self
            .t
            .t
            .sum_bits
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        let count_val = self
            .t
            .t
            .count_bits
            .get_field_element_from_bits(&self.pb)
            .as_ulong();

        let mut result = RcCell::new(tally_pcd_message::<CPH::FieldT>::new(
            type_val,
            self.t.t.wordsize,
            sum_val,
            count_val,
        ));
        result
    }
}

#[derive(Default, Clone)]
pub struct tally_pcd_local_data_variable<CPH: TallyCPHConfig> {
    // : public r1cs_pcd_local_data_variable
    summand: variable<CPH::FieldT, pb_variable>,
}
pub type tally_pcd_local_data_variables<CPH> =
    r1cs_pcd_local_data_variables<tally_pcd_local_data_variable<CPH>>;
impl<CPH: TallyCPHConfig> tally_pcd_local_data_variable<CPH> {
    pub fn new(
        pb: RcCell<protoboard<CPH::FieldT, CPH::PB>>,
        annotation_prefix: String,
    ) -> tally_pcd_local_data_variables<CPH> {
        let mut summand = variable::<CPH::FieldT, pb_variable>::default();
        summand.allocate(&pb, prefix_format!(annotation_prefix, " summand"));

        let mut _self =
            r1cs_pcd_local_data_variable::<Self>::new(pb, annotation_prefix, Self { summand });
        _self.update_all_vars();
        _self
    }
}
impl<CPH: TallyCPHConfig> LocalDataVariableConfig for tally_pcd_local_data_variable<CPH> {
    type FieldT = CPH::FieldT;
    type PB = CPH::PB;
    type Output = tally_pcd_local_data<CPH::FieldT>;
    fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<Self::FieldT, Self::Output>> {
        // let summand_val = self.pb.borrow().val(&self.t.summand).as_ulong();

        // RcCell::new(tally_pcd_local_data::<CPH::FieldT>::new(summand_val))
        panic!("tally_pcd_local_data_variable")
    }
}
impl<CPH: TallyCPHConfig> tally_pcd_local_data_variables<CPH> {
    //  type FieldT = CPH::FieldT;
    fn get_local_data(
        &self,
    ) -> RcCell<r1cs_pcd_local_data<CPH::FieldT, tally_pcd_local_data<CPH::FieldT>>> {
        let summand_val = self.pb.borrow().val(&self.t.t.summand).as_ulong();

        RcCell::new(tally_pcd_local_data::<CPH::FieldT>::new(summand_val))
    }
}
pub type tally_cp_handlers<CPH> = compliance_predicate_handler<CPH, tally_cp_handler<CPH>>;

impl<
    CPH: TallyCPHConfig<
        protoboard_type = protoboard<<CPH as ppTConfig>::FieldT, <CPH as ppTConfig>::PB>,
    >,
> tally_cp_handler<CPH>
{
    pub fn new(
        types: usize,
        max_arity: usize,
        wordsize: usize,
        relies_on_same_type_inputs: bool,
        accepted_input_types: BTreeSet<usize>,
    ) -> tally_cp_handlers<CPH> {
        let mut pb = RcCell::new(protoboard::<CPH::FieldT, CPH::PB>::default());
        let outgoing_message = RcCell::new(tally_pcd_message_variable::<CPH>::new(
            pb.clone(),
            wordsize,
            "outgoing_message".to_owned(),
        ));
        let mut arity = variable::<CPH::FieldT, pb_variable>::default();
        arity.allocate(&pb, "arity");
        let mut incoming_messages = Vec::with_capacity(max_arity);
        for i in 0..max_arity {
            incoming_messages.push(RcCell::new(tally_pcd_message_variable::<CPH>::new(
                pb.clone(),
                wordsize,
                prefix_format!("", "incoming_messages_{}", i),
            )));
        }

        let local_data = RcCell::new(tally_pcd_local_data_variable::<CPH>::new(
            pb.clone(),
            "local_data".to_owned(),
        ));

        let mut sum_out_packed = variable::<CPH::FieldT, pb_variable>::default();
        let mut count_out_packed = variable::<CPH::FieldT, pb_variable>::default();
        let mut sum_in_packed = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        let mut count_in_packed = pb_variable_array::<CPH::FieldT, CPH::PB>::default();

        let mut sum_in_packed_aux = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        let mut count_in_packed_aux = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        let mut type_val_inner_product = variable::<CPH::FieldT, pb_variable>::default();
        let mut arity_indicators = pb_variable_array::<CPH::FieldT, CPH::PB>::default();

        sum_out_packed.allocate(&pb, "sum_out_packed");
        count_out_packed.allocate(&pb, "count_out_packed");

        sum_in_packed.allocate(&pb, max_arity, "sum_in_packed");
        count_in_packed.allocate(&pb, max_arity, "count_in_packed");

        sum_in_packed_aux.allocate(&pb, max_arity, "sum_in_packed_aux");
        count_in_packed_aux.allocate(&pb, max_arity, "count_in_packed_aux");

        type_val_inner_product.allocate(&pb, "type_val_inner_product");

        let mut incoming_types = pb_variable_array::<CPH::FieldT, CPH::PB>::default();
        for msg in &incoming_messages {
            incoming_types.contents.push(msg.borrow().t.types.clone());
        }

        let compute_type_val_inner_product =
            RcCell::new(inner_product_gadget::<CPH::FieldT, CPH::PB>::new(
                pb.clone(),
                incoming_types.clone().into(),
                sum_in_packed.clone().into(),
                type_val_inner_product.clone(),
                "compute_type_val_inner_product".to_owned(),
            ));

        let unpack_sum_out = RcCell::new(packing_gadget::<CPH::FieldT, CPH::PB>::new(
            pb.clone(),
            outgoing_message.borrow().t.t.sum_bits.clone().into(),
            sum_out_packed.clone().into(),
            "pack_sum_out".to_owned(),
        ));
        let unpack_count_out = RcCell::new(packing_gadget::<CPH::FieldT, CPH::PB>::new(
            pb.clone(),
            outgoing_message.borrow().t.t.count_bits.clone().into(),
            count_out_packed.clone().into(),
            "pack_count_out".to_owned(),
        ));
        let (mut pack_sum_in, mut pack_count_in) =
            (Vec::with_capacity(max_arity), Vec::with_capacity(max_arity));
        for i in 0..max_arity {
            pack_sum_in.push(packing_gadget::<CPH::FieldT, CPH::PB>::new(
                pb.clone(),
                incoming_messages[i].borrow().t.t.sum_bits.clone().into(),
                sum_in_packed[i].clone().into(),
                prefix_format!("", "pack_sum_in_{}", i),
            ));
            pack_count_in.push(packing_gadget::<CPH::FieldT, CPH::PB>::new(
                pb.clone(),
                incoming_messages[i].borrow().t.t.sum_bits.clone().into(),
                count_in_packed[i].clone().into(),
                prefix_format!("", "pack_count_in_{}", i),
            ));
        }

        arity_indicators.allocate(&pb, max_arity + 1, "arity_indicators");
        compliance_predicate_handler::<CPH, Self>::new(
            pb.borrow().clone().into_p(),
            types * 100,
            types,
            max_arity,
            relies_on_same_type_inputs,
            accepted_input_types,
            Self {
                incoming_types,
                sum_out_packed,
                count_out_packed,
                sum_in_packed,
                count_in_packed,

                sum_in_packed_aux,
                count_in_packed_aux,

                unpack_sum_out,
                unpack_count_out,
                pack_sum_in,
                pack_count_in,

                type_val_inner_product,
                compute_type_val_inner_product,

                arity_indicators,

                wordsize,
                message_length: 0,
            },
        )
    }
}
impl<CPH: TallyCPHConfig> tally_cp_handlers<CPH> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .unpack_sum_out
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .unpack_count_out
            .borrow()
            .generate_r1cs_constraints(true);

        for i in 0..self.max_arity {
            self.t.pack_sum_in[i].generate_r1cs_constraints(true);
            self.t.pack_count_in[i].generate_r1cs_constraints(true);
        }

        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.incoming_types[i].clone().into(),
                    self.t.sum_in_packed_aux[i].clone().into(),
                    self.t.sum_in_packed[i].clone().into(),
                ),
                prefix_format!("", "initial_sum_{}_is_zero", i),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.incoming_types[i].clone().into(),
                    self.t.count_in_packed_aux[i].clone().into(),
                    self.t.count_in_packed[i].clone().into(),
                ),
                prefix_format!("", "initial_sum_{}_is_zero", i),
            );
        }

        /* constrain arity indicator variables so that arity_indicators[arity] = 1 and arity_indicators[i] = 0 for any other i */
        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                    self.arity.clone() - linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(CPH::FieldT::from(i)),
                    self.t.arity_indicators[i].clone().into(),
                    linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(CPH::FieldT::from(0)),
                ),
                prefix_format!("", "arity_indicators_{}", i),
            );
        }

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    CPH::FieldT::from(1),
                ),
                pb_sum::<CPH::FieldT, CPH::PB, pb_variable>(
                    &(self.t.arity_indicators.clone().into()),
                ),
                linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    CPH::FieldT::from(1),
                ),
            ),
            "arity_indicators".to_owned(),
        );

        /* require that types of messages that are past arity (i.e. unbound wires) carry 0 */
        for i in 0..self.max_arity {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                    linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                        CPH::FieldT::from(0),
                    ) + pb_sum::<CPH::FieldT, CPH::PB, pb_variable>(
                        &(pb_variable_array::<CPH::FieldT, CPH::PB>::new(
                            self.t.arity_indicators.contents[..i].to_vec(),
                        )
                        .into()),
                    ),
                    self.t.incoming_types[i].clone().into(),
                    linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                        CPH::FieldT::from(0),
                    ),
                ),
                prefix_format!("", "unbound_types_{}", i),
            );
        }

        /* sum_out = local_data + \sum_i.types[i] * sum_in[i] */
        self.t
            .compute_type_val_inner_product
            .borrow()
            .generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    CPH::FieldT::from(1),
                ),
                (linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.type_val_inner_product.clone(),
                ) + self.local_data.borrow().t.t.summand.clone())
                .into(),
                self.t.sum_out_packed.clone().into(),
            ),
            "update_sum".to_owned(),
        );

        /* count_out = 1 + \sum_i count_in[i] */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<CPH::FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    CPH::FieldT::from(1),
                ),
                linear_combination::<CPH::FieldT, pb_variable, pb_linear_combination>::from(
                    CPH::FieldT::from(1),
                ) + pb_sum::<CPH::FieldT, CPH::PB, pb_variable>(
                    &(self.t.count_in_packed.clone().into()),
                ),
                self.t.count_out_packed.clone().into(),
            ),
            "update_count".to_owned(),
        );
    }

    pub fn generate_r1cs_witness(
        &mut self,
        incoming_messages: &Vec<RcCell<tally_pcd_messages<CPH::FieldT>>>,
        local_data: &RcCell<tally_pcd_local_datas<CPH::FieldT>>,
    ) {
        self.generate_r1cs_witness_base(incoming_messages, local_data);

        for i in 0..self.max_arity {
            self.t.pack_sum_in[i].generate_r1cs_witness_from_bits();
            self.t.pack_count_in[i].generate_r1cs_witness_from_bits();

            if !self.pb.borrow().val(&self.t.incoming_types[i]).is_zero() {
                *self.pb.borrow_mut().val_ref(&self.t.sum_in_packed_aux[i]) =
                    self.pb.borrow().val(&self.t.sum_in_packed[i])
                        * self.pb.borrow().val(&self.t.incoming_types[i]).inverse();
                *self.pb.borrow_mut().val_ref(&self.t.count_in_packed_aux[i]) =
                    self.pb.borrow().val(&self.t.count_in_packed[i])
                        * self.pb.borrow().val(&self.t.incoming_types[i]).inverse();
            }
        }

        for i in 0..self.max_arity + 1 {
            *self.pb.borrow_mut().val_ref(&self.t.arity_indicators[i]) =
                if incoming_messages.len() == i {
                    CPH::FieldT::one()
                } else {
                    CPH::FieldT::zero()
                };
        }

        self.t
            .compute_type_val_inner_product
            .borrow()
            .generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.sum_out_packed) = self
            .pb
            .borrow()
            .val(&(local_data.borrow().t.summand.into()))
            + self.pb.borrow().val(&self.t.type_val_inner_product);

        *self.pb.borrow_mut().val_ref(&self.t.count_out_packed) = CPH::FieldT::one();
        for i in 0..self.max_arity {
            *self.pb.borrow_mut().val_ref(&self.t.count_out_packed) +=
                self.pb.borrow().val(&self.t.count_in_packed[i]);
        }

        self.t
            .unpack_sum_out
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .unpack_count_out
            .borrow()
            .generate_r1cs_witness_from_packed();
    }

    pub fn get_base_case_message(&self) -> RcCell<tally_pcd_messages<CPH::FieldT>> {
        let types = 0;
        let sum = 0;
        let count = 0;

        RcCell::new(tally_pcd_message::<CPH::FieldT>::new(
            types,
            self.t.wordsize,
            sum,
            count,
        ))
    }
}
