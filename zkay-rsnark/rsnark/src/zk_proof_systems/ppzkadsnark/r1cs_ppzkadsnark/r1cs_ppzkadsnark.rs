// /** @file
//  *****************************************************************************

//  Declaration of interfaces for a ppzkADSNARK for R1CS.

//  This includes:
//  - pub struct  for authentication key (public and symmetric)
//  - pub struct  for authentication verification key (public and symmetric)
//  - pub struct  for proving key
//  - pub struct  for verification key
//  - pub struct  for processed verification key
//  - pub struct  for key tuple (authentication key & proving key & verification key)
//  - pub struct  for authenticated data
//  - pub struct  for proof
//  - generator algorithm
//  - authentication key generator algorithm
//  - prover algorithm
//  - verifier algorithm (public and symmetric)
//  - online verifier algorithm (public and symmetric)

//  The implementation instantiates the construction in \[BBFR15], which in turn
//  is based on the r1cs_ppzkadsnark proof system.

//  Acronyms:

//  - R1CS = "Rank-1 Constraint Systems"
//  - ppzkADSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge Over Authenticated Data"

//  References:

// \[BBFR15]
// "ADSNARK: Nearly Practical and Privacy-Preserving Proofs on Authenticated Data",
// Michael Backes, Manuel Barbosa, Dario Fiore, Raphael M. Reischuk,
// IEEE Symposium on Security and Privacy 2015,
//  <http://eprint.iacr.org/2014/617>

//  *****************************************************************************
//  * @author     This file is part of crate, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef R1CS_PPZKADSNARK_HPP_
// #define R1CS_PPZKADSNARK_HPP_

// 

use ffec::algebra::curves::public_params;

use crate::common::data_structures::accumulation_vector;
use crate::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature;



/******************************** Public authentication parameters ********************************/

// 
// pub struct  r1cs_ppzkadsnark_pub_auth_prms;

// 
// std::ostream& operator<<(std::ostream &out, pap:&r1cs_ppzkadsnark_pub_auth_prms<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);

/**
 * Public authentication parameters for the R1CS ppzkADSNARK
 */
// 
pub struct  r1cs_ppzkadsnark_pub_auth_prms<ppT>{

I1:    G1::<snark_pp::<ppT>>,
}
impl r1cs_ppzkadsnark_pub_auth_prms<ppT>{
    // r1cs_ppzkadsnark_pub_auth_prms() {};
    // r1cs_ppzkadsnark_pub_auth_prms<ppT>& operator=(other:&r1cs_ppzkadsnark_pub_auth_prms<ppT>) = default;
    // r1cs_ppzkadsnark_pub_auth_prms(other:&r1cs_ppzkadsnark_pub_auth_prms<ppT>) = default;
    // r1cs_ppzkadsnark_pub_auth_prms(r1cs_ppzkadsnark_pub_auth_prms<ppT> &&other) = default;
    pub fn new (I1:G1<snark_pp<ppT>>) ->Self  {Self{I1}}

    // bool operator==(other:&r1cs_ppzkadsnark_pub_auth_prms<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pap:&r1cs_ppzkadsnark_pub_auth_prms<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);
}

/******************************** Secret authentication key ********************************/

// 
// pub struct  r1cs_ppzkadsnark_sec_auth_key;

// 
// std::ostream& operator<<(std::ostream &out, key:&r1cs_ppzkadsnark_sec_auth_key<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key);

/**
 * Secret authentication key for the R1CS ppzkADSNARK
 */
// 
pub struct  r1cs_ppzkadsnark_sec_auth_key<ppT> {

i:    Fr::<snark_pp::<ppT>>,
skp:    r1cs_ppzkadsnark_skT<ppT>,
S:    r1cs_ppzkadsnark_prfKeyT<ppT>,
}
impl r1cs_ppzkadsnark_sec_auth_key<ppT> 
{
    // r1cs_ppzkadsnark_sec_auth_key() {};
    // r1cs_ppzkadsnark_sec_auth_key<ppT>& operator=(other:&r1cs_ppzkadsnark_sec_auth_key<ppT>) = default;
    // r1cs_ppzkadsnark_sec_auth_key(other:&r1cs_ppzkadsnark_sec_auth_key<ppT>) = default;
    // r1cs_ppzkadsnark_sec_auth_key(r1cs_ppzkadsnark_sec_auth_key<ppT> &&other) = default;
    pub fn new(i:Fr::<snark_pp::<ppT>>,
skp:                                  r1cs_ppzkadsnark_skT<ppT>,S: r1cs_ppzkadsnark_prfKeyT<ppT>)->Self
      {  Self{i,
        skp,
        S }}

    // bool operator==(other:&r1cs_ppzkadsnark_sec_auth_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, key:&r1cs_ppzkadsnark_sec_auth_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key);
}

/******************************** Public authentication key ********************************/

// 
// pub struct  r1cs_ppzkadsnark_pub_auth_key;

// 
// std::ostream& operator<<(std::ostream &out, key:&r1cs_ppzkadsnark_pub_auth_key<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key);

/**
 * Public authentication key for the R1CS ppzkADSNARK
 */
// 
pub struct  r1cs_ppzkadsnark_pub_auth_key<ppT> {

    minusI2: G2::<snark_pp::<ppT>>,
     vkp:r1cs_ppzkadsnark_vkT<ppT>,
}
impl r1cs_ppzkadsnark_pub_auth_key<ppT> {
    // r1cs_ppzkadsnark_pub_auth_key() {};
    // r1cs_ppzkadsnark_pub_auth_key<ppT>& operator=(other:&r1cs_ppzkadsnark_pub_auth_key<ppT>) = default;
    // r1cs_ppzkadsnark_pub_auth_key(other:&r1cs_ppzkadsnark_pub_auth_key<ppT>) = default;
    // r1cs_ppzkadsnark_pub_auth_key(r1cs_ppzkadsnark_pub_auth_key<ppT> &&other) = default;
    pub fn new(minusI2:G2::<snark_pp::<ppT>>,vkp: r1cs_ppzkadsnark_vkT<ppT>)->Self
        { Self{minusI2,
        vkp}}

    // bool operator==(other:&r1cs_ppzkadsnark_pub_auth_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, key:&r1cs_ppzkadsnark_pub_auth_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key);
}

/******************************** Authentication key material ********************************/

// 
pub struct  r1cs_ppzkadsnark_auth_keys<ppT> {

pap:    r1cs_ppzkadsnark_pub_auth_prms<ppT>,
pak:    r1cs_ppzkadsnark_pub_auth_key<ppT>,
sak:    r1cs_ppzkadsnark_sec_auth_key<ppT>,
}
impl r1cs_ppzkadsnark_auth_keys<ppT> {
    // r1cs_ppzkadsnark_auth_keys() {};
    // r1cs_ppzkadsnark_auth_keys(r1cs_ppzkadsnark_auth_keys<ppT> &&other) = default;
    pub fn new(pap:r1cs_ppzkadsnark_pub_auth_prms<ppT>,
pak:                               r1cs_ppzkadsnark_pub_auth_key<ppT>,
sak:                               r1cs_ppzkadsnark_sec_auth_key<ppT>)->Self
      
    {  Self{pap,
        pak,
        sak}}
}

/******************************** Authenticated data ********************************/

// 
// pub struct  r1cs_ppzkadsnark_auth_data;

// 
// std::ostream& operator<<(std::ostream &out, data:&r1cs_ppzkadsnark_auth_data<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &data);

/**
 * Authenticated data for the R1CS ppzkADSNARK
 */
// 
pub struct  r1cs_ppzkadsnark_auth_data<ppT> {

mu:    Fr::<snark_pp::<ppT>>,
Lambda:    G2::<snark_pp::<ppT>>,
sigma:    r1cs_ppzkadsnark_sigT<ppT>,
}
impl r1cs_ppzkadsnark_auth_data<ppT> {
    // r1cs_ppzkadsnark_auth_data() {};
    // r1cs_ppzkadsnark_auth_data<ppT>& operator=(other:&r1cs_ppzkadsnark_auth_data<ppT>) = default;
    // r1cs_ppzkadsnark_auth_data(other:&r1cs_ppzkadsnark_auth_data<ppT>) = default;
    // r1cs_ppzkadsnark_auth_data(r1cs_ppzkadsnark_auth_data<ppT> &&other) = default;
    pub fn new(mu:Fr::<snark_pp::<ppT>>,
Lambda:                               G2::<snark_pp::<ppT>>,
                               sigma: r1cs_ppzkadsnark_sigT<ppT>)->Self
        {Self{mu,
        Lambda,
        sigma} }

    // bool operator==(other:&r1cs_ppzkadsnark_auth_data<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, key:&r1cs_ppzkadsnark_auth_data<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &key);
}

/******************************** Proving key ********************************/

// 
// pub struct  r1cs_ppzkadsnark_proving_key;

// 
// std::ostream& operator<<(std::ostream &out, pk:&r1cs_ppzkadsnark_proving_key<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk);

/**
 * A proving key for the R1CS ppzkADSNARK.
 */
// 
pub struct  r1cs_ppzkadsnark_proving_key<ppT> {

A_query:    knowledge_commitment_vector<G1::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
B_query:    knowledge_commitment_vector<G2::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
C_query:    knowledge_commitment_vector<G1::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
H_query:    G1_vector<snark_pp<ppT>>, // t powers
K_query:    G1_vector<snark_pp<ppT>>,
    /* Now come the additional elements for ad */
rA_i_Z_g1:    G1::<snark_pp::<ppT>>,

constraint_system:    r1cs_ppzkadsnark_constraint_system<ppT>,
}

    // r1cs_ppzkadsnark_proving_key() {};
    // r1cs_ppzkadsnark_proving_key<ppT>& operator=(other:&r1cs_ppzkadsnark_proving_key<ppT>) = default;
    // r1cs_ppzkadsnark_proving_key(other:&r1cs_ppzkadsnark_proving_key<ppT>) = default;
    // r1cs_ppzkadsnark_proving_key(r1cs_ppzkadsnark_proving_key<ppT> &&other) = default;

impl r1cs_ppzkadsnark_proving_key<ppT>
{
    pub fn new(A_query:knowledge_commitment_vector<G1::<snark_pp::<ppT>>,
                                 G1<snark_pp<ppT>> > ,
                                  B_query:knowledge_commitment_vector<G2::<snark_pp::<ppT>>,
                                 G1<snark_pp<ppT>> >,
                                 C_query:knowledge_commitment_vector<G1::<snark_pp::<ppT>>,
                                 G1<snark_pp<ppT>> >,
H_query:                                 G1_vector<snark_pp<ppT>>,
K_query:                                 G1_vector<snark_pp<ppT>>,
rA_i_Z_g1:                                 G1::<snark_pp::<ppT>>,
constraint_system:                                 r1cs_ppzkadsnark_constraint_system<ppT>)->Self
        
    {Self{A_query,
        B_query,
        C_query,
        H_query,
        K_query,
        rA_i_Z_g1,
        constraint_system}}

    pub fn G1_size(&self)->usize
    {
        return 2*(A_query.domain_size() + C_query.domain_size()) + B_query.domain_size() + H_query.len() + K_query.len() + 1;
    }

    pub fn G2_size(&self)->usize
    {
        return B_query.domain_size();
    }

    pub fn  G1_sparse_size(&self) ->usize
    {
        return 2*(A_query.len() + C_query.len()) + B_query.len() + H_query.len() + K_query.len() + 1;
    }

    pub fn  G2_sparse_size(&self) ->usize
    {
        return B_query.len();
    }

    pub fn size_in_bits(&self)->usize
    {
        return A_query.size_in_bits() + B_query.size_in_bits() + C_query.size_in_bits() + size_in_bits(H_query) + size_in_bits(K_query) + G1::<snark_pp::<ppT>>::size_in_bits();
    }

    pub fn print_size(&self)
    {
        print_indent(); print!("* G1 elements in PK: {}\n", self.G1_size());
        print_indent(); print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        print_indent(); print!("* G2 elements in PK: {}\n", self.G2_size());
        print_indent(); print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
        print_indent(); print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(other:&r1cs_ppzkadsnark_proving_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pk:&r1cs_ppzkadsnark_proving_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk);
}


/******************************* Verification key ****************************/

// 
// pub struct  r1cs_ppzkadsnark_verification_key;

// 
// std::ostream& operator<<(std::ostream &out, vk:&r1cs_ppzkadsnark_verification_key<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk);

/**
 * A verification key for the R1CS ppzkADSNARK.
 */
// 
pub struct  r1cs_ppzkadsnark_verification_key<ppT> {

alphaA_g2:    G2::<snark_pp::<ppT>>,
alphaB_g1:    G1::<snark_pp::<ppT>>,
alphaC_g2:    G2::<snark_pp::<ppT>>,
gamma_g2:    G2::<snark_pp::<ppT>>,
gamma_beta_g1:    G1::<snark_pp::<ppT>>,
gamma_beta_g2:    G2::<snark_pp::<ppT>>,
rC_Z_g2:    G2::<snark_pp::<ppT>>,

A0:    G1::<snark_pp::<ppT>>,
Ain:    G1_vector<snark_pp<ppT>>,
}
 impl r1cs_ppzkadsnark_verification_key<ppT> 
{
    // r1cs_ppzkadsnark_verification_key() = default;
    pub fn new(alphaA_g2:G2::<snark_pp::<ppT>>,
                                      alphaB_g1:G1::<snark_pp::<ppT>>,
                                      alphaC_g2:G2::<snark_pp::<ppT>>,
                                      gamma_g2:G2::<snark_pp::<ppT>>,
                                      gamma_beta_g1:G1::<snark_pp::<ppT>>,
                                      gamma_beta_g2:G2::<snark_pp::<ppT>>,
                                      rC_Z_g2:G2::<snark_pp::<ppT>>,
                                      A0:G1::<snark_pp::<ppT>>,
                                      Ain:G1_vector<snark_pp<ppT>>)->Self
       
    { Self{alphaA_g2,
        alphaB_g1,
        alphaC_g2,
        gamma_g2,
        gamma_beta_g1,
        gamma_beta_g2,
        rC_Z_g2,
        A0,
        Ain}}

    pub fn G1_size(&self)->usize
    {
        return 3 + Ain.len();
    }

    pub fn G2_size(&self)->usize
    {
        return 5;
    }

    pub fn size_in_bits(&self)->usize
    {
        return G1_size() * G1::<snark_pp::<ppT>>::size_in_bits() + G2_size() * G2::<snark_pp::<ppT>>::size_in_bits(); // possible zksnark bug
    }

    pub fn print_size(&self)
    {
        print_indent(); print!("* G1 elements in VK: {}\n", self.G1_size());
        print_indent(); print!("* G2 elements in VK: {}\n", self.G2_size());
        print_indent(); print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    // bool operator==(other:&r1cs_ppzkadsnark_verification_key<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, vk:&r1cs_ppzkadsnark_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk);

    // static r1cs_ppzkadsnark_verification_key<ppT> dummy_verification_key(input_size:usize);
}


/************************ Processed verification key *************************/

// 
// pub struct  r1cs_ppzkadsnark_processed_verification_key;

// 
// std::ostream& operator<<(std::ostream &out, pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the R1CS ppzkADSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
// 
pub struct  r1cs_ppzkadsnark_processed_verification_key {

pp_G2_one_precomp:    G2_precomp<snark_pp<ppT>>,
vk_alphaA_g2_precomp:    G2_precomp<snark_pp<ppT>>,
vk_alphaB_g1_precomp:    G1_precomp<snark_pp<ppT>>,
vk_alphaC_g2_precomp:    G2_precomp<snark_pp<ppT>>,
vk_rC_Z_g2_precomp:    G2_precomp<snark_pp<ppT>>,
vk_gamma_g2_precomp:    G2_precomp<snark_pp<ppT>>,
vk_gamma_beta_g1_precomp:    G1_precomp<snark_pp<ppT>>,
vk_gamma_beta_g2_precomp:    G2_precomp<snark_pp<ppT>>,
vk_rC_i_g2_precomp:    G2_precomp<snark_pp<ppT>>,

A0:    G1::<snark_pp::<ppT>>,
Ain:    G1_vector<snark_pp<ppT>>,

proof_g_vki_precomp:    Vec<G1_precomp<snark_pp<ppT>>>,

    // bool operator==(other:&r1cs_ppzkadsnark_processed_verification_key) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);
}


/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS ppzkADSNARK, which consists of a proving key and a verification key.
 */
// 
pub struct  r1cs_ppzkadsnark_keypair<ppT> {

pk:    r1cs_ppzkadsnark_proving_key<ppT>,
vk:    r1cs_ppzkadsnark_verification_key<ppT>,
}
impl<ppT> r1cs_ppzkadsnark_keypair<ppT> {
    // r1cs_ppzkadsnark_keypair() = default;
    // r1cs_ppzkadsnark_keypair(other:&r1cs_ppzkadsnark_keypair<ppT>) = default;
    pub fn new(pk:r1cs_ppzkadsnark_proving_key<ppT>,
vk:                             r1cs_ppzkadsnark_verification_key<ppT>)->Self
       
    { Self{pk,
        vk}}

    // r1cs_ppzkadsnark_keypair(r1cs_ppzkadsnark_keypair<ppT> &&other) = default;
}


/*********************************** Proof ***********************************/

// 
// pub struct  r1cs_ppzkadsnark_proof;

// 
// std::ostream& operator<<(std::ostream &out, proof:&r1cs_ppzkadsnark_proof<ppT>);

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof);

/**
 * A proof for the R1CS ppzkADSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
// 
pub struct  r1cs_ppzkadsnark_proof<ppT> {

g_A:    knowledge_commitment<G1::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
g_B:    knowledge_commitment<G2::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
g_C:    knowledge_commitment<G1::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
g_H:    G1::<snark_pp::<ppT>>,
g_K:    G1::<snark_pp::<ppT>>,
g_Aau:    knowledge_commitment<G1::<snark_pp::<ppT>>, G1<snark_pp<ppT>> >,
muA:    G1::<snark_pp::<ppT>>,
}
impl r1cs_ppzkadsnark_proof<ppT> 
{
    pub fn default()
    {
        // invalid proof with valid curve points
        self.g_A.g = G1::<snark_pp::<ppT>> ::one();
        self.g_A.h = G1::<snark_pp::<ppT>>::one();
        self.g_B.g = G2::<snark_pp::<ppT>> ::one();
        self.g_B.h = G1::<snark_pp::<ppT>>::one();
        self.g_C.g = G1::<snark_pp::<ppT>> ::one();
        self.g_C.h = G1::<snark_pp::<ppT>>::one();
        self.g_H = G1::<snark_pp::<ppT>>::one();
        self.g_K = G1::<snark_pp::<ppT>>::one();
        g_Aau = knowledge_commitment::<G1::<snark_pp::<ppT>>, G1::<snark_pp::<ppT>> >
            (G1::<snark_pp::<ppT>>::one(),G1::<snark_pp::<ppT>>::one());
        self.muA = G1::<snark_pp::<ppT>>::one();
    }
    pub fn new(g_A:knowledge_commitment<G1::<snark_pp::<ppT>>,
                           G1<snark_pp<ppT>> > ,
                            g_B:knowledge_commitment<G2::<snark_pp::<ppT>>,
                           G1<snark_pp<ppT>> >,
                          g_C:knowledge_commitment<G1::<snark_pp::<ppT>>,
                           G1<snark_pp<ppT>> >,
g_H:                           G1::<snark_pp::<ppT>>,
g_K:                           G1::<snark_pp::<ppT>>,
                            g_Aau:knowledge_commitment<G1::<snark_pp::<ppT>>,
                           G1<snark_pp<ppT>> >,
muA:                           G1<snark_pp<ppT>>) ->Self
       
    {Self{ g_A,
        g_B,
        g_C,
        g_H,
        g_K,
        g_Aau,
        muA}}

    pub fn G1_size(&self)->usize
    {
        return 10;
    }

    pub fn G2_size(&self)->usize
    {
        return 1;
    }

    pub fn size_in_bits(&self)->usize
    {
        return G1_size() * G1::<snark_pp::<ppT>>::size_in_bits() + G2_size() * G2::<snark_pp::<ppT>>::size_in_bits();
    }

    pub fn  print_size(&self)
    {
        print_indent(); print!("* G1 elements in proof: {}\n", self.G1_size());
        print_indent(); print!("* G2 elements in proof: {}\n", self.G2_size());
        print_indent(); print!("* Proof size in bits: {}\n", self.size_in_bits());
    }

    pub fn  is_well_formed(&self) ->bool
    {
        return (g_A.g.is_well_formed() && g_A.h.is_well_formed() &&
                g_B.g.is_well_formed() && g_B.h.is_well_formed() &&
                g_C.g.is_well_formed() && g_C.h.is_well_formed() &&
                g_H.is_well_formed() &&
                g_K.is_well_formed() &&
                g_Aau.g.is_well_formed() && g_Aau.h.is_well_formed() &&
                muA.is_well_formed());
    }

    // bool operator==(other:&r1cs_ppzkadsnark_proof<ppT>) const;
    // friend std::ostream& operator<< <ppT>(std::ostream &out, proof:&r1cs_ppzkadsnark_proof<ppT>);
    // friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof);
}


/***************************** Main algorithms *******************************/

/**
 * R1CS ppZKADSNARK authentication parameters generator algorithm.
 */
// 
// r1cs_ppzkadsnark_auth_keys<ppT> r1cs_ppzkadsnark_auth_generator(pub fn );

/**
 * R1CS ppZKADSNARK authentication algorithm.
 */
// 
// Vec<r1cs_ppzkadsnark_auth_data<ppT>> r1cs_ppzkadsnark_auth_sign(
//     ins:&Vec<Fr<snark_pp<ppT>>>,
//     sk:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
//     labels:Vec<labelT>);

/**
 * R1CS ppZKADSNARK authentication verification algorithms.
 */
// 
// bool r1cs_ppzkadsnark_auth_verify(data:&Vec<Fr<snark_pp<ppT>>>,
//                                   auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>,
//                                   sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
//                                   labels:&Vec<labelT>);

// 
// bool r1cs_ppzkadsnark_auth_verify(data:&Vec<Fr<snark_pp<ppT>>>,
//                                   auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>,
//                                   pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
//                                   labels:&Vec<labelT>);

/**
 * A generator algorithm for the R1CS ppzkADSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */
// 
// r1cs_ppzkadsnark_keypair<ppT> r1cs_ppzkadsnark_generator(cs:&r1cs_ppzkadsnark_constraint_system<ppT>,
//                                                          prms:&r1cs_ppzkadsnark_pub_auth_prms<ppT>);

// /**
//  * A prover algorithm for the R1CS ppzkADSNARK.
//  *
//  * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
//  * produces a proof (of knowledge) that attests to the following statement:
//  *               ``there exists Y such that CS(X,Y)=0''.
//  * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
//  */
// 
// r1cs_ppzkadsnark_proof<ppT> r1cs_ppzkadsnark_prover(pk:&r1cs_ppzkadsnark_proving_key<ppT>,
//                                                     primary_input:&r1cs_ppzkadsnark_primary_input<ppT>,
//                                                     auxiliary_input:&r1cs_ppzkadsnark_auxiliary_input<ppT>,
//                                                     auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>);

// /*
//  Below are two variants of verifier algorithm for the R1CS ppzkADSNARK.

//  These are the four cases that arise from the following choices:

// 1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
//      In the latter case, we call the algorithm an "online verifier".

// 2) The verifier uses the symmetric key or the public verification key.
//      In the former case we call the algorithm a "symmetric verifier".

// */

// /**
//  * Convert a (non-processed) verification key into a processed verification key.
//  */
// 
// r1cs_ppzkadsnark_processed_verification_key<ppT> r1cs_ppzkadsnark_verifier_process_vk(
//     vk:&r1cs_ppzkadsnark_verification_key<ppT>);

// /**
//  * A symmetric verifier algorithm for the R1CS ppzkADSNARK that
//  * accepts a non-processed verification key
//  */
// 
// bool r1cs_ppzkadsnark_verifier(vk:&r1cs_ppzkadsnark_verification_key<ppT>,
//                                proof:&r1cs_ppzkadsnark_proof<ppT>,
//                                sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
//                                labels:&Vec<labelT>);

// /**
//  * A symmetric verifier algorithm for the R1CS ppzkADSNARK that
//  * accepts a processed verification key.
//  */
// 
// bool r1cs_ppzkadsnark_online_verifier(pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>,
//                                       proof:&r1cs_ppzkadsnark_proof<ppT>,
//                                       sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
//                                       labels:&Vec<labelT>);


// /**
//  * A verifier algorithm for the R1CS ppzkADSNARK that
//  * accepts a non-processed verification key
//  */
// 
// bool r1cs_ppzkadsnark_verifier(vk:&r1cs_ppzkadsnark_verification_key<ppT>,
//                                auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>> ,
//                                proof:&r1cs_ppzkadsnark_proof<ppT>,
//                                pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
//                                labels:&Vec<labelT>);

// /**
//  * A verifier algorithm for the R1CS ppzkADSNARK that
//  * accepts a processed verification key.
//  */
// 
// bool r1cs_ppzkadsnark_online_verifier(pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>,
//                                       auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>> ,
//                                       proof:&r1cs_ppzkadsnark_proof<ppT>,
//                                       pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
//                                       labels:&Vec<labelT>);




// use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark;

//#endif // R1CS_PPZKSNARK_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a ppzkADSNARK for R1CS.

See r1cs_ppzkadsnark.hpp .

*****************************************************************************
* @author     This file is part of crate, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef R1CS_PPZKADSNARK_TCC_
// #define R1CS_PPZKADSNARK_TCC_

// use  <algorithm>
// use  <cassert>
// use  <functional>
// use  <iostream>
// use  <sstream>

 use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

// #ifdef MULTICORE
// use  <omp.h>
//#endif

use crate::knowledge_commitment::kc_multiexp;
use crate::reductions::r1cs_to_qap::r1cs_to_qap;




// 
// bool r1cs_ppzkadsnark_pub_auth_prms<ppT>::operator==(other:&r1cs_ppzkadsnark_pub_auth_prms<ppT>) const
// {
//     return (self.I1 == other.I1);
// }

// 
// std::ostream& operator<<(std::ostream &out, pap:&r1cs_ppzkadsnark_pub_auth_prms<ppT>)
// {
//     out << pap.I1;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap)
// {
//     in >> pap.I1;

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_sec_auth_key<ppT>::operator==(other:&r1cs_ppzkadsnark_sec_auth_key<ppT>) const
// {
//     return (self.i == other.i) &&
//         (self.skp == other.skp) &&
//         (self.S == other.S);
// }

// 
// std::ostream& operator<<(std::ostream &out, key:&r1cs_ppzkadsnark_sec_auth_key<ppT>)
// {
//     out << key.i;
//     out << key.skp;
//     out << key.S;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key)
// {
//     in >> key.i;
//     in >> key.skp;
//     in >> key.S;

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_pub_auth_key<ppT>::operator==(other:&r1cs_ppzkadsnark_pub_auth_key<ppT>) const
// {
//     return (self.minusI2 == other.minusI2) &&
//         (self.vkp == other.vkp);
// }

// 
// std::ostream& operator<<(std::ostream &out, key:&r1cs_ppzkadsnark_pub_auth_key<ppT>)
// {
//     out << key.minusI2;
//     out << key.vkp;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key)
// {
//     in >> key.minusI2;
//     in >> key.vkp;

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_auth_data<ppT>::operator==(other:&r1cs_ppzkadsnark_auth_data<ppT>) const
// {
//     return (self.mu == other.mu) &&
//         (self.Lambda == other.Lambda) &&
//         (self.sigma == other.sigma);
// }

// 
// std::ostream& operator<<(std::ostream &out, data:&r1cs_ppzkadsnark_auth_data<ppT>)
// {
//     out << data.mu;
//     out << data.Lambda;
//     out << data.sigma;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &data)
// {
//     in >> data.mu;
//     in >> data.Lambda;
//     data.sigma;

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_proving_key<ppT>::operator==(other:&r1cs_ppzkadsnark_proving_key<ppT>) const
// {
//     return (self.A_query == other.A_query &&
//             self.B_query == other.B_query &&
//             self.C_query == other.C_query &&
//             self.H_query == other.H_query &&
//             self.K_query == other.K_query &&
//             self.rA_i_Z_g1 == other.rA_i_Z_g1 &&
//             self.constraint_system == other.constraint_system);
// }

// 
// std::ostream& operator<<(std::ostream &out, pk:&r1cs_ppzkadsnark_proving_key<ppT>)
// {
//     out << pk.A_query;
//     out << pk.B_query;
//     out << pk.C_query;
//     out << pk.H_query;
//     out << pk.K_query;
//     out << pk.rA_i_Z_g1;
//     out << pk.constraint_system;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk)
// {
//     in >> pk.A_query;
//     in >> pk.B_query;
//     in >> pk.C_query;
//     in >> pk.H_query;
//     in >> pk.K_query;
//     in >> pk.rA_i_Z_g1;
//     in >> pk.constraint_system;

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_verification_key<ppT>::operator==(other:&r1cs_ppzkadsnark_verification_key<ppT>) const
// {
//     return (self.alphaA_g2 == other.alphaA_g2 &&
//             self.alphaB_g1 == other.alphaB_g1 &&
//             self.alphaC_g2 == other.alphaC_g2 &&
//             self.gamma_g2 == other.gamma_g2 &&
//             self.gamma_beta_g1 == other.gamma_beta_g1 &&
//             self.gamma_beta_g2 == other.gamma_beta_g2 &&
//             self.rC_Z_g2 == other.rC_Z_g2 &&
//             self.A0 == other.A0 &&
//             self.Ain == other.Ain);
// }

// 
// std::ostream& operator<<(std::ostream &out, vk:&r1cs_ppzkadsnark_verification_key<ppT>)
// {
//     out << vk.alphaA_g2 << OUTPUT_NEWLINE;
//     out << vk.alphaB_g1 << OUTPUT_NEWLINE;
//     out << vk.alphaC_g2 << OUTPUT_NEWLINE;
//     out << vk.gamma_g2 << OUTPUT_NEWLINE;
//     out << vk.gamma_beta_g1 << OUTPUT_NEWLINE;
//     out << vk.gamma_beta_g2 << OUTPUT_NEWLINE;
//     out << vk.rC_Z_g2 << OUTPUT_NEWLINE;
//     out << vk.A0 << OUTPUT_NEWLINE;
//     out << vk.Ain << OUTPUT_NEWLINE;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk)
// {
//     in >> vk.alphaA_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaB_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.alphaC_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g1;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.gamma_beta_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.rC_Z_g2;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.A0;
//     consume_OUTPUT_NEWLINE(in);
//     in >> vk.Ain;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_processed_verification_key<ppT>::operator==(
//     other:&r1cs_ppzkadsnark_processed_verification_key<ppT>) const
// {
//     bool result = (self.pp_G2_one_precomp == other.pp_G2_one_precomp &&
//                    self.vk_alphaA_g2_precomp == other.vk_alphaA_g2_precomp &&
//                    self.vk_alphaB_g1_precomp == other.vk_alphaB_g1_precomp &&
//                    self.vk_alphaC_g2_precomp == other.vk_alphaC_g2_precomp &&
//                    self.vk_rC_Z_g2_precomp == other.vk_rC_Z_g2_precomp &&
//                    self.vk_gamma_g2_precomp == other.vk_gamma_g2_precomp &&
//                    self.vk_gamma_beta_g1_precomp == other.vk_gamma_beta_g1_precomp &&
//                    self.vk_gamma_beta_g2_precomp == other.vk_gamma_beta_g2_precomp &&
//                    self.vk_rC_i_g2_precomp == other.vk_rC_i_g2_precomp &&
//                    self.A0 == other.A0 &&
//                    self.Ain == other.Ain &&
//                    self.proof_g_vki_precomp.len() == other.proof_g_vki_precomp.len());
//     if result {
//         for i in 0..self.proof_g_vki_precomp.len(){
//             result &= self.proof_g_vki_precomp[i] == other.proof_g_vki_precomp[i];
//     }
//     return result;
// }

// 
// std::ostream& operator<<(std::ostream &out, pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>)
// {
//     out << pvk.pp_G2_one_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_alphaA_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_alphaB_g1_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_alphaC_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_rC_Z_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_gamma_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_gamma_beta_g1_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_gamma_beta_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.vk_rC_i_g2_precomp << OUTPUT_NEWLINE;
//     out << pvk.A0 << OUTPUT_NEWLINE;
//     out << pvk.Ain << OUTPUT_NEWLINE;
//     out << pvk.proof_g_vki_precomp  << OUTPUT_NEWLINE;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk)
// {
//     in >> pvk.pp_G2_one_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaA_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaB_g1_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_alphaC_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_rC_Z_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g1_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_gamma_beta_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.vk_rC_i_g2_precomp;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.A0;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.Ain;
//     consume_OUTPUT_NEWLINE(in);
//     in >> pvk.proof_g_vki_precomp;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

// 
// bool r1cs_ppzkadsnark_proof<ppT>::operator==(other:&r1cs_ppzkadsnark_proof<ppT>) const
// {
//     return (self.g_A == other.g_A &&
//             self.g_B == other.g_B &&
//             self.g_C == other.g_C &&
//             self.g_H == other.g_H &&
//             self.g_K == other.g_K &&
//             self.g_Aau == other.g_Aau &&
//             self.muA == other.muA);
// }

// 
// std::ostream& operator<<(std::ostream &out, proof:&r1cs_ppzkadsnark_proof<ppT>)
// {
//     out << proof.g_A << OUTPUT_NEWLINE;
//     out << proof.g_B << OUTPUT_NEWLINE;
//     out << proof.g_C << OUTPUT_NEWLINE;
//     out << proof.g_H << OUTPUT_NEWLINE;
//     out << proof.g_K << OUTPUT_NEWLINE;
//     out << proof.g_Aau << OUTPUT_NEWLINE;
//     out << proof.muA << OUTPUT_NEWLINE;

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof)
// {
//     in >> proof.g_A;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_B;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_C;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_H;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_K;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.g_Aau;
//     consume_OUTPUT_NEWLINE(in);
//     in >> proof.muA;
//     consume_OUTPUT_NEWLINE(in);

//     return in;
// }

impl<ppT> r1cs_ppzkadsnark_verification_key<ppT>{
// 
pub fn dummy_verification_key(input_size:usize)->r1cs_ppzkadsnark_verification_key<ppT> 
{
    let mut  result=r1cs_ppzkadsnark_verification_key::<ppT>::new();
    result.alphaA_g2 = Fr::<snark_pp::<ppT>>::random_element() * G2::<snark_pp::<ppT>>::one();
    result.alphaB_g1 = Fr::<snark_pp::<ppT>>::random_element() * G1::<snark_pp::<ppT>>::one();
    result.alphaC_g2 = Fr::<snark_pp::<ppT>>::random_element() * G2::<snark_pp::<ppT>>::one();
    result.gamma_g2 = Fr::<snark_pp::<ppT>>::random_element() * G2::<snark_pp::<ppT>>::one();
    result.gamma_beta_g1 = Fr::<snark_pp::<ppT>>::random_element() * G1::<snark_pp::<ppT>>::one();
    result.gamma_beta_g2 = Fr::<snark_pp::<ppT>>::random_element() * G2::<snark_pp::<ppT>>::one();
    result.rC_Z_g2 = Fr::<snark_pp::<ppT>>::random_element() * G2::<snark_pp::<ppT>>::one();

    result.A0 = Fr::<snark_pp::<ppT>>::random_element() * G1::<snark_pp::<ppT>>::one();
    for i in 0..input_size
    {
        result.Ain.push(Fr::<snark_pp::<ppT>>::random_element() *
                                G1::<snark_pp::<ppT>>::one());
    }

    return result;
}
}


 pub fn r1cs_ppzkadsnark_auth_generator<ppT>()->r1cs_ppzkadsnark_auth_keys<ppT> {
    let mut  sigkp = sigGen::<ppT>();
    let mut  prfseed = prfGen::<ppT>();
   let mut  i = Fr::<snark_pp::<ppT>>::random_element();
    let mut  I1 = i * G1::<snark_pp::<ppT>>::one();
    let mut  minusI2 = G2::<snark_pp::<ppT>>::zero() -
        i * G2::<snark_pp::<ppT>>::one();
    return r1cs_ppzkadsnark_auth_keys::<ppT>(
        r1cs_ppzkadsnark_pub_auth_prms::<ppT>(I1),
        r1cs_ppzkadsnark_pub_auth_key::<ppT>(minusI2,(sigkp.vk)),
        r1cs_ppzkadsnark_sec_auth_key::<ppT>(i,(sigkp.sk),prfseed));
}


 pub fn r1cs_ppzkadsnark_auth_sign<ppT>(
    ins:&Vec<Fr<snark_pp<ppT>>>,
    sk:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
    labels:Vec<labelT>)->Vec<r1cs_ppzkadsnark_auth_data<ppT>>  {
    enter_block("Call to r1cs_ppzkadsnark_auth_sign",false);
    assert !(labels.len()==ins.len());
    let mut  res=Vec::with_capacity(ins.len());
    for i in 0..ins.len(){
        let mut  lambda = prfCompute::<ppT>(sk.S,labels[i]);
        let mut  Lambda = lambda * G2::<snark_pp::<ppT>>::one();
        let mut sig = sigSign::<ppT>(sk.skp,labels[i],Lambda);
        let mut val= r1cs_ppzkadsnark_auth_data::<ppT>::new((lambda + sk.i * ins[i]),
                                            Lambda,
                                            sig);
        res.push(val);
    }
    leave_block("Call to r1cs_ppzkadsnark_auth_sign",false);
    return (res);
}

// symmetric

  pub fn r1cs_ppzkadsnark_auth_verify<ppT>(data:&Vec<Fr<snark_pp<ppT>>>,
                                  auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>,
                                  sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
                                  labels:&Vec<labelT>)->bool {
    enter_block("Call to r1cs_ppzkadsnark_auth_verify",false);
    assert !((data.len()==labels.len()) && (auth_data.len()==labels.len()));
    let mut  res = true;
    for i in 0..data.len(){
        let mut  lambda = prfCompute::<ppT>(sak.S,labels[i]);
       let mut  mup = lambda + sak.i * data[i];
        res = res && (auth_data[i].mu == mup);
    }
    leave_block("Call to r1cs_ppzkadsnark_auth_verify",false);
    return res;
}

// public

  pub fn r1cs_ppzkadsnark_auth_verify2<ppT>(data:&Vec<Fr<snark_pp<ppT>>>,
                                  auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>,
                                  pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
                                  labels:&Vec<labelT>) ->bool{
    enter_block("Call to r1cs_ppzkadsnark_auth_verify",false);
    assert !((data.len()==labels.len()) && (data.len()==auth_data.len()));
    let mut  res = true;
    for i in 0..auth_data.len(){
        let mut  Mup = auth_data[i].Lambda - data[i] * pak.minusI2;
        res = res && (auth_data[i].mu * G2::<snark_pp::<ppT>>::one() == Mup);
        res = res && sigVerif::<ppT>(pak.vkp,labels[i],auth_data[i].Lambda,auth_data[i].sigma);
    }
    leave_block("Call to r1cs_ppzkadsnark_auth_verify",false);
    return res;
}


pub fn r1cs_ppzkadsnark_generator<ppT>(cs:&r1cs_ppzkadsnark_constraint_system<ppT>,
                                                         prms:&r1cs_ppzkadsnark_pub_auth_prms<ppT>)->r1cs_ppzkadsnark_keypair<ppT> 
{
    enter_block("Call to r1cs_ppzkadsnark_generator",false);

    /* make the B_query "lighter" if possible */
    let mut cs_copy=cs.clone();
    cs_copy.swap_AB_if_beneficial();

    /* draw random element at which the QAP is evaluated */
    let mut  t = Fr::<snark_pp::<ppT>>::random_element();

    let mut  qap_inst =
        r1cs_to_qap_instance_map_with_evaluation(cs_copy, t);

    print_indent(); print!("* QAP number of variables: {}\n", qap_inst.num_variables());
    print_indent(); print!("* QAP pre degree: {}\n", cs_copy.constraints.len());
    print_indent(); print!("* QAP degree: {}\n", qap_inst.degree());
    print_indent(); print!("* QAP number of input variables: {}\n", qap_inst.num_inputs());

    enter_block("Compute query densities",false);
    let mut  non_zero_At = 0;let mut non_zero_Bt = 0;let mut  non_zero_Ct = 0;let mut  non_zero_Ht = 0;
    for i in 0..qap_inst.num_variables()+1
    {
        if !qap_inst.At[i].is_zero()
        {
            non_zero_At+=1;
        }
        if !qap_inst.Bt[i].is_zero()
        {
            non_zero_Bt+=1;
        }
        if !qap_inst.Ct[i].is_zero()
        {
            non_zero_Ct+=1;
        }
    }
    for i in 0..qap_inst.degree()+1
    {
        if !qap_inst.Ht[i].is_zero()
        {
            non_zero_Ht+=1;
        }
    }
    leave_block("Compute query densities",false);

    let mut  At = (qap_inst.At); // qap_inst.At is now in unspecified state, but we do not use it later
    let mut  Bt = (qap_inst.Bt); // qap_inst.Bt is now in unspecified state, but we do not use it later
      let mut  Ct = (qap_inst.Ct); // qap_inst.Ct is now in unspecified state, but we do not use it later
    let mut  Ht = (qap_inst.Ht); // qap_inst.Ht is now in unspecified state, but we do not use it later

    /* append Zt to At,Bt,Ct with */
    At.push(qap_inst.Zt);
    Bt.push(qap_inst.Zt);
    Ct.push(qap_inst.Zt);

     let   alphaA = Fr::<snark_pp::<ppT>>::random_element();
      let  alphaB = Fr::<snark_pp::<ppT>>::random_element();
       let alphaC = Fr::<snark_pp::<ppT>>::random_element();
       let rA = Fr::<snark_pp::<ppT>>::random_element();
       let rB = Fr::<snark_pp::<ppT>>::random_element();
      let  beta = Fr::<snark_pp::<ppT>>::random_element();
       let gamma = Fr::<snark_pp::<ppT>>::random_element();
    let     rC = rA * rB;

    // construct the same-coefficient-check query (must happen before zeroing out the prefix of At)
    let mut  Kt=Vec::with_capacity(qap_inst.num_variables()+4);
  
    for i in 0..qap_inst.num_variables()+1
    {
        Kt.push( beta * (rA * At[i] + rB * Bt[i] + rC * Ct[i] ) );
    }
    Kt.push(beta * rA * qap_inst.Zt);
    Kt.push(beta * rB * qap_inst.Zt);
    Kt.push(beta * rC * qap_inst.Zt);

    let g1_exp_count = 2*(non_zero_At - qap_inst.num_inputs() + non_zero_Ct) + non_zero_Bt + non_zero_Ht + Kt.len();
    let g2_exp_count = non_zero_Bt;

    let  g1_window = get_exp_window_size::<G1::<snark_pp::<ppT>> >(g1_exp_count);
    let g2_window = get_exp_window_size::<G2::<snark_pp::<ppT>> >(g2_exp_count);
    print_indent(); print!("* G1 window: {}\n", g1_window);
    print_indent(); print!("* G2 window: {}\n", g2_window);

// #ifdef MULTICORE
//     chunks:usize = omp_get_max_threads(); // to override, set OMP_NUM_THREADS env var or call omp_set_num_threads()
// #else
    let chunks:usize = 1;
//#endif

    enter_block("Generating G1 multiexp table",false);
    let g1_table =
        get_window_table(Fr::<snark_pp::<ppT>>::size_in_bits(), g1_window,
                         G1::<snark_pp::<ppT>>::one());
    leave_block("Generating G1 multiexp table",false);

    enter_block("Generating G2 multiexp table",false);
    let g2_table =
        get_window_table(Fr::<snark_pp::<ppT>>::size_in_bits(),
                         g2_window, G2::<snark_pp::<ppT>>::one());
    leave_block("Generating G2 multiexp table",false);

    enter_block("Generate R1CS proving key",false);

    enter_block("Generate knowledge commitments",false);
    enter_block("Compute the A-query", false,false);
    let A_query =
        kc_batch_exp(Fr::<snark_pp::<ppT>>::size_in_bits(), g1_window, g1_window, g1_table,
                     g1_table, rA, rA*alphaA, At, chunks);
    leave_block("Compute the A-query", false,false);

    enter_block("Compute the B-query", false,false);
   let B_query =
        kc_batch_exp(Fr::<snark_pp::<ppT>>::size_in_bits(), g2_window, g1_window, g2_table,
                     g1_table, rB, rB*alphaB, Bt, chunks);
    leave_block("Compute the B-query", false,false);

    enter_block("Compute the C-query", false,false);
   let C_query =
        kc_batch_exp(Fr::<snark_pp::<ppT>>::size_in_bits(), g1_window, g1_window, g1_table,
                     g1_table, rC, rC*alphaC, Ct, chunks);
    leave_block("Compute the C-query", false,false);

    enter_block("Compute the H-query", false,false);
    let  H_query = batch_exp(Fr::<snark_pp::<ppT>>::size_in_bits(), g1_window, g1_table, Ht);
// #ifdef USE_MIXED_ADDITION
    // batch_to_special<G1<snark_pp<ppT>> >(H_query);
//#endif
    leave_block("Compute the H-query", false,false);

    enter_block("Compute the K-query", false,false);
    let  K_query = batch_exp(Fr::<snark_pp::<ppT>>::size_in_bits(), g1_window, g1_table, Kt);
// #ifdef USE_MIXED_ADDITION
    // batch_to_special<G1<snark_pp<ppT>> >(K_query);
//#endif
    leave_block("Compute the K-query", false,false);

    leave_block("Generate knowledge commitments",false);

    leave_block("Generate R1CS proving key",false);

    enter_block("Generate R1CS verification key",false);
    let mut alphaA_g2 = alphaA * G2::<snark_pp::<ppT>>::one();
    let alphaB_g1 = alphaB * G1::<snark_pp::<ppT>>::one();
    let mut alphaC_g2 = alphaC * G2::<snark_pp::<ppT>>::one();
    let mut gamma_g2 = gamma * G2::<snark_pp::<ppT>>::one();
    letgamma_beta_g1 = (gamma * beta) * G1::<snark_pp::<ppT>>::one();
    let mut gamma_beta_g2 = (gamma * beta) * G2::<snark_pp::<ppT>>::one();
    let mut rC_Z_g2 = (rC * qap_inst.Zt) * G2::<snark_pp::<ppT>>::one();

    enter_block("Generate extra authentication elements",false);
   let rA_i_Z_g1 = (rA * qap_inst.Zt) * prms.I1;
    leave_block("Generate extra authentication elements",false);

    enter_block("Copy encoded input coefficients for R1CS verification key",false);
   letA0 = A_query[0].g;
    let mut  Ain=Vec::with_capacity(qap_inst.num_inputs());
    for i in 0..qap_inst.num_inputs()
    {
        Ain.push(A_query[1+i].g);
    }

    leave_block("Copy encoded input coefficients for R1CS verification key",false);

    leave_block("Generate R1CS verification key",false);

    leave_block("Call to r1cs_ppzkadsnark_generator",false);

    let mut vk = r1cs_ppzkadsnark_verification_key::<ppT>(alphaA_g2,
                                                                                       alphaB_g1,
                                                                                       alphaC_g2,
                                                                                       gamma_g2,
                                                                                       gamma_beta_g1,
                                                                                       gamma_beta_g2,
                                                                                       rC_Z_g2,
                                                                                       A0,
                                                                                       Ain);
    let mut  pk = r1cs_ppzkadsnark_proving_key::<ppT>(A_query,
                                                                             B_query,
                                                                             C_query,
                                                                             H_query,
                                                                             K_query,
                                                                             rA_i_Z_g1,
                                                                             cs_copy);

    pk.print_size();
    vk.print_size();

    return r1cs_ppzkadsnark_keypair::<ppT>(pk, vk);
}


 pub fn r1cs_ppzkadsnark_prover<ppT>(pk:&r1cs_ppzkadsnark_proving_key<ppT>,
                                                    primary_input:&r1cs_ppzkadsnark_primary_input<ppT>,
                                                    auxiliary_input:&r1cs_ppzkadsnark_auxiliary_input<ppT>,
                                                    auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>)->r1cs_ppzkadsnark_proof<ppT>
{
    enter_block("Call to r1cs_ppzkadsnark_prover",false);

// #ifdef DEBUG
    assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
//#endif

   let  d1 = Fr::<snark_pp::<ppT>>::random_element();
     let   d2 = Fr::<snark_pp::<ppT>>::random_element();
     let   d3 = Fr::<snark_pp::<ppT>>::random_element();
     let   dauth = Fr::<snark_pp::<ppT>>::random_element();

    enter_block("Compute the polynomial H",false);
    let qap_wit = r1cs_to_qap_witness_map(pk.constraint_system, primary_input,
                                                                            auxiliary_input, d1 + dauth, d2, d3);
    leave_block("Compute the polynomial H",false);

// #ifdef DEBUG
    let t = Fr::<snark_pp::<ppT>>::random_element();
   let qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    assert!(qap_inst.is_satisfied(qap_wit));
//#endif

    let g_A =
        /* pk.A_query[0] + */ d1*pk.A_query[qap_wit.num_variables()+1];
    let  g_B =
        pk.B_query[0] + qap_wit.d2*pk.B_query[qap_wit.num_variables()+1];
   let g_C =
        pk.C_query[0] + qap_wit.d3*pk.C_query[qap_wit.num_variables()+1];

    let  g_Ain = dauth*pk.A_query[qap_wit.num_variables()+1];

    let  g_H = G1::<snark_pp::<ppT>>::zero();
    let  g_K = (pk.K_query[0] +
                             qap_wit.d1*pk.K_query[qap_wit.num_variables()+1] +
                             qap_wit.d2*pk.K_query[qap_wit.num_variables()+2] +
                             qap_wit.d3*pk.K_query[qap_wit.num_variables()+3]);

// #ifdef DEBUG
    for i in 0..qap_wit.num_inputs() + 1
    {
        assert!(pk.A_query[i].g == G1::<snark_pp::<ppT>>::zero());
    }
    assert!(pk.A_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.B_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.C_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.H_query.len() == qap_wit.degree()+1);
    assert!(pk.K_query.len() == qap_wit.num_variables()+4);
//#endif

// #ifdef MULTICORE
    // let  chunks = omp_get_max_threads(); // to override, set OMP_NUM_THREADS env var or call omp_set_num_threads()
// #else
    let chunks = 1;
//#endif

    enter_block("Compute the proof",false);

    enter_block("Compute answer to A-query", false,false);
    g_A = g_A + kc_multi_exp_with_mixed_addition::<G1::<snark_pp::<ppT>>,
                                                 G1::<snark_pp::<ppT>>,
                                                 Fr::<snark_pp::<ppT>>,
                                                 multi_exp_method_bos_coster>(
        pk.A_query,
        1+qap_wit.num_inputs(), 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_inputs(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    leave_block("Compute answer to A-query", false,false);

    enter_block("Compute answer to Ain-query", false,false);
    g_Ain = g_Ain + kc_multi_exp_with_mixed_addition::<G1::<snark_pp::<ppT>>,
                                                     G1::<snark_pp::<ppT>>,
                                                     Fr::<snark_pp::<ppT>>,
                                                     multi_exp_method_bos_coster>(
        pk.A_query,
        1, 1+qap_wit.num_inputs(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_inputs(),
        chunks);
    //std :: cout << "The input proof term: " << g_Ain << "\n";
    leave_block("Compute answer to Ain-query", false,false);

    enter_block("Compute answer to B-query", false,false);
    g_B = g_B + kc_multi_exp_with_mixed_addition::<G2::<snark_pp::<ppT>>,
                                                 G1::<snark_pp::<ppT>>,
                                                 Fr::<snark_pp::<ppT>>,
                                                 multi_exp_method_bos_coster>(
        pk.B_query,
        1, 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    leave_block("Compute answer to B-query", false,false);

    enter_block("Compute answer to C-query", false,false);
    g_C = g_C + kc_multi_exp_with_mixed_addition::<G1::<snark_pp::<ppT>>,
                                                 G1::<snark_pp::<ppT>>,
                                                 Fr::<snark_pp::<ppT>>,
                                                 multi_exp_method_bos_coster>(
        pk.C_query,
        1, 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    leave_block("Compute answer to C-query", false,false);

    enter_block("Compute answer to H-query", false,false);
    g_H = g_H + multi_exp::<G1::<snark_pp::<ppT>>,
                                 Fr::<snark_pp::<ppT>>,
                                 multi_exp_method_BDLO12>(
        pk.H_query.begin(),
        pk.H_query.begin()+qap_wit.degree()+1,
        qap_wit.coefficients_for_H.begin(),
        qap_wit.coefficients_for_H.begin()+qap_wit.degree()+1,
        chunks);
    leave_block("Compute answer to H-query", false,false);

    enter_block("Compute answer to K-query", false,false);
    g_K = g_K + multi_exp_with_mixed_addition::<G1::<snark_pp::<ppT>>,
                                                     Fr::<snark_pp::<ppT>>,
                                                     multi_exp_method_bos_coster>(
        pk.K_query.begin()+1,
        pk.K_query.begin()+1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    leave_block("Compute answer to K-query", false,false);

    enter_block("Compute extra auth terms", false,false);
    let mut  mus=Vec::with_capacity(qap_wit.num_inputs());
    let mut  Ains=Vec::with_capacity(qap_wit.num_inputs());

    for i in 0..qap_wit.num_inputs(){
        mus.push(auth_data[i].mu);
        Ains.push(pk.A_query[i+1].g);
    }
    let mut  muA = dauth * pk.rA_i_Z_g1;
    muA = muA + multi_exp::<G1::<snark_pp::<ppT>>,
                                 Fr::<snark_pp::<ppT>>,
                                 multi_exp_method_bos_coster>(
        Ains.begin(), Ains.begin()+qap_wit.num_inputs(),
        mus.begin(), mus.begin()+qap_wit.num_inputs(),
        chunks);

    // To Do: Decide whether to include relevant parts of auth_data in proof
    leave_block("Compute extra auth terms", false,false);

    leave_block("Compute the proof",false);

    leave_block("Call to r1cs_ppzkadsnark_prover",false);

    let mut  proof = r1cs_ppzkadsnark_proof::<ppT>(g_A,
                                                                    g_B,
                                                                    g_C,
                                                                    g_H,
                                                                    g_K,
                                                                    g_Ain,
                                                                    muA);
    proof.print_size();

    return proof;
}


pub fn r1cs_ppzkadsnark_verifier_process_vk<ppT>(
    vk:&r1cs_ppzkadsnark_verification_key<ppT>)->r1cs_ppzkadsnark_processed_verification_key<ppT> 
{
    enter_block("Call to r1cs_ppzkadsnark_verifier_process_vk",false);

    let mut pvk= r1cs_ppzkadsnark_processed_verification_key::<ppT>::new();
    pvk.pp_G2_one_precomp        = snark_pp::<ppT>::precompute_G2(G2::<snark_pp::<ppT>>::one());
    pvk.vk_alphaA_g2_precomp     = snark_pp::<ppT>::precompute_G2(vk.alphaA_g2);
    pvk.vk_alphaB_g1_precomp     = snark_pp::<ppT>::precompute_G1(vk.alphaB_g1);
    pvk.vk_alphaC_g2_precomp     = snark_pp::<ppT>::precompute_G2(vk.alphaC_g2);
    pvk.vk_rC_Z_g2_precomp       = snark_pp::<ppT>::precompute_G2(vk.rC_Z_g2);
    pvk.vk_gamma_g2_precomp      = snark_pp::<ppT>::precompute_G2(vk.gamma_g2);
    pvk.vk_gamma_beta_g1_precomp = snark_pp::<ppT>::precompute_G1(vk.gamma_beta_g1);
    pvk.vk_gamma_beta_g2_precomp = snark_pp::<ppT>::precompute_G2(vk.gamma_beta_g2);

    enter_block("Pre-processing for additional auth elements",false);
    let mut  vk_rC_z_g2_precomp = snark_pp::<ppT>::precompute_G2(vk.rC_Z_g2);

    pvk.A0 = G1::<snark_pp::<ppT>>(vk.A0);
    pvk.Ain = G1_vector::<snark_pp::<ppT>>(vk.Ain);

    pvk.proof_g_vki_precomp.reserve(pvk.Ain.len());
    for i in 0..pvk.Ain.len(){
        pvk.proof_g_vki_precomp.push(snark_pp::<ppT>::precompute_G1(pvk.Ain[i]));
    }

    leave_block("Pre-processing for additional auth elements",false);

    leave_block("Call to r1cs_ppzkadsnark_verifier_process_vk",false);

    return pvk;
}

// symmetric

pub fn  r1cs_ppzkadsnark_online_verifier<ppT>(pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>,
                                      proof:&r1cs_ppzkadsnark_proof<ppT>,
                                      sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
                                      labels:&Vec<labelT>)->bool
{
    let mut  result = true;
    enter_block("Call to r1cs_ppzkadsnark_online_verifier",false);

    enter_block("Check if the proof is well-formed",false);
    if !proof.is_well_formed()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed",false);

    enter_block("Checking auth-specific elements",false);

    enter_block("Checking A1",false);

    enter_block("Compute PRFs",false);
    let mut  lambdas=Vec::with_capacity(labels.len());
  
    for i in 0..labels.len(){
        lambdas.push(prfCompute::<ppT>(sak.S,labels[i]));
    }
    leave_block("Compute PRFs",false);
    let mut  prodA = sak.i * proof.g_Aau.g;
    prodA = prodA + multi_exp::<G1::<snark_pp::<ppT>>,
                                     Fr::<snark_pp::<ppT>>,
                                     multi_exp_method_bos_coster>(
        pvk.Ain.begin(),
        pvk.Ain.begin() + labels.len(),
        lambdas.begin(),
        lambdas.begin() + labels.len(), 1);

    let mut  result_auth = true;

    if !(prodA == proof.muA) {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Authentication check failed.\n");
        }
        result_auth = false;
    }

    leave_block("Checking A1",false);

    enter_block("Checking A2",false);
    let mut  proof_g_Aau_g_precomp      = snark_pp::<ppT>::precompute_G1(proof.g_Aau.g);
    let mut  proof_g_Aau_h_precomp = snark_pp::<ppT>::precompute_G1(proof.g_Aau.h);
    let mut  kc_Aau_1 = snark_pp::<ppT>::miller_loop(proof_g_Aau_g_precomp, pvk.vk_alphaA_g2_precomp);
    let mut  kc_Aau_2 = snark_pp::<ppT>::miller_loop(proof_g_Aau_h_precomp, pvk.pp_G2_one_precomp);
    let mut  kc_Aau = snark_pp::<ppT>::final_exponentiation(kc_Aau_1 * kc_Aau_2.unitary_inverse());
    if kc_Aau != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for Aau query incorrect.\n");
        }
        result_auth = false;
    }
    leave_block("Checking A2",false);

    leave_block("Checking auth-specific elements",false);

    result &= result_auth;

    enter_block("Online pairing computations",false);
    enter_block("Check knowledge commitment for A is valid",false);
    let mut  proof_g_A_g_precomp      = snark_pp::<ppT>::precompute_G1(proof.g_A.g);
    let mut  proof_g_A_h_precomp = snark_pp::<ppT>::precompute_G1(proof.g_A.h);
    let mut  kc_A_1 = snark_pp::<ppT>::miller_loop(proof_g_A_g_precomp,      pvk.vk_alphaA_g2_precomp);
    let mut  kc_A_2 = snark_pp::<ppT>::miller_loop(proof_g_A_h_precomp, pvk.pp_G2_one_precomp);
    let mut  kc_A = snark_pp::<ppT>::final_exponentiation(kc_A_1 * kc_A_2.unitary_inverse());
    if kc_A != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for A is valid",false);

    enter_block("Check knowledge commitment for B is valid",false);
   let mut   proof_g_B_g_precomp      = snark_pp::<ppT>::precompute_G2(proof.g_B.g);
    let mut  proof_g_B_h_precomp = snark_pp::<ppT>::precompute_G1(proof.g_B.h);
    let mut  kc_B_1 = snark_pp::<ppT>::miller_loop(pvk.vk_alphaB_g1_precomp, proof_g_B_g_precomp);
    let mut  kc_B_2 = snark_pp::<ppT>::miller_loop(proof_g_B_h_precomp,    pvk.pp_G2_one_precomp);
    let mut  kc_B = snark_pp::<ppT>::final_exponentiation(kc_B_1 * kc_B_2.unitary_inverse());
    if kc_B != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for B is valid",false);

    enter_block("Check knowledge commitment for C is valid",false);
    let mut  proof_g_C_g_precomp      = snark_pp::<ppT>::precompute_G1(proof.g_C.g);
    let mut proof_g_C_h_precomp = snark_pp::<ppT>::precompute_G1(proof.g_C.h);
    let mut  kc_C_1 = snark_pp::<ppT>::miller_loop(proof_g_C_g_precomp,      pvk.vk_alphaC_g2_precomp);
   let mut  kc_C_2 = snark_pp::<ppT>::miller_loop(proof_g_C_h_precomp, pvk.pp_G2_one_precomp);
    let mut  kc_C = snark_pp::<ppT>::final_exponentiation(kc_C_1 * kc_C_2.unitary_inverse());
    if kc_C != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for C is valid",false);

    let mut  Aacc = pvk.A0 + proof.g_Aau.g + proof.g_A.g;

    enter_block("Check QAP divisibility",false);
    let mut  proof_g_Aacc_precomp = snark_pp::<ppT>::precompute_G1(Aacc);
    let mut  proof_g_H_precomp = snark_pp::<ppT>::precompute_G1(proof.g_H);
    let mut  QAP_1  = snark_pp::<ppT>::miller_loop(proof_g_Aacc_precomp,  proof_g_B_g_precomp);
    let mut  QAP_23  = snark_pp::<ppT>::double_miller_loop(proof_g_H_precomp, pvk.vk_rC_Z_g2_precomp,
                                                                   proof_g_C_g_precomp, pvk.pp_G2_one_precomp);
    let mut  QAP = snark_pp::<ppT>::final_exponentiation(QAP_1 * QAP_23.unitary_inverse());
    if QAP != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility",false);

    enter_block("Check same coefficients were used",false);
    let mut  proof_g_K_precomp = snark_pp::<ppT>::precompute_G1(proof.g_K);
    let mut  proof_g_Aacc_C_precomp = snark_pp::<ppT>::precompute_G1(Aacc + proof.g_C.g);
    let mut  K_1 = snark_pp::<ppT>::miller_loop(proof_g_K_precomp, pvk.vk_gamma_g2_precomp);
    let mut  K_23 = snark_pp::<ppT>::double_miller_loop(proof_g_Aacc_C_precomp, pvk.vk_gamma_beta_g2_precomp,
                                                                pvk.vk_gamma_beta_g1_precomp, proof_g_B_g_precomp);
    let mut K = snark_pp::<ppT>::final_exponentiation(K_1 * K_23.unitary_inverse());
    if K != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    leave_block("Check same coefficients were used",false);
    leave_block("Online pairing computations",false);
    leave_block("Call to r1cs_ppzkadsnark_online_verifier",false);

    return result;
}


 pub fn r1cs_ppzkadsnark_verifier<ppT>(vk:&r1cs_ppzkadsnark_verification_key<ppT>,
                               proof:&r1cs_ppzkadsnark_proof<ppT>,
                               sak:&r1cs_ppzkadsnark_sec_auth_key<ppT>,
                               labels:&Vec<labelT>)->bool
{
    enter_block("Call to r1cs_ppzkadsnark_verifier",false);
    let mut  pvk = r1cs_ppzkadsnark_verifier_process_vk::<ppT>::new(vk);
let mut result= r1cs_ppzkadsnark_online_verifier::<ppT>(pvk, proof, sak, labels);
    leave_block("Call to r1cs_ppzkadsnark_verifier",false);
    return result;
}


// public

pub fn  r1cs_ppzkadsnark_online_verifier2<ppT>(pvk:&r1cs_ppzkadsnark_processed_verification_key<ppT>,
                                      auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>> ,
                                      proof:&r1cs_ppzkadsnark_proof<ppT>,
                                      pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
                                      labels:&Vec<labelT>)->bool
{
let mut result= true;
    enter_block("Call to r1cs_ppzkadsnark_online_verifier",false);

    enter_block("Check if the proof is well-formed",false);
    if !proof.is_well_formed()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    leave_block("Check if the proof is well-formed",false);

    enter_block("Checking auth-specific elements",false);
    assert !(labels.len()==auth_data.len());

    enter_block("Checking A1",false);

    enter_block("Checking signatures",false);
    let mut  Lambdas=Vec::with_capacity(labels.len());
    let mut sigs=Vec::with_capacity(labels.len());

    for i in 0..labels.len(){
        Lambdas.push(auth_data[i].Lambda);
        sigs.push(auth_data[i].sigma);
    }
let mut result_auth= sigBatchVerif::<ppT>(pak.vkp,labels,Lambdas,sigs);
    if ! result_auth
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Auth sig check failed.\n");
        }
    }

    leave_block("Checking signatures",false);

    enter_block("Checking pairings",false);
    // To Do: Decide whether to move pak and lambda preprocessing to offline
    let mut  g_Lambdas_precomp=Vec::with_capacity(auth_data.len());
    for i in 0..auth_data.len(){
        g_Lambdas_precomp.push(snark_pp::<ppT>::precompute_G2(auth_data[i].Lambda));
    }
let mut g_minusi_precomp= snark_pp::<ppT>::precompute_G2(pak.minusI2);

    enter_block("Computation",false);
    let mut  accum=Fqk::<snark_pp::<ppT>>::new();
    if auth_data.len() % 2 == 1 {
        accum = snark_pp::<ppT>::miller_loop(pvk.proof_g_vki_precomp[0]  , g_Lambdas_precomp[0]);
    }
    else {
        accum = Fqk::<snark_pp::<ppT>>::one();
    }
    for i in (auth_data.len() % 2.. labels.len()).step_by(2) {
        accum = accum * snark_pp::<ppT>::double_miller_loop(pvk.proof_g_vki_precomp[i]  , g_Lambdas_precomp[i],
                                                          pvk.proof_g_vki_precomp[i+1], g_Lambdas_precomp[i+1]);
    }
let mut proof_g_muA_precomp=snark_pp::<ppT>::precompute_G1(proof.muA);
let mut proof_g_Aau_precomp=snark_pp::<ppT>::precompute_G1(proof.g_Aau.g);
let mut accum2=snark_pp::<ppT>::double_miller_loop(proof_g_muA_precomp, pvk.pp_G2_one_precomp,
                                                                  proof_g_Aau_precomp, g_minusi_precomp);
let mut authPair=snark_pp::<ppT>::final_exponentiation(accum * accum2.unitary_inverse());
    if authPair != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Auth pairing check failed.\n");
        }
        result_auth = false;
    }
    leave_block("Computation",false);
    leave_block("Checking pairings",false);


    if !result_auth {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Authentication check failed.\n");
        }
    }

    leave_block("Checking A1",false);

    enter_block("Checking A2",false);
let mut proof_g_Aau_g_precomp=snark_pp::<ppT>::precompute_G1(proof.g_Aau.g);
let mut proof_g_Aau_h_precomp=snark_pp::<ppT>::precompute_G1(proof.g_Aau.h);
let mut kc_Aau_1=snark_pp::<ppT>::miller_loop(proof_g_Aau_g_precomp, pvk.vk_alphaA_g2_precomp);
let mut kc_Aau_2=snark_pp::<ppT>::miller_loop(proof_g_Aau_h_precomp, pvk.pp_G2_one_precomp);
let mut kc_Aau=snark_pp::<ppT>::final_exponentiation(kc_Aau_1 * kc_Aau_2.unitary_inverse());
    if kc_Aau != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for Aau query incorrect.\n");
        }
        result_auth = false;
    }
    leave_block("Checking A2",false);

    leave_block("Checking auth-specific elements",false);

    result &= result_auth;

    enter_block("Online pairing computations",false);
    enter_block("Check knowledge commitment for A is valid",false);
let mut proof_g_A_g_precomp=snark_pp::<ppT>::precompute_G1(proof.g_A.g);
let mut proof_g_A_h_precomp=snark_pp::<ppT>::precompute_G1(proof.g_A.h);
let mut kc_A_1=snark_pp::<ppT>::miller_loop(proof_g_A_g_precomp,      pvk.vk_alphaA_g2_precomp);
let mut kc_A_2=snark_pp::<ppT>::miller_loop(proof_g_A_h_precomp, pvk.pp_G2_one_precomp);
let mut kc_A=snark_pp::<ppT>::final_exponentiation(kc_A_1 * kc_A_2.unitary_inverse());
    if kc_A != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for A is valid",false);

    enter_block("Check knowledge commitment for B is valid",false);
let mut proof_g_B_g_precomp=snark_pp::<ppT>::precompute_G2(proof.g_B.g);
let mut proof_g_B_h_precomp=snark_pp::<ppT>::precompute_G1(proof.g_B.h);
let mut kc_B_1=snark_pp::<ppT>::miller_loop(pvk.vk_alphaB_g1_precomp, proof_g_B_g_precomp);
let mut kc_B_2=snark_pp::<ppT>::miller_loop(proof_g_B_h_precomp,    pvk.pp_G2_one_precomp);
let mut kc_B=snark_pp::<ppT>::final_exponentiation(kc_B_1 * kc_B_2.unitary_inverse());
    if kc_B != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for B is valid",false);

    enter_block("Check knowledge commitment for C is valid",false);
let mut proof_g_C_g_precomp=snark_pp::<ppT>::precompute_G1(proof.g_C.g);
let mut proof_g_C_h_precomp=snark_pp::<ppT>::precompute_G1(proof.g_C.h);
let mut kc_C_1=snark_pp::<ppT>::miller_loop(proof_g_C_g_precomp,      pvk.vk_alphaC_g2_precomp);
let mut kc_C_2=snark_pp::<ppT>::miller_loop(proof_g_C_h_precomp, pvk.pp_G2_one_precomp);
let mut kc_C=snark_pp::<ppT>::final_exponentiation(kc_C_1 * kc_C_2.unitary_inverse());
    if kc_C != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    leave_block("Check knowledge commitment for C is valid",false);

let mut Aacc=pvk.A0 + proof.g_Aau.g + proof.g_A.g;

    enter_block("Check QAP divisibility",false);
let mut proof_g_Aacc_precomp=snark_pp::<ppT>::precompute_G1(Aacc);
let mut proof_g_H_precomp=snark_pp::<ppT>::precompute_G1(proof.g_H);
let mut QAP_1=snark_pp::<ppT>::miller_loop(proof_g_Aacc_precomp,  proof_g_B_g_precomp);
let mut QAP_23=snark_pp::<ppT>::double_miller_loop(proof_g_H_precomp, pvk.vk_rC_Z_g2_precomp,
                                                                   proof_g_C_g_precomp, pvk.pp_G2_one_precomp);
let mut QAP=snark_pp::<ppT>::final_exponentiation(QAP_1 * QAP_23.unitary_inverse());
    if QAP != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    leave_block("Check QAP divisibility",false);

    enter_block("Check same coefficients were used",false);
let mut proof_g_K_precomp=snark_pp::<ppT>::precompute_G1(proof.g_K);
let mut proof_g_Aacc_C_precomp=snark_pp::<ppT>::precompute_G1(Aacc + proof.g_C.g);
let mut K_1=snark_pp::<ppT>::miller_loop(proof_g_K_precomp, pvk.vk_gamma_g2_precomp);
let mut K_23=snark_pp::<ppT>::double_miller_loop(proof_g_Aacc_C_precomp, pvk.vk_gamma_beta_g2_precomp,
                                                                pvk.vk_gamma_beta_g1_precomp, proof_g_B_g_precomp);
let mut K=snark_pp::<ppT>::final_exponentiation(K_1 * K_23.unitary_inverse());
    if K != GT::<snark_pp::<ppT>>::one()
    {
        if !inhibit_profiling_info
        {
            print_indent(); print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    leave_block("Check same coefficients were used",false);
    leave_block("Online pairing computations",false);
    leave_block("Call to r1cs_ppzkadsnark_online_verifier",false);

    return result;
}

// public

 pub fn r1cs_ppzkadsnark_verifier2<ppT>(vk:&r1cs_ppzkadsnark_verification_key<ppT>,
                               auth_data:&Vec<r1cs_ppzkadsnark_auth_data<ppT>>,
                               proof:&r1cs_ppzkadsnark_proof<ppT>,
                               pak:&r1cs_ppzkadsnark_pub_auth_key<ppT>,
                               labels:&Vec<labelT>)->bool
{
    assert!(labels.len() == auth_data.len());
    enter_block("Call to r1cs_ppzkadsnark_verifier",false);
let mut pvk=r1cs_ppzkadsnark_verifier_process_vk::<ppT>(vk);
let mut result=r1cs_ppzkadsnark_online_verifier::<ppT>(pvk, auth_data, proof, pak,labels);
    leave_block("Call to r1cs_ppzkadsnark_verifier",false);
    return result;
}


//#endif // R1CS_PPZKADSNARK_TCC_
