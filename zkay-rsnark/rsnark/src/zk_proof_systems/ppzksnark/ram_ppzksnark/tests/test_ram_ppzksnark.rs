// use crate::common::default_types::ram_ppzksnark_pp::default_ram_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::rams::examples::ram_examples::gen_ram_example_complex;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark::run_ram_ppzksnark;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::{
    ram_ppzksnark_architecture_params, ram_ppzksnark_machine_pp, ram_ppzksnark_snark_pp,
};
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use std::ops::Mul;

type machine_ppT<ppT> = ram_ppzksnark_machine_pp<ppT>;
pub fn test_ram_ppzksnark<ppT: RamPptConfig>(
    w: usize,
    k: usize,
    program_size: usize,
    input_size: usize,
    time_bound: usize,
) where
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
    print_header("(enter) Test RAM ppzkSNARK");

    let boot_trace_size_bound = program_size + input_size;
    let mut satisfiable = true;

    let ap = ram_ppzksnark_architecture_params::<ppT>::fromss(w, k);
    let example = gen_ram_example_complex::<machine_ppT<ppT>>(
        ap,
        boot_trace_size_bound,
        time_bound,
        satisfiable,
    );

    let mut test_serialization = true;
    let mut bit = run_ram_ppzksnark::<ppT>(&example, test_serialization);
    assert!(bit);

    print_header("(leave) Test RAM ppzkSNARK");
}

fn main<default_ram_ppzksnark_pp: RamPptConfig>() -> i32 where
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
>,{
    ram_ppzksnark_snark_pp::<default_ram_ppzksnark_pp>::init_public_params();
    start_profiling();

    let program_size = 100;
    let input_size = 2;
    let time_bound = 20;

    // 16-bit TinyRAM with 16 registers
    test_ram_ppzksnark::<default_ram_ppzksnark_pp>(16, 16, program_size, input_size, time_bound);

    // 32-bit TinyRAM with 16 registers
    test_ram_ppzksnark::<default_ram_ppzksnark_pp>(32, 16, program_size, input_size, time_bound);
    0
}
