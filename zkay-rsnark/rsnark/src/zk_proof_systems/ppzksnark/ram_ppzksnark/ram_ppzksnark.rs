/** @file
 *****************************************************************************

 Declaration of interfaces for a ppzkSNARK for RAM.

 This includes:
 - the class for a proving key;
 - the class for a verification key;
 - the class for a key pair (proving key & verification key);
 - the class for a proof;
 - the generator algorithm;
 - the prover algorithm;
 - the verifier algorithm.

 The implementation follows, extends, and optimizes the approach described
 in \[BCTV14] (itself building on \[BCGTV13]). In particular, the ppzkSNARK
 for RAM is constructed from a ppzkSNARK for R1CS.


 Acronyms:

 "R1CS" = "Rank-1 Constraint Systems"
 "RAM" = "Random-Access Machines"
 "ppzkSNARK" = "Pre-Processing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"

 References:

 \[BCGTV13]:
 "SNARKs for C: verifying program executions succinctly and in zero knowledge",
 Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
 CRYPTO 2014,
 <http://eprint.iacr.org/2013/507>

 \[BCTV14]:
 "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 USENIX Security 2014,
 <http://eprint.iacr.org/2013/879>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_PPZKSNARK_HPP_
// #define RAM_PPZKSNARK_HPP_

use  <memory>

use libsnark/reductions/ram_to_r1cs/ram_to_r1cs;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
use libsnark/zk_proof_systems/ppzksnark/ram_ppzksnark/ram_ppzksnark_params;



/******************************** Proving key ********************************/

template<typename ram_ppzksnark_ppT>
class ram_ppzksnark_proving_key;

template<typename ram_ppzksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk);

template<typename ram_ppzksnark_ppT>
std::istream& operator>>(std::istream &in, ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk);

/**
 * A proving key for the RAM ppzkSNARK.
 */
template<typename ram_ppzksnark_ppT>
class ram_ppzksnark_proving_key {
public:
    type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> snark_ppT;

    r1cs_ppzksnark_proving_key<snark_ppT> r1cs_pk;
    ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> ap;
    size_t primary_input_size_bound;
    size_t time_bound;

    ram_ppzksnark_proving_key() {}
    ram_ppzksnark_proving_key(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &other) = default;
    ram_ppzksnark_proving_key(ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &&other) = default;
    ram_ppzksnark_proving_key(r1cs_ppzksnark_proving_key<snark_ppT> &&r1cs_pk,
                              const ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> &ap,
                              const size_t primary_input_size_bound,
                              const size_t time_bound) :
        r1cs_pk((r1cs_pk)),
        ap(ap),
        primary_input_size_bound(primary_input_size_bound),
        time_bound(time_bound)
    {}

    ram_ppzksnark_proving_key<ram_ppzksnark_ppT>& operator=(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &other) = default;

    bool operator==(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &other) const;
    friend std::ostream& operator<< <ram_ppzksnark_ppT>(std::ostream &out, const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk);
    friend std::istream& operator>> <ram_ppzksnark_ppT>(std::istream &in, ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk);
};


/******************************* Verification key ****************************/

template<typename ram_ppzksnark_ppT>
class ram_ppzksnark_verification_key;

template<typename ram_ppzksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk);

template<typename ram_ppzksnark_ppT>
std::istream& operator>>(std::istream &in, ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk);

/**
 * A verification key for the RAM ppzkSNARK.
 */
template<typename ram_ppzksnark_ppT>
class ram_ppzksnark_verification_key {
public:
    type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> snark_ppT;

    r1cs_ppzksnark_verification_key<snark_ppT> r1cs_vk;
    ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> ap;
    size_t primary_input_size_bound;
    size_t time_bound;

    std::set<size_t> bound_primary_input_locations;

    ram_ppzksnark_verification_key() = default;
    ram_ppzksnark_verification_key(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &other) = default;
    ram_ppzksnark_verification_key(ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &&other) = default;
    ram_ppzksnark_verification_key(const r1cs_ppzksnark_verification_key<snark_ppT> &r1cs_vk,
                                   const ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> &ap,
                                   const size_t primary_input_size_bound,
                                   const size_t time_bound) :
        r1cs_vk(r1cs_vk),
        ap(ap),
        primary_input_size_bound(primary_input_size_bound),
        time_bound(time_bound)
    {}

    ram_ppzksnark_verification_key<ram_ppzksnark_ppT>& operator=(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &other) = default;

    ram_ppzksnark_verification_key<ram_ppzksnark_ppT> bind_primary_input(const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input) const;

    bool operator==(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &other) const;
    friend std::ostream& operator<< <ram_ppzksnark_ppT>(std::ostream &out, const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk);
    friend std::istream& operator>> <ram_ppzksnark_ppT>(std::istream &in, ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk);
};


/********************************** Key pair *********************************/

/**
 * A key pair for the RAM ppzkSNARK, which consists of a proving key and a verification key.
 */
template<typename ram_ppzksnark_ppT>
struct ram_ppzksnark_keypair {
public:
    ram_ppzksnark_proving_key<ram_ppzksnark_ppT> pk;
    ram_ppzksnark_verification_key<ram_ppzksnark_ppT> vk;

    ram_ppzksnark_keypair() = default;
    ram_ppzksnark_keypair(ram_ppzksnark_keypair<ram_ppzksnark_ppT> &&other) = default;
    ram_ppzksnark_keypair(const ram_ppzksnark_keypair<ram_ppzksnark_ppT> &other) = default;
    ram_ppzksnark_keypair(ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &&pk,
                          ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &&vk) :
        pk((pk)),
        vk((vk))
    {}
};


/*********************************** Proof ***********************************/

/**
 * A proof for the RAM ppzkSNARK.
 */
template<typename ram_ppzksnark_ppT>
using ram_ppzksnark_proof = r1cs_ppzksnark_proof<ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> >;


/***************************** Main algorithms *******************************/

/**
 * A generator algorithm for the RAM ppzkSNARK.
 *
 * Given a choice of architecture parameters and computation bounds, this algorithm
 * produces proving and verification keys for all computations that respect these choices.
 */
template<typename ram_ppzksnark_ppT>
ram_ppzksnark_keypair<ram_ppzksnark_ppT> ram_ppzksnark_generator(const ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> &ap,
                                                                 const size_t primary_input_size_bound,
                                                                 const size_t time_bound);

/**
 * A prover algorithm for the RAM ppzkSNARK.
 *
 * Given a proving key, primary input X, and auxiliary input Y, this algorithm
 * produces a proof (of knowledge) that attests to the following statement:
 *               ``there exists Y such that X(Y) accepts''.
 *
 * Above, it has to be the case that the computation respects the bounds:
 * - the size of X is at most primary_input_size_bound, and
 * - the time to compute X(Y) is at most time_bound.
 */
template<typename ram_ppzksnark_ppT>
ram_ppzksnark_proof<ram_ppzksnark_ppT> ram_ppzksnark_prover(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk,
                                                            const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input,
                                                            const ram_ppzksnark_auxiliary_input<ram_ppzksnark_ppT> &auxiliary_input);

/**
 * A verifier algorithm for the RAM ppzkSNARK.
 *
 * This algorithm is universal in the sense that the verification key
 * supports proof verification for any choice of primary input
 * provided that the computation respects the bounds.
 */
template<typename ram_ppzksnark_ppT>
bool ram_ppzksnark_verifier(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk,
                            const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input,
                            const ram_ppzksnark_proof<ram_ppzksnark_ppT> &proof);



use libsnark/zk_proof_systems/ppzksnark/ram_ppzksnark/ram_ppzksnark;

//#endif // RAM_PPZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a ppzkSNARK for RAM.

 See ram_ppzksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_PPZKSNARK_TCC_
// #define RAM_PPZKSNARK_TCC_

use ffec::common::profiling;

use libsnark/reductions/ram_to_r1cs/ram_to_r1cs;



template<typename ram_ppzksnark_ppT>
bool ram_ppzksnark_proving_key<ram_ppzksnark_ppT>::operator==(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &other) const
{
    return (self.r1cs_pk == other.r1cs_pk &&
            self.ap == other.ap &&
            self.primary_input_size_bound == other.primary_input_size_bound &&
            self.time_bound == other.time_bound);
}

template<typename ram_ppzksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk)
{
    out << pk.r1cs_pk;
    out << pk.ap;
    out << pk.primary_input_size_bound << "\n";
    out << pk.time_bound << "\n";

    return out;
}

template<typename ram_ppzksnark_ppT>
std::istream& operator>>(std::istream &in, ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk)
{
    in >> pk.r1cs_pk;
    in >> pk.ap;
    in >> pk.primary_input_size_bound;
    ffec::consume_newline(in);
    in >> pk.time_bound;
    ffec::consume_newline(in);

    return in;
}

template<typename ram_ppzksnark_ppT>
ram_ppzksnark_verification_key<ram_ppzksnark_ppT> ram_ppzksnark_verification_key<ram_ppzksnark_ppT>::bind_primary_input(const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input) const
{
    type ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> ram_ppT;
    type ram_base_field<ram_ppT> FieldT;

    ffec::enter_block("Call to ram_ppzksnark_verification_key::bind_primary_input");
    ram_ppzksnark_verification_key<ram_ppzksnark_ppT> result(*this);

    const size_t packed_input_element_size = ram_universal_gadget<ram_ppT>::packed_input_element_size(ap);

    for it in &primary_input.get_all_trace_entries()
    {
        const size_t input_pos = it.first;
        const address_and_value av = it.second;

        assert!(input_pos < primary_input_size_bound);
        assert!(result.bound_primary_input_locations.find(input_pos) == result.bound_primary_input_locations.end());

        const std::vector<FieldT> packed_input_element = ram_to_r1cs<ram_ppT>::pack_primary_input_address_and_value(ap, av);
        result.r1cs_vk.encoded_IC_query = result.r1cs_vk.encoded_IC_query.accumulate_chunk<FieldT>(packed_input_element.begin(), packed_input_element.end(), packed_input_element_size * (primary_input_size_bound - 1 - input_pos));

        result.bound_primary_input_locations.insert(input_pos);
    }

    ffec::leave_block("Call to ram_ppzksnark_verification_key::bind_primary_input");
    return result;
}

template<typename ram_ppzksnark_ppT>
bool ram_ppzksnark_verification_key<ram_ppzksnark_ppT>::operator==(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &other) const
{
    return (self.r1cs_vk == other.r1cs_vk &&
            self.ap == other.ap &&
            self.primary_input_size_bound == other.primary_input_size_bound &&
            self.time_bound == other.time_bound);
}

template<typename ram_ppzksnark_ppT>
std::ostream& operator<<(std::ostream &out, const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk)
{
    out << vk.r1cs_vk;
    out << vk.ap;
    out << vk.primary_input_size_bound << "\n";
    out << vk.time_bound << "\n";

    return out;
}

template<typename ram_ppzksnark_ppT>
std::istream& operator>>(std::istream &in, ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk)
{
    in >> vk.r1cs_vk;
    in >> vk.ap;
    in >> vk.primary_input_size_bound;
    ffec::consume_newline(in);
    in >> vk.time_bound;
    ffec::consume_newline(in);

    return in;
}

template<typename ram_ppzksnark_ppT>
ram_ppzksnark_keypair<ram_ppzksnark_ppT> ram_ppzksnark_generator(const ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> &ap,
                                                                 const size_t primary_input_size_bound,
                                                                 const size_t time_bound)
{
    type ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> ram_ppT;
    type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> snark_ppT;

    ffec::enter_block("Call to ram_ppzksnark_generator");
    ram_to_r1cs<ram_ppT> universal_r1cs(ap, primary_input_size_bound, time_bound);
    universal_r1cs.instance_map();
    r1cs_ppzksnark_keypair<snark_ppT> ppzksnark_keypair = r1cs_ppzksnark_generator<snark_ppT>(universal_r1cs.get_constraint_system());
    ffec::leave_block("Call to ram_ppzksnark_generator");

    ram_ppzksnark_proving_key<ram_ppzksnark_ppT> pk = ram_ppzksnark_proving_key<ram_ppzksnark_ppT>((ppzksnark_keypair.pk), ap, primary_input_size_bound, time_bound);
    ram_ppzksnark_verification_key<ram_ppzksnark_ppT> vk = ram_ppzksnark_verification_key<ram_ppzksnark_ppT>((ppzksnark_keypair.vk), ap, primary_input_size_bound, time_bound);

    return ram_ppzksnark_keypair<ram_ppzksnark_ppT>((pk), (vk));
}

template<typename ram_ppzksnark_ppT>
ram_ppzksnark_proof<ram_ppzksnark_ppT> ram_ppzksnark_prover(const ram_ppzksnark_proving_key<ram_ppzksnark_ppT> &pk,
                                                            const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input,
                                                            const ram_ppzksnark_auxiliary_input<ram_ppzksnark_ppT> &auxiliary_input)
{
    type ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> ram_ppT;
    type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> snark_ppT;
    type ffec::Fr<snark_ppT> FieldT;

    ffec::enter_block("Call to ram_ppzksnark_prover");
    ram_to_r1cs<ram_ppT> universal_r1cs(pk.ap, pk.primary_input_size_bound, pk.time_bound);
    const r1cs_primary_input<FieldT> r1cs_primary_input = ram_to_r1cs<ram_ppT>::primary_input_map(pk.ap, pk.primary_input_size_bound, primary_input);

    const r1cs_auxiliary_input<FieldT> r1cs_auxiliary_input = universal_r1cs.auxiliary_input_map(primary_input, auxiliary_input);
#if DEBUG
    universal_r1cs.print_execution_trace();
    universal_r1cs.print_memory_trace();
//#endif
    const r1cs_ppzksnark_proof<snark_ppT> proof = r1cs_ppzksnark_prover<snark_ppT>(pk.r1cs_pk, r1cs_primary_input, r1cs_auxiliary_input);
    ffec::leave_block("Call to ram_ppzksnark_prover");

    return proof;
}

template<typename ram_ppzksnark_ppT>
bool ram_ppzksnark_verifier(const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> &vk,
                            const ram_ppzksnark_primary_input<ram_ppzksnark_ppT> &primary_input,
                            const ram_ppzksnark_proof<ram_ppzksnark_ppT> &proof)
{
    type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> snark_ppT;

    ffec::enter_block("Call to ram_ppzksnark_verifier");
    const ram_ppzksnark_verification_key<ram_ppzksnark_ppT> input_specific_vk = vk.bind_primary_input(primary_input);
    const bool ans = r1cs_ppzksnark_verifier_weak_IC<snark_ppT>(input_specific_vk.r1cs_vk, r1cs_primary_input<ffec::Fr<snark_ppT> >(), proof);
    ffec::leave_block("Call to ram_ppzksnark_verifier");

    return ans;
}



//#endif // RAM_PPZKSNARK_TCC_
