/** @file
 *****************************************************************************

 Declaration of interfaces for a zkSNARK for RAM.

 This includes:
 - the class for a proving key;
 - the class for a verification key;
 - the class for a key pair (proving key & verification key);
 - the class for a proof;
 - the generator algorithm;
 - the prover algorithm;
 - the verifier algorithm.

 The implementation follows, extends, and optimizes the approach described
 in \[BCTV14]. Thus, the zkSNARK is constructed from a ppzkPCD for R1CS.


 Acronyms:

 "R1CS" = "Rank-1 Constraint Systems"
 "RAM" = "Random-Access Machines"
 "zkSNARK" = "Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
 "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

 References:

 \[BCTV14]:
 "Scalable Zero Knowledge via Cycles of Elliptic Curves",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 CRYPTO 2014,
 <http://eprint.iacr.org/2014/595>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef RAM_ZKSNARK_HPP_
#define RAM_ZKSNARK_HPP_

use  <memory>

use  <libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/r1cs_sp_ppzkpcd.hpp>
use  <libsnark/zk_proof_systems/zksnark/ram_zksnark/ram_compliance_predicate.hpp>
use  <libsnark/zk_proof_systems/zksnark/ram_zksnark/ram_zksnark_params.hpp>

namespace libsnark {

/******************************** Proving key ********************************/

template<typename ram_zksnark_ppT>
class ram_zksnark_proving_key;

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_proving_key<ram_zksnark_ppT> &pk);

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_proving_key<ram_zksnark_ppT> &pk);

/**
 * A proving key for the RAM zkSNARK.
 */
template<typename ram_zksnark_ppT>
class ram_zksnark_proving_key {
public:
    ram_zksnark_architecture_params<ram_zksnark_ppT> ap;
    r1cs_sp_ppzkpcd_proving_key<ram_zksnark_PCD_pp<ram_zksnark_ppT> > pcd_pk;

    ram_zksnark_proving_key() {}
    ram_zksnark_proving_key(const ram_zksnark_proving_key<ram_zksnark_ppT> &other) = default;
    ram_zksnark_proving_key(ram_zksnark_proving_key<ram_zksnark_ppT> &&other) = default;
    ram_zksnark_proving_key(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap,
                            r1cs_sp_ppzkpcd_proving_key<ram_zksnark_PCD_pp<ram_zksnark_ppT> > &&pcd_pk) :
        ap(ap),
        pcd_pk(std::move(pcd_pk))
    {};

    ram_zksnark_proving_key<ram_zksnark_ppT>& operator=(const ram_zksnark_proving_key<ram_zksnark_ppT> &other) = default;

    bool operator==(const ram_zksnark_proving_key<ram_zksnark_ppT> &other) const;
    friend std::ostream& operator<< <ram_zksnark_ppT>(std::ostream &out, const ram_zksnark_proving_key<ram_zksnark_ppT> &pk);
    friend std::istream& operator>> <ram_zksnark_ppT>(std::istream &in, ram_zksnark_proving_key<ram_zksnark_ppT> &pk);
};


/******************************* Verification key ****************************/

template<typename ram_zksnark_ppT>
class ram_zksnark_verification_key;

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_verification_key<ram_zksnark_ppT> &vk);

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_verification_key<ram_zksnark_ppT> &vk);

/**
 * A verification key for the RAM zkSNARK.
 */
template<typename ram_zksnark_ppT>
class ram_zksnark_verification_key {
public:
    ram_zksnark_architecture_params<ram_zksnark_ppT> ap;
    r1cs_sp_ppzkpcd_verification_key<ram_zksnark_PCD_pp<ram_zksnark_ppT> > pcd_vk;

    ram_zksnark_verification_key() = default;
    ram_zksnark_verification_key(const ram_zksnark_verification_key<ram_zksnark_ppT> &other) = default;
    ram_zksnark_verification_key(ram_zksnark_verification_key<ram_zksnark_ppT> &&other) = default;
    ram_zksnark_verification_key(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap,
                                 r1cs_sp_ppzkpcd_verification_key<ram_zksnark_PCD_pp<ram_zksnark_ppT> > &&pcd_vk) :
        ap(ap),
        pcd_vk(std::move(pcd_vk))
    {};

    ram_zksnark_verification_key<ram_zksnark_ppT>& operator=(const ram_zksnark_verification_key<ram_zksnark_ppT> &other) = default;

    bool operator==(const ram_zksnark_verification_key<ram_zksnark_ppT> &other) const;
    friend std::ostream& operator<< <ram_zksnark_ppT>(std::ostream &out, const ram_zksnark_verification_key<ram_zksnark_ppT> &vk);
    friend std::istream& operator>> <ram_zksnark_ppT>(std::istream &in, ram_zksnark_verification_key<ram_zksnark_ppT> &vk);

    static ram_zksnark_verification_key<ram_zksnark_ppT> dummy_verification_key(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the RAM zkSNARK, which consists of a proving key and a verification key.
 */
template<typename ram_zksnark_ppT>
struct ram_zksnark_keypair {
public:
    ram_zksnark_proving_key<ram_zksnark_ppT> pk;
    ram_zksnark_verification_key<ram_zksnark_ppT> vk;

    ram_zksnark_keypair() {};
    ram_zksnark_keypair(ram_zksnark_keypair<ram_zksnark_ppT> &&other) = default;
    ram_zksnark_keypair(ram_zksnark_proving_key<ram_zksnark_ppT> &&pk,
                        ram_zksnark_verification_key<ram_zksnark_ppT> &&vk) :
        pk(std::move(pk)),
        vk(std::move(vk))
    {};
};


/*********************************** Proof ***********************************/

template<typename ram_zksnark_ppT>
class ram_zksnark_proof;

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_proof<ram_zksnark_ppT> &proof);

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_proof<ram_zksnark_ppT> &proof);

/**
 * A proof for the RAM zkSNARK.
 */
template<typename ram_zksnark_ppT>
class ram_zksnark_proof {
public:
    r1cs_sp_ppzkpcd_proof<ram_zksnark_PCD_pp<ram_zksnark_ppT> > PCD_proof;

    ram_zksnark_proof() = default;
    ram_zksnark_proof(r1cs_sp_ppzkpcd_proof<ram_zksnark_PCD_pp<ram_zksnark_ppT> > &&PCD_proof) :
        PCD_proof(std::move(PCD_proof)) {};
    ram_zksnark_proof(const r1cs_sp_ppzkpcd_proof<ram_zksnark_PCD_pp<ram_zksnark_ppT> > &PCD_proof) :
        PCD_proof(PCD_proof) {};

    size_t size_in_bits() const
    {
        return PCD_proof.size_in_bits();
    }

    bool operator==(const ram_zksnark_proof<ram_zksnark_ppT> &other) const;
    friend std::ostream& operator<< <ram_zksnark_ppT>(std::ostream &out, const ram_zksnark_proof<ram_zksnark_ppT> &proof);
    friend std::istream& operator>> <ram_zksnark_ppT>(std::istream &in, ram_zksnark_proof<ram_zksnark_ppT> &proof);
};


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the RAM zkSNARK.
 *
 * Given a choice of architecture parameters, this algorithm produces proving
 * and verification keys for all computations that respect this choice.
 */
template<typename ram_zksnark_ppT>
ram_zksnark_keypair<ram_zksnark_ppT> ram_zksnark_generator(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap);

/**
 * A prover algorithm for the RAM zkSNARK.
 *
 * Given a proving key, primary input X, time bound T, and auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that X(Y) accepts within T steps''.
 */
template<typename ram_zksnark_ppT>
ram_zksnark_proof<ram_zksnark_ppT> ram_zksnark_prover(const ram_zksnark_proving_key<ram_zksnark_ppT> &pk,
                                                      const ram_zksnark_primary_input<ram_zksnark_ppT> &primary_input,
                                                      const size_t time_bound,
                                                      const ram_zksnark_auxiliary_input<ram_zksnark_ppT> &auxiliary_input);

/**
 * A verifier algorithm for the RAM zkSNARK.
 *
 * This algorithm is universal in the sense that the verification key
 * supports proof verification for *any* choice of primary input and time bound.
 */
template<typename ram_zksnark_ppT>
bool ram_zksnark_verifier(const ram_zksnark_verification_key<ram_zksnark_ppT> &vk,
                          const ram_zksnark_primary_input<ram_zksnark_ppT> &primary_input,
                          const size_t time_bound,
                          const ram_zksnark_proof<ram_zksnark_ppT> &proof);

} // libsnark

use  <libsnark/zk_proof_systems/zksnark/ram_zksnark/ram_zksnark.tcc>

#endif // RAM_ZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a zkSNARK for RAM.

 See ram_zksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef RAM_ZKSNARK_TCC_
#define RAM_ZKSNARK_TCC_

use  <libff/common/profiling.hpp>

namespace libsnark {

template<typename ram_zksnark_ppT>
bool ram_zksnark_proving_key<ram_zksnark_ppT>::operator==(const ram_zksnark_proving_key<ram_zksnark_ppT> &other) const
{
    return (this->ap == other.ap &&
            this->pcd_pk == other.pcd_pk);
}

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_proving_key<ram_zksnark_ppT> &pk)
{
    out << pk.ap;
    out << pk.pcd_pk;

    return out;
}

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_proving_key<ram_zksnark_ppT> &pk)
{
    in >> pk.ap;
    in >> pk.pcd_pk;

    return in;
}

template<typename ram_zksnark_ppT>
bool ram_zksnark_verification_key<ram_zksnark_ppT>::operator==(const ram_zksnark_verification_key<ram_zksnark_ppT> &other) const
{
    return (this->ap == other.ap &&
            this->pcd_vk == other.pcd_vk);
}

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_verification_key<ram_zksnark_ppT> &vk)
{
    out << vk.ap;
    out << vk.pcd_vk;

    return out;
}

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_verification_key<ram_zksnark_ppT> &vk)
{
    in >> vk.ap;
    in >> vk.pcd_vk;

    return in;
}

template<typename ram_zksnark_ppT>
bool ram_zksnark_proof<ram_zksnark_ppT>::operator==(const ram_zksnark_proof<ram_zksnark_ppT> &other) const
{
    return (this->PCD_proof == other.PCD_proof);
}

template<typename ram_zksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_zksnark_proof<ram_zksnark_ppT> &proof)
{
    out << proof.PCD_proof;
    return out;
}

template<typename ram_zksnark_ppT>
std::istream& operator>>(std::istream &in, ram_zksnark_proof<ram_zksnark_ppT> &proof)
{
    in >> proof.PCD_proof;
    return in;
}

template<typename ram_zksnark_ppT>
ram_zksnark_verification_key<ram_zksnark_ppT> ram_zksnark_verification_key<ram_zksnark_ppT>::dummy_verification_key(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap)
{
    type ram_zksnark_PCD_pp<ram_zksnark_ppT> pcdT;

    return ram_zksnark_verification_key<ram_zksnark_ppT>(ap, r1cs_sp_ppzkpcd_verification_key<pcdT>::dummy_verification_key());
}

template<typename ram_zksnark_ppT>
ram_zksnark_keypair<ram_zksnark_ppT> ram_zksnark_generator(const ram_zksnark_architecture_params<ram_zksnark_ppT> &ap)
{
    type ram_zksnark_machine_pp<ram_zksnark_ppT> ramT;
    type ram_zksnark_PCD_pp<ram_zksnark_ppT> pcdT;
    libff::enter_block("Call to ram_zksnark_generator");

    libff::enter_block("Generate compliance predicate for RAM");
    ram_compliance_predicate_handler<ramT> cp_handler(ap);
    cp_handler.generate_r1cs_constraints();
    r1cs_sp_ppzkpcd_compliance_predicate<pcdT> ram_compliance_predicate = cp_handler.get_compliance_predicate();
    libff::leave_block("Generate compliance predicate for RAM");

    libff::enter_block("Generate PCD key pair");
    r1cs_sp_ppzkpcd_keypair<pcdT> kp = r1cs_sp_ppzkpcd_generator<pcdT>(ram_compliance_predicate);
    libff::leave_block("Generate PCD key pair");

    libff::leave_block("Call to ram_zksnark_generator");

    ram_zksnark_proving_key<ram_zksnark_ppT> pk = ram_zksnark_proving_key<ram_zksnark_ppT>(ap, std::move(kp.pk));
    ram_zksnark_verification_key<ram_zksnark_ppT> vk = ram_zksnark_verification_key<ram_zksnark_ppT>(ap, std::move(kp.vk));

    return ram_zksnark_keypair<ram_zksnark_ppT>(std::move(pk), std::move(vk));
}

template<typename ram_zksnark_ppT>
ram_zksnark_proof<ram_zksnark_ppT> ram_zksnark_prover(const ram_zksnark_proving_key<ram_zksnark_ppT> &pk,
                                                      const ram_zksnark_primary_input<ram_zksnark_ppT> &primary_input,
                                                      const size_t time_bound,
                                                      const ram_zksnark_auxiliary_input<ram_zksnark_ppT> &auxiliary_input)
{
    type ram_zksnark_machine_pp<ram_zksnark_ppT> ramT;
    type ram_zksnark_PCD_pp<ram_zksnark_ppT> pcdT;
    type libff::Fr<typename pcdT::curve_A_pp> FieldT; // XXX

    assert(libff::log2(time_bound) <= ramT::timestamp_length);

    libff::enter_block("Call to ram_zksnark_prover");
    libff::enter_block("Generate compliance predicate for RAM");
    ram_compliance_predicate_handler<ramT> cp_handler(pk.ap);
    libff::leave_block("Generate compliance predicate for RAM");

    libff::enter_block("Initialize the RAM computation");
    r1cs_sp_ppzkpcd_proof<pcdT> cur_proof; // start out with an empty proof

    /* initialize memory with the correct values */
    const size_t num_addresses = 1ul << pk.ap.address_size();
    const size_t value_size = pk.ap.value_size();

    delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > mem(num_addresses, value_size, primary_input.as_memory_contents());
    std::shared_ptr<r1cs_pcd_message<FieldT> > msg = ram_compliance_predicate_handler<ramT>::get_base_case_message(pk.ap, primary_input);

    typename ram_input_tape<ramT>::const_iterator aux_it = auxiliary_input.begin();
    libff::leave_block("Initialize the RAM computation");

    libff::enter_block("Execute and prove the computation");
    bool want_halt = false;
    for (size_t step = 1; step <= time_bound; ++step)
    {
        libff::enter_block(FMT("", "Prove step %zu out of %zu", step, time_bound));

        libff::enter_block("Execute witness map");

        std::shared_ptr<r1cs_pcd_local_data<FieldT> > local_data;
        local_data.reset(new ram_pcd_local_data<ramT>(want_halt, mem, aux_it, auxiliary_input.end()));

        cp_handler.generate_r1cs_witness({ msg }, local_data);

        const r1cs_pcd_compliance_predicate_primary_input<FieldT> cp_primary_input(cp_handler.get_outgoing_message());
        const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> cp_auxiliary_input({ msg }, local_data, cp_handler.get_witness());

#ifdef DEBUG
        printf("Current state:\n");
        msg->print();
#endif

        msg = cp_handler.get_outgoing_message();

#ifdef DEBUG
        printf("Next state:\n");
        msg->print();
#endif
        libff::leave_block("Execute witness map");

        cur_proof = r1cs_sp_ppzkpcd_prover<pcdT>(pk.pcd_pk, cp_primary_input, cp_auxiliary_input, { cur_proof });
        libff::leave_block(FMT("", "Prove step %zu out of %zu", step, time_bound));
    }
    libff::leave_block("Execute and prove the computation");

    libff::enter_block("Finalize the computation");
    want_halt = true;

    libff::enter_block("Execute witness map");

    std::shared_ptr<r1cs_pcd_local_data<FieldT> > local_data;
    local_data.reset(new ram_pcd_local_data<ramT>(want_halt, mem, aux_it, auxiliary_input.end()));

    cp_handler.generate_r1cs_witness({ msg }, local_data);

    const r1cs_pcd_compliance_predicate_primary_input<FieldT> cp_primary_input(cp_handler.get_outgoing_message());
    const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> cp_auxiliary_input({ msg }, local_data, cp_handler.get_witness());
    libff::leave_block("Execute witness map");

    cur_proof = r1cs_sp_ppzkpcd_prover<pcdT>(pk.pcd_pk, cp_primary_input, cp_auxiliary_input, { cur_proof });
    libff::leave_block("Finalize the computation");

    libff::leave_block("Call to ram_zksnark_prover");

    return cur_proof;
}

template<typename ram_zksnark_ppT>
bool ram_zksnark_verifier(const ram_zksnark_verification_key<ram_zksnark_ppT> &vk,
                          const ram_zksnark_primary_input<ram_zksnark_ppT> &primary_input,
                          const size_t time_bound,
                          const ram_zksnark_proof<ram_zksnark_ppT> &proof)
{
    type ram_zksnark_machine_pp<ram_zksnark_ppT> ramT;
    type ram_zksnark_PCD_pp<ram_zksnark_ppT> pcdT;
    type libff::Fr<typename pcdT::curve_A_pp> FieldT; // XXX

    libff::enter_block("Call to ram_zksnark_verifier");
    const r1cs_pcd_compliance_predicate_primary_input<FieldT> cp_primary_input(ram_compliance_predicate_handler<ramT>::get_final_case_msg(vk.ap, primary_input, time_bound));
    bool ans = r1cs_sp_ppzkpcd_verifier<pcdT>(vk.pcd_vk, cp_primary_input, proof.PCD_proof);
    libff::leave_block("Call to ram_zksnark_verifier");

    return ans;
}

} // libsnark

#endif // RAM_ZKSNARK_TCC_
