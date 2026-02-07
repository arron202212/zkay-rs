use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::examples::run_r1cs_mp_ppzkpcd::run_r1cs_mp_ppzkpcd_tally_example;
use ff_curves::Fr;

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp::TallyCPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

pub fn test_tally<
    PCD_ppT: TallyCPHConfig<
            protoboard_type = protoboard<
                <PCD_ppT as ppTConfig>::FieldT,
                <PCD_ppT as ppTConfig>::PB,
            >,
        > + PcdPptConfig<curve_A_pp = PCD_ppT>,
>(
    arity: usize,
    max_layer: usize,
    test_multi_type: bool,
    test_same_type_optimization: bool,
) where
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    let wordsize = 32;
    let mut test_serialization = true;
    let bit = run_r1cs_mp_ppzkpcd_tally_example::<PCD_ppT>(
        wordsize,
        arity,
        max_layer,
        test_serialization,
        test_multi_type,
        test_same_type_optimization,
    );
    assert!(bit);
}

pub fn main<default_r1cs_ppzkpcd_pp: PcdPptConfig<curve_A_pp = default_r1cs_ppzkpcd_pp> + TallyCPHConfig<protoboard_type =protoboard<<default_r1cs_ppzkpcd_pp as ppTConfig>::FieldT, <default_r1cs_ppzkpcd_pp as ppTConfig>::PB>>>() -> i32 where
    knowledge_commitment<
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<default_r1cs_ppzkpcd_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
>,{
    start_profiling();
    default_r1cs_ppzkpcd_pp::init_public_params();

    let max_arity = 2;
    let max_layer = 2;

    test_tally::<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, false, false);
    test_tally::<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, false, true);
    test_tally::<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, true, false);
    test_tally::<default_r1cs_ppzkpcd_pp>(max_arity, max_layer, true, true);
    0
}
