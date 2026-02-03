// use crate::common::default_types::r1cs_ppzkpcd_pp;
// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::examples::run_r1cs_sp_ppzkpcd;
// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd;
use crate::common::default_types::r1cs_ppzkpcd_pp::default_r1cs_ppzkpcd_ppConfig;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp::TallyCPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::examples::run_r1cs_sp_ppzkpcd::run_r1cs_sp_ppzkpcd_tally_example;
use ff_curves::Fr;
use ffec::common::profiling::{
    enter_block, last_times, leave_block, print_compilation_info, start_profiling,
};
use std::ops::Mul;

pub fn profile_tally<
    PCD_ppT: PcdPptConfig<curve_A_pp = PCD_ppT>
        + TallyCPHConfig<
            protoboard_type = protoboard<
                <PCD_ppT as ppTConfig>::FieldT,
                <PCD_ppT as ppTConfig>::PB,
            >,
        >,
>(
    arity: usize,
    max_layer: usize,
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
    let bit = run_r1cs_sp_ppzkpcd_tally_example::<PCD_ppT>(
        wordsize,
        arity,
        max_layer,
        test_serialization,
    );
    assert!(bit);
}
// type PCD_pp = default_r1cs_ppzkpcd_pp;

pub fn main<
    PCD_pp: default_r1cs_ppzkpcd_ppConfig
        + PcdPptConfig<curve_A_pp = PCD_pp>
        + TallyCPHConfig<
            protoboard_type = protoboard<<PCD_pp as ppTConfig>::FieldT, <PCD_pp as ppTConfig>::PB>,
        >,
>() -> i32
where
    knowledge_commitment<
        <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
        <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
                <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
                <<PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
                <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    start_profiling();
    <PCD_pp as default_r1cs_ppzkpcd_ppConfig>::init_public_params();

    let arity = 2;
    let max_layer = 2;

    profile_tally::<PCD_pp>(arity, max_layer);
    0
}
