// use common::default_types::ec_pp;
// use common::profiling;

// use crate::common::default_types::tinyram_ppzksnark_pp;
// use crate::reductions::ram_to_r1cs::ram_to_r1cs;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;
// use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_pp;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::reductions::ram_to_r1cs::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_architecture_params;
use crate::relations::ram_computations::rams::ram_params::ram_base_field;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    load_preprocessed_program, load_tape, tinyram_boot_trace_from_program_and_input,
};
use crate::relations::ram_computations::rams::tinyram::tinyram_params::ram_tinyram;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::{
    ram_ppzksnark_generator, ram_ppzksnark_prover, ram_ppzksnark_verifier,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::{
    ram_ppzksnark_architecture_params, ram_ppzksnark_machine_pp,
};
use clap::{ArgAction, Command, arg, command, value_parser};
use ff_curves::Fr;
use ff_curves::PublicParams;
use ff_curves::default_ec_pp;
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use std::io;
use std::ops::Mul;

fn process_arithm_command_line(
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

    // po::variables_map vm;
    // po::store(po::parse_command_line(argc, argv, desc), vm);

    // if vm.count("help")
    // {
    //     std::cout << desc << "\n";
    //     return false;
    // }

    // po::notify(vm);
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

type FieldT = Fr<default_ec_pp>;
type default_ram<default_tinyram_ppzksnark_pp> =
    <default_tinyram_ppzksnark_pp as default_tinyram_ppzksnark_ppConfig>::machine_ppp; //ram_tinyram<FieldT>;
type default_rams<default_tinyram_ppzksnark_pps> =
    ram_ppzksnark_machine_pp<default_tinyram_ppzksnark_pps>;
fn main<default_tinyram_ppzksnark_pp: default_tinyram_ppzksnark_ppConfig>(
    argc: i32,
    argv: &[&str],
) -> i32 {
    default_ec_pp::init_public_params();

    // #ifdef MINDEPS
    // String assembly_fn = "assembly.s";
    // String processed_assembly_fn = "processed.txt";
    // String architecture_params_fn = "architecture_params.txt";
    // String computation_bounds_fn = "computation_bounds.txt";
    // String primary_input_fn = "primary_input.txt";
    // String auxiliary_input_fn = "auxiliary_input.txt";
    // #else
    let mut assembly_fn = String::new();
    let mut processed_assembly_fn = String::new();
    let mut architecture_params_fn = String::new();
    let mut computation_bounds_fn = String::new();
    let mut primary_input_fn = String::new();
    let mut auxiliary_input_fn = String::new();

    if (!process_arithm_command_line(
        argc,
        argv,
        &mut assembly_fn,
        &mut processed_assembly_fn,
        &mut architecture_params_fn,
        &mut computation_bounds_fn,
        &mut primary_input_fn,
        &mut auxiliary_input_fn,
    )) {
        return 1;
    }

    start_profiling();

    print!("================================================================================\n");
    print!("TinyRAM example loader\n");
    print!("================================================================================\n\n");

    /* load everything */
    let mut ap = ram_architecture_params::<default_ram<default_tinyram_ppzksnark_pp>>::default();
    let mut f_ap = (architecture_params_fn);
    // f_ap >> ap;

    print!(
        "Will run on {} register machine (word size = {})\n",
        ap.k, ap.w
    );

    let mut f_rp = (computation_bounds_fn);
    let (tinyram_input_size_bound, tinyram_program_size_bound, time_bound) = (0, 0, 0);
    // f_rp >> tinyram_input_size_bound >> tinyram_program_size_bound >> time_bound;

    let mut processed = (processed_assembly_fn);
    let mut raw = (assembly_fn);
    let program = load_preprocessed_program(&ap, &processed);
    print!("Program:\n{}\n", raw);

    let mut f_primary_input = primary_input_fn;
    let mut f_auxiliary_input = auxiliary_input_fn;

    enter_block("Loading primary input", false);
    let primary_input = (load_tape(&f_primary_input));
    leave_block("Loading primary input", false);

    enter_block("Loading auxiliary input", false);
    let auxiliary_input = load_tape(&f_auxiliary_input);
    leave_block("Loading auxiliary input", false);

    let boot_trace_size_bound = tinyram_input_size_bound + tinyram_program_size_bound;
    let boot_trace = tinyram_boot_trace_from_program_and_input(
        &ap,
        boot_trace_size_bound,
        &program,
        &primary_input,
    );

    let mut r = ram_to_r1cs::<default_rams<default_tinyram_ppzksnark_pp>>::new(
        ap.clone(),
        boot_trace_size_bound,
        time_bound,
    );
    r.instance_map();

    let r1cs_primary_input =
        ram_to_r1cs::<default_rams<default_tinyram_ppzksnark_pp>>::primary_input_map(
            &ap,
            boot_trace_size_bound,
            &boot_trace,
        );
    let r1cs_auxiliary_input = r.auxiliary_input_map(&boot_trace, &auxiliary_input);
    let constraint_system = r.get_constraint_system();

    r.print_execution_trace();
    assert!(constraint_system.is_satisfied(&r1cs_primary_input, &r1cs_auxiliary_input));
    0
}
