/* NOTE: all examples here actually generate one constraint less to account for soundness constraint in QAP */
use crate::gadgetlib1::gadgets::basic_gadgets::inner_product_gadget;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::r1cs_example;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use rccell::RcCell;

pub fn gen_r1cs_example_from_protoboard<FieldT: FieldTConfig, PB: PBConfig>(
    num_constraints: usize,
) -> r1cs_example<FieldT, pb_variable, pb_linear_combination> {
    let new_num_constraints = num_constraints - 1;

    /* construct dummy example: inner products of two vectors */
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut A = pb_variable_array::<FieldT, PB>::default();
    let mut B = pb_variable_array::<FieldT, PB>::default();
    let mut res = variable::<FieldT, pb_variable>::default();

    // the variables on the protoboard are (ONE (constant 1 term), res, A[0], ..., A[num_constraints-1], B[0], ..., B[num_constraints-1])
    res.allocate(&pb, "res".to_owned());
    A.allocate(&pb, new_num_constraints, "A");
    B.allocate(&pb, new_num_constraints, "B");

    let mut compute_inner_product = inner_product_gadget::<FieldT, PB>::new(
        pb.clone(),
        A.clone().into(),
        B.clone().into(),
        res,
        "compute_inner_product".to_owned(),
    );
    compute_inner_product.generate_r1cs_constraints();

    /* fill in random example */
    for i in 0..new_num_constraints {
        *pb.borrow_mut().val_ref(&A[i]) = FieldT::random_element();
        *pb.borrow_mut().val_ref(&B[i]) = FieldT::random_element();
    }

    compute_inner_product.generate_r1cs_witness();
    r1cs_example::<FieldT, pb_variable, pb_linear_combination>::new(
        pb.borrow().get_constraint_system(),
        pb.borrow().primary_input(),
        pb.borrow().auxiliary_input(),
    )
}
