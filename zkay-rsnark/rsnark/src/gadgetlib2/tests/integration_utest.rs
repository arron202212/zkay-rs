// use crate::common::default_types::r1cs_ppzksnark_pp;
// use crate::gadgetlib2::examples::simple_example;
// use crate::gadgetlib2::gadget;
// use crate::gadgetlib2::pp;
// use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::gadgetlib2::examples::simple_example::gen_r1cs_example_from_gadgetlib2_protoboard;
use crate::gadgetlib2::pp::initPublicParamsFromDefaultPp;
use crate::gadgetlib2::protoboard::Protoboard;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark::run_r1cs_ppzksnark;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Integration() {
        initPublicParamsFromDefaultPp();
        let example = gen_r1cs_example_from_gadgetlib2_protoboard(100);
        let mut test_serialization = false;

        let mut bit = run_r1cs_ppzksnark::<default_r1cs_ppzksnark_pp>(&example, test_serialization);
        assert!(bit);
    }
}
