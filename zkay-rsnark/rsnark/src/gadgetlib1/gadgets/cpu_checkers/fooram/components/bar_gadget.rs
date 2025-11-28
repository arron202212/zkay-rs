// Declaration of interfaces for an auxiliarry gadget for the FOORAM CPU.

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;

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

pub struct bar_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<FieldT>
    X: pb_linear_combination_array<FieldT>,
    a: FieldT,
    Y: pb_linear_combination_array<FieldT>,
    b: FieldT,
    Z_packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    Z_bits: pb_variable_array<FieldT>,

    result: variable<FieldT, pb_variable>,
    overflow: pb_variable_array<FieldT>,
    unpacked_result: pb_variable_array<FieldT>,

    unpack_result: RcCell<gadget<FieldT, packing_gadget<FieldT>>>,
    pack_Z: RcCell<gadget<FieldT, packing_gadget<FieldT>>>,

    width: usize,
    _pb: PhantomData<PB>,
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

impl<FieldT: FieldTConfig, PB: PBConfig> bar_gadget<FieldT,PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT>>,
        X: pb_linear_combination_array<FieldT>,
        a: FieldT,
        Y: pb_linear_combination_array<FieldT>,
        b: FieldT,
        Z_packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> gadget<FieldT,PB, Self> {
        assert!(X.len() == Y.len());
        let width = X.len();
        let mut result = variable::<FieldT, pb_variable>::default();
        result.allocate(&pb, prefix_format!(annotation_prefix, " result"));
        let mut Z_bits = pb_variable_array::<FieldT>::default();
        Z_bits.allocate(&pb, width, prefix_format!(annotation_prefix, " Z_bits"));
        let mut overflow = pb_variable_array::<FieldT>::default();
        overflow.allocate(
            &pb,
            2 * width,
            prefix_format!(annotation_prefix, " overflow"),
        );
        let mut unpacked_result = pb_variable_array::<FieldT>::default();
        unpacked_result.contents.extend(Z_bits);
        unpacked_result.contents.extend(overflow);

        let unpack_result = RcCell::new(packing_gadget::<FieldT>(
            &pb,
            unpacked_result.clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " unpack_result"),
        ));
        let pack_Z = RcCell::new(packing_gadget::<FieldT>::new(
            pb,
            Z_bits.clone(),
            Z_packed.clone(),
            prefix_format!(annotation_prefix, " pack_Z"),
        ));
        gadget::<FieldT,PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                a,
                Y,
                b,
                Z_packed,
                _pb: PhantomData,
            },
        )
    }
}
impl<FieldT: FieldTConfig> gadget<FieldT, bar_gadget<FieldT>> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.unpack_result.generate_r1cs_constraints(true);
        self.t.pack_Z.generate_r1cs_constraints(false);

        self.pb.borrow().add_r1cs_constraint(
            r1cs_constraint::<FieldT>::new(
                1,
                self.t.a * pb_packing_sum::<FieldT>(self.t.X)
                    + self.t.b * pb_packing_sum::<FieldT>(self.t.Y),
                self.t.result,
            ),
            prefix_format!(self.annotation_prefix, " compute_result"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(self.t.result) =
            self.t.X.get_field_element_from_bits(self.pb) * self.t.a
                + self.t.Y.get_field_element_from_bits(self.pb) * self.t.b;
        self.t.unpack_result.generate_r1cs_witness_from_packed();

        self.t.pack_Z.generate_r1cs_witness_from_bits();
    }
}

//#endif // BAR_GADGET_TCC_
