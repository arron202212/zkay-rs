

use  crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
use  crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;
use  crate::zk_proof_systems::ppzksnark::r1cs_se_ppzksnark::r1cs_se_ppzksnark;
use  crate::jsnark_interface::circuit_reader;

enum ProvingScheme {
    PGHR13,
    GROTH16,
    GM17
}
use std::io::{BufWriter, Write};
// External interface
// extern "C" {
// int generate_keys(input_directory:&str, output_directory:&str, proving_scheme:i32);
// int generate_proof(keys_dir:&str, input_dir:&str, output_filename:&str, proving_scheme:i32, check_verification:i32);
// }

type   ppT=ffec::default_ec_pp ;

fn   serialize( pt:ffec::G1<ppT>)->String {
    let  num_limbs = ffec::alt_bn128_Fq::num_limbs;
    pt.to_affine_coordinates();
   


 let mut buf=[' ';256];
let mut stream = BufWriter::new(buf.as_mut());
    write!(stream, "{:num_limbs$x}\n{:num_limbs$x}\n", pt.X.as_bigint().data, pt.Y.as_bigint().data);
    buf.into_iter().collect()
}

fn serialize2( pt:ffec::G2<ppT>)->String  {
    let  num_limbs = ffec::alt_bn128_Fq::num_limbs;
    pt.to_affine_coordinates();
    let mut buf=[' ';512];
let mut stream = BufWriter::new(buf.as_mut());
    write!(stream, "{:num_limbs$x}\n{:num_limbs$x}\n{:num_limbs$x}\n{:num_limbs$x}\n",
                 pt.X.c1.as_bigint().data,  pt.X.c0.as_bigint().data, 
                 pt.Y.c1.as_bigint().data,  pt.Y.c0.as_bigint().data,);
     buf.into_iter().collect()
}


fn  writeToFile<T>(path:&str, obj:&T) {
    // std::ofstream fh(path, std::ios::binary);
    // fh << obj;
}


fn  loadFromFile<T>(path:&str)->T{
    // std::ifstream fh(path, std::ios::binary);
    // T obj;
    // fh >> obj;
    // return obj;
}

fn serialize_vk3( vk_out:&mut String,vk:&r1cs_ppzksnark_verification_key<ppT>,
                         _: &r1cs_ppzksnark_proving_key<ppT> ) {
    vk_out.push_str(serialize(vk.alphaA_g2));
    vk_out.push_str( serialize(vk.alphaB_g1));
    vk_out.push_str( serialize(vk.alphaC_g2));
    vk_out.push_str( serialize(vk.gamma_g2));
    vk_out.push_str( serialize(vk.gamma_beta_g1));
    vk_out.push_str( serialize(vk.gamma_beta_g2));
    vk_out.push_str( serialize(vk.rC_Z_g2));

    let IC = vk.encoded_IC_query;
    vk_out.push_str( format!("{}\n",IC.len() + 1 ));
    vk_out.push_str( serialize(IC.0));
    for i in 0.. IC.len() {
        let  IC_N=IC.rest[i].clone();
        vk_out.push_str( serialize(IC_N));
    }
}

fn serialize_vk(vk_out:&mut String, vk:& r1cs_gg_ppzksnark_verification_key<ppT> ,
                         pk:& r1cs_gg_ppzksnark_proving_key<ppT> ) {
    vk_out.push_str( serialize(pk.alpha_g1));
    vk_out.push_str( serialize(pk.beta_g2));
    vk_out.push_str( serialize(vk.gamma_g2));
    vk_out.push_str( serialize(vk.delta_g2));

    let abc = vk.gamma_ABC_g1;
    vk_out.push_str( format!("{}\n",abc.len() + 1));
    vk_out.push_str( serialize(abc.first));
    for i in 0.. abc.len() {
        let  abc_n=abc.rest[i].clone();
        vk_out.push_str( serialize(abc_n));
    }
}

fn serialize_vk2(vk_out:&mut String, vk:& r1cs_se_ppzksnark_verification_key<ppT>,
                         _:& r1cs_se_ppzksnark_proving_key<ppT> ) {
    vk_out.push_str( serialize(vk.H));
    vk_out.push_str( serialize(vk.G_alpha));
    vk_out.push_str( serialize(vk.H_beta));
    vk_out.push_str( serialize(vk.G_gamma));
    vk_out.push_str( serialize(vk.H_gamma));

    vk_out.push_str( format!("{}\n",vk.query.len()));
    for q in  vk.query {
        vk_out.push_str( serialize(q));
    }
}

fn serialize_proof3(p_out:&mut String,p:& r1cs_ppzksnark_proof<ppT>) {
    p_out.push_str(serialize(p.g_A.g));
    p_out.push_str(serialize(p.g_A.h));
    p_out.push_str(serialize(p.g_B.g));
    p_out.push_str(serialize(p.g_B.h));
    p_out.push_str(serialize(p.g_C.g));
    p_out.push_str(serialize(p.g_C.h));
    p_out.push_str(serialize(p.g_K));
    p_out.push_str(serialize(p.g_H));
}

fn serialize_proof(p_out:&mut String,p:& r1cs_gg_ppzksnark_proof<ppT>) {
    p_out.push_str(serialize(p.g_A));
    p_out.push_str(serialize(p.g_B));
    p_out.push_str(serialize(p.g_C));
}

fn serialize_proof2(p_out:&mut String ,p:& r1cs_se_ppzksnark_proof<ppT>) {
    p_out.push_str(serialize(p.A));
    p_out.push_str(serialize(p.B));
    p_out.push_str(serialize(p.C));
}


fn keygen<KeyPairT,F:Fn(&r1cs_constraint_system<FieldT>)->KeyPairT>(cs:& r1cs_constraint_system<FieldT> ,
                   prover_key_filename:&String,
                   verification_key_filename:&String,generate:F) {
    // Generate keypair
    let  keypair = generate(cs);

    // Dump proving key to binary file
    ffec::enter_block("WritingProverKey");
    writeToFile(prover_key_filename, keypair.pk);
    ffec::leave_block("WritingProverKey");

    // Dump verification key in text format
    ffec::enter_block("SerializeVk");
    let  vk_out=File::open(verification_key_filename);
    serialize_vk(vk_out, keypair.vk, keypair.pk);
    ffec::leave_block("SerializeVk");

    // Also dump in binary format for local verification
    writeToFile(verification_key_filename + ".bin", keypair.vk);
}

// template<ProofT, ProvingKeyT,
//         ProofT (*prove)():&ProvingKeyT &, const &, const
//         VerificationKeyT,
//         bool (*verify)(:&VerificationKeyT &, &:r1cs_primary_input<FieldT> const ProofT)>
fn  proofgen<ProofT,  ProvingKeyT,F:Fn(&ProvingKeyT,&r1cs_primary_input<FieldT>,& r1cs_auxiliary_input<FieldT>)->ProofT,
VerificationKeyT,F2:Fn(& VerificationKeyT , & r1cs_primary_input<FieldT> ,  ProofT  )->bool
>(public_inputs:&r1cs_primary_input<FieldT>,
                     private_inputs:& r1cs_auxiliary_input<FieldT> ,
                     prover_key_filename:&String,
                     verification_key_filename:&String,
                     proof_filename:&String,
                      check_verification:bool,prove:F,verify:F2)->bool{

    let mut  proof;
    {
        // Read proving key
        ffec::enter_block("ReadingProverKey");
        let  pk = loadFromFile::<ProvingKeyT>(prover_key_filename);
        ffec::leave_block("ReadingProverKey");

        // Generate proof
        proof = prove(pk, public_inputs, private_inputs);
    }

    // Dump proof in text format
    ffec::enter_block("SerializeProof");
    let  p(proof_filename);
    serialize_proof(p, proof);
    ffec::leave_block("SerializeProof");

    if check_verification {
        // Check if verification works
        let  vk = loadFromFile::<VerificationKeyT>(verification_key_filename);

        ffec::enter_block("Verifying proof");
        let  ans = verify(vk, public_inputs, proof);
        println!("\n");
        println!("* The verification result is: {}\n", if ans { "PASS"} else {"FAIL"});
        ffec::leave_block("Verifying proof");
        return ans;
    }
    true
}

fn  generate_keys(input_directory:&str, output_directory:&str, proving_scheme:i32)->i32 {
    ffec::start_profiling();
    gadgetlib2::initPublicParamsFromDefaultPp();
    gadgetlib2::GadgetLibAdapter::resetVariableIndex();

    let  ps = ProvingScheme::new(proving_scheme);
    let  in_dir=input_directory;
    let  out_dir=output_directory;

    let  arith_filename = in_dir + "/circuit.arith";
    let  dummy_input_filename = in_dir + "/circuit.in";
    let  prover_key_filename = out_dir + "/proving.key";
    let  verification_key_filename = out_dir + "/verification.key";

    // Read the circuit, evaluate, and translate constraints
    let mut  cs;
    {
        let  pb = gadgetlib2::Protoboard::create(gadgetlib2::R1P);
        ffec::enter_block("CircuitReading");
        let  reader=CircuitReader::new(arith_filename, dummy_input_filename, pb);
        ffec::leave_block("CircuitReading");
        ffec::enter_block("Extract constraint system");
        cs = get_constraint_system_from_gadgetlib2(*pb);
        cs.primary_input_size = reader.getNumInputs() + reader.getNumOutputs();
        cs.auxiliary_input_size = gadgetlib2::GadgetLibAdapter::getNextFreeIndex() - cs.num_inputs();
        ffec::leave_block("Extract constraint system");
    }

    match ps {
         ProvingScheme::PGHR13=>
          {  ffec::print_header("PGHR13 Generator");
            keygen::<r1cs_ppzksnark_keypair<ppT>, r1cs_ppzksnark_generator<ppT>>(cs, prover_key_filename,
                                                                               verification_key_filename);
        }
         ProvingScheme::GROTH16=>
           { ffec::print_header("Groth16 Generator");
            keygen::<r1cs_gg_ppzksnark_keypair<ppT>, r1cs_gg_ppzksnark_generator<ppT>>(cs, prover_key_filename,
                                                                                     verification_key_filename);
            }
         ProvingScheme::GM17=>
           { ffec::print_header("GM17 Generator");
            keygen::<r1cs_se_ppzksnark_keypair<ppT>, r1cs_se_ppzksnark_generator<ppT>>(cs, prover_key_filename,
                                                                                     verification_key_filename);
            }
        _=>   {         return -1;}
    }

     0
}

fn generate_proof(keys_dir:&str, input_dir:&str, output_filename:&str, proving_scheme:i32, check_verification:i32)->i32 {
    ffec::start_profiling();
    gadgetlib2::initPublicParamsFromDefaultPp();
    gadgetlib2::GadgetLibAdapter::resetVariableIndex();

    let  ps = ProvingScheme::new(proving_scheme);
    let  in_dir=input_dir;
    let  key_dir=keys_dir;

    let  arith_filename = in_dir + "/circuit.arith";
    let  in_filename = in_dir + "/circuit.in";
    let  prover_key_filename = key_dir + "/proving.key";
    let  verification_key_filename = key_dir + "/verification.key.bin";

    let  primary_input;
    let auxiliary_input;
    {
        let mut  cs;
        {
            // Read the circuit, evaluate, and translate constraints
            ffec::enter_block("CircuitReading");
            let mut  full_assignment;
            {
                let mut  pb = gadgetlib2::Protoboard::create(gadgetlib2::R1P);
                let mut  primary_input_size;
                {
                   let   reader=CircuitReader::new(arith_filename, in_filename, pb);
                    primary_input_size = reader.getNumInputs() + reader.getNumOutputs();
                }
                cs = get_constraint_system_from_gadgetlib2(*pb);
                full_assignment = get_variable_assignment_from_gadgetlib2(*pb);
                cs.primary_input_size = primary_input_size;
                cs.auxiliary_input_size = full_assignment.len() - primary_input_size;
                ffec::leave_block("CircuitReading");
            }

            // extract primary and auxiliary input
            primary_input.assign(full_assignment.begin(), full_assignment.begin() + cs.num_inputs());
            auxiliary_input.assign(full_assignment.begin() + cs.num_inputs(), full_assignment.end());
        }

        if !cs.is_satisfied(primary_input, auxiliary_input) {
            cout << "The constraint system is not satisfied by the value assignment - Terminating." << endl;
            return -2;
        }
    }

    let  ret;
    match ps {
         ProvingScheme::PGHR13=> {
            ffec::print_header("PGHR13 Prover");
            ret = proofgen::<
                    r1cs_ppzksnark_proof<ppT>, r1cs_ppzksnark_proving_key<ppT>, r1cs_ppzksnark_prover<ppT>,
                    r1cs_ppzksnark_verification_key<ppT>, r1cs_ppzksnark_verifier_strong_IC<ppT>>(
                            primary_input, auxiliary_input, prover_key_filename, verification_key_filename, output_filename,
                            check_verification
            );
           
        }
        ProvingScheme::GROTH16=> {
            ffec::print_header("Groth16 Prover");
            ret = proofgen::<
                    r1cs_gg_ppzksnark_proof<ppT>, r1cs_gg_ppzksnark_proving_key<ppT>, r1cs_gg_ppzksnark_prover<ppT>,
                    r1cs_gg_ppzksnark_verification_key<ppT>, r1cs_gg_ppzksnark_verifier_strong_IC<ppT>>(
                            primary_input, auxiliary_input, prover_key_filename, verification_key_filename, output_filename,
                            check_verification
            );
           
        }
         ProvingScheme::GM17=> {
            ffec::print_header("GM17 Prover");
            ret = proofgen::<
                    r1cs_se_ppzksnark_proof<ppT>, r1cs_se_ppzksnark_proving_key<ppT>, r1cs_se_ppzksnark_prover<ppT>,
                    r1cs_se_ppzksnark_verification_key<ppT>, r1cs_se_ppzksnark_verifier_strong_IC<ppT>>(
                            primary_input, auxiliary_input, prover_key_filename, verification_key_filename, output_filename,
                            check_verification
            );
           
        }
        _=>{
            return -1;}
    }
    if ret{0}else{-2}
}

fn main(argc:i32, argv:Vec<String>)->i32 {
    if argc <5 {
            eprintln!("Invalid command");
        return -1
    }
        let in_dir = argv[2];
        let out_path = argv[3];
        if argc == 5 && "keygen"==argv[1] {
             generate_keys(in_dir, out_path, argv[4].parse::<i32>().unwrap())
        } else if argc == 7 &&  "proofgen"==argv[1] {
            let key_dir = argv[4];
             generate_proof(key_dir, in_dir, out_path, argv[5].parse::<i32>().unwrap(), argv[6].parse::<i32>().unwrap())
        }else{
 eprintln!("Invalid command");
     -1
        }
    
   
}