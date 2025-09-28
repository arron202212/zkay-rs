/** @file
 *****************************************************************************

 Declaration of interfaces for a compliance predicate for R1CS PCD.

 A compliance predicate specifies a local invariant to be enforced, by PCD,
 throughout a dynamic distributed computation. A compliance predicate
 receives input messages, local data, and an output message (and perhaps some
 other auxiliary information), and then either accepts or rejects.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef COMPLIANCE_PREDICATE_HPP_
#define COMPLIANCE_PREDICATE_HPP_

use  <memory>

use  <libsnark/relations/constraint_satisfaction_problems/r1cs/r1cs.hpp>

namespace libsnark {

/********************************* Message ***********************************/

/**
 * A message for R1CS PCD.
 *
 * It is a pair, consisting of
 * - a type (a positive integer), and
 * - a payload (a vector of field elements).
 */
template<typename FieldT>
class r1cs_pcd_message {
public:
    size_t type;

    r1cs_pcd_message(const size_t type);
    virtual r1cs_variable_assignment<FieldT> payload_as_r1cs_variable_assignment() const = 0;
    r1cs_variable_assignment<FieldT> as_r1cs_variable_assignment() const;

    virtual void print() const;
    virtual ~r1cs_pcd_message() = default;
};

/******************************* Local data **********************************/

/**
 * A local data for R1CS PCD.
 */
template<typename FieldT>
class r1cs_pcd_local_data {
public:
    r1cs_pcd_local_data() = default;
    virtual r1cs_variable_assignment<FieldT> as_r1cs_variable_assignment() const = 0;
    virtual ~r1cs_pcd_local_data() = default;
};

/******************************** Witness ************************************/

template<typename FieldT>
using r1cs_pcd_witness = std::vector<FieldT>;

/*************************** Compliance predicate ****************************/

template<typename FieldT>
class r1cs_pcd_compliance_predicate;

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const r1cs_pcd_compliance_predicate<FieldT> &cp);

template<typename FieldT>
std::istream& operator>>(std::istream &in, r1cs_pcd_compliance_predicate<FieldT> &cp);

/**
 * A compliance predicate for R1CS PCD.
 *
 * It is a wrapper around R1CS that also specifies how to parse a
 * variable assignment as:
 * - output message (the input)
 * - some number of input messages (part of the witness)
 * - local data (also part of the witness)
 * - auxiliary information (the remaining variables of the witness)
 *
 * A compliance predicate also has a type, allegedly the same
 * as the type of the output message.
 *
 * The input wires of R1CS appear in the following order:
 * - (1 + outgoing_message_payload_length) wires for outgoing message
 * - 1 wire for arity (allegedly, 0 <= arity <= max_arity)
 * - for i = 0, ..., max_arity-1:
 * - (1 + incoming_message_payload_lengths[i]) wires for i-th message of
 *   the input (in the array that's padded to max_arity messages)
 * - local_data_length wires for local data
 *
 * The rest witness_length wires of the R1CS constitute the witness.
 *
 * To allow for optimizations, the compliance predicate also
 * specififies a flag, called relies_on_same_type_inputs, denoting
 * whether the predicate works under the assumption that all input
 * messages have the same type. In such case a member
 * accepted_input_types lists all types accepted by the predicate
 * (accepted_input_types has no meaning if
 * relies_on_same_type_inputs=false).
 */

template<typename FieldT>
class r1cs_pcd_compliance_predicate {
public:

    size_t name;
    size_t type;

    r1cs_constraint_system<FieldT> constraint_system;

    size_t outgoing_message_payload_length;
    size_t max_arity;
    std::vector<size_t> incoming_message_payload_lengths;
    size_t local_data_length;
    size_t witness_length;

    bool relies_on_same_type_inputs;
    std::set<size_t> accepted_input_types;

    r1cs_pcd_compliance_predicate() = default;
    r1cs_pcd_compliance_predicate(r1cs_pcd_compliance_predicate<FieldT> &&other) = default;
    r1cs_pcd_compliance_predicate(const r1cs_pcd_compliance_predicate<FieldT> &other) = default;
    r1cs_pcd_compliance_predicate(const size_t name,
                                  const size_t type,
                                  const r1cs_constraint_system<FieldT> &constraint_system,
                                  const size_t outgoing_message_payload_length,
                                  const size_t max_arity,
                                  const std::vector<size_t> &incoming_message_payload_lengths,
                                  const size_t local_data_length,
                                  const size_t witness_length,
                                  const bool relies_on_same_type_inputs,
                                  const std::set<size_t> accepted_input_types = std::set<size_t>());

    r1cs_pcd_compliance_predicate<FieldT> & operator=(const r1cs_pcd_compliance_predicate<FieldT> &other) = default;

    bool is_well_formed() const;
    bool has_equal_input_and_output_lengths() const;
    bool has_equal_input_lengths() const;

    bool is_satisfied(const std::shared_ptr<r1cs_pcd_message<FieldT> > &outgoing_message,
                      const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_messages,
                      const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data,
                      const r1cs_pcd_witness<FieldT> &witness) const;

    bool operator==(const r1cs_pcd_compliance_predicate<FieldT> &other) const;
    friend std::ostream& operator<< <FieldT>(std::ostream &out, const r1cs_pcd_compliance_predicate<FieldT> &cp);
    friend std::istream& operator>> <FieldT>(std::istream &in, r1cs_pcd_compliance_predicate<FieldT> &cp);
};


} // libsnark

use  <libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/compliance_predicate.tcc>

#endif // COMPLIANCE_PREDICATE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a compliance predicate for R1CS PCD.

 See compliance_predicate.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef COMPLIANCE_PREDICATE_TCC_
#define COMPLIANCE_PREDICATE_TCC_

use  <libff/common/utils.hpp>

namespace libsnark {

template<typename FieldT>
class r1cs_pcd_compliance_predicate_primary_input;

template<typename FieldT>
class r1cs_pcd_compliance_predicate_auxiliary_input;

template<typename FieldT>
r1cs_variable_assignment<FieldT> r1cs_pcd_message<FieldT>::as_r1cs_variable_assignment() const
{
    r1cs_variable_assignment<FieldT> result = this->payload_as_r1cs_variable_assignment();
    result.insert(result.begin(), FieldT(this->type));
    return result;
}

template<typename FieldT>
r1cs_pcd_message<FieldT>::r1cs_pcd_message(const size_t type) : type(type)
{
}

template<typename FieldT>
void r1cs_pcd_message<FieldT>::print() const
{
    printf("PCD message (default print routines):\n");
    printf("  Type: %zu\n", this->type);

    printf("  Payload\n");
    const r1cs_variable_assignment<FieldT> payload = this->payload_as_r1cs_variable_assignment();
    for (auto &elt: payload)
    {
        elt.print();
    }
}

template<typename FieldT>
r1cs_pcd_compliance_predicate<FieldT>::r1cs_pcd_compliance_predicate(const size_t name,
                                                                     const size_t type,
                                                                     const r1cs_constraint_system<FieldT> &constraint_system,
                                                                     const size_t outgoing_message_payload_length,
                                                                     const size_t max_arity,
                                                                     const std::vector<size_t> &incoming_message_payload_lengths,
                                                                     const size_t local_data_length,
                                                                     const size_t witness_length,
                                                                     const bool relies_on_same_type_inputs,
                                                                     const std::set<size_t> accepted_input_types) :
    name(name),
    type(type),
    constraint_system(constraint_system),
    outgoing_message_payload_length(outgoing_message_payload_length),
    max_arity(max_arity),
    incoming_message_payload_lengths(incoming_message_payload_lengths),
    local_data_length(local_data_length),
    witness_length(witness_length),
    relies_on_same_type_inputs(relies_on_same_type_inputs),
    accepted_input_types(accepted_input_types)
{
    assert(max_arity == incoming_message_payload_lengths.size());
}

template<typename FieldT>
bool r1cs_pcd_compliance_predicate<FieldT>::is_well_formed() const
{
    const bool type_not_zero = (type != 0);
    const bool incoming_message_payload_lengths_well_specified = (incoming_message_payload_lengths.size() == max_arity);

    size_t all_message_payload_lengths = outgoing_message_payload_length;
    for (size_t i = 0; i < incoming_message_payload_lengths.size(); ++i)
    {
        all_message_payload_lengths += incoming_message_payload_lengths[i];
    }
    const size_t type_vec_length = max_arity+1;
    const size_t arity_length = 1;

    const bool correct_num_inputs = ((outgoing_message_payload_length + 1) == constraint_system.num_inputs());
    const bool correct_num_variables = ((all_message_payload_lengths + local_data_length + type_vec_length + arity_length + witness_length) == constraint_system.num_variables());

#ifdef DEBUG
    printf("outgoing_message_payload_length: %zu\n", outgoing_message_payload_length);
    printf("incoming_message_payload_lengths:");
    for (auto l : incoming_message_payload_lengths)
    {
        printf(" %zu", l);
    }
    printf("\n");
    printf("type_not_zero: %d\n", type_not_zero);
    printf("incoming_message_payload_lengths_well_specified: %d\n", incoming_message_payload_lengths_well_specified);
    printf("correct_num_inputs: %d (outgoing_message_payload_length = %zu, constraint_system.num_inputs() = %zu)\n",
           correct_num_inputs, outgoing_message_payload_length, constraint_system.num_inputs());
    printf("correct_num_variables: %d (all_message_payload_lengths = %zu, local_data_length = %zu, type_vec_length = %zu, arity_length = %zu, witness_length = %zu, constraint_system.num_variables() = %zu)\n",
           correct_num_variables,
           all_message_payload_lengths, local_data_length, type_vec_length, arity_length, witness_length,
           constraint_system.num_variables());
#endif

    return (type_not_zero && incoming_message_payload_lengths_well_specified && correct_num_inputs && correct_num_variables);
}

template<typename FieldT>
bool r1cs_pcd_compliance_predicate<FieldT>::has_equal_input_and_output_lengths() const
{
    for (size_t i = 0; i < incoming_message_payload_lengths.size(); ++i)
    {
        if (incoming_message_payload_lengths[i] != outgoing_message_payload_length)
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
bool r1cs_pcd_compliance_predicate<FieldT>::has_equal_input_lengths() const
{
    for (size_t i = 1; i < incoming_message_payload_lengths.size(); ++i)
    {
        if (incoming_message_payload_lengths[i] != incoming_message_payload_lengths[0])
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
bool r1cs_pcd_compliance_predicate<FieldT>::operator==(const r1cs_pcd_compliance_predicate<FieldT> &other) const
{
    return (this->name == other.name &&
            this->type == other.type &&
            this->constraint_system == other.constraint_system &&
            this->outgoing_message_payload_length == other.outgoing_message_payload_length &&
            this->max_arity == other.max_arity &&
            this->incoming_message_payload_lengths == other.incoming_message_payload_lengths &&
            this->local_data_length == other.local_data_length &&
            this->witness_length == other.witness_length &&
            this->relies_on_same_type_inputs == other.relies_on_same_type_inputs &&
            this->accepted_input_types == other.accepted_input_types);
}

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const r1cs_pcd_compliance_predicate<FieldT> &cp)
{
    out << cp.name << "\n";
    out << cp.type << "\n";
    out << cp.max_arity << "\n";
    assert(cp.max_arity == cp.incoming_message_payload_lengths.size());
    for (size_t i = 0; i < cp.max_arity; ++i)
    {
        out << cp.incoming_message_payload_lengths[i] << "\n";
    }
    out << cp.outgoing_message_payload_length << "\n";
    out << cp.local_data_length << "\n";
    out << cp.witness_length << "\n";
    libff::output_bool(out, cp.relies_on_same_type_inputs);
    libff::operator<<(out, cp.accepted_input_types);
    out << "\n" << cp.constraint_system << "\n";

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, r1cs_pcd_compliance_predicate<FieldT> &cp)
{
    in >> cp.name;
    libff::consume_newline(in);
    in >> cp.type;
    libff::consume_newline(in);
    in >> cp.max_arity;
    libff::consume_newline(in);
    cp.incoming_message_payload_lengths.resize(cp.max_arity);
    for (size_t i = 0; i < cp.max_arity; ++i)
    {
        in >> cp.incoming_message_payload_lengths[i];
        libff::consume_newline(in);
    }
    in >> cp.outgoing_message_payload_length;
    libff::consume_newline(in);
    in >> cp.local_data_length;
    libff::consume_newline(in);
    in >> cp.witness_length;
    libff::consume_newline(in);
    libff::input_bool(in, cp.relies_on_same_type_inputs);
    libff::operator>>(in, cp.accepted_input_types);
    libff::consume_newline(in);
    in >> cp.constraint_system;
    libff::consume_newline(in);

    return in;
}

template<typename FieldT>
bool r1cs_pcd_compliance_predicate<FieldT>::is_satisfied(const std::shared_ptr<r1cs_pcd_message<FieldT> > &outgoing_message,
                                                         const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_messages,
                                                         const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data,
                                                         const r1cs_pcd_witness<FieldT> &witness) const
{
    assert(outgoing_message.payload_as_r1cs_variable_assignment().size() == outgoing_message_payload_length);
    assert(incoming_messages.size() <= max_arity);
    for (size_t i = 0; i < incoming_messages.size(); ++i)
    {
        assert(incoming_messages[i].payload_as_r1cs_variable_assignment().size() == incoming_message_payload_lengths[i]);
    }
    assert(local_data.as_r1cs_variable_assignment().size() == local_data_length);

    r1cs_pcd_compliance_predicate_primary_input<FieldT> cp_primary_input(outgoing_message);
    r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> cp_auxiliary_input(incoming_messages, local_data, witness);

    return constraint_system.is_satisfied(cp_primary_input.as_r1cs_primary_input(),
                                          cp_auxiliary_input.as_r1cs_auxiliary_input(incoming_message_payload_lengths));
}

} // libsnark

#endif //  COMPLIANCE_PREDICATE_TCC_
