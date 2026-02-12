use ffec::common::default_types::ec_pp;

use super::super::pp::initPublicParamsFromDefaultPp;
use crate::gadgetlib2::adapters::{GLA, GadgetLibAdapter};
use crate::gadgetlib2::gadget::{GadgetConfig, InnerProduct_Gadget};
use crate::gadgetlib2::integration::{
    get_constraint_system_from_gadgetlib2, get_variable_assignment_from_gadgetlib2,
};
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{FieldType, Variable, VariableArray, VariableArrayBase};
use crate::relations::constraint_satisfaction_problems::r1cs::{
    examples::r1cs_examples::r1cs_example,
    r1cs::{r1cs_auxiliary_input, r1cs_primary_input},
};
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig};
use ff_curves::{Fr, default_ec_pp};
use ffec::FieldTConfig;
type FieldT = Fr<default_ec_pp>;

/* NOTE: all examples here actually generate one constraint less to account for soundness constraint in QAP */
pub fn gen_r1cs_example_from_gadgetlib2_protoboard<
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    size: usize,
) -> r1cs_example<FieldT, SV, SLC> {
    // type FieldT=ffec::Fr<ffec::default_ec_pp> ;

    initPublicParamsFromDefaultPp();
    // necessary in case a protoboard was built before,  libsnark assumes variable indices always
    // begin with 0 so we must reset the index before creating constraints which will be used by
    // libsnark
    GLA::resetVariableIndex();

    // create a gadgetlib2 gadget. This part is done by both generator and prover.
    let mut pb = Protoboard::create(FieldType::R1P, None);
    let A = VariableArray::<VariableArrayBase>::new(size, "A".to_owned(), VariableArrayBase);
    let B = VariableArray::<VariableArrayBase>::new(size, "B".to_owned(), VariableArrayBase);
    let result = Variable::from("result");
    let mut g = InnerProduct_Gadget::create(pb.clone(), A.clone().into(), B.clone().into(), result);
    // create constraints. This part is done by generator.
    g.borrow().generateConstraints();
    // create assignment (witness). This part is done by prover.

    for k in 0..size {
        *pb.as_mut().unwrap().borrow_mut().val(&A[k]) = (rand::random::<i32>() % 2).into();
        *pb.as_mut().unwrap().borrow_mut().val(&B[k]) = (rand::random::<i32>() % 2).into();
    }
    g.borrow().generateWitness();
    // translate constraint system to libsnark format.
    let cs = get_constraint_system_from_gadgetlib2::<SV, SLC>(&pb.as_ref().unwrap().borrow());
    // translate full variable assignment to libsnark format
    let full_assignment =
        get_variable_assignment_from_gadgetlib2::<SV, SLC>(&pb.as_ref().unwrap().borrow());
    // extract primary and auxiliary input
    let primary_input = full_assignment[..cs.num_inputs()].to_vec();
    let auxiliary_input = full_assignment[cs.num_inputs()..].to_vec();

    assert!(cs.is_valid());
    assert!(cs.is_satisfied(&primary_input, &auxiliary_input));

    r1cs_example::<FieldT, SV, SLC>::new(cs, primary_input, auxiliary_input)
}
