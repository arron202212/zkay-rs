// use crate::common::default_types::ram_zksnark_pp;
// use crate::relations::ram_computations::rams::examples::ram_examples;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;
use crate::common::default_types::ram_zksnark_pp::default_ram_zksnark_pp;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples::gen_ram_example_complex;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_architecture_params;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::CPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark::run_ram_zksnark;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::{
    ram_zksnark_PCD_pp, ram_zksnark_machine_pp,
};
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

type RamT<ppT> = ram_zksnark_machine_pp<ppT>;
pub fn test_ram_zksnark<ppT:RamConfig>(w: usize, k: usize, boot_trace_size_bound: usize, time_bound: usize) where
    <ppT as RamConfig>::machine_pp: CPHConfig,
    knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =  knowledge_commitment<
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<ppT as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
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
    knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<ppT as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
{
    let ap = ram_architecture_params::<RamT<ppT>>::fromss(w, k);
    let example = gen_ram_example_complex::<RamT<ppT>>(ap, boot_trace_size_bound, time_bound, true);
    let mut test_serialization = true;
    let mut ans = run_ram_zksnark::<ppT>(&example, test_serialization);
    assert!(ans);
}

pub fn main<default_ram_zksnark_pp:RamConfig>() -> i32 where
    <default_ram_zksnark_pp as RamConfig>::machine_pp: CPHConfig,
    knowledge_commitment<
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =  knowledge_commitment<
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G2,
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output =knowledge_commitment<
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
        <<default_ram_zksnark_pp as RamConfig>::machine_pp as ff_curves::PublicParams>::G1,
    >,
        >,
    knowledge_commitment<
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_zksnark_pp as RamConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<<default_ram_zksnark_pp as RamConfig>::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >,
        >,
{
    start_profiling();
    ram_zksnark_PCD_pp::<default_ram_zksnark_pp>::init_public_params();

    let w = 32;
    let k = 16;

    let boot_trace_size_bound = 20;
    let time_bound = 10;

    test_ram_zksnark::<default_ram_zksnark_pp>(w, k, boot_trace_size_bound, time_bound);
    0
}
