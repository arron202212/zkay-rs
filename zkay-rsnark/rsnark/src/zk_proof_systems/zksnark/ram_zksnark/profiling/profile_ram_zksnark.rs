// use common::profiling;

// use crate::common::default_types::ram_zksnark_pp;
// use crate::relations::ram_computations::memory::examples::memory_contents_examples;
// use crate::relations::ram_computations::rams::examples::ram_examples;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
// use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;
// use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark;
use crate::common::data_structures::merkle_tree::HashTConfig;
use crate::common::default_types::r1cs_ppzkpcd_pp::default_r1cs_ppzkpcd_ppConfig;
use crate::common::default_types::ram_zksnark_pp::default_ram_zksnark_pp;
use crate::common::default_types::tinyram_ppzksnark_pp::default_tinyram_ppzksnark_ppConfig;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::ram_computations::memory::delegated_ra_memory::delegated_ra_memory;
use crate::relations::ram_computations::memory::examples::memory_contents_examples::random_memory_contents;
use crate::relations::ram_computations::rams::examples::ram_examples::gen_ram_example_complex;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_architecture_params;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::CPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark::run_ram_zksnark;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark::{
    ram_zksnark_proof, ram_zksnark_verification_key, ram_zksnark_verifier,
};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::ram_zksnark_PCD_pp;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::ram_zksnark_machine_pp;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::common::profiling::{
    enter_block, last_times, leave_block, print_compilation_info, start_profiling,
};
use std::ops::Mul;

pub fn simulate_random_memory_contents<FieldT: HashTConfig>(
    ap: tinyram_architecture_params,
    input_size: usize,
    program_size: usize,
) {
    let num_addresses = 1usize << ap.dwaddr_len();
    let value_size = 2 * ap.w;
    let init_random = random_memory_contents(
        num_addresses,
        value_size,
        program_size + (input_size + 1) / 2,
    );

    enter_block("Initialize random delegated memory", false);
    let dm_random = delegated_ra_memory::<FieldT>::new3(num_addresses, value_size, init_random);
    leave_block("Initialize random delegated memory", false);
}
// type RamT=ram_zksnark_machine_pp<ppT> ;
pub fn profile_ram_zksnark_verifier<ppT: default_tinyram_ppzksnark_ppConfig>(
    ap: tinyram_architecture_params,
    input_size: usize,
    program_size: usize,
) {
    let time_bound = 10;

    let boot_trace_size_bound = program_size + input_size;
    let example =
        gen_ram_example_complex::<RamT<ppT>>(ap.clone(), boot_trace_size_bound, time_bound, true);

    let mut pi = ram_zksnark_proof::<ppT>::default();
    let vk = ram_zksnark_verification_key::<ppT>::dummy_verification_key(&ap);

    enter_block("Verify fake proof", false);
    ram_zksnark_verifier::<ppT>(&vk, &example.boot_trace, time_bound, &pi);
    leave_block("Verify fake proof", false);
}

pub fn print_ram_zksnark_verifier_profiling<ppT: default_tinyram_ppzksnark_ppConfig>()
where
    <<ppT as RamConfig>::machine_pp as ram_params_type>::base_field_type: HashTConfig,
{
    // inhibit_profiling_info = true;
    for &w in &[16, 32] {
        let k = 16;

        for &input_size in &[0, 10, 100] {
            let mut program_size = 10;
            while program_size <= 10000 {
                let ap = tinyram_architecture_params::new(w, k);

                profile_ram_zksnark_verifier::<ppT>(ap.clone(), input_size, program_size);

                let input_map = last_times("Call to ram_zksnark_verifier_input_map");
                let preprocessing = last_times("Call to r1cs_ppzksnark_verifier_process_vk");
                let accumulate = last_times("Call to r1cs_ppzksnark_IC_query::accumulate");
                let pairings = last_times("Online pairing computations");
                let total = last_times("Call to ram_zksnark_verifier");
                let rest = total - (input_map + preprocessing + accumulate + pairings);

                let delegated_ra_memory_init =
                    last_times("Construct delegated_ra_memory from memory map");
                simulate_random_memory_contents::<Fr<<ppT as PcdPptConfig>::curve_A_pp>>(
                    ap,
                    input_size,
                    program_size,
                );
                let delegated_ra_memory_init_random =
                    last_times("Initialize random delegated memory");
                let input_map_random =
                    input_map - delegated_ra_memory_init + delegated_ra_memory_init_random;
                let total_random =
                    total - delegated_ra_memory_init + delegated_ra_memory_init_random;

                print!(
                    "w = {}, k = {}, program_size = {}, input_size = {}, input_map = {:.2}ms, preprocessing = {:.2}ms, accumulate = {:.2}ms, pairings = {:.2}ms, rest = {:.2}ms, total = {:.2}ms (input_map_random = {:.2}ms, total_random = {:.2}ms)\n",
                    w,
                    k,
                    program_size,
                    input_size,
                    input_map * 1e-6,
                    preprocessing * 1e-6,
                    accumulate * 1e-6,
                    pairings * 1e-6,
                    rest * 1e-6,
                    total * 1e-6,
                    input_map_random * 1e-6,
                    total_random * 1e-6
                );
                program_size *= 10;
            }
        }
    }
}
type RamT<ppT> = ram_zksnark_machine_pp<ppT>;
pub fn profile_ram_zksnark<ppT: default_tinyram_ppzksnark_ppConfig>(
    ap: tinyram_architecture_params,
    program_size: usize,
    input_size: usize,
    time_bound: usize,
) where
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
    let boot_trace_size_bound = program_size + input_size;
    let example = gen_ram_example_complex::<RamT<ppT>>(ap, boot_trace_size_bound, time_bound, true);
    let test_serialization = true;
    let bit = run_ram_zksnark::<ppT>(&example, test_serialization);
    assert!(bit);
}

fn process_command_line(
    argc: i32,
    argv: &[&str],
    profile_gp: &mut bool,
    w: &mut usize,
    k: &mut usize,
    profile_v: &mut bool,
    l: &mut usize,
) -> bool {
    // po::options_description desc("Usage");
    // desc.add_options()
    //     ("help", "print this help message")
    //     ("profile_gp", "profile generator and prover")
    //     ("w", po::value<usize>(&w)->default_value(16), "word size")
    //     ("k", po::value<usize>(&k)->default_value(16), "register count")
    //     ("profile_v", "profile verifier")
    //     ("v", "print version info")
    //     ("l", po::value<usize>(&l)->default_value(10), "program length");

    // po::variables_map vm;
    // po::store(po::parse_command_line(argc, argv, desc), vm);

    // if vm.count("v")
    // {
    //     print_compilation_info();
    //     exit(0);
    // }

    // if vm.count("help")
    // {
    //     std::cout << desc << "\n";
    //     return false;
    // }

    // profile_gp = vm.count("profile_gp");
    // profile_v = vm.count("profile_v");

    // if !(vm.count("profile_gp") ^ vm.count("profile_v"))
    // {
    //     std::cout << "Must choose between profiling generator/prover and profiling verifier (see --help)\n";
    //     return false;
    // }

    // po::notify(vm);
    use clap::{Arg, ArgAction, ArgGroup, Command};

    let matches = Command::new("Usage")
        .version("1.0")
        .about("Profile generator/prover and verifier")
        // 1. 定义参数
        .arg(
            Arg::new("profile_gp")
                .long("profile_gp")
                .help("profile generator and prover")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("profile_v")
                .long("profile_v")
                .help("profile verifier")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("w")
                .short('w')
                .help("word size")
                .default_value("16")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("k")
                .short('k')
                .help("register count")
                .default_value("16")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("l")
                .short('l')
                .help("program length")
                .default_value("10")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .help("print version info")
                .action(ArgAction::SetTrue),
        )
        // 2. 核心逻辑：强制 profile_gp 和 profile_v 互斥且必选其一
        .group(
            ArgGroup::new("mode")
                .args(["profile_gp", "profile_v"])
                .required(true),
        )
        .get_matches();

    // 3. 处理版本信息 (对应 vm.count("v"))
    if matches.get_flag("v") {
        println!("Compilation info: ...");
        std::process::exit(0);
    }

    // 4. 读取解析后的数值 (对应变量绑定)
    let w: usize = *matches.get_one::<usize>("w").expect("has default");
    let k: usize = *matches.get_one::<usize>("k").expect("has default");
    let l: usize = *matches.get_one::<usize>("l").expect("has default");

    let profile_gp = matches.get_flag("profile_gp");
    let profile_v = matches.get_flag("profile_v");

    println!(
        "Config: w={}, k={}, l={}, gp={}, v={}",
        w, k, l, profile_gp, profile_v
    );

    true
}
// trait default_tinyram_zksnark_ppConfig {}
fn main<default_ram_zksnark_pp: default_tinyram_ppzksnark_ppConfig>(
    argc: i32,
    argv: &[&str],
) -> i32 where
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

    let mut profile_gp = false;
    let mut w = 0;
    let mut k = 0;
    let mut profile_v = false;
    let mut l = 0;

    if !process_command_line(
        argc,
        argv,
        &mut profile_gp,
        &mut w,
        &mut k,
        &mut profile_v,
        &mut l,
    ) {
        return 1;
    }

    let mut ap = tinyram_architecture_params::new(w, k);

    if profile_gp {
        profile_ram_zksnark::<default_ram_zksnark_pp>(ap.clone(), 100, 100, 10); // w, k, l, n, T
    }

    if profile_v {
        profile_ram_zksnark_verifier::<default_ram_zksnark_pp>(ap, l / 2, l / 2);
    }
    0
}
