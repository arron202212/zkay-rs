// use crate::common::default_types::tinyram_ppzksnark_pp;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::reductions::ram_to_r1cs::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::ram_params::ram_base_field;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    load_preprocessed_program, load_tape, tinyram_boot_trace_from_program_and_input,
};
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark::ram_ppzksnark_proving_key;
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

// namespace po = boost::program_options;

fn process_prover_command_line(
    argc: i32,
    argv: &[&str],
    processed_assembly_fn: &mut String,
    proving_key_fn: &mut String,
    primary_input_fn: &mut String,
    auxiliary_input_fn: &mut String,
    proof_fn: &mut String,
) -> bool {
    // po::options_description desc("Usage");
    // desc.add_options()
    //     ("help", "print this help message")
    //     ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
    //     ("proving_key", po::value<String>(&proving_key_fn)->required())
    //     ("primary_input", po::value<String>(&primary_input_fn)->required())
    //     ("auxiliary_input", po::value<String>(&auxiliary_input_fn)->required())
    //     ("proof", po::value<String>(&proof_fn)->required());

    let matches = Command::new("Usage")
        .arg(arg!([processed_assembly]).required(true))
        .arg(arg!([proving_key]).required(true))
        .arg(arg!([primary_input]).required(true))
        .arg(arg!([auxiliary_input]).required(true))
        .arg(arg!([proof]).required(true))
        .get_matches();

    *processed_assembly_fn = matches
        .get_one::<String>("processed_assembly")
        .unwrap()
        .clone();
    *proving_key_fn = matches.get_one::<String>("proving_key").unwrap().clone();
    *primary_input_fn = matches.get_one::<String>("primary_input").unwrap().clone();
    *auxiliary_input_fn = matches
        .get_one::<String>("auxiliary_input")
        .unwrap()
        .clone();
    *proof_fn = matches.get_one::<String>("proof").unwrap().clone();
    true
}


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
        >,
{
    <default_tinyram_ppzksnark_pp as RamPptConfig>::init_public_params();

    // #ifdef MINDEPS
    //     let mut  processed_assembly_fn = fs::read_to_string("processed.txt");
    //     let mut  proving_key_fn = fs::read_to_string("proving_key.txt");
    //     let mut  primary_input_fn = fs::read_to_string("primary_input.txt");
    //     let mut  auxiliary_input_fn = fs::read_to_string("auxiliary_input.txt");
    //     let mut  proof_fn = "proof.txt";
    // #else
    let mut processed_assembly_fn = String::new();
    let mut proving_key_fn = String::new();
    let mut primary_input_fn = String::new();
    let mut auxiliary_input_fn = String::new();
    let mut proof_fn = String::new();

    if (!process_prover_command_line(
        argc,
        argv,
        &mut processed_assembly_fn,
        &mut proving_key_fn,
        &mut primary_input_fn,
        &mut auxiliary_input_fn,
        &mut proof_fn,
    )) {
        return Ok(1);
    }

    start_profiling();

    /* load everything */
    let mut pk = ram_ppzksnark_proving_key::<default_tinyram_ppzksnark_pp>::default();
    let mut pk_file = proving_key_fn.lines();
    // pk_file >> pk;

    let mut processed = processed_assembly_fn;
    let mut program = load_preprocessed_program(&pk.ap, &processed);

    let mut f_primary_input = primary_input_fn;
    let mut f_auxiliary_input = auxiliary_input_fn;
    let primary_input = load_tape(&f_primary_input);
    let auxiliary_input = load_tape(&f_auxiliary_input);

    let boot_trace = tinyram_boot_trace_from_program_and_input(
        &pk.ap,
        pk.primary_input_size_bound,
        &program,
        &primary_input,
    );
    let proof =
        ram_ppzksnark_prover::<default_tinyram_ppzksnark_pp>(&pk, &boot_trace, &auxiliary_input);

    // std::ofstream proof_file(proof_fn);
    // proof_file << proof;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    let mut proof_file = OpenOptions::new().append(true).open(proof_fn)?;
    writeln!(proof_file, "{}", proof)?;
    Ok(0)
}
