/** @file
 *****************************************************************************

 Declaration of interfaces for a ppzkADSNARK for R1CS.

 This includes:
 - class for authentication key (public and symmetric)
 - class for authentication verification key (public and symmetric)
 - class for proving key
 - class for verification key
 - class for processed verification key
 - class for key tuple (authentication key & proving key & verification key)
 - class for authenticated data
 - class for proof
 - generator algorithm
 - authentication key generator algorithm
 - prover algorithm
 - verifier algorithm (public and symmetric)
 - online verifier algorithm (public and symmetric)

 The implementation instantiates the construction in \[BBFR15], which in turn
 is based on the r1cs_ppzkadsnark proof system.

 Acronyms:

 - R1CS = "Rank-1 Constraint Systems"
 - ppzkADSNARK = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge Over Authenticated Data"

 References:

\[BBFR15]
"ADSNARK: Nearly Practical and Privacy-Preserving Proofs on Authenticated Data",
Michael Backes, Manuel Barbosa, Dario Fiore, Raphael M. Reischuk,
IEEE Symposium on Security and Privacy 2015,
 <http://eprint.iacr.org/2014/617>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_PPZKADSNARK_HPP_
// #define R1CS_PPZKADSNARK_HPP_

use  <memory>

use ffec::algebra::curves::public_params;

use crate::common::data_structures::accumulation_vector;
use crate::knowledge_commitment::knowledge_commitment;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/r1cs_ppzkadsnark_params;
use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/r1cs_ppzkadsnark_prf;
use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/r1cs_ppzkadsnark_signature;



/******************************** Public authentication parameters ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_pub_auth_prms;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);

/**
 * Public authentication parameters for the R1CS ppzkADSNARK
 */
template<typename ppT>
class r1cs_ppzkadsnark_pub_auth_prms {
public:
    ffec::G1<snark_pp<ppT>> I1;

    r1cs_ppzkadsnark_pub_auth_prms() {};
    r1cs_ppzkadsnark_pub_auth_prms<ppT>& operator=(const r1cs_ppzkadsnark_pub_auth_prms<ppT> &other) = default;
    r1cs_ppzkadsnark_pub_auth_prms(const r1cs_ppzkadsnark_pub_auth_prms<ppT> &other) = default;
    r1cs_ppzkadsnark_pub_auth_prms(r1cs_ppzkadsnark_pub_auth_prms<ppT> &&other) = default;
    r1cs_ppzkadsnark_pub_auth_prms(ffec::G1<snark_pp<ppT>> &&I1) : I1((I1)) {};

    bool operator==(const r1cs_ppzkadsnark_pub_auth_prms<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap);
};

/******************************** Secret authentication key ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_sec_auth_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_sec_auth_key<ppT> &key);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key);

/**
 * Secret authentication key for the R1CS ppzkADSNARK
 */
template<typename ppT>
class r1cs_ppzkadsnark_sec_auth_key {
public:
    ffec::Fr<snark_pp<ppT>> i;
    r1cs_ppzkadsnark_skT<ppT>skp;
    r1cs_ppzkadsnark_prfKeyT<ppT>S;

    r1cs_ppzkadsnark_sec_auth_key() {};
    r1cs_ppzkadsnark_sec_auth_key<ppT>& operator=(const r1cs_ppzkadsnark_sec_auth_key<ppT> &other) = default;
    r1cs_ppzkadsnark_sec_auth_key(const r1cs_ppzkadsnark_sec_auth_key<ppT> &other) = default;
    r1cs_ppzkadsnark_sec_auth_key(r1cs_ppzkadsnark_sec_auth_key<ppT> &&other) = default;
    r1cs_ppzkadsnark_sec_auth_key(ffec::Fr<snark_pp<ppT>> &&i,
                                  r1cs_ppzkadsnark_skT<ppT>&&skp, r1cs_ppzkadsnark_prfKeyT<ppT>&&S) :
        i((i)),
        skp((skp)),
        S((S)) {};

    bool operator==(const r1cs_ppzkadsnark_sec_auth_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_sec_auth_key<ppT> &key);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key);
};

/******************************** Public authentication key ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_pub_auth_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_key<ppT> &key);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key);

/**
 * Public authentication key for the R1CS ppzkADSNARK
 */
template<typename ppT>
class r1cs_ppzkadsnark_pub_auth_key {
public:
    ffec::G2<snark_pp<ppT>> minusI2;
    r1cs_ppzkadsnark_vkT<ppT>vkp;

    r1cs_ppzkadsnark_pub_auth_key() {};
    r1cs_ppzkadsnark_pub_auth_key<ppT>& operator=(const r1cs_ppzkadsnark_pub_auth_key<ppT> &other) = default;
    r1cs_ppzkadsnark_pub_auth_key(const r1cs_ppzkadsnark_pub_auth_key<ppT> &other) = default;
    r1cs_ppzkadsnark_pub_auth_key(r1cs_ppzkadsnark_pub_auth_key<ppT> &&other) = default;
    r1cs_ppzkadsnark_pub_auth_key(ffec::G2<snark_pp<ppT>> &&minusI2, r1cs_ppzkadsnark_vkT<ppT>&&vkp) :
        minusI2((minusI2)),
        vkp((vkp)) {};

    bool operator==(const r1cs_ppzkadsnark_pub_auth_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_key<ppT> &key);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key);
};

/******************************** Authentication key material ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_auth_keys {
public:
    r1cs_ppzkadsnark_pub_auth_prms<ppT> pap;
    r1cs_ppzkadsnark_pub_auth_key<ppT> pak;
    r1cs_ppzkadsnark_sec_auth_key<ppT> sak;

    r1cs_ppzkadsnark_auth_keys() {};
    r1cs_ppzkadsnark_auth_keys(r1cs_ppzkadsnark_auth_keys<ppT> &&other) = default;
    r1cs_ppzkadsnark_auth_keys(r1cs_ppzkadsnark_pub_auth_prms<ppT> &&pap,
                               r1cs_ppzkadsnark_pub_auth_key<ppT> &&pak,
                               r1cs_ppzkadsnark_sec_auth_key<ppT> &&sak) :
        pap((pap)),
        pak((pak)),
        sak((sak))
    {}
};

/******************************** Authenticated data ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_auth_data;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_auth_data<ppT> &data);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &data);

/**
 * Authenticated data for the R1CS ppzkADSNARK
 */
template<typename ppT>
class r1cs_ppzkadsnark_auth_data {
public:
    ffec::Fr<snark_pp<ppT>> mu;
    ffec::G2<snark_pp<ppT>> Lambda;
    r1cs_ppzkadsnark_sigT<ppT>sigma;

    r1cs_ppzkadsnark_auth_data() {};
    r1cs_ppzkadsnark_auth_data<ppT>& operator=(const r1cs_ppzkadsnark_auth_data<ppT> &other) = default;
    r1cs_ppzkadsnark_auth_data(const r1cs_ppzkadsnark_auth_data<ppT> &other) = default;
    r1cs_ppzkadsnark_auth_data(r1cs_ppzkadsnark_auth_data<ppT> &&other) = default;
    r1cs_ppzkadsnark_auth_data(ffec::Fr<snark_pp<ppT>> &&mu,
                               ffec::G2<snark_pp<ppT>> &&Lambda,
                               r1cs_ppzkadsnark_sigT<ppT>&&sigma) :
        mu((mu)),
        Lambda((Lambda)),
        sigma((sigma)) {};

    bool operator==(const r1cs_ppzkadsnark_auth_data<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_auth_data<ppT> &key);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &key);
};

/******************************** Proving key ********************************/

template<typename ppT>
class r1cs_ppzkadsnark_proving_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_proving_key<ppT> &pk);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk);

/**
 * A proving key for the R1CS ppzkADSNARK.
 */
template<typename ppT>
class r1cs_ppzkadsnark_proving_key {
public:
    knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > A_query;
    knowledge_commitment_vector<ffec::G2<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > B_query;
    knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > C_query;
    ffec::G1_vector<snark_pp<ppT>> H_query; // t powers
    ffec::G1_vector<snark_pp<ppT>> K_query;
    /* Now come the additional elements for ad */
    ffec::G1<snark_pp<ppT>> rA_i_Z_g1;

    r1cs_ppzkadsnark_constraint_system<ppT> constraint_system;

    r1cs_ppzkadsnark_proving_key() {};
    r1cs_ppzkadsnark_proving_key<ppT>& operator=(const r1cs_ppzkadsnark_proving_key<ppT> &other) = default;
    r1cs_ppzkadsnark_proving_key(const r1cs_ppzkadsnark_proving_key<ppT> &other) = default;
    r1cs_ppzkadsnark_proving_key(r1cs_ppzkadsnark_proving_key<ppT> &&other) = default;
    r1cs_ppzkadsnark_proving_key(knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>,
                                 ffec::G1<snark_pp<ppT>> > &&A_query,
                                 knowledge_commitment_vector<ffec::G2<snark_pp<ppT>>,
                                 ffec::G1<snark_pp<ppT>> > &&B_query,
                                 knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>,
                                 ffec::G1<snark_pp<ppT>> > &&C_query,
                                 ffec::G1_vector<snark_pp<ppT>> &&H_query,
                                 ffec::G1_vector<snark_pp<ppT>> &&K_query,
                                 ffec::G1<snark_pp<ppT>> &&rA_i_Z_g1,
                                 r1cs_ppzkadsnark_constraint_system<ppT> &&constraint_system) :
        A_query((A_query)),
        B_query((B_query)),
        C_query((C_query)),
        H_query((H_query)),
        K_query((K_query)),
        rA_i_Z_g1((rA_i_Z_g1)),
        constraint_system((constraint_system))
    {};

    size_t G1_size() const
    {
        return 2*(A_query.domain_size() + C_query.domain_size()) + B_query.domain_size() + H_query.size() + K_query.size() + 1;
    }

    size_t G2_size() const
    {
        return B_query.domain_size();
    }

    size_t G1_sparse_size() const
    {
        return 2*(A_query.size() + C_query.size()) + B_query.size() + H_query.size() + K_query.size() + 1;
    }

    size_t G2_sparse_size() const
    {
        return B_query.size();
    }

    size_t size_in_bits() const
    {
        return A_query.size_in_bits() + B_query.size_in_bits() + C_query.size_in_bits() + ffec::size_in_bits(H_query) + ffec::size_in_bits(K_query) + ffec::G1<snark_pp<ppT>>::size_in_bits();
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in PK: {}\n", self.G1_size());
        ffec::print_indent(); print!("* Non-zero G1 elements in PK: {}\n", self.G1_sparse_size());
        ffec::print_indent(); print!("* G2 elements in PK: {}\n", self.G2_size());
        ffec::print_indent(); print!("* Non-zero G2 elements in PK: {}\n", self.G2_sparse_size());
        ffec::print_indent(); print!("* PK size in bits: {}\n", self.size_in_bits());
    }

    bool operator==(const r1cs_ppzkadsnark_proving_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_proving_key<ppT> &pk);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk);
};


/******************************* Verification key ****************************/

template<typename ppT>
class r1cs_ppzkadsnark_verification_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_verification_key<ppT> &vk);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk);

/**
 * A verification key for the R1CS ppzkADSNARK.
 */
template<typename ppT>
class r1cs_ppzkadsnark_verification_key {
public:
    ffec::G2<snark_pp<ppT>> alphaA_g2;
    ffec::G1<snark_pp<ppT>> alphaB_g1;
    ffec::G2<snark_pp<ppT>> alphaC_g2;
    ffec::G2<snark_pp<ppT>> gamma_g2;
    ffec::G1<snark_pp<ppT>> gamma_beta_g1;
    ffec::G2<snark_pp<ppT>> gamma_beta_g2;
    ffec::G2<snark_pp<ppT>> rC_Z_g2;

    ffec::G1<snark_pp<ppT>> A0;
    ffec::G1_vector<snark_pp<ppT>> Ain;

    r1cs_ppzkadsnark_verification_key() = default;
    r1cs_ppzkadsnark_verification_key(const ffec::G2<snark_pp<ppT>> &alphaA_g2,
                                      const ffec::G1<snark_pp<ppT>> &alphaB_g1,
                                      const ffec::G2<snark_pp<ppT>> &alphaC_g2,
                                      const ffec::G2<snark_pp<ppT>> &gamma_g2,
                                      const ffec::G1<snark_pp<ppT>> &gamma_beta_g1,
                                      const ffec::G2<snark_pp<ppT>> &gamma_beta_g2,
                                      const ffec::G2<snark_pp<ppT>> &rC_Z_g2,
                                      const ffec::G1<snark_pp<ppT>> A0,
                                      const ffec::G1_vector<snark_pp<ppT>> Ain) :
        alphaA_g2(alphaA_g2),
        alphaB_g1(alphaB_g1),
        alphaC_g2(alphaC_g2),
        gamma_g2(gamma_g2),
        gamma_beta_g1(gamma_beta_g1),
        gamma_beta_g2(gamma_beta_g2),
        rC_Z_g2(rC_Z_g2),
        A0(A0),
        Ain(Ain)
    {};

    size_t G1_size() const
    {
        return 3 + Ain.size();
    }

    size_t G2_size() const
    {
        return 5;
    }

    size_t size_in_bits() const
    {
        return G1_size() * ffec::G1<snark_pp<ppT>>::size_in_bits() + G2_size() * ffec::G2<snark_pp<ppT>>::size_in_bits(); // possible zksnark bug
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in VK: {}\n", self.G1_size());
        ffec::print_indent(); print!("* G2 elements in VK: {}\n", self.G2_size());
        ffec::print_indent(); print!("* VK size in bits: {}\n", self.size_in_bits());
    }

    bool operator==(const r1cs_ppzkadsnark_verification_key<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_verification_key<ppT> &vk);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk);

    static r1cs_ppzkadsnark_verification_key<ppT> dummy_verification_key(const size_t input_size);
};


/************************ Processed verification key *************************/

template<typename ppT>
class r1cs_ppzkadsnark_processed_verification_key;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);

/**
 * A processed verification key for the R1CS ppzkADSNARK.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
template<typename ppT>
class r1cs_ppzkadsnark_processed_verification_key {
public:
    ffec::G2_precomp<snark_pp<ppT>> pp_G2_one_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_alphaA_g2_precomp;
    ffec::G1_precomp<snark_pp<ppT>> vk_alphaB_g1_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_alphaC_g2_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_rC_Z_g2_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_gamma_g2_precomp;
    ffec::G1_precomp<snark_pp<ppT>> vk_gamma_beta_g1_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_gamma_beta_g2_precomp;
    ffec::G2_precomp<snark_pp<ppT>> vk_rC_i_g2_precomp;

    ffec::G1<snark_pp<ppT>> A0;
    ffec::G1_vector<snark_pp<ppT>> Ain;

    std::vector<ffec::G1_precomp<snark_pp<ppT>>> proof_g_vki_precomp;

    bool operator==(const r1cs_ppzkadsnark_processed_verification_key &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the R1CS ppzkADSNARK, which consists of a proving key and a verification key.
 */
template<typename ppT>
class r1cs_ppzkadsnark_keypair {
public:
    r1cs_ppzkadsnark_proving_key<ppT> pk;
    r1cs_ppzkadsnark_verification_key<ppT> vk;

    r1cs_ppzkadsnark_keypair() = default;
    r1cs_ppzkadsnark_keypair(const r1cs_ppzkadsnark_keypair<ppT> &other) = default;
    r1cs_ppzkadsnark_keypair(r1cs_ppzkadsnark_proving_key<ppT> &&pk,
                             r1cs_ppzkadsnark_verification_key<ppT> &&vk) :
        pk((pk)),
        vk((vk))
    {}

    r1cs_ppzkadsnark_keypair(r1cs_ppzkadsnark_keypair<ppT> &&other) = default;
};


/*********************************** Proof ***********************************/

template<typename ppT>
class r1cs_ppzkadsnark_proof;

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_proof<ppT> &proof);

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof);

/**
 * A proof for the R1CS ppzkADSNARK.
 *
 * While the proof has a structure, externally one merely opaquely produces,
 * serializes/deserializes, and verifies proofs. We only expose some information
 * about the structure for statistics purposes.
 */
template<typename ppT>
class r1cs_ppzkadsnark_proof {
public:
    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_A;
    knowledge_commitment<ffec::G2<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_B;
    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_C;
    ffec::G1<snark_pp<ppT>> g_H;
    ffec::G1<snark_pp<ppT>> g_K;
    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_Aau;
    ffec::G1<snark_pp<ppT>> muA;

    r1cs_ppzkadsnark_proof()
    {
        // invalid proof with valid curve points
        self.g_A.g = ffec::G1<snark_pp<ppT>> ::one();
        self.g_A.h = ffec::G1<snark_pp<ppT>>::one();
        self.g_B.g = ffec::G2<snark_pp<ppT>> ::one();
        self.g_B.h = ffec::G1<snark_pp<ppT>>::one();
        self.g_C.g = ffec::G1<snark_pp<ppT>> ::one();
        self.g_C.h = ffec::G1<snark_pp<ppT>>::one();
        self.g_H = ffec::G1<snark_pp<ppT>>::one();
        self.g_K = ffec::G1<snark_pp<ppT>>::one();
        g_Aau = knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> >
            (ffec::G1<snark_pp<ppT>>::one(),ffec::G1<snark_pp<ppT>>::one());
        self.muA = ffec::G1<snark_pp<ppT>>::one();
    }
    r1cs_ppzkadsnark_proof(knowledge_commitment<ffec::G1<snark_pp<ppT>>,
                           ffec::G1<snark_pp<ppT>> > &&g_A,
                           knowledge_commitment<ffec::G2<snark_pp<ppT>>,
                           ffec::G1<snark_pp<ppT>> > &&g_B,
                           knowledge_commitment<ffec::G1<snark_pp<ppT>>,
                           ffec::G1<snark_pp<ppT>> > &&g_C,
                           ffec::G1<snark_pp<ppT>> &&g_H,
                           ffec::G1<snark_pp<ppT>> &&g_K,
                           knowledge_commitment<ffec::G1<snark_pp<ppT>>,
                           ffec::G1<snark_pp<ppT>> > &&g_Aau,
                           ffec::G1<snark_pp<ppT>> &&muA) :
        g_A((g_A)),
        g_B((g_B)),
        g_C((g_C)),
        g_H((g_H)),
        g_K((g_K)),
        g_Aau((g_Aau)),
        muA((muA))
    {};

    size_t G1_size() const
    {
        return 10;
    }

    size_t G2_size() const
    {
        return 1;
    }

    size_t size_in_bits() const
    {
        return G1_size() * ffec::G1<snark_pp<ppT>>::size_in_bits() + G2_size() * ffec::G2<snark_pp<ppT>>::size_in_bits();
    }

    void print_size() const
    {
        ffec::print_indent(); print!("* G1 elements in proof: {}\n", self.G1_size());
        ffec::print_indent(); print!("* G2 elements in proof: {}\n", self.G2_size());
        ffec::print_indent(); print!("* Proof size in bits: {}\n", self.size_in_bits());
    }

    bool is_well_formed() const
    {
        return (g_A.g.is_well_formed() && g_A.h.is_well_formed() &&
                g_B.g.is_well_formed() && g_B.h.is_well_formed() &&
                g_C.g.is_well_formed() && g_C.h.is_well_formed() &&
                g_H.is_well_formed() &&
                g_K.is_well_formed() &&
                g_Aau.g.is_well_formed() && g_Aau.h.is_well_formed() &&
                muA.is_well_formed());
    }

    bool operator==(const r1cs_ppzkadsnark_proof<ppT> &other) const;
    friend std::ostream& operator<< <ppT>(std::ostream &out, const r1cs_ppzkadsnark_proof<ppT> &proof);
    friend std::istream& operator>> <ppT>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof);
};


/***************************** Main algorithms *******************************/

/**
 * R1CS ppZKADSNARK authentication parameters generator algorithm.
 */
template<typename ppT>
r1cs_ppzkadsnark_auth_keys<ppT> r1cs_ppzkadsnark_auth_generator(void);

/**
 * R1CS ppZKADSNARK authentication algorithm.
 */
template<typename ppT>
std::vector<r1cs_ppzkadsnark_auth_data<ppT>> r1cs_ppzkadsnark_auth_sign(
    const std::vector<ffec::Fr<snark_pp<ppT>>> &ins,
    const r1cs_ppzkadsnark_sec_auth_key<ppT> &sk,
    const std::vector<labelT> labels);

/**
 * R1CS ppZKADSNARK authentication verification algorithms.
 */
template<typename ppT>
bool r1cs_ppzkadsnark_auth_verify(const std::vector<ffec::Fr<snark_pp<ppT>>> &data,
                                  const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> & auth_data,
                                  const r1cs_ppzkadsnark_sec_auth_key<ppT> &sak,
                                  const std::vector<labelT> &labels);

template<typename ppT>
bool r1cs_ppzkadsnark_auth_verify(const std::vector<ffec::Fr<snark_pp<ppT>>> &data,
                                  const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> & auth_data,
                                  const r1cs_ppzkadsnark_pub_auth_key<ppT> &pak,
                                  const std::vector<labelT> &labels);

/**
 * A generator algorithm for the R1CS ppzkADSNARK.
 *
 * Given a R1CS constraint system CS, this algorithm produces proving and verification keys for CS.
 */
template<typename ppT>
r1cs_ppzkadsnark_keypair<ppT> r1cs_ppzkadsnark_generator(const r1cs_ppzkadsnark_constraint_system<ppT> &cs,
                                                         const r1cs_ppzkadsnark_pub_auth_prms<ppT> &prms);

/**
 * A prover algorithm for the R1CS ppzkADSNARK.
 *
 * Given a R1CS primary input X and a R1CS auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that CS(X,Y)=0''.
 * Above, CS is the R1CS constraint system that was given as input to the generator algorithm.
 */
template<typename ppT>
r1cs_ppzkadsnark_proof<ppT> r1cs_ppzkadsnark_prover(const r1cs_ppzkadsnark_proving_key<ppT> &pk,
                                                    const r1cs_ppzkadsnark_primary_input<ppT> &primary_input,
                                                    const r1cs_ppzkadsnark_auxiliary_input<ppT> &auxiliary_input,
                                                    const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> &auth_data);

/*
 Below are two variants of verifier algorithm for the R1CS ppzkADSNARK.

 These are the four cases that arise from the following choices:

1) The verifier accepts a (non-processed) verification key or, instead, a processed verification key.
     In the latter case, we call the algorithm an "online verifier".

2) The verifier uses the symmetric key or the public verification key.
     In the former case we call the algorithm a "symmetric verifier".

*/

/**
 * Convert a (non-processed) verification key into a processed verification key.
 */
template<typename ppT>
r1cs_ppzkadsnark_processed_verification_key<ppT> r1cs_ppzkadsnark_verifier_process_vk(
    const r1cs_ppzkadsnark_verification_key<ppT> &vk);

/**
 * A symmetric verifier algorithm for the R1CS ppzkADSNARK that
 * accepts a non-processed verification key
 */
template<typename ppT>
bool r1cs_ppzkadsnark_verifier(const r1cs_ppzkadsnark_verification_key<ppT> &vk,
                               const r1cs_ppzkadsnark_proof<ppT> &proof,
                               const r1cs_ppzkadsnark_sec_auth_key<ppT> & sak,
                               const std::vector<labelT> &labels);

/**
 * A symmetric verifier algorithm for the R1CS ppzkADSNARK that
 * accepts a processed verification key.
 */
template<typename ppT>
bool r1cs_ppzkadsnark_online_verifier(const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk,
                                      const r1cs_ppzkadsnark_proof<ppT> &proof,
                                      const r1cs_ppzkadsnark_sec_auth_key<ppT> & sak,
                                      const std::vector<labelT> &labels);


/**
 * A verifier algorithm for the R1CS ppzkADSNARK that
 * accepts a non-processed verification key
 */
template<typename ppT>
bool r1cs_ppzkadsnark_verifier(const r1cs_ppzkadsnark_verification_key<ppT> &vk,
                               const std::vector<r1cs_ppzkadsnark_auth_data<ppT>>  &auth_data,
                               const r1cs_ppzkadsnark_proof<ppT> &proof,
                               const r1cs_ppzkadsnark_pub_auth_key<ppT> & pak,
                               const std::vector<labelT> &labels);

/**
 * A verifier algorithm for the R1CS ppzkADSNARK that
 * accepts a processed verification key.
 */
template<typename ppT>
bool r1cs_ppzkadsnark_online_verifier(const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk,
                                      const std::vector<r1cs_ppzkadsnark_auth_data<ppT>>  &auth_data,
                                      const r1cs_ppzkadsnark_proof<ppT> &proof,
                                      const r1cs_ppzkadsnark_pub_auth_key<ppT> & pak,
                                      const std::vector<labelT> &labels);




use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/r1cs_ppzkadsnark;

//#endif // R1CS_PPZKSNARK_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a ppzkADSNARK for R1CS.

See r1cs_ppzkadsnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef R1CS_PPZKADSNARK_TCC_
// #define R1CS_PPZKADSNARK_TCC_

use  <algorithm>
use  <cassert>
use  <functional>
use  <iostream>
use  <sstream>

 use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

// #ifdef MULTICORE
use  <omp.h>
//#endif

use crate::knowledge_commitment::kc_multiexp;
use crate::reductions::r1cs_to_qap::r1cs_to_qap;




template<typename ppT>
bool r1cs_ppzkadsnark_pub_auth_prms<ppT>::operator==(const r1cs_ppzkadsnark_pub_auth_prms<ppT> &other) const
{
    return (self.I1 == other.I1);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap)
{
    out << pap.I1;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_prms<ppT> &pap)
{
    in >> pap.I1;

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_sec_auth_key<ppT>::operator==(const r1cs_ppzkadsnark_sec_auth_key<ppT> &other) const
{
    return (self.i == other.i) &&
        (self.skp == other.skp) &&
        (self.S == other.S);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_sec_auth_key<ppT> &key)
{
    out << key.i;
    out << key.skp;
    out << key.S;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_sec_auth_key<ppT> &key)
{
    in >> key.i;
    in >> key.skp;
    in >> key.S;

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_pub_auth_key<ppT>::operator==(const r1cs_ppzkadsnark_pub_auth_key<ppT> &other) const
{
    return (self.minusI2 == other.minusI2) &&
        (self.vkp == other.vkp);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_pub_auth_key<ppT> &key)
{
    out << key.minusI2;
    out << key.vkp;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_pub_auth_key<ppT> &key)
{
    in >> key.minusI2;
    in >> key.vkp;

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_auth_data<ppT>::operator==(const r1cs_ppzkadsnark_auth_data<ppT> &other) const
{
    return (self.mu == other.mu) &&
        (self.Lambda == other.Lambda) &&
        (self.sigma == other.sigma);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_auth_data<ppT> &data)
{
    out << data.mu;
    out << data.Lambda;
    out << data.sigma;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_auth_data<ppT> &data)
{
    in >> data.mu;
    in >> data.Lambda;
    data.sigma;

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_proving_key<ppT>::operator==(const r1cs_ppzkadsnark_proving_key<ppT> &other) const
{
    return (self.A_query == other.A_query &&
            self.B_query == other.B_query &&
            self.C_query == other.C_query &&
            self.H_query == other.H_query &&
            self.K_query == other.K_query &&
            self.rA_i_Z_g1 == other.rA_i_Z_g1 &&
            self.constraint_system == other.constraint_system);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_proving_key<ppT> &pk)
{
    out << pk.A_query;
    out << pk.B_query;
    out << pk.C_query;
    out << pk.H_query;
    out << pk.K_query;
    out << pk.rA_i_Z_g1;
    out << pk.constraint_system;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proving_key<ppT> &pk)
{
    in >> pk.A_query;
    in >> pk.B_query;
    in >> pk.C_query;
    in >> pk.H_query;
    in >> pk.K_query;
    in >> pk.rA_i_Z_g1;
    in >> pk.constraint_system;

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_verification_key<ppT>::operator==(const r1cs_ppzkadsnark_verification_key<ppT> &other) const
{
    return (self.alphaA_g2 == other.alphaA_g2 &&
            self.alphaB_g1 == other.alphaB_g1 &&
            self.alphaC_g2 == other.alphaC_g2 &&
            self.gamma_g2 == other.gamma_g2 &&
            self.gamma_beta_g1 == other.gamma_beta_g1 &&
            self.gamma_beta_g2 == other.gamma_beta_g2 &&
            self.rC_Z_g2 == other.rC_Z_g2 &&
            self.A0 == other.A0 &&
            self.Ain == other.Ain);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_verification_key<ppT> &vk)
{
    out << vk.alphaA_g2 << OUTPUT_NEWLINE;
    out << vk.alphaB_g1 << OUTPUT_NEWLINE;
    out << vk.alphaC_g2 << OUTPUT_NEWLINE;
    out << vk.gamma_g2 << OUTPUT_NEWLINE;
    out << vk.gamma_beta_g1 << OUTPUT_NEWLINE;
    out << vk.gamma_beta_g2 << OUTPUT_NEWLINE;
    out << vk.rC_Z_g2 << OUTPUT_NEWLINE;
    out << vk.A0 << OUTPUT_NEWLINE;
    out << vk.Ain << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_verification_key<ppT> &vk)
{
    in >> vk.alphaA_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.alphaB_g1;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.alphaC_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.gamma_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.gamma_beta_g1;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.gamma_beta_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.rC_Z_g2;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.A0;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> vk.Ain;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_processed_verification_key<ppT>::operator==(
    const r1cs_ppzkadsnark_processed_verification_key<ppT> &other) const
{
    bool result = (self.pp_G2_one_precomp == other.pp_G2_one_precomp &&
                   self.vk_alphaA_g2_precomp == other.vk_alphaA_g2_precomp &&
                   self.vk_alphaB_g1_precomp == other.vk_alphaB_g1_precomp &&
                   self.vk_alphaC_g2_precomp == other.vk_alphaC_g2_precomp &&
                   self.vk_rC_Z_g2_precomp == other.vk_rC_Z_g2_precomp &&
                   self.vk_gamma_g2_precomp == other.vk_gamma_g2_precomp &&
                   self.vk_gamma_beta_g1_precomp == other.vk_gamma_beta_g1_precomp &&
                   self.vk_gamma_beta_g2_precomp == other.vk_gamma_beta_g2_precomp &&
                   self.vk_rC_i_g2_precomp == other.vk_rC_i_g2_precomp &&
                   self.A0 == other.A0 &&
                   self.Ain == other.Ain &&
                   self.proof_g_vki_precomp.size() == other.proof_g_vki_precomp.size());
    if (result) {
        for(size_t i=0;i<self.proof_g_vki_precomp.size();i++)
            result &= self.proof_g_vki_precomp[i] == other.proof_g_vki_precomp[i];
    }
    return result;
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk)
{
    out << pvk.pp_G2_one_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_alphaA_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_alphaB_g1_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_alphaC_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_rC_Z_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_gamma_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_gamma_beta_g1_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_gamma_beta_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.vk_rC_i_g2_precomp << OUTPUT_NEWLINE;
    out << pvk.A0 << OUTPUT_NEWLINE;
    out << pvk.Ain << OUTPUT_NEWLINE;
    out << pvk.proof_g_vki_precomp  << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk)
{
    in >> pvk.pp_G2_one_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_alphaA_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_alphaB_g1_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_alphaC_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_rC_Z_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_gamma_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_gamma_beta_g1_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_gamma_beta_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.vk_rC_i_g2_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.A0;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.Ain;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> pvk.proof_g_vki_precomp;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
bool r1cs_ppzkadsnark_proof<ppT>::operator==(const r1cs_ppzkadsnark_proof<ppT> &other) const
{
    return (self.g_A == other.g_A &&
            self.g_B == other.g_B &&
            self.g_C == other.g_C &&
            self.g_H == other.g_H &&
            self.g_K == other.g_K &&
            self.g_Aau == other.g_Aau &&
            self.muA == other.muA);
}

template<typename ppT>
std::ostream& operator<<(std::ostream &out, const r1cs_ppzkadsnark_proof<ppT> &proof)
{
    out << proof.g_A << OUTPUT_NEWLINE;
    out << proof.g_B << OUTPUT_NEWLINE;
    out << proof.g_C << OUTPUT_NEWLINE;
    out << proof.g_H << OUTPUT_NEWLINE;
    out << proof.g_K << OUTPUT_NEWLINE;
    out << proof.g_Aau << OUTPUT_NEWLINE;
    out << proof.muA << OUTPUT_NEWLINE;

    return out;
}

template<typename ppT>
std::istream& operator>>(std::istream &in, r1cs_ppzkadsnark_proof<ppT> &proof)
{
    in >> proof.g_A;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.g_B;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.g_C;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.g_H;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.g_K;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.g_Aau;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> proof.muA;
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename ppT>
r1cs_ppzkadsnark_verification_key<ppT> r1cs_ppzkadsnark_verification_key<ppT>::dummy_verification_key(const size_t input_size)
{
    r1cs_ppzkadsnark_verification_key<ppT> result;
    result.alphaA_g2 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G2<snark_pp<ppT>>::one();
    result.alphaB_g1 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G1<snark_pp<ppT>>::one();
    result.alphaC_g2 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G2<snark_pp<ppT>>::one();
    result.gamma_g2 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G2<snark_pp<ppT>>::one();
    result.gamma_beta_g1 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G1<snark_pp<ppT>>::one();
    result.gamma_beta_g2 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G2<snark_pp<ppT>>::one();
    result.rC_Z_g2 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G2<snark_pp<ppT>>::one();

    result.A0 = ffec::Fr<snark_pp<ppT>>::random_element() * ffec::G1<snark_pp<ppT>>::one();
    for (size_t i = 0; i < input_size; ++i)
    {
        result.Ain.push(ffec::Fr<snark_pp<ppT>>::random_element() *
                                ffec::G1<snark_pp<ppT>>::one());
    }

    return result;
}

template<typename ppT>
r1cs_ppzkadsnark_auth_keys<ppT> r1cs_ppzkadsnark_auth_generator(void) {
    kpT<ppT> sigkp = sigGen<ppT>();
    r1cs_ppzkadsnark_prfKeyT<ppT>prfseed = prfGen<ppT>();
    ffec::Fr<snark_pp<ppT>> i = ffec::Fr<snark_pp<ppT>>::random_element();
    ffec::G1<snark_pp<ppT>> I1 = i * ffec::G1<snark_pp<ppT>>::one();
    ffec::G2<snark_pp<ppT>> minusI2 = ffec::G2<snark_pp<ppT>>::zero() -
        i * ffec::G2<snark_pp<ppT>>::one();
    return r1cs_ppzkadsnark_auth_keys<ppT>(
        r1cs_ppzkadsnark_pub_auth_prms<ppT>((I1)),
        r1cs_ppzkadsnark_pub_auth_key<ppT>((minusI2),(sigkp.vk)),
        r1cs_ppzkadsnark_sec_auth_key<ppT>((i),(sigkp.sk),(prfseed)));
}

template<typename ppT>
std::vector<r1cs_ppzkadsnark_auth_data<ppT>> r1cs_ppzkadsnark_auth_sign(
    const std::vector<ffec::Fr<snark_pp<ppT>>> &ins,
    const r1cs_ppzkadsnark_sec_auth_key<ppT> &sk,
    const std::vector<labelT> labels) {
    ffec::enter_block("Call to r1cs_ppzkadsnark_auth_sign");
    assert (labels.size()==ins.size());
    std::vector<r1cs_ppzkadsnark_auth_data<ppT>> res;
    res.reserve(ins.size());
    for (size_t i = 0; i < ins.size();i++) {
        ffec::Fr<snark_pp<ppT>> lambda = prfCompute<ppT>(sk.S,labels[i]);
        ffec::G2<snark_pp<ppT>> Lambda = lambda * ffec::G2<snark_pp<ppT>>::one();
        r1cs_ppzkadsnark_sigT<ppT>sig = sigSign<ppT>(sk.skp,labels[i],Lambda);
        r1cs_ppzkadsnark_auth_data<ppT> val((lambda + sk.i * ins[i]),
                                            (Lambda),
                                            (sig));
        res.push(val);
    }
    ffec::leave_block("Call to r1cs_ppzkadsnark_auth_sign");
    return (res);
}

// symmetric
template<typename ppT>
bool r1cs_ppzkadsnark_auth_verify(const std::vector<ffec::Fr<snark_pp<ppT>>> &data,
                                  const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> & auth_data,
                                  const r1cs_ppzkadsnark_sec_auth_key<ppT> &sak,
                                  const std::vector<labelT> &labels) {
    ffec::enter_block("Call to r1cs_ppzkadsnark_auth_verify");
    assert ((data.size()==labels.size()) && (auth_data.size()==labels.size()));
    bool res = true;
    for (size_t i = 0; i < data.size();i++) {
        ffec::Fr<snark_pp<ppT>> lambda = prfCompute<ppT>(sak.S,labels[i]);
        ffec::Fr<snark_pp<ppT>> mup = lambda + sak.i * data[i];
        res = res && (auth_data[i].mu == mup);
    }
    ffec::leave_block("Call to r1cs_ppzkadsnark_auth_verify");
    return res;
}

// public
template<typename ppT>
bool r1cs_ppzkadsnark_auth_verify(const std::vector<ffec::Fr<snark_pp<ppT>>> &data,
                                  const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> & auth_data,
                                  const r1cs_ppzkadsnark_pub_auth_key<ppT> &pak,
                                  const std::vector<labelT> &labels) {
    ffec::enter_block("Call to r1cs_ppzkadsnark_auth_verify");
    assert ((data.size()==labels.size()) && (data.size()==auth_data.size()));
    bool res = true;
    for (size_t i = 0; i < auth_data.size();i++) {
        ffec::G2<snark_pp<ppT>> Mup = auth_data[i].Lambda - data[i] * pak.minusI2;
        res = res && (auth_data[i].mu * ffec::G2<snark_pp<ppT>>::one() == Mup);
        res = res && sigVerif<ppT>(pak.vkp,labels[i],auth_data[i].Lambda,auth_data[i].sigma);
    }
    ffec::leave_block("Call to r1cs_ppzkadsnark_auth_verify");
    return res;
}

template <typename ppT>
r1cs_ppzkadsnark_keypair<ppT> r1cs_ppzkadsnark_generator(const r1cs_ppzkadsnark_constraint_system<ppT> &cs,
                                                         const r1cs_ppzkadsnark_pub_auth_prms<ppT> &prms)
{
    ffec::enter_block("Call to r1cs_ppzkadsnark_generator");

    /* make the B_query "lighter" if possible */
    r1cs_ppzkadsnark_constraint_system<ppT> cs_copy(cs);
    cs_copy.swap_AB_if_beneficial();

    /* draw random element at which the QAP is evaluated */
    const  ffec::Fr<snark_pp<ppT>> t = ffec::Fr<snark_pp<ppT>>::random_element();

    qap_instance_evaluation<ffec::Fr<snark_pp<ppT>> > qap_inst =
        r1cs_to_qap_instance_map_with_evaluation(cs_copy, t);

    ffec::print_indent(); print!("* QAP number of variables: {}\n", qap_inst.num_variables());
    ffec::print_indent(); print!("* QAP pre degree: {}\n", cs_copy.constraints.size());
    ffec::print_indent(); print!("* QAP degree: {}\n", qap_inst.degree());
    ffec::print_indent(); print!("* QAP number of input variables: {}\n", qap_inst.num_inputs());

    ffec::enter_block("Compute query densities");
    size_t non_zero_At = 0, non_zero_Bt = 0, non_zero_Ct = 0, non_zero_Ht = 0;
    for (size_t i = 0; i < qap_inst.num_variables()+1; ++i)
    {
        if (!qap_inst.At[i].is_zero())
        {
            non_zero_At+=1;
        }
        if (!qap_inst.Bt[i].is_zero())
        {
            non_zero_Bt+=1;
        }
        if (!qap_inst.Ct[i].is_zero())
        {
            non_zero_Ct+=1;
        }
    }
    for (size_t i = 0; i < qap_inst.degree()+1; ++i)
    {
        if (!qap_inst.Ht[i].is_zero())
        {
            non_zero_Ht+=1;
        }
    }
    ffec::leave_block("Compute query densities");

    ffec::Fr_vector<snark_pp<ppT>> At = (qap_inst.At); // qap_inst.At is now in unspecified state, but we do not use it later
    ffec::Fr_vector<snark_pp<ppT>> Bt = (qap_inst.Bt); // qap_inst.Bt is now in unspecified state, but we do not use it later
    ffec::Fr_vector<snark_pp<ppT>> Ct = (qap_inst.Ct); // qap_inst.Ct is now in unspecified state, but we do not use it later
    ffec::Fr_vector<snark_pp<ppT>> Ht = (qap_inst.Ht); // qap_inst.Ht is now in unspecified state, but we do not use it later

    /* append Zt to At,Bt,Ct with */
    At.push(qap_inst.Zt);
    Bt.push(qap_inst.Zt);
    Ct.push(qap_inst.Zt);

    const  ffec::Fr<snark_pp<ppT>> alphaA = ffec::Fr<snark_pp<ppT>>::random_element(),
        alphaB = ffec::Fr<snark_pp<ppT>>::random_element(),
        alphaC = ffec::Fr<snark_pp<ppT>>::random_element(),
        rA = ffec::Fr<snark_pp<ppT>>::random_element(),
        rB = ffec::Fr<snark_pp<ppT>>::random_element(),
        beta = ffec::Fr<snark_pp<ppT>>::random_element(),
        gamma = ffec::Fr<snark_pp<ppT>>::random_element();
    const ffec::Fr<snark_pp<ppT>>      rC = rA * rB;

    // construct the same-coefficient-check query (must happen before zeroing out the prefix of At)
    ffec::Fr_vector<snark_pp<ppT>> Kt;
    Kt.reserve(qap_inst.num_variables()+4);
    for (size_t i = 0; i < qap_inst.num_variables()+1; ++i)
    {
        Kt.push( beta * (rA * At[i] + rB * Bt[i] + rC * Ct[i] ) );
    }
    Kt.push(beta * rA * qap_inst.Zt);
    Kt.push(beta * rB * qap_inst.Zt);
    Kt.push(beta * rC * qap_inst.Zt);

    const size_t g1_exp_count = 2*(non_zero_At - qap_inst.num_inputs() + non_zero_Ct) + non_zero_Bt + non_zero_Ht + Kt.size();
    const size_t g2_exp_count = non_zero_Bt;

    size_t g1_window = ffec::get_exp_window_size<ffec::G1<snark_pp<ppT>> >(g1_exp_count);
    size_t g2_window = ffec::get_exp_window_size<ffec::G2<snark_pp<ppT>> >(g2_exp_count);
    ffec::print_indent(); print!("* G1 window: {}\n", g1_window);
    ffec::print_indent(); print!("* G2 window: {}\n", g2_window);

// #ifdef MULTICORE
    const size_t chunks = omp_get_max_threads(); // to override, set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
//#endif

    ffec::enter_block("Generating G1 multiexp table");
    ffec::window_table<ffec::G1<snark_pp<ppT>> > g1_table =
        get_window_table(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g1_window,
                         ffec::G1<snark_pp<ppT>>::one());
    ffec::leave_block("Generating G1 multiexp table");

    ffec::enter_block("Generating G2 multiexp table");
    ffec::window_table<ffec::G2<snark_pp<ppT>> > g2_table =
        get_window_table(ffec::Fr<snark_pp<ppT>>::size_in_bits(),
                         g2_window, ffec::G2<snark_pp<ppT>>::one());
    ffec::leave_block("Generating G2 multiexp table");

    ffec::enter_block("Generate R1CS proving key");

    ffec::enter_block("Generate knowledge commitments");
    ffec::enter_block("Compute the A-query", false);
    knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > A_query =
        kc_batch_exp(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g1_window, g1_window, g1_table,
                     g1_table, rA, rA*alphaA, At, chunks);
    ffec::leave_block("Compute the A-query", false);

    ffec::enter_block("Compute the B-query", false);
    knowledge_commitment_vector<ffec::G2<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > B_query =
        kc_batch_exp(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g2_window, g1_window, g2_table,
                     g1_table, rB, rB*alphaB, Bt, chunks);
    ffec::leave_block("Compute the B-query", false);

    ffec::enter_block("Compute the C-query", false);
    knowledge_commitment_vector<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > C_query =
        kc_batch_exp(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g1_window, g1_window, g1_table,
                     g1_table, rC, rC*alphaC, Ct, chunks);
    ffec::leave_block("Compute the C-query", false);

    ffec::enter_block("Compute the H-query", false);
    ffec::G1_vector<snark_pp<ppT>> H_query = batch_exp(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g1_window, g1_table, Ht);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G1<snark_pp<ppT>> >(H_query);
//#endif
    ffec::leave_block("Compute the H-query", false);

    ffec::enter_block("Compute the K-query", false);
    ffec::G1_vector<snark_pp<ppT>> K_query = batch_exp(ffec::Fr<snark_pp<ppT>>::size_in_bits(), g1_window, g1_table, Kt);
// #ifdef USE_MIXED_ADDITION
    ffec::batch_to_special<ffec::G1<snark_pp<ppT>> >(K_query);
//#endif
    ffec::leave_block("Compute the K-query", false);

    ffec::leave_block("Generate knowledge commitments");

    ffec::leave_block("Generate R1CS proving key");

    ffec::enter_block("Generate R1CS verification key");
    ffec::G2<snark_pp<ppT>> alphaA_g2 = alphaA * ffec::G2<snark_pp<ppT>>::one();
    ffec::G1<snark_pp<ppT>> alphaB_g1 = alphaB * ffec::G1<snark_pp<ppT>>::one();
    ffec::G2<snark_pp<ppT>> alphaC_g2 = alphaC * ffec::G2<snark_pp<ppT>>::one();
    ffec::G2<snark_pp<ppT>> gamma_g2 = gamma * ffec::G2<snark_pp<ppT>>::one();
    ffec::G1<snark_pp<ppT>> gamma_beta_g1 = (gamma * beta) * ffec::G1<snark_pp<ppT>>::one();
    ffec::G2<snark_pp<ppT>> gamma_beta_g2 = (gamma * beta) * ffec::G2<snark_pp<ppT>>::one();
    ffec::G2<snark_pp<ppT>> rC_Z_g2 = (rC * qap_inst.Zt) * ffec::G2<snark_pp<ppT>>::one();

    ffec::enter_block("Generate extra authentication elements");
    ffec::G1<snark_pp<ppT>> rA_i_Z_g1 = (rA * qap_inst.Zt) * prms.I1;
    ffec::leave_block("Generate extra authentication elements");

    ffec::enter_block("Copy encoded input coefficients for R1CS verification key");
    ffec::G1<snark_pp<ppT>> A0 = A_query[0].g;
    ffec::G1_vector<snark_pp<ppT>> Ain;
    Ain.reserve(qap_inst.num_inputs());
    for (size_t i = 0; i < qap_inst.num_inputs(); ++i)
    {
        Ain.push(A_query[1+i].g);
    }

    ffec::leave_block("Copy encoded input coefficients for R1CS verification key");

    ffec::leave_block("Generate R1CS verification key");

    ffec::leave_block("Call to r1cs_ppzkadsnark_generator");

    r1cs_ppzkadsnark_verification_key<ppT> vk = r1cs_ppzkadsnark_verification_key<ppT>(alphaA_g2,
                                                                                       alphaB_g1,
                                                                                       alphaC_g2,
                                                                                       gamma_g2,
                                                                                       gamma_beta_g1,
                                                                                       gamma_beta_g2,
                                                                                       rC_Z_g2,
                                                                                       A0,
                                                                                       Ain);
    r1cs_ppzkadsnark_proving_key<ppT> pk = r1cs_ppzkadsnark_proving_key<ppT>((A_query),
                                                                             (B_query),
                                                                             (C_query),
                                                                             (H_query),
                                                                             (K_query),
                                                                             (rA_i_Z_g1),
                                                                             (cs_copy));

    pk.print_size();
    vk.print_size();

    return r1cs_ppzkadsnark_keypair<ppT>((pk), (vk));
}

template <typename ppT>
r1cs_ppzkadsnark_proof<ppT> r1cs_ppzkadsnark_prover(const r1cs_ppzkadsnark_proving_key<ppT> &pk,
                                                    const r1cs_ppzkadsnark_primary_input<ppT> &primary_input,
                                                    const r1cs_ppzkadsnark_auxiliary_input<ppT> &auxiliary_input,
                                                    const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> &auth_data)
{
    ffec::enter_block("Call to r1cs_ppzkadsnark_prover");

// #ifdef DEBUG
    assert!(pk.constraint_system.is_satisfied(primary_input, auxiliary_input));
//#endif

    const ffec::Fr<snark_pp<ppT>> d1 = ffec::Fr<snark_pp<ppT>>::random_element(),
        d2 = ffec::Fr<snark_pp<ppT>>::random_element(),
        d3 = ffec::Fr<snark_pp<ppT>>::random_element(),
        dauth = ffec::Fr<snark_pp<ppT>>::random_element();

    ffec::enter_block("Compute the polynomial H");
    const qap_witness<ffec::Fr<snark_pp<ppT>> > qap_wit = r1cs_to_qap_witness_map(pk.constraint_system, primary_input,
                                                                            auxiliary_input, d1 + dauth, d2, d3);
    ffec::leave_block("Compute the polynomial H");

// #ifdef DEBUG
    const ffec::Fr<snark_pp<ppT>> t = ffec::Fr<snark_pp<ppT>>::random_element();
    qap_instance_evaluation<ffec::Fr<snark_pp<ppT>> > qap_inst = r1cs_to_qap_instance_map_with_evaluation(pk.constraint_system, t);
    assert!(qap_inst.is_satisfied(qap_wit));
//#endif

    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_A =
        /* pk.A_query[0] + */ d1*pk.A_query[qap_wit.num_variables()+1];
    knowledge_commitment<ffec::G2<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_B =
        pk.B_query[0] + qap_wit.d2*pk.B_query[qap_wit.num_variables()+1];
    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_C =
        pk.C_query[0] + qap_wit.d3*pk.C_query[qap_wit.num_variables()+1];

    knowledge_commitment<ffec::G1<snark_pp<ppT>>, ffec::G1<snark_pp<ppT>> > g_Ain = dauth*pk.A_query[qap_wit.num_variables()+1];

    ffec::G1<snark_pp<ppT>> g_H = ffec::G1<snark_pp<ppT>>::zero();
    ffec::G1<snark_pp<ppT>> g_K = (pk.K_query[0] +
                             qap_wit.d1*pk.K_query[qap_wit.num_variables()+1] +
                             qap_wit.d2*pk.K_query[qap_wit.num_variables()+2] +
                             qap_wit.d3*pk.K_query[qap_wit.num_variables()+3]);

// #ifdef DEBUG
    for (size_t i = 0; i < qap_wit.num_inputs() + 1; ++i)
    {
        assert!(pk.A_query[i].g == ffec::G1<snark_pp<ppT>>::zero());
    }
    assert!(pk.A_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.B_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.C_query.domain_size() == qap_wit.num_variables()+2);
    assert!(pk.H_query.size() == qap_wit.degree()+1);
    assert!(pk.K_query.size() == qap_wit.num_variables()+4);
//#endif

// #ifdef MULTICORE
    const size_t chunks = omp_get_max_threads(); // to override, set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
//#endif

    ffec::enter_block("Compute the proof");

    ffec::enter_block("Compute answer to A-query", false);
    g_A = g_A + kc_multi_exp_with_mixed_addition<ffec::G1<snark_pp<ppT>>,
                                                 ffec::G1<snark_pp<ppT>>,
                                                 ffec::Fr<snark_pp<ppT>>,
                                                 ffec::multi_exp_method_bos_coster>(
        pk.A_query,
        1+qap_wit.num_inputs(), 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_inputs(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute answer to A-query", false);

    ffec::enter_block("Compute answer to Ain-query", false);
    g_Ain = g_Ain + kc_multi_exp_with_mixed_addition<ffec::G1<snark_pp<ppT>>,
                                                     ffec::G1<snark_pp<ppT>>,
                                                     ffec::Fr<snark_pp<ppT>>,
                                                     ffec::multi_exp_method_bos_coster>(
        pk.A_query,
        1, 1+qap_wit.num_inputs(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_inputs(),
        chunks);
    //std :: cout << "The input proof term: " << g_Ain << "\n";
    ffec::leave_block("Compute answer to Ain-query", false);

    ffec::enter_block("Compute answer to B-query", false);
    g_B = g_B + kc_multi_exp_with_mixed_addition<ffec::G2<snark_pp<ppT>>,
                                                 ffec::G1<snark_pp<ppT>>,
                                                 ffec::Fr<snark_pp<ppT>>,
                                                 ffec::multi_exp_method_bos_coster>(
        pk.B_query,
        1, 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute answer to B-query", false);

    ffec::enter_block("Compute answer to C-query", false);
    g_C = g_C + kc_multi_exp_with_mixed_addition<ffec::G1<snark_pp<ppT>>,
                                                 ffec::G1<snark_pp<ppT>>,
                                                 ffec::Fr<snark_pp<ppT>>,
                                                 ffec::multi_exp_method_bos_coster>(
        pk.C_query,
        1, 1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute answer to C-query", false);

    ffec::enter_block("Compute answer to H-query", false);
    g_H = g_H + ffec::multi_exp<ffec::G1<snark_pp<ppT>>,
                                 ffec::Fr<snark_pp<ppT>>,
                                 ffec::multi_exp_method_BDLO12>(
        pk.H_query.begin(),
        pk.H_query.begin()+qap_wit.degree()+1,
        qap_wit.coefficients_for_H.begin(),
        qap_wit.coefficients_for_H.begin()+qap_wit.degree()+1,
        chunks);
    ffec::leave_block("Compute answer to H-query", false);

    ffec::enter_block("Compute answer to K-query", false);
    g_K = g_K + ffec::multi_exp_with_mixed_addition<ffec::G1<snark_pp<ppT>>,
                                                     ffec::Fr<snark_pp<ppT>>,
                                                     ffec::multi_exp_method_bos_coster>(
        pk.K_query.begin()+1,
        pk.K_query.begin()+1+qap_wit.num_variables(),
        qap_wit.coefficients_for_ABCs.begin(),
        qap_wit.coefficients_for_ABCs.begin()+qap_wit.num_variables(),
        chunks);
    ffec::leave_block("Compute answer to K-query", false);

    ffec::enter_block("Compute extra auth terms", false);
    std::vector<ffec::Fr<snark_pp<ppT>>> mus;
    std::vector<ffec::G1<snark_pp<ppT>>> Ains;
    mus.reserve(qap_wit.num_inputs());
    Ains.reserve(qap_wit.num_inputs());
    for (size_t i=0;i<qap_wit.num_inputs();i++) {
        mus.push(auth_data[i].mu);
        Ains.push(pk.A_query[i+1].g);
    }
    ffec::G1<snark_pp<ppT>> muA = dauth * pk.rA_i_Z_g1;
    muA = muA + ffec::multi_exp<ffec::G1<snark_pp<ppT>>,
                                 ffec::Fr<snark_pp<ppT>>,
                                 ffec::multi_exp_method_bos_coster>(
        Ains.begin(), Ains.begin()+qap_wit.num_inputs(),
        mus.begin(), mus.begin()+qap_wit.num_inputs(),
        chunks);

    // To Do: Decide whether to include relevant parts of auth_data in proof
    ffec::leave_block("Compute extra auth terms", false);

    ffec::leave_block("Compute the proof");

    ffec::leave_block("Call to r1cs_ppzkadsnark_prover");

    r1cs_ppzkadsnark_proof<ppT> proof = r1cs_ppzkadsnark_proof<ppT>((g_A),
                                                                    (g_B),
                                                                    (g_C),
                                                                    (g_H),
                                                                    (g_K),
                                                                    (g_Ain),
                                                                    (muA));
    proof.print_size();

    return proof;
}

template <typename ppT>
r1cs_ppzkadsnark_processed_verification_key<ppT> r1cs_ppzkadsnark_verifier_process_vk(
    const r1cs_ppzkadsnark_verification_key<ppT> &vk)
{
    ffec::enter_block("Call to r1cs_ppzkadsnark_verifier_process_vk");

    r1cs_ppzkadsnark_processed_verification_key<ppT> pvk;
    pvk.pp_G2_one_precomp        = snark_pp<ppT>::precompute_G2(ffec::G2<snark_pp<ppT>>::one());
    pvk.vk_alphaA_g2_precomp     = snark_pp<ppT>::precompute_G2(vk.alphaA_g2);
    pvk.vk_alphaB_g1_precomp     = snark_pp<ppT>::precompute_G1(vk.alphaB_g1);
    pvk.vk_alphaC_g2_precomp     = snark_pp<ppT>::precompute_G2(vk.alphaC_g2);
    pvk.vk_rC_Z_g2_precomp       = snark_pp<ppT>::precompute_G2(vk.rC_Z_g2);
    pvk.vk_gamma_g2_precomp      = snark_pp<ppT>::precompute_G2(vk.gamma_g2);
    pvk.vk_gamma_beta_g1_precomp = snark_pp<ppT>::precompute_G1(vk.gamma_beta_g1);
    pvk.vk_gamma_beta_g2_precomp = snark_pp<ppT>::precompute_G2(vk.gamma_beta_g2);

    ffec::enter_block("Pre-processing for additional auth elements");
    ffec::G2_precomp<snark_pp<ppT>> vk_rC_z_g2_precomp = snark_pp<ppT>::precompute_G2(vk.rC_Z_g2);

    pvk.A0 = ffec::G1<snark_pp<ppT>>(vk.A0);
    pvk.Ain = ffec::G1_vector<snark_pp<ppT>>(vk.Ain);

    pvk.proof_g_vki_precomp.reserve(pvk.Ain.size());
    for(size_t i = 0; i < pvk.Ain.size();i++) {
        pvk.proof_g_vki_precomp.push(snark_pp<ppT>::precompute_G1(pvk.Ain[i]));
    }

    ffec::leave_block("Pre-processing for additional auth elements");

    ffec::leave_block("Call to r1cs_ppzkadsnark_verifier_process_vk");

    return pvk;
}

// symmetric
template<typename ppT>
bool r1cs_ppzkadsnark_online_verifier(const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk,
                                      const r1cs_ppzkadsnark_proof<ppT> &proof,
                                      const r1cs_ppzkadsnark_sec_auth_key<ppT> & sak,
                                      const std::vector<labelT> &labels)
{
    bool result = true;
    ffec::enter_block("Call to r1cs_ppzkadsnark_online_verifier");

    ffec::enter_block("Check if the proof is well-formed");
    if (!proof.is_well_formed())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    ffec::leave_block("Check if the proof is well-formed");

    ffec::enter_block("Checking auth-specific elements");

    ffec::enter_block("Checking A1");

    ffec::enter_block("Compute PRFs");
    std::vector<ffec::Fr<snark_pp<ppT>>>lambdas;
    lambdas.reserve(labels.size());
    for (size_t i = 0; i < labels.size();i++) {
        lambdas.push(prfCompute<ppT>(sak.S,labels[i]));
    }
    ffec::leave_block("Compute PRFs");
    ffec::G1<snark_pp<ppT>> prodA = sak.i * proof.g_Aau.g;
    prodA = prodA + ffec::multi_exp<ffec::G1<snark_pp<ppT>>,
                                     ffec::Fr<snark_pp<ppT>>,
                                     ffec::multi_exp_method_bos_coster>(
        pvk.Ain.begin(),
        pvk.Ain.begin() + labels.size(),
        lambdas.begin(),
        lambdas.begin() + labels.size(), 1);

    bool result_auth = true;

    if (!(prodA == proof.muA)) {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Authentication check failed.\n");
        }
        result_auth = false;
    }

    ffec::leave_block("Checking A1");

    ffec::enter_block("Checking A2");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aau_g_precomp      = snark_pp<ppT>::precompute_G1(proof.g_Aau.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aau_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_Aau.h);
    ffec::Fqk<snark_pp<ppT>> kc_Aau_1 = snark_pp<ppT>::miller_loop(proof_g_Aau_g_precomp, pvk.vk_alphaA_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_Aau_2 = snark_pp<ppT>::miller_loop(proof_g_Aau_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_Aau = snark_pp<ppT>::final_exponentiation(kc_Aau_1 * kc_Aau_2.unitary_inverse());
    if (kc_Aau != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for Aau query incorrect.\n");
        }
        result_auth = false;
    }
    ffec::leave_block("Checking A2");

    ffec::leave_block("Checking auth-specific elements");

    result &= result_auth;

    ffec::enter_block("Online pairing computations");
    ffec::enter_block("Check knowledge commitment for A is valid");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_A_g_precomp      = snark_pp<ppT>::precompute_G1(proof.g_A.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_A_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_A.h);
    ffec::Fqk<snark_pp<ppT>> kc_A_1 = snark_pp<ppT>::miller_loop(proof_g_A_g_precomp,      pvk.vk_alphaA_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_A_2 = snark_pp<ppT>::miller_loop(proof_g_A_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_A = snark_pp<ppT>::final_exponentiation(kc_A_1 * kc_A_2.unitary_inverse());
    if (kc_A != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for A is valid");

    ffec::enter_block("Check knowledge commitment for B is valid");
    ffec::G2_precomp<snark_pp<ppT>> proof_g_B_g_precomp      = snark_pp<ppT>::precompute_G2(proof.g_B.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_B_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_B.h);
    ffec::Fqk<snark_pp<ppT>> kc_B_1 = snark_pp<ppT>::miller_loop(pvk.vk_alphaB_g1_precomp, proof_g_B_g_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_B_2 = snark_pp<ppT>::miller_loop(proof_g_B_h_precomp,    pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_B = snark_pp<ppT>::final_exponentiation(kc_B_1 * kc_B_2.unitary_inverse());
    if (kc_B != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for B is valid");

    ffec::enter_block("Check knowledge commitment for C is valid");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_C_g_precomp      = snark_pp<ppT>::precompute_G1(proof.g_C.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_C_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_C.h);
    ffec::Fqk<snark_pp<ppT>> kc_C_1 = snark_pp<ppT>::miller_loop(proof_g_C_g_precomp,      pvk.vk_alphaC_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_C_2 = snark_pp<ppT>::miller_loop(proof_g_C_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_C = snark_pp<ppT>::final_exponentiation(kc_C_1 * kc_C_2.unitary_inverse());
    if (kc_C != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for C is valid");

    ffec::G1<snark_pp<ppT>> Aacc = pvk.A0 + proof.g_Aau.g + proof.g_A.g;

    ffec::enter_block("Check QAP divisibility");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aacc_precomp = snark_pp<ppT>::precompute_G1(Aacc);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_H_precomp = snark_pp<ppT>::precompute_G1(proof.g_H);
    ffec::Fqk<snark_pp<ppT>> QAP_1  = snark_pp<ppT>::miller_loop(proof_g_Aacc_precomp,  proof_g_B_g_precomp);
    ffec::Fqk<snark_pp<ppT>> QAP_23  = snark_pp<ppT>::double_miller_loop(proof_g_H_precomp, pvk.vk_rC_Z_g2_precomp,
                                                                   proof_g_C_g_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> QAP = snark_pp<ppT>::final_exponentiation(QAP_1 * QAP_23.unitary_inverse());
    if (QAP != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check QAP divisibility");

    ffec::enter_block("Check same coefficients were used");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_K_precomp = snark_pp<ppT>::precompute_G1(proof.g_K);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aacc_C_precomp = snark_pp<ppT>::precompute_G1(Aacc + proof.g_C.g);
    ffec::Fqk<snark_pp<ppT>> K_1 = snark_pp<ppT>::miller_loop(proof_g_K_precomp, pvk.vk_gamma_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> K_23 = snark_pp<ppT>::double_miller_loop(proof_g_Aacc_C_precomp, pvk.vk_gamma_beta_g2_precomp,
                                                                pvk.vk_gamma_beta_g1_precomp, proof_g_B_g_precomp);
    ffec::GT<snark_pp<ppT>> K = snark_pp<ppT>::final_exponentiation(K_1 * K_23.unitary_inverse());
    if (K != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check same coefficients were used");
    ffec::leave_block("Online pairing computations");
    ffec::leave_block("Call to r1cs_ppzkadsnark_online_verifier");

    return result;
}

template<typename ppT>
bool r1cs_ppzkadsnark_verifier(const r1cs_ppzkadsnark_verification_key<ppT> &vk,
                               const r1cs_ppzkadsnark_proof<ppT> &proof,
                               const r1cs_ppzkadsnark_sec_auth_key<ppT> &sak,
                               const std::vector<labelT> &labels)
{
    ffec::enter_block("Call to r1cs_ppzkadsnark_verifier");
    r1cs_ppzkadsnark_processed_verification_key<ppT> pvk = r1cs_ppzkadsnark_verifier_process_vk<ppT>(vk);
    bool result = r1cs_ppzkadsnark_online_verifier<ppT>(pvk, proof, sak, labels);
    ffec::leave_block("Call to r1cs_ppzkadsnark_verifier");
    return result;
}


// public
template<typename ppT>
bool r1cs_ppzkadsnark_online_verifier(const r1cs_ppzkadsnark_processed_verification_key<ppT> &pvk,
                                      const std::vector<r1cs_ppzkadsnark_auth_data<ppT>>  &auth_data,
                                      const r1cs_ppzkadsnark_proof<ppT> &proof,
                                      const r1cs_ppzkadsnark_pub_auth_key<ppT> & pak,
                                      const std::vector<labelT> &labels)
{
    bool result = true;
    ffec::enter_block("Call to r1cs_ppzkadsnark_online_verifier");

    ffec::enter_block("Check if the proof is well-formed");
    if (!proof.is_well_formed())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("At least one of the proof elements does not lie on the curve.\n");
        }
        result = false;
    }
    ffec::leave_block("Check if the proof is well-formed");

    ffec::enter_block("Checking auth-specific elements");
    assert (labels.size()==auth_data.size());

    ffec::enter_block("Checking A1");

    ffec::enter_block("Checking signatures");
    std::vector<ffec::G2<snark_pp<ppT>>> Lambdas;
    std::vector<r1cs_ppzkadsnark_sigT<ppT>> sigs;
    Lambdas.reserve(labels.size());
    sigs.reserve(labels.size());
    for (size_t i = 0; i < labels.size();i++) {
        Lambdas.push(auth_data[i].Lambda);
        sigs.push(auth_data[i].sigma);
    }
    bool result_auth = sigBatchVerif<ppT>(pak.vkp,labels,Lambdas,sigs);
    if (! result_auth)
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Auth sig check failed.\n");
        }
    }

    ffec::leave_block("Checking signatures");

    ffec::enter_block("Checking pairings");
    // To Do: Decide whether to move pak and lambda preprocessing to offline
    std::vector<ffec::G2_precomp<snark_pp<ppT>>> g_Lambdas_precomp;
    g_Lambdas_precomp.reserve(auth_data.size());
    for(size_t i=0; i < auth_data.size(); i++)
        g_Lambdas_precomp.push(snark_pp<ppT>::precompute_G2(auth_data[i].Lambda));
    ffec::G2_precomp<snark_pp<ppT>> g_minusi_precomp = snark_pp<ppT>::precompute_G2(pak.minusI2);

    ffec::enter_block("Computation");
    ffec::Fqk<snark_pp<ppT>> accum;
    if(auth_data.size() % 2 == 1) {
        accum = snark_pp<ppT>::miller_loop(pvk.proof_g_vki_precomp[0]  , g_Lambdas_precomp[0]);
    }
    else {
        accum = ffec::Fqk<snark_pp<ppT>>::one();
    }
    for(size_t i = auth_data.size() % 2; i < labels.size();i=i+2) {
        accum = accum * snark_pp<ppT>::double_miller_loop(pvk.proof_g_vki_precomp[i]  , g_Lambdas_precomp[i],
                                                          pvk.proof_g_vki_precomp[i+1], g_Lambdas_precomp[i+1]);
    }
    ffec::G1_precomp<snark_pp<ppT>> proof_g_muA_precomp = snark_pp<ppT>::precompute_G1(proof.muA);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aau_precomp = snark_pp<ppT>::precompute_G1(proof.g_Aau.g);
    ffec::Fqk<snark_pp<ppT>> accum2 = snark_pp<ppT>::double_miller_loop(proof_g_muA_precomp, pvk.pp_G2_one_precomp,
                                                                  proof_g_Aau_precomp, g_minusi_precomp);
    ffec::GT<snark_pp<ppT>> authPair = snark_pp<ppT>::final_exponentiation(accum * accum2.unitary_inverse());
    if (authPair != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Auth pairing check failed.\n");
        }
        result_auth = false;
    }
    ffec::leave_block("Computation");
    ffec::leave_block("Checking pairings");


    if (!(result_auth)) {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Authentication check failed.\n");
        }
    }

    ffec::leave_block("Checking A1");

    ffec::enter_block("Checking A2");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aau_g_precomp = snark_pp<ppT>::precompute_G1(proof.g_Aau.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aau_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_Aau.h);
    ffec::Fqk<snark_pp<ppT>> kc_Aau_1 = snark_pp<ppT>::miller_loop(proof_g_Aau_g_precomp, pvk.vk_alphaA_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_Aau_2 = snark_pp<ppT>::miller_loop(proof_g_Aau_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_Aau = snark_pp<ppT>::final_exponentiation(kc_Aau_1 * kc_Aau_2.unitary_inverse());
    if (kc_Aau != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for Aau query incorrect.\n");
        }
        result_auth = false;
    }
    ffec::leave_block("Checking A2");

    ffec::leave_block("Checking auth-specific elements");

    result &= result_auth;

    ffec::enter_block("Online pairing computations");
    ffec::enter_block("Check knowledge commitment for A is valid");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_A_g_precomp      = snark_pp<ppT>::precompute_G1(proof.g_A.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_A_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_A.h);
    ffec::Fqk<snark_pp<ppT>> kc_A_1 = snark_pp<ppT>::miller_loop(proof_g_A_g_precomp,      pvk.vk_alphaA_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_A_2 = snark_pp<ppT>::miller_loop(proof_g_A_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_A = snark_pp<ppT>::final_exponentiation(kc_A_1 * kc_A_2.unitary_inverse());
    if (kc_A != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for A query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for A is valid");

    ffec::enter_block("Check knowledge commitment for B is valid");
    ffec::G2_precomp<snark_pp<ppT>> proof_g_B_g_precomp      = snark_pp<ppT>::precompute_G2(proof.g_B.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_B_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_B.h);
    ffec::Fqk<snark_pp<ppT>> kc_B_1 = snark_pp<ppT>::miller_loop(pvk.vk_alphaB_g1_precomp, proof_g_B_g_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_B_2 = snark_pp<ppT>::miller_loop(proof_g_B_h_precomp,    pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_B = snark_pp<ppT>::final_exponentiation(kc_B_1 * kc_B_2.unitary_inverse());
    if (kc_B != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for B query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for B is valid");

    ffec::enter_block("Check knowledge commitment for C is valid");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_C_g_precomp      = snark_pp<ppT>::precompute_G1(proof.g_C.g);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_C_h_precomp = snark_pp<ppT>::precompute_G1(proof.g_C.h);
    ffec::Fqk<snark_pp<ppT>> kc_C_1 = snark_pp<ppT>::miller_loop(proof_g_C_g_precomp,      pvk.vk_alphaC_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> kc_C_2 = snark_pp<ppT>::miller_loop(proof_g_C_h_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> kc_C = snark_pp<ppT>::final_exponentiation(kc_C_1 * kc_C_2.unitary_inverse());
    if (kc_C != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Knowledge commitment for C query incorrect.\n");
        }
        result = false;
    }
    ffec::leave_block("Check knowledge commitment for C is valid");

    ffec::G1<snark_pp<ppT>> Aacc = pvk.A0 + proof.g_Aau.g + proof.g_A.g;

    ffec::enter_block("Check QAP divisibility");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aacc_precomp = snark_pp<ppT>::precompute_G1(Aacc);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_H_precomp = snark_pp<ppT>::precompute_G1(proof.g_H);
    ffec::Fqk<snark_pp<ppT>> QAP_1  = snark_pp<ppT>::miller_loop(proof_g_Aacc_precomp,  proof_g_B_g_precomp);
    ffec::Fqk<snark_pp<ppT>> QAP_23  = snark_pp<ppT>::double_miller_loop(proof_g_H_precomp, pvk.vk_rC_Z_g2_precomp,
                                                                   proof_g_C_g_precomp, pvk.pp_G2_one_precomp);
    ffec::GT<snark_pp<ppT>> QAP = snark_pp<ppT>::final_exponentiation(QAP_1 * QAP_23.unitary_inverse());
    if (QAP != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("QAP divisibility check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check QAP divisibility");

    ffec::enter_block("Check same coefficients were used");
    ffec::G1_precomp<snark_pp<ppT>> proof_g_K_precomp = snark_pp<ppT>::precompute_G1(proof.g_K);
    ffec::G1_precomp<snark_pp<ppT>> proof_g_Aacc_C_precomp = snark_pp<ppT>::precompute_G1(Aacc + proof.g_C.g);
    ffec::Fqk<snark_pp<ppT>> K_1 = snark_pp<ppT>::miller_loop(proof_g_K_precomp, pvk.vk_gamma_g2_precomp);
    ffec::Fqk<snark_pp<ppT>> K_23 = snark_pp<ppT>::double_miller_loop(proof_g_Aacc_C_precomp, pvk.vk_gamma_beta_g2_precomp,
                                                                pvk.vk_gamma_beta_g1_precomp, proof_g_B_g_precomp);
    ffec::GT<snark_pp<ppT>> K = snark_pp<ppT>::final_exponentiation(K_1 * K_23.unitary_inverse());
    if (K != ffec::GT<snark_pp<ppT>>::one())
    {
        if (!ffec::inhibit_profiling_info)
        {
            ffec::print_indent(); print!("Same-coefficient check failed.\n");
        }
        result = false;
    }
    ffec::leave_block("Check same coefficients were used");
    ffec::leave_block("Online pairing computations");
    ffec::leave_block("Call to r1cs_ppzkadsnark_online_verifier");

    return result;
}

// public
template<typename ppT>
bool r1cs_ppzkadsnark_verifier(const r1cs_ppzkadsnark_verification_key<ppT> &vk,
                               const std::vector<r1cs_ppzkadsnark_auth_data<ppT>> &auth_data,
                               const r1cs_ppzkadsnark_proof<ppT> &proof,
                               const r1cs_ppzkadsnark_pub_auth_key<ppT> &pak,
                               const std::vector<labelT> &labels)
{
    assert!(labels.size() == auth_data.size());
    ffec::enter_block("Call to r1cs_ppzkadsnark_verifier");
    r1cs_ppzkadsnark_processed_verification_key<ppT> pvk = r1cs_ppzkadsnark_verifier_process_vk<ppT>(vk);
    bool result = r1cs_ppzkadsnark_online_verifier<ppT>(pvk, auth_data, proof, pak,labels);
    ffec::leave_block("Call to r1cs_ppzkadsnark_verifier");
    return result;
}


//#endif // R1CS_PPZKADSNARK_TCC_
