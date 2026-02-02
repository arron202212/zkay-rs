// use common::utils;
use crate::common::default_types::r1cs_ppzkpcd_pp::default_r1cs_ppzkpcd_ppConfig;
use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples::ram_example;
use crate::relations::ram_computations::rams::fooram::fooram_params::ram_fooram;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_architecture_params;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::CPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark::run_ram_ppzksnark;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_machine_pp;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark::run_ram_zksnark;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::ram_zksnark_machine_pp;
use ff_curves::Fr;
use ffec::FieldTConfig;
use ffec::common::profiling::start_profiling;
use std::ops::Mul;

pub trait default_fooram_zksnark_ppConfig {
    type PCD_pp: default_r1cs_ppzkpcd_ppConfig; // = default_r1cs_ppzkpcd_pp;
    type FieldT: FieldTConfig; // = PCD_pp::scalar_field_A;
    type machine_pp = ram_fooram<Self::FieldT>;

    fn init_public_params() {
        // PCD_pp::init_public_params();
    }
}

pub trait default_fooram_ppzksnark_ppConfig {
    type snark_pp = default_r1cs_ppzksnark_pp;
    type FieldT: FieldTConfig = Fr<default_r1cs_ppzksnark_pp>;
    type machine_pp = ram_fooram<Self::FieldT>;

    fn init_public_params() {
        // Self::snark_pp::init_public_params();
    }
}

type RamTZ<ppT> = ram_zksnark_machine_pp<ppT>;
pub fn profile_ram_zksnark<ppT: ppTConfig + RamPptConfig + RamConfig>(w: usize)
where
    <ppT as RamConfig>::machine_pp: CPHConfig,
    knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >
        >,
    knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
{
    let mut example = ram_example::<RamTZ<ppT>>::default();
    example.ap = ram_architecture_params::<RamTZ<ppT>>::froms(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 10;
    let test_serialization = true;
    let bit = run_ram_zksnark::<ppT>(&example, test_serialization);
    assert!(bit);
}

type RamT<ppT> = ram_ppzksnark_machine_pp<ppT>;
pub fn profile_ram_ppzksnark<ppT: ppTConfig + RamPptConfig>(w: usize)
where
    knowledge_commitment<
        <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
        <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
        <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<ppT as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    //

    let mut example = ram_example::<RamT<ppT>>::default();
    example.ap = ram_architecture_params::<RamT<ppT>>::froms(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 100;
    let test_serialization = true;
    let bit = run_ram_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);
}

fn main<
    default_fooram_ppzksnark_pp: default_fooram_ppzksnark_ppConfig + ppTConfig + RamPptConfig+RamConfig,
    default_fooram_zksnark_pp: default_fooram_zksnark_ppConfig + ppTConfig + RamPptConfig+RamConfig,
>(
    argc: i32,
    argv: &[&str],
) -> i32  where knowledge_commitment<<<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2, <<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_ppzksnark_pp as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2, <<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1>>,
 <default_fooram_zksnark_pp as RamConfig>::machine_pp: CPHConfig,
 knowledge_commitment<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G2, <<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G2, <<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1>>,
knowledge_commitment<<<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2, <<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2, <<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>>,
knowledge_commitment<<<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1, <<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_ppzksnark_pp as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1, <<default_fooram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1>>,
knowledge_commitment<<<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1, <<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1, <<<default_fooram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>>,
knowledge_commitment<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1, <<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1>: Mul<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,Output=knowledge_commitment<<<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1, <<default_fooram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1>>{
    // //UNUSED(argv);
    start_profiling();
    <default_fooram_ppzksnark_pp as ff_curves::PublicParams>::init_public_params();
    <default_fooram_zksnark_pp as RamPptConfig>::init_public_params();

    if argc == 1 {
        profile_ram_zksnark::<default_fooram_zksnark_pp>(32);
    } else {
        profile_ram_ppzksnark::<default_fooram_ppzksnark_pp>(8);
    }
    0
}
