use crate::common::default_types::r1cs_ppzkadsnark_pp::default_r1cs_ppzkadsnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::generate_r1cs_example_with_field_input;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::run_r1cs_ppzkadsnark::run_r1cs_ppzkadsnark;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    ppzkadsnarkConfig, snark_pp,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf::PrfConfig;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature::SigConfig;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::FieldTConfig;
use ffec::Fp_modelConfig;
use ffec::PpConfig;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::div_ceil;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};

pub fn main<ppT: ppzkadsnarkConfig>(argc: i32, argv: &[&str]) -> i32
where
    knowledge_commitment<
        <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
        <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as ppzkadsnarkConfig>::snark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
                <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
        <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<ppT as ppzkadsnarkConfig>::snark_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
                <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
            >,
        >, // where
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //         >,
           //     ED: evaluation_domain<<<ppT as ppzkadsnarkConfig>::snark_pp as PublicParams>::Fr>,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1:
           //         Add<Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1>,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2:
           //         Add<Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2>,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2: Mul<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2: FieldTConfig,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2: Mul<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2: Add<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr:
           //         Mul<Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr>,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     for<'a> <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             &'a FieldT,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //         >,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             knowledge_commitment<
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             >,
           //             Output = knowledge_commitment<
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             >,
           //         >,
           //     <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             knowledge_commitment<
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             >,
           //             Output = knowledge_commitment<
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //                 <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             >,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G1,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::G2,
           //         >,
           //     for<'a> &'a <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Add<
           //             <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //         >,
           //     for<'a> <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr: Mul<
           //             &'a FieldT,
           //             Output = <<ppT as ppzkadsnarkConfig>::snark_pp as ff_curves::PublicParams>::Fr,
           //         >,
{
    <ppT as ff_curves::PublicParams>::init_public_params();
    // start_profiling();

    if argc == 2 && argv[1] == "-v" {
        // print_compilation_info();
        return 0;
    }

    if argc != 3 && argc != 4 {
        print!("usage: {} num_constraints input_size [Fr|bytes]\n", argv[0]);
        return 1;
    }
    let num_constraints = argv[1].parse::<usize>().unwrap();
    let mut input_size = argv[2].parse::<usize>().unwrap();
    if argc == 4 {
        assert!(argv[3] == "Fr" || argv[3] == "bytes");
        if argv[3] == "bytes" {
            input_size = div_ceil((8 * input_size), Fr::<snark_pp<ppT>>::num_bits() - 1).unwrap();
        }
    }

    enter_block("Generate R1CS example", false);
    let example = generate_r1cs_example_with_field_input::<
        Fr<snark_pp<ppT>>,
        pb_variable,
        pb_linear_combination,
    >(num_constraints, input_size);
    leave_block("Generate R1CS example", false);

    println!("(enter) Profile R1CS ppzkADSNARK");
    let test_serialization = true;
    run_r1cs_ppzkadsnark::<ppT>(example, test_serialization);
    println!("(leave) Profile R1CS ppzkADSNARK");
    0
}
