// Declaration of interfaces for an auxiliarry gadget for the FOORAM CPU.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::packing_gadget;
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use rccell::RcCell;
use std::marker::PhantomData;
/**
 * The bar gadget checks linear combination
 *                   Z = aX + bY (mod 2^w)
 * for a, b - const, X, Y - vectors of w bits,
 * where w is implicitly inferred, Z - a packed variable.
 *
 * This gadget is used four times in fooram:
 * - PC' = PC + 1
 * - load_addr = 2 * x + PC'
 * - store_addr = x + PC
 */
#[derive(Clone, Default)]
pub struct bar_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<FieldT>
    X: pb_linear_combination_array<FieldT, PB>,
    a: FieldT,
    Y: pb_linear_combination_array<FieldT, PB>,
    b: FieldT,
    Z_packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    Z_bits: pb_variable_array<FieldT, PB>,

    result: variable<FieldT, pb_variable>,
    overflow: pb_variable_array<FieldT, PB>,
    unpacked_result: pb_variable_array<FieldT, PB>,

    unpack_result: RcCell<gadget<FieldT, PB, packing_gadget<FieldT, PB>>>,
    pack_Z: RcCell<gadget<FieldT, PB, packing_gadget<FieldT, PB>>>,

    width: usize,
    // bar_gadget(pb:RcCell<protoboard<FieldT>>,
    //            X:&pb_linear_combination_array<FieldT>,
    //            a:&FieldT,
    //            Y:&pb_linear_combination_array<FieldT>,
    //            b:&FieldT,
    //            Z_packed:&linear_combination<FieldT,pb_variable,pb_linear_combination>,,
    //            annotation_prefix:&String);
    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness();
}

impl<FieldT: FieldTConfig, PB: PBConfig> bar_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        X: pb_linear_combination_array<FieldT, PB>,
        a: FieldT,
        Y: pb_linear_combination_array<FieldT, PB>,
        b: FieldT,
        Z_packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        assert!(X.len() == Y.len());
        let width = X.len();
        let mut result = variable::<FieldT, pb_variable>::default();
        result.allocate(&pb, prefix_format!(annotation_prefix, " result"));
        let mut Z_bits = pb_variable_array::<FieldT, PB>::default();
        Z_bits.allocate(&pb, width, prefix_format!(annotation_prefix, " Z_bits"));
        let mut overflow = pb_variable_array::<FieldT, PB>::default();
        overflow.allocate(
            &pb,
            2 * width,
            prefix_format!(annotation_prefix, " overflow"),
        );
        let mut unpacked_result = pb_variable_array::<FieldT, PB>::default();
        unpacked_result.contents.extend(Z_bits.clone());
        unpacked_result.contents.extend(overflow.clone());

        let unpack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            unpacked_result.clone().into(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " unpack_result"),
        ));
        let pack_Z = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            Z_bits.clone().into(),
            Z_packed.clone().into(),
            prefix_format!(annotation_prefix, " pack_Z"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                width,
                result,
                unpack_result,
                X,
                a,
                Y,
                b,
                Z_packed,
                pack_Z,
                overflow,
                Z_bits,
                unpacked_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, bar_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .unpack_result
            .borrow()
            .generate_r1cs_constraints(true);
        self.t.pack_Z.borrow().generate_r1cs_constraints(false);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                pb_packing_sum::<FieldT, PB>(&self.t.X) * self.t.a.clone()
                    + pb_packing_sum::<FieldT, PB>(&self.t.Y) * self.t.b.clone(),
                self.t.result.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " compute_result"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.result) =
            self.t.X.get_field_element_from_bits(&self.pb) * self.t.a.clone()
                + self.t.Y.get_field_element_from_bits(&self.pb) * self.t.b.clone();
        self.t
            .unpack_result
            .borrow()
            .generate_r1cs_witness_from_packed();

        self.t.pack_Z.borrow().generate_r1cs_witness_from_bits();
    }
}
