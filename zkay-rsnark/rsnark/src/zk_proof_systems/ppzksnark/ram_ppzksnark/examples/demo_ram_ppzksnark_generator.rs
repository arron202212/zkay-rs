// use crate::common::default_types::ram_ppzksnark_pp;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;
use crate::common::default_types::ram_ppzksnark_pp::default_ram_ppzksnark_pp;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::reductions::ram_to_r1cs::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::ram_params::ram_base_field;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    load_preprocessed_program, load_tape, tinyram_boot_trace_from_program_and_input,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::{
    ram_ppzksnark_generator, ram_ppzksnark_prover, ram_ppzksnark_verifier,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::ram_ppzksnark_snark_pp;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::{
    ram_ppzksnark_architecture_params, ram_ppzksnark_machine_pp,
};
use clap::{ArgAction, Command, arg, command, value_parser};
use ff_curves::PublicParams;
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use std::io;

//#ifndef MINDEPS
// namespace po = boost::program_options;

pub fn process_generator_command_line(
    argc: i32,
    argv: &[&str],
    architecture_params_fn: &mut String,
    computation_bounds_fn: &mut String,
    proving_key_fn: &mut String,
    verification_key_fn: &mut String,
) -> bool {
    // po::options_description desc("Usage");
    // desc.add_options()
    //     ("help", "print this help message")
    //     ("architecture_params", po::value<String>(&architecture_params_fn)->required())
    //     ("computation_bounds", po::value<String>(&computation_bounds_fn)->required())
    //     ("proving_key", po::value<String>(&proving_key_fn)->required())
    //     ("verification_key", po::value<String>(&verification_key_fn)->required());

    let matches = Command::new("Usage")
        .arg(arg!([architecture_params]).required(true))
        .arg(arg!([computation_bounds]).required(true))
        .arg(arg!([proving_key]).required(true))
        .arg(arg!([verification_key]).required(true))
        .get_matches();

    *architecture_params_fn = matches
        .get_one::<String>("architecture_params")
        .unwrap()
        .clone();
    *computation_bounds_fn = matches
        .get_one::<String>("computation_bounds")
        .unwrap()
        .clone();
    *proving_key_fn = matches.get_one::<String>("proving_key").unwrap().clone();
    *verification_key_fn = matches
        .get_one::<String>("verification_key")
        .unwrap()
        .clone();

    //     std::cerr << "Error: " << e.what() << "\n";
    //     return false;

    true
}
// #endif

fn main<default_ram_ppzksnark_pp: default_tinyram_ppzksnark_ppConfig + RamPptConfig>(
    argc: i32,
    argv: &[&str],
) -> io::Result<i32> {
    ram_ppzksnark_snark_pp::<default_ram_ppzksnark_pp>::init_public_params();
    // #ifdef MINDEPS
    // let mut  architecture_params_fn =fs::read_to_string( "architecture_params.txt");
    // let mut  computation_bounds_fn = fs::read_to_string("computation_bounds.txt");
    // let mut  proving_key_fn = fs::read_to_string("proving_key.txt");
    // let mut  verification_key_fn = fs::read_to_string("verification_key.txt");
    // #else
    let mut architecture_params_fn = String::new();
    let mut computation_bounds_fn = String::new();
    let mut proving_key_fn = String::new();
    let mut verification_key_fn = String::new();

    if (!process_generator_command_line(
        argc,
        argv,
        &mut architecture_params_fn,
        &mut computation_bounds_fn,
        &mut proving_key_fn,
        &mut verification_key_fn,
    )) {
        return Ok(1);
    }
    //#endif
    start_profiling();

    /* load everything */
    let mut ap = ram_ppzksnark_architecture_params::<default_ram_ppzksnark_pp>::default();

    // use std::io::{self, BufRead};

    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // let mut f_ap = stdin.lock();

    // f_ap.read_line(&mut buffer)?;

    let mut f_ap = architecture_params_fn.lines();
    // f_ap >> ap;

    let mut f_rp = computation_bounds_fn.lines();
    let (mut tinyram_input_size_bound, mut tinyram_program_size_bound, mut time_bound) = (0, 0, 0);
    // f_rp >> tinyram_input_size_bound >> tinyram_program_size_bound >> time_bound;

    let boot_trace_size_bound = tinyram_program_size_bound + tinyram_input_size_bound;

    let keypair =
        ram_ppzksnark_generator::<default_ram_ppzksnark_pp>(&ap, boot_trace_size_bound, time_bound);
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut pk = OpenOptions::new().append(true).open(proving_key_fn)?;
    writeln!(pk, "{}", keypair.pk)?;

    let mut vk = OpenOptions::new().append(true).open(verification_key_fn)?;
    writeln!(vk, "{}", keypair.vk)?;
    Ok(0)
}
