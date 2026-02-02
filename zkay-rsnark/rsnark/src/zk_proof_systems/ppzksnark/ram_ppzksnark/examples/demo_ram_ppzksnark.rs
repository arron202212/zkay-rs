// use common::profiling;

// use crate::common::default_types::tinyram_ppzksnark_pp;
// use crate::reductions::ram_to_r1cs::ram_to_r1cs;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::reductions::ram_to_r1cs::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_base_field;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    load_preprocessed_program, load_tape, tinyram_boot_trace_from_program_and_input,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::{
    ram_ppzksnark_generator, ram_ppzksnark_prover, ram_ppzksnark_verifier,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::{
    ram_ppzksnark_architecture_params, ram_ppzksnark_machine_pp,
};
use clap::{ArgAction, Command, arg, command, value_parser};
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use std::io;
use std::ops::Mul;

//#ifndef MINDEPS
// // namespace po = boost::program_options;

fn process_demo_command_line(
    argc: i32,
    argv: &[&str],
    assembly_fn: &mut String,
    processed_assembly_fn: &mut String,
    architecture_params_fn: &mut String,
    computation_bounds_fn: &mut String,
    primary_input_fn: &mut String,
    auxiliary_input_fn: &mut String,
) -> bool {
    // po::options_description desc("Usage");
    // desc.add_options()
    //     ("help", "print this help message")
    //     ("assembly", po::value<String>(&assembly_fn)->required())
    //     ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
    //     ("architecture_params", po::value<String>(&architecture_params_fn)->required())
    //     ("computation_bounds", po::value<String>(&computation_bounds_fn)->required())
    //     ("primary_input", po::value<String>(&primary_input_fn)->required())
    //     ("auxiliary_input", po::value<String>(&auxiliary_input_fn)->required());

    let matches = Command::new("Usage")
        .arg(arg!([assembly]).required(true))
        .arg(arg!([processed_assembly]).required(true))
        .arg(arg!([architecture_params]).required(true))
        .arg(arg!([computation_bounds]).required(true))
        .arg(arg!([primary_input]).required(true))
        .arg(arg!([auxiliary_input]).required(true))
        .get_matches();
    *assembly_fn = matches.get_one::<String>("assembly").unwrap().clone();
    *processed_assembly_fn = matches
        .get_one::<String>("processed_assembly")
        .unwrap()
        .clone();
    *architecture_params_fn = matches
        .get_one::<String>("architecture_params")
        .unwrap()
        .clone();
    *computation_bounds_fn = matches
        .get_one::<String>("computation_bounds")
        .unwrap()
        .clone();
    *primary_input_fn = matches.get_one::<String>("primary_input").unwrap().clone();
    *auxiliary_input_fn = matches
        .get_one::<String>("auxiliary_input")
        .unwrap()
        .clone();

    true
}

type default_ram<default_tinyram_ppzksnark_pp> =
    ram_ppzksnark_machine_pp<default_tinyram_ppzksnark_pp>;
type FieldT<default_tinyram_ppzksnark_pp> =
    ram_base_field<default_ram<default_tinyram_ppzksnark_pp>>;
fn main<default_tinyram_ppzksnark_pp:default_tinyram_ppzksnark_ppConfig+RamPptConfig>(argc: i32, argv: &[&str]) -> io::Result<i32> where
    knowledge_commitment<
        <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
        <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
        <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<default_tinyram_ppzksnark_pp as RamPptConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
>,{
    <default_tinyram_ppzksnark_pp as default_tinyram_ppzksnark_ppConfig>::init_public_params();
    // #ifdef MINDEPS
    // let mut  assembly_fn = fs::read_to_string("assembly.s");
    // let mut  processed_assembly_fn =fs::read_to_string ("processed.txt");
    // let mut  architecture_params_fn = fs::read_to_string("architecture_params.txt");
    // let mut  computation_bounds_fn = fs::read_to_string("computation_bounds.txt");
    // let mut  primary_input_fn = fs::read_to_string("primary_input.txt");
    // let mut  auxiliary_input_fn = fs::read_to_string("auxiliary_input.txt");
    // #else
    let mut assembly_fn = String::new();
    let mut processed_assembly_fn = String::new();
    let mut architecture_params_fn = String::new();
    let mut computation_bounds_fn = String::new();
    let mut primary_input_fn = String::new();
    let mut auxiliary_input_fn = String::new();

    if (!process_demo_command_line(
        argc,
        argv,
        &mut assembly_fn,
        &mut processed_assembly_fn,
        &mut architecture_params_fn,
        &mut computation_bounds_fn,
        &mut primary_input_fn,
        &mut auxiliary_input_fn,
    )) {
        return Ok(1);
    }
    //#endif
    start_profiling();

    print!("================================================================================\n");
    print!("TinyRAM example loader\n");
    print!("================================================================================\n\n");

    /* load everything */
    let mut ap = ram_ppzksnark_architecture_params::<default_tinyram_ppzksnark_pp>::default();
    let mut f_ap = architecture_params_fn.lines();
    // f_ap >> ap;

    print!(
        "Will run on {} register machine (word size = {})\n",
        ap.k(),
        ap.w()
    );

    let mut f_rp = computation_bounds_fn.lines();
    let (tinyram_input_size_bound, tinyram_program_size_bound, time_bound) = (0, 0, 0);
    // f_rp >> tinyram_input_size_bound >> tinyram_program_size_bound >> time_bound;

    let mut processed = processed_assembly_fn;
    let mut raw = assembly_fn;
    let program = load_preprocessed_program(&ap, &processed);

    print!("Program:\n{}\n", raw);

    let mut f_primary_input = primary_input_fn;
    let mut f_auxiliary_input = auxiliary_input_fn;

    enter_block("Loading primary input", false);
    let primary_input = load_tape(&f_primary_input);
    leave_block("Loading primary input", false);

    enter_block("Loading auxiliary input", false);
    let auxiliary_input = load_tape(&f_auxiliary_input);
    leave_block("Loading auxiliary input", false);

    print!("\nPress enter to continue.\n");
    // std::cin.get();

    let boot_trace_size_bound = tinyram_program_size_bound + tinyram_input_size_bound;
    let boot_trace = tinyram_boot_trace_from_program_and_input(
        &ap,
        boot_trace_size_bound,
        &program,
        &primary_input,
    );

    print!("================================================================================\n");
    print!(
        "TinyRAM arithmetization test for T = {} time steps\n",
        time_bound
    );
    print!("================================================================================\n\n");

    let mut r = ram_to_r1cs::<default_ram<default_tinyram_ppzksnark_pp>>::new(
        ap.clone(),
        boot_trace_size_bound,
        time_bound,
    );
    r.instance_map();

    let r1cs_primary_input =
        ram_to_r1cs::<default_ram<default_tinyram_ppzksnark_pp>>::primary_input_map(
            &ap,
            boot_trace_size_bound,
            &boot_trace,
        );
    let r1cs_auxiliary_input = r.auxiliary_input_map(&boot_trace, &auxiliary_input);
    let constraint_system = r.get_constraint_system();

    r.print_execution_trace();
    assert!(constraint_system.is_satisfied(&r1cs_primary_input, &r1cs_auxiliary_input));

    print!("\nPress enter to continue.\n");
    // std::cin.get();

    print!("================================================================================\n");
    print!("TinyRAM ppzkSNARK Key Pair Generator\n");
    print!("================================================================================\n\n");
    let keypair = ram_ppzksnark_generator::<default_tinyram_ppzksnark_pp>(
        &ap,
        boot_trace_size_bound,
        time_bound,
    );

    print!("\nPress enter to continue.\n");
    // std::cin.get();

    print!("================================================================================\n");
    print!("TinyRAM ppzkSNARK Prover\n");
    print!("================================================================================\n\n");
    let proof = ram_ppzksnark_prover::<default_tinyram_ppzksnark_pp>(
        &keypair.pk,
        &boot_trace,
        &auxiliary_input,
    );

    print!("\nPress enter to continue.\n");
    // std::cin.get();

    print!("================================================================================\n");
    print!("TinyRAM ppzkSNARK Verifier\n");
    print!("================================================================================\n\n");
    let bit =
        ram_ppzksnark_verifier::<default_tinyram_ppzksnark_pp>(&keypair.vk, &boot_trace, &proof);

    print!("================================================================================\n");
    print!(
        "The verification result is: {}\n",
        if bit { "PASS" } else { "FAIL" }
    );
    print!("================================================================================\n");
    println!();
    print!("================================================================================\n");
    Ok(0)
}
