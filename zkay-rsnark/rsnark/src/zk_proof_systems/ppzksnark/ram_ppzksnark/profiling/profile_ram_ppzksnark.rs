// use common::profiling;

// use crate::common::default_types::ram_ppzksnark_pp;
// use crate::relations::ram_computations::rams::examples::ram_examples;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark;
use crate::common::default_types::ram_ppzksnark_pp::default_ram_ppzksnark_pp;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples::gen_ram_example_complex;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark::run_ram_ppzksnark;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_architecture_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_machine_pp;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_snark_pp;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, last_times, leave_block, print_compilation_info, start_profiling,
};
use std::ops::Mul;

type machine_ppT<default_ram_ppzksnark_pp> = ram_ppzksnark_machine_pp<default_ram_ppzksnark_pp>;

fn main<default_ram_ppzksnark_pp:RamPptConfig>(argc: i32, argv: &[&str]) -> i32 where
    knowledge_commitment<
        <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
        <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_ppzksnark_pp as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
        <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_ram_ppzksnark_pp as RamPptConfig>::machine_pp as ram_params_type>::base_field_type,
            Output = knowledge_commitment<
                <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<default_ram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    ram_ppzksnark_snark_pp::<default_ram_ppzksnark_pp>::init_public_params();
    start_profiling();

    if argc == 2 && argv[1] == "-v" {
        print_compilation_info();
        return 0;
    }

    if argc != 6 {
        print!(
            "usage: {} word_size reg_count program_size input_size time_bound\n",
            argv[0]
        );
        return 1;
    }

    let w = argv[1].parse::<usize>().unwrap();
    let k = argv[2].parse::<usize>().unwrap();
    let program_size = argv[3].parse::<usize>().unwrap();
    let input_size = argv[4].parse::<usize>().unwrap();
    let time_bound = argv[5].parse::<usize>().unwrap();

    let ap = ram_ppzksnark_architecture_params::<default_ram_ppzksnark_pp>::fromss(w, k);

    enter_block("Generate RAM example", false);
    let boot_trace_size_bound = program_size + input_size;
    let mut satisfiable = true;
    let example = gen_ram_example_complex::<machine_ppT<default_ram_ppzksnark_pp>>(
        ap,
        boot_trace_size_bound,
        time_bound,
        satisfiable,
    );
    leave_block("Generate RAM example", false);

    println!("(enter) Profile RAM ppzkSNARK");
    let mut test_serialization = true;
    run_ram_ppzksnark::<default_ram_ppzksnark_pp>(&example, test_serialization);
    println!("(leave) Profile RAM ppzkSNARK");
    0
}
