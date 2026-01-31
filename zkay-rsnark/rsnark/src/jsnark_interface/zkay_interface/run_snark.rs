use crate::jsnark_interface::circuit_reader::{CircuitReader, FieldT};
// use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;
// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
// use crate::zk_proof_systems::ppzksnark::r1cs_se_ppzksnark::r1cs_se_ppzksnark;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::gadgetlib2::adapters::GLA;
use crate::gadgetlib2::adapters::GadgetLibAdapter;
use crate::gadgetlib2::integration::{
    get_constraint_system_from_gadgetlib2, get_variable_assignment_from_gadgetlib2,
};
use crate::gadgetlib2::pp::initPublicParamsFromDefaultPp;
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::FieldType;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint_system;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_primary_input,
};
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark::{
    r1cs_gg_ppzksnark_generator, r1cs_gg_ppzksnark_keypair, r1cs_gg_ppzksnark_proof,
    r1cs_gg_ppzksnark_prover, r1cs_gg_ppzksnark_proving_key, r1cs_gg_ppzksnark_verification_key,
    r1cs_gg_ppzksnark_verifier_strong_IC,
};
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_generator, r1cs_ppzksnark_keypair, r1cs_ppzksnark_proof, r1cs_ppzksnark_prover,
    r1cs_ppzksnark_proving_key, r1cs_ppzksnark_verification_key, r1cs_ppzksnark_verifier_strong_IC,
};
use crate::zk_proof_systems::ppzksnark::r1cs_se_ppzksnark::r1cs_se_ppzksnark::{
    r1cs_se_ppzksnark_generator, r1cs_se_ppzksnark_keypair, r1cs_se_ppzksnark_proof,
    r1cs_se_ppzksnark_prover, r1cs_se_ppzksnark_proving_key, r1cs_se_ppzksnark_verification_key,
    r1cs_se_ppzksnark_verifier_strong_IC,
};
use crate::zk_proof_systems::ppzksnark::{
    KeyPairTConfig, ProofTConfig, ProvingKeyTConfig, VerificationKeyTConfig,
};
use ff_curves::FpmConfig;
use ff_curves::algebra::curves::alt_bn128::alt_bn128_fields::alt_bn128_Fq;
use ff_curves::default_ec_pp;
use ffec::PpConfig;
use ffec::common::profiling::{enter_block, leave_block, start_profiling};
use std::fs;
use std::fs::File;

use num_enum::{FromPrimitive, IntoPrimitive};
use strum::Display;
#[derive(Display, Debug, Default, Clone, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum ProvingScheme {
    PGHR13,
    #[default]
    GROTH16,
    GM17,
}
use std::io::{BufWriter, Write};
// External interface
// extern "C" {
// int generate_keys(input_directory:&str, output_directory:&str, proving_scheme:i32);
// int generate_proof(keys_dir:&str, input_dir:&str, output_filename:&str, proving_scheme:i32, check_verification:i32);
// }

type ppT = default_ec_pp;
type G1 = <ppT as ff_curves::PublicParams>::G1;
type G2 = <ppT as ff_curves::PublicParams>::G2;

fn serialize(mut pt: G1) -> String {
    let num_limbs = alt_bn128_Fq::num_limbs;
    pt.to_affine_coordinates();

    let mut buf = [b' '; 256];
    {
        let mut stream = BufWriter::new(buf.as_mut());
        write!(
            stream,
            "{:num_limbs$x}\n{:num_limbs$x}\n",
            pt.X().as_bigint().0.0[0],
            pt.Y().as_bigint().0.0[0]
        );
    }
    buf.into_iter().map(|b| b as char).collect()
}

fn serialize2(mut pt: G2) -> String {
    let num_limbs = alt_bn128_Fq::num_limbs;
    pt.to_affine_coordinates();
    let mut buf = [b' '; 512];
    {
        let mut stream = BufWriter::new(buf.as_mut());
        write!(
            stream,
            "{:num_limbs$x}\n{:num_limbs$x}\n{:num_limbs$x}\n{:num_limbs$x}\n",
            pt.X.c1.as_bigint().0.0[0],
            pt.X.c0.as_bigint().0.0[0],
            pt.Y.c1.as_bigint().0.0[0],
            pt.Y.c0.as_bigint().0.0[0],
        );
    }
    buf.into_iter().map(|b| b as char).collect()
}

fn writeToFile<T>(path: &str, obj: &T) {
    // std::ofstream fh(path, std::ios::binary);
    // fh << obj;
}

fn loadFromFile<T: Default>(path: &str) -> T {
    // std::ifstream fh(path, std::ios::binary);
    // T obj;
    // fh >> obj;
    // return obj;
    T::default()
}
struct VkSerializer;
trait VkSerializeConfig<KeyPairT: KeyPairTConfig> {
    fn serialize_vk(vk_out: &mut String, kp: &KeyPairT);
}
impl VkSerializeConfig<r1cs_ppzksnark_keypair<ppT>> for VkSerializer {
    fn serialize_vk(vk_out: &mut String, kp: &r1cs_ppzksnark_keypair<ppT>) {
        let vk = &kp.vk;
        vk_out.push_str(&serialize2(vk.alphaA_g2.clone()));
        vk_out.push_str(&serialize(vk.alphaB_g1.clone()));
        vk_out.push_str(&serialize2(vk.alphaC_g2.clone()));
        vk_out.push_str(&serialize2(vk.gamma_g2.clone()));
        vk_out.push_str(&serialize(vk.gamma_beta_g1.clone()));
        vk_out.push_str(&serialize2(vk.gamma_beta_g2.clone()));
        vk_out.push_str(&serialize2(vk.rC_Z_g2.clone()));

        let IC = vk.encoded_IC_query.clone();
        vk_out.push_str(&format!("{}\n", IC.len() + 1));
        vk_out.push_str(&serialize(IC.first.clone()));
        for i in 0..IC.len() {
            let IC_N = IC.rest[i].clone();
            vk_out.push_str(&serialize(IC_N));
        }
    }
}
impl VkSerializeConfig<r1cs_gg_ppzksnark_keypair<ppT>> for VkSerializer {
    fn serialize_vk(vk_out: &mut String, kp: &r1cs_gg_ppzksnark_keypair<ppT>) {
        let (vk, pk) = (&kp.vk, &kp.pk);
        vk_out.push_str(&serialize(pk.alpha_g1.clone()));
        vk_out.push_str(&serialize2(pk.beta_g2.clone()));
        vk_out.push_str(&serialize2(vk.gamma_g2.clone()));
        vk_out.push_str(&serialize2(vk.delta_g2.clone()));

        let abc = vk.gamma_ABC_g1.clone();
        vk_out.push_str(&format!("{}\n", abc.len() + 1));
        vk_out.push_str(&serialize(abc.first.clone()));
        for i in 0..abc.len() {
            let abc_n = abc.rest[i].clone();
            vk_out.push_str(&serialize(abc_n.clone()));
        }
    }
}

impl VkSerializeConfig<r1cs_se_ppzksnark_keypair<ppT>> for VkSerializer {
    fn serialize_vk(vk_out: &mut String, kp: &r1cs_se_ppzksnark_keypair<ppT>) {
        let vk = &kp.vk;
        vk_out.push_str(&serialize2(vk.H.clone()));
        vk_out.push_str(&serialize(vk.G_alpha.clone()));
        vk_out.push_str(&serialize2(vk.H_beta.clone()));
        vk_out.push_str(&serialize(vk.G_gamma.clone()));
        vk_out.push_str(&serialize2(vk.H_gamma.clone()));

        vk_out.push_str(&format!("{}\n", vk.query.len()));
        for q in &vk.query {
            vk_out.push_str(&serialize(q.clone()));
        }
    }
}

// impl<KeyPairT: KeyPairTConfig> VkSerializeConfig<KeyPairT::PK, KeyPairT::VK>
//     for VkSerializer
// {
fn serialize_vk<KeyPairT: KeyPairTConfig>(vk_out: &mut String, kp: &KeyPairT)
where
    VkSerializer: VkSerializeConfig<KeyPairT>,
{
    VkSerializer::serialize_vk(vk_out, kp)
}
// }

struct ProofSerializer;
trait ProofSerializeConfig<ProofT: ProofTConfig> {
    fn serialize_proof(p_out: &mut String, p: &ProofT);
}
impl ProofSerializeConfig<r1cs_ppzksnark_proof<ppT>> for ProofSerializer {
    fn serialize_proof(p_out: &mut String, p: &r1cs_ppzksnark_proof<ppT>) {
        p_out.push_str(&serialize(p.g_A.g.clone()));
        p_out.push_str(&serialize(p.g_A.h.clone()));
        p_out.push_str(&serialize2(p.g_B.g.clone()));
        p_out.push_str(&serialize(p.g_B.h.clone()));
        p_out.push_str(&serialize(p.g_C.g.clone()));
        p_out.push_str(&serialize(p.g_C.h.clone()));
        p_out.push_str(&serialize(p.g_K.clone()));
        p_out.push_str(&serialize(p.g_H.clone()));
    }
}
impl ProofSerializeConfig<r1cs_gg_ppzksnark_proof<ppT>> for ProofSerializer {
    fn serialize_proof(p_out: &mut String, p: &r1cs_gg_ppzksnark_proof<ppT>) {
        p_out.push_str(&serialize(p.g_A.clone()));
        p_out.push_str(&serialize2(p.g_B.clone()));
        p_out.push_str(&serialize(p.g_C.clone()));
    }
}
impl ProofSerializeConfig<r1cs_se_ppzksnark_proof<ppT>> for ProofSerializer {
    fn serialize_proof(p_out: &mut String, p: &r1cs_se_ppzksnark_proof<ppT>) {
        p_out.push_str(&serialize(p.A.clone()));
        p_out.push_str(&serialize2(p.B.clone()));
        p_out.push_str(&serialize(p.C.clone()));
    }
}

fn keygen<KeyPairT: KeyPairTConfig, F>(
    cs: &r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
    prover_key_filename: &String,
    verification_key_filename: &String,
    generate: F,
) where
    F: for<'a> Fn(
        &'a r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
    ) -> KeyPairT,
    <KeyPairT as KeyPairTConfig>::VK: VerificationKeyTConfig,
    <KeyPairT as KeyPairTConfig>::PK: ProvingKeyTConfig,
    VkSerializer: VkSerializeConfig<KeyPairT>,
{
    // Generate keypair
    let keypair = generate(cs);

    // Dump proving key to binary file
    enter_block("WritingProverKey", false);
    writeToFile(prover_key_filename, keypair.pk());
    leave_block("WritingProverKey", false);

    // Dump verification key in text format
    enter_block("SerializeVk", false);
    let mut vk_out = fs::read_to_string(verification_key_filename).unwrap();
    serialize_vk::<KeyPairT>(&mut vk_out, &keypair);
    leave_block("SerializeVk", false);

    // Also dump in binary format for local verification
    writeToFile(
        &(verification_key_filename.to_owned() + ".bin"),
        keypair.vk(),
    );
}

// template<ProofT, ProvingKeyT,
//         ProofT (*prove)():&ProvingKeyT &, const &, const
//         VerificationKeyT,
//         bool (*verify)(:&VerificationKeyT &, &:r1cs_primary_input<FieldT> const ProofT)>
fn proofgen<
    ProofT: ProofTConfig,
    ProvingKeyT: ProvingKeyTConfig,
    F,
    VerificationKeyT: VerificationKeyTConfig,
    F2,
>(
    public_inputs: &r1cs_primary_input<FieldT>,
    private_inputs: &r1cs_auxiliary_input<FieldT>,
    prover_key_filename: &str,
    verification_key_filename: &str,
    proof_filename: &str,
    check_verification: bool,
    prove: F,
    verify: F2,
) -> bool
where
    F: Fn(ProvingKeyT, r1cs_primary_input<FieldT>, r1cs_auxiliary_input<FieldT>) -> ProofT,
    F2: Fn(VerificationKeyT, r1cs_primary_input<FieldT>, ProofT) -> bool,
    ProofSerializer: ProofSerializeConfig<ProofT>,
{
    let mut proof;
    {
        // Read proving key
        enter_block("ReadingProverKey", false);
        let pk = loadFromFile::<ProvingKeyT>(prover_key_filename);
        leave_block("ReadingProverKey", false);

        // Generate proof
        proof = prove(pk, public_inputs.clone(), private_inputs.clone());
    }

    // Dump proof in text format
    enter_block("SerializeProof", false);
    let mut p = proof_filename.to_string();
    ProofSerializer::serialize_proof(&mut p, &proof);
    leave_block("SerializeProof", false);

    if check_verification {
        // Check if verification works
        let vk = loadFromFile::<VerificationKeyT>(verification_key_filename);

        enter_block("Verifying proof", false);
        let ans = verify(vk, public_inputs.clone(), proof);
        println!("\n");
        println!(
            "* The verification result is: {}\n",
            if ans { "PASS" } else { "FAIL" }
        );
        leave_block("Verifying proof", false);
        return ans;
    }
    true
}

fn generate_keys(input_directory: &str, output_directory: &str, proving_scheme: u8) -> i32 {
    start_profiling();
    initPublicParamsFromDefaultPp();
    <GLA as GadgetLibAdapter>::resetVariableIndex();

    let ps = ProvingScheme::from(proving_scheme);
    let in_dir = input_directory;
    let out_dir = output_directory;

    let arith_filename = in_dir.to_owned() + "/circuit.arith";
    let dummy_input_filename = in_dir.to_owned() + "/circuit.in";
    let prover_key_filename = out_dir.to_owned() + "/proving.key";
    let verification_key_filename = out_dir.to_owned() + "/verification.key";

    // Read the circuit, evaluate, and translate constraints
    let mut cs;
    {
        let pb = Protoboard::create(FieldType::R1P, None);
        enter_block("CircuitReading", false);
        let reader = CircuitReader::new(&arith_filename, &dummy_input_filename, pb.clone());
        leave_block("CircuitReading", false);
        enter_block("Extract constraint system", false);
        cs = get_constraint_system_from_gadgetlib2(&pb.as_ref().unwrap().borrow());
        cs.primary_input_size = (reader.getNumInputs() + reader.getNumOutputs()) as usize;
        cs.auxiliary_input_size = <GLA as GadgetLibAdapter>::getNextFreeIndex() - cs.num_inputs();
        leave_block("Extract constraint system", false);
    }

    match ps {
        ProvingScheme::PGHR13 => {
            println!("PGHR13 Generator"); //r1cs_ppzksnark_generator<ppT>
            keygen::<r1cs_ppzksnark_keypair<ppT>, _>(
                &cs,
                &prover_key_filename,
                &verification_key_filename,
                r1cs_ppzksnark_generator::<ppT>,
            );
        }
        ProvingScheme::GROTH16 => {
            println!("Groth16 Generator");
            keygen::<r1cs_gg_ppzksnark_keypair<ppT>, _>(
                &cs,
                &prover_key_filename,
                &verification_key_filename,
                r1cs_gg_ppzksnark_generator::<ppT>,
            );
        }
        ProvingScheme::GM17 => {
            println!("GM17 Generator");
            keygen::<r1cs_se_ppzksnark_keypair<ppT>, _>(
                &cs,
                &prover_key_filename,
                &verification_key_filename,
                r1cs_se_ppzksnark_generator::<ppT>,
            );
        }
        _ => {
            return -1;
        }
    }

    0
}

fn generate_proof(
    keys_dir: &str,
    input_dir: &str,
    output_filename: &str,
    proving_scheme: u8,
    check_verification: bool,
) -> i32 {
    start_profiling();
    initPublicParamsFromDefaultPp();
    <GLA as GadgetLibAdapter>::resetVariableIndex();

    let ps = ProvingScheme::from(proving_scheme);
    let in_dir = input_dir;
    let key_dir = keys_dir;

    let arith_filename = in_dir.to_owned() + "/circuit.arith";
    let in_filename = in_dir.to_owned() + "/circuit.in";
    let prover_key_filename = key_dir.to_owned() + "/proving.key";
    let verification_key_filename = key_dir.to_owned() + "/verification.key.bin";

    let mut primary_input = r1cs_primary_input::<FieldT>::default();
    let mut auxiliary_input = r1cs_auxiliary_input::<FieldT>::default();
    {
        let mut cs;
        {
            // Read the circuit, evaluate, and translate constraints
            enter_block("CircuitReading", false);
            let mut full_assignment;
            {
                let mut pb = Protoboard::create(FieldType::R1P, None);
                let mut primary_input_size;
                {
                    let reader = CircuitReader::new(&arith_filename, &in_filename, pb.clone());
                    primary_input_size = (reader.getNumInputs() + reader.getNumOutputs()) as usize;
                }
                cs = get_constraint_system_from_gadgetlib2::<pb_variable, pb_linear_combination>(
                    &pb.as_ref().unwrap().borrow(),
                );
                full_assignment = get_variable_assignment_from_gadgetlib2::<
                    pb_variable,
                    pb_linear_combination,
                >(&pb.as_ref().unwrap().borrow());
                cs.primary_input_size = primary_input_size;
                cs.auxiliary_input_size = full_assignment.len() - primary_input_size;
                leave_block("CircuitReading", false);
            }

            // extract primary and auxiliary input
            primary_input = full_assignment[..cs.num_inputs()].to_vec();
            auxiliary_input = full_assignment[cs.num_inputs()..].to_vec();
        }

        if !cs.is_satisfied(&primary_input, &auxiliary_input) {
            println!(
                "The constraint system is not satisfied by the value assignment - Terminating."
            );
            return -2;
        }
    }

    let ret;
    match ps {
        ProvingScheme::PGHR13 => {
            println!("PGHR13 Prover");
            ret = proofgen::<
                r1cs_ppzksnark_proof<ppT>,
                r1cs_ppzksnark_proving_key<ppT>,
                _,
                r1cs_ppzksnark_verification_key<ppT>,
                _,
            >(
                &primary_input,
                &auxiliary_input,
                &prover_key_filename,
                &verification_key_filename,
                output_filename,
                check_verification,
                |pk: r1cs_ppzksnark_proving_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 auxiliary_input: Vec<FieldT>| {
                    r1cs_ppzksnark_prover::<ppT>(&pk, &primary_input, &auxiliary_input)
                },
                |vk: r1cs_ppzksnark_verification_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 proof: r1cs_ppzksnark_proof<default_ec_pp>| {
                    r1cs_ppzksnark_verifier_strong_IC::<ppT>(&vk, &primary_input, &proof)
                },
            );
        }
        ProvingScheme::GROTH16 => {
            println!("Groth16 Prover");
            ret = proofgen::<
                r1cs_gg_ppzksnark_proof<ppT>,
                r1cs_gg_ppzksnark_proving_key<ppT>,
                _,
                r1cs_gg_ppzksnark_verification_key<ppT>,
                _,
            >(
                &primary_input,
                &auxiliary_input,
                &prover_key_filename,
                &verification_key_filename,
                output_filename,
                check_verification,
                |pk: r1cs_gg_ppzksnark_proving_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 auxiliary_input: Vec<FieldT>| {
                    r1cs_gg_ppzksnark_prover::<ppT>(&pk, &primary_input, &auxiliary_input)
                },
                |vk: r1cs_gg_ppzksnark_verification_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 proof: r1cs_gg_ppzksnark_proof<default_ec_pp>| {
                    r1cs_gg_ppzksnark_verifier_strong_IC::<ppT>(&vk, &primary_input, &proof)
                },
            );
        }
        ProvingScheme::GM17 => {
            println!("GM17 Prover");
            ret = proofgen::<
                r1cs_se_ppzksnark_proof<ppT>,
                r1cs_se_ppzksnark_proving_key<ppT>,
                _,
                r1cs_se_ppzksnark_verification_key<ppT>,
                _,
            >(
                &primary_input,
                &auxiliary_input,
                &prover_key_filename,
                &verification_key_filename,
                output_filename,
                check_verification,
                |pk: r1cs_se_ppzksnark_proving_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 auxiliary_input: Vec<FieldT>| {
                    r1cs_se_ppzksnark_prover::<ppT>(&pk, &primary_input, &auxiliary_input)
                },
                |vk: r1cs_se_ppzksnark_verification_key<default_ec_pp>,
                 primary_input: Vec<FieldT>,
                 proof: r1cs_se_ppzksnark_proof<default_ec_pp>| {
                    r1cs_se_ppzksnark_verifier_strong_IC::<ppT>(&vk, &primary_input, &proof)
                },
            );
        }
        _ => {
            return -1;
        }
    }
    if ret { 0 } else { -2 }
}

fn main(argc: i32, argv: Vec<String>) -> i32 {
    if argc < 5 {
        eprintln!("Invalid command");
        return -1;
    }
    let in_dir = &argv[2];
    let out_path = &argv[3];
    if argc == 5 && "keygen" == argv[1] {
        generate_keys(&in_dir, &out_path, argv[4].parse::<u8>().unwrap())
    } else if argc == 7 && "proofgen" == argv[1] {
        let key_dir = &argv[4];
        generate_proof(
            key_dir,
            &in_dir,
            &out_path,
            argv[5].parse::<u8>().unwrap(),
            argv[6].parse::<bool>().unwrap(),
        )
    } else {
        eprintln!("Invalid command");
        -1
    }
}
