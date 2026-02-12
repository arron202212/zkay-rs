// use crate::common::default_types::tinyram_ppzksnark_pp;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::reductions::ram_to_r1cs::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::ram_params::ram_base_field;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    load_preprocessed_program, load_tape, tinyram_boot_trace_from_program_and_input,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::ram_ppzksnark_proof;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::ram_ppzksnark_verification_key;
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

//
// namespace po = boost::program_options;

fn process_verifier_command_line(
    argc: i32,
    argv: &[&str],
    processed_assembly_fn: &mut String,
    verification_key_fn: &mut String,
    primary_input_fn: &mut String,
    proof_fn: &mut String,
    verification_result_fn: &mut String,
) -> bool {
    // po::options_description desc("Usage");
    // desc.add_options()
    //     ("help", "print this help message")
    //     ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
    //     ("verification_key", po::value<String>(&verification_key_fn)->required())
    //     ("primary_input", po::value<String>(&primary_input_fn)->required())
    //     ("proof", po::value<String>(&proof_fn)->required())
    //     ("verification_result", po::value<String>(&verification_result_fn)->required());

    let matches = Command::new("Usage")
        .arg(arg!([processed_assembly]).required(true))
        .arg(arg!([verification_key]).required(true))
        .arg(arg!([primary_input]).required(true))
        .arg(arg!([proof]).required(true))
        .arg(arg!([verification_result]).required(true))
        .get_matches();

    *processed_assembly_fn = matches
        .get_one::<String>("processed_assembly")
        .unwrap()
        .clone();
    *verification_key_fn = matches
        .get_one::<String>("verification_key")
        .unwrap()
        .clone();
    *primary_input_fn = matches.get_one::<String>("primary_input").unwrap().clone();
    *proof_fn = matches.get_one::<String>("proof").unwrap().clone();
    *verification_result_fn = matches
        .get_one::<String>("verification_result")
        .unwrap()
        .clone();

    true
}

fn main<default_tinyram_ppzksnark_pp: default_tinyram_ppzksnark_ppConfig + RamPptConfig>(
    argc: i32,
    argv: &[&str],
) -> io::Result<i32> {
    <default_tinyram_ppzksnark_pp as RamPptConfig>::init_public_params();

    // #ifdef MINDEPS
    //     let mut  processed_assembly_fn = fs::read_to_string("processed.txt");
    //     let mut  verification_key_fn = fs::read_to_string("verification_key.txt");
    //     let mut  proof_fn =fs::read_to_string ("proof.txt");
    //     let mut  primary_input_fn = fs::read_to_string("primary_input.txt");
    //     let mut  verification_result_fn = fs::read_to_string("verification_result.txt");
    // #else
    let mut processed_assembly_fn = String::new();
    let mut verification_key_fn = String::new();
    let mut proof_fn = String::new();
    let mut primary_input_fn = String::new();
    let mut verification_result_fn = String::new();

    if !process_verifier_command_line(
        argc,
        argv,
        &mut processed_assembly_fn,
        &mut verification_key_fn,
        &mut primary_input_fn,
        &mut proof_fn,
        &mut verification_result_fn,
    ) {
        return Ok(1);
    }

    start_profiling();

    let mut vk = ram_ppzksnark_verification_key::<default_tinyram_ppzksnark_pp>::default();
    let mut vk_file = verification_key_fn.lines();
    // vk_file >> vk;

    let mut processed = processed_assembly_fn;
    let program = load_preprocessed_program(&vk.ap, &processed);

    let mut f_primary_input = primary_input_fn;
    let primary_input = load_tape(&f_primary_input);

    let mut proof_file = proof_fn.lines();
    let mut pi = ram_ppzksnark_proof::<default_tinyram_ppzksnark_pp>::default();
    // proof_file >> pi;

    let boot_trace = tinyram_boot_trace_from_program_and_input(
        &vk.ap,
        vk.primary_input_size_bound,
        &program,
        &primary_input,
    );
    let bit = ram_ppzksnark_verifier::<default_tinyram_ppzksnark_pp>(&vk, &boot_trace, &pi);

    print!("================================================================================\n");
    print!(
        "The verification result is: {}\n",
        if bit { "PASS" } else { "FAIL" }
    );
    print!("================================================================================\n");
    // std::ofstream vr_file(verification_result_fn);
    // vr_file << bit << "\n";
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut vr_file = OpenOptions::new()
        .append(true)
        .open(verification_result_fn)?;
    writeln!(vr_file, "{}", bit)?;
    Ok(0)
}
