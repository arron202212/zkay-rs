/** @file
 *****************************************************************************

 Declaration of interfaces for the tally compliance predicate.

 The tally compliance predicate has two purposes:
 (1) it exemplifies the use of interfaces declared in cp_handler.hpp, and
 (2) it enables us to test r1cs_pcd functionalities.

 See
 - libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/examples/run_r1cs_sp_ppzkpcd.hpp
 - libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/examples/run_r1cs_mp_ppzkpcd.hpp
 for code that uses the tally compliance predicate.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TALLY_CP_HPP_
// #define TALLY_CP_HPP_

use crate::gadgetlib1::gadgets/basic_gadgets;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/compliance_predicate;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/cp_handler;



/**
 * Subclasses a R1CS PCD message to the tally compliance predicate.
 */
template<typename FieldT>
class tally_pcd_message : public r1cs_pcd_message<FieldT> {
public:
    size_t wordsize;

    size_t sum;
    size_t count;

    tally_pcd_message(const size_t type,
                      const size_t wordsize,
                      const size_t sum,
                      const size_t count);
    r1cs_variable_assignment<FieldT> payload_as_r1cs_variable_assignment() const;
    void print() const;

    ~tally_pcd_message() = default;
};

template<typename FieldT>
class tally_pcd_local_data : public r1cs_pcd_local_data<FieldT> {
public:
    size_t summand;

    tally_pcd_local_data(const size_t summand);
    r1cs_variable_assignment<FieldT> as_r1cs_variable_assignment() const;
    void print() const;

    ~tally_pcd_local_data() = default;
};

/**
 * Subclass a R1CS compliance predicate handler to the tally compliance predicate handler.
 */
template<typename FieldT>
class tally_cp_handler : public compliance_predicate_handler<FieldT, protoboard<FieldT> > {
public:
    type compliance_predicate_handler<FieldT, protoboard<FieldT> > base_handler;
    pb_variable_array<FieldT> incoming_types;

    pb_variable<FieldT> sum_out_packed;
    pb_variable<FieldT> count_out_packed;
    pb_variable_array<FieldT> sum_in_packed;
    pb_variable_array<FieldT> count_in_packed;

    pb_variable_array<FieldT> sum_in_packed_aux;
    pb_variable_array<FieldT> count_in_packed_aux;

    std::shared_ptr<packing_gadget<FieldT> > unpack_sum_out;
    std::shared_ptr<packing_gadget<FieldT> > unpack_count_out;
    std::vector<packing_gadget<FieldT> > pack_sum_in;
    std::vector<packing_gadget<FieldT> > pack_count_in;

    pb_variable<FieldT> type_val_inner_product;
    std::shared_ptr<inner_product_gadget<FieldT> > compute_type_val_inner_product;

    pb_variable_array<FieldT> arity_indicators;

    size_t wordsize;
    size_t message_length;

    tally_cp_handler(const size_t type,
                     const size_t max_arity,
                     const size_t wordsize,
                     const bool relies_on_same_type_inputs = false,
                     const std::set<size_t> accepted_input_types = std::set<size_t>());

    void generate_r1cs_constraints();
    void generate_r1cs_witness(const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_messages,
                               const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data);

    std::shared_ptr<r1cs_pcd_message<FieldT> > get_base_case_message() const;
};



use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/examples/tally_cp;

//#endif // TALLY_CP_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the tally compliance predicate.

 See tally_cp.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TALLY_CP_TCC_
// #define TALLY_CP_TCC_

use  <algorithm>
use  <functional>

use ffec::algebra::field_utils::field_utils;



template<typename FieldT>
tally_pcd_message<FieldT>::tally_pcd_message(const size_t type,
                                             const size_t wordsize,
                                             const size_t sum,
                                             const size_t count) :
    r1cs_pcd_message<FieldT>(type), wordsize(wordsize), sum(sum), count(count)
{
}

template<typename FieldT>
r1cs_variable_assignment<FieldT> tally_pcd_message<FieldT>::payload_as_r1cs_variable_assignment() const
{
    std::function<FieldT(bool)> bit_to_FieldT = [] (const bool bit) { return bit ? FieldT::one() : FieldT::zero(); };

    const ffec::bit_vector sum_bits = ffec::convert_field_element_to_bit_vector<FieldT>(sum, wordsize);
    const ffec::bit_vector count_bits = ffec::convert_field_element_to_bit_vector<FieldT>(count, wordsize);

    r1cs_variable_assignment<FieldT> result(2 * wordsize);
    std::transform(sum_bits.begin(), sum_bits.end(), result.begin() , bit_to_FieldT);
    std::transform(count_bits.begin(), count_bits.end(), result.begin() + wordsize, bit_to_FieldT);

    return result;
}

template<typename FieldT>
void tally_pcd_message<FieldT>::print() const
{
    print!("Tally message of type {}:\n", self.type);
    print!("  wordsize: {}\n", wordsize);
    print!("  sum: {}\n", sum);
    print!("  count: {}\n", count);
}

template<typename FieldT>
tally_pcd_local_data<FieldT>::tally_pcd_local_data(const size_t summand) :
    summand(summand)
{
}

template<typename FieldT>
r1cs_variable_assignment<FieldT> tally_pcd_local_data<FieldT>::as_r1cs_variable_assignment() const
{
    const r1cs_variable_assignment<FieldT> result = { FieldT(summand) };
    return result;
}

template<typename FieldT>
void tally_pcd_local_data<FieldT>::print() const
{
    print!("Tally PCD local data:\n");
    print!("  summand: {}\n", summand);
}

template<typename FieldT>
class tally_pcd_message_variable: public r1cs_pcd_message_variable<FieldT> {
public:
    pb_variable_array<FieldT> sum_bits;
    pb_variable_array<FieldT> count_bits;
    size_t wordsize;

    tally_pcd_message_variable(protoboard<FieldT> &pb,
                               const size_t wordsize,
                               const std::string &annotation_prefix) :
        r1cs_pcd_message_variable<FieldT>(pb, annotation_prefix), wordsize(wordsize)
    {
        sum_bits.allocate(pb, wordsize, FMT(annotation_prefix, " sum_bits"));
        count_bits.allocate(pb, wordsize, FMT(annotation_prefix, " count_bits"));

        self.update_all_vars();
    }

    std::shared_ptr<r1cs_pcd_message<FieldT> > get_message() const
    {
        const size_t type_val = self.pb.val(self.type).as_ulong();
        const size_t sum_val = sum_bits.get_field_element_from_bits(self.pb).as_ulong();
        const size_t count_val = count_bits.get_field_element_from_bits(self.pb).as_ulong();

        std::shared_ptr<r1cs_pcd_message<FieldT> > result;
        result.reset(new tally_pcd_message<FieldT>(type_val, wordsize, sum_val, count_val));
        return result;
    }

    ~tally_pcd_message_variable() = default;
};

template<typename FieldT>
class tally_pcd_local_data_variable : public r1cs_pcd_local_data_variable<FieldT> {
public:

    pb_variable<FieldT> summand;

    tally_pcd_local_data_variable(protoboard<FieldT> &pb,
                                  const std::string &annotation_prefix) :
        r1cs_pcd_local_data_variable<FieldT>(pb, annotation_prefix)
    {
        summand.allocate(pb, FMT(annotation_prefix, " summand"));

        self.update_all_vars();
    }

    std::shared_ptr<r1cs_pcd_local_data<FieldT> > get_local_data() const
    {
        const size_t summand_val = self.pb.val(summand).as_ulong();

        std::shared_ptr<r1cs_pcd_local_data<FieldT> > result;
        result.reset(new tally_pcd_local_data<FieldT>(summand_val));
        return result;
    }

    ~tally_pcd_local_data_variable() = default;
};

template<typename FieldT>
tally_cp_handler<FieldT>::tally_cp_handler(const size_t type, const size_t max_arity, const size_t wordsize,
                                           const bool relies_on_same_type_inputs,
                                           const std::set<size_t> accepted_input_types) :
    compliance_predicate_handler<FieldT, protoboard<FieldT> >(protoboard<FieldT>(),
                                                              type*100,
                                                              type,
                                                              max_arity,
                                                              relies_on_same_type_inputs,
                                                              accepted_input_types),
    wordsize(wordsize)
{
    self.outgoing_message.reset(new tally_pcd_message_variable<FieldT>(self.pb, wordsize, "outgoing_message"));
    self.arity.allocate(self.pb, "arity");

    for i in 0..max_arity
    {
        self.incoming_messages[i].reset(new tally_pcd_message_variable<FieldT>(self.pb, wordsize, FMT("", "incoming_messages_{}", i)));
    }

    self.local_data.reset(new tally_pcd_local_data_variable<FieldT>(self.pb, "local_data"));

    sum_out_packed.allocate(self.pb, "sum_out_packed");
    count_out_packed.allocate(self.pb, "count_out_packed");

    sum_in_packed.allocate(self.pb, max_arity, "sum_in_packed");
    count_in_packed.allocate(self.pb, max_arity, "count_in_packed");

    sum_in_packed_aux.allocate(self.pb, max_arity, "sum_in_packed_aux");
    count_in_packed_aux.allocate(self.pb, max_arity, "count_in_packed_aux");

    type_val_inner_product.allocate(self.pb, "type_val_inner_product");
    for msg in &self.incoming_messages
    {
        incoming_types.push(msg->type);
    }

    compute_type_val_inner_product.reset(new inner_product_gadget<FieldT>(self.pb, incoming_types, sum_in_packed, type_val_inner_product, "compute_type_val_inner_product"));

    unpack_sum_out.reset(new packing_gadget<FieldT>(self.pb, std::dynamic_pointer_cast<tally_pcd_message_variable<FieldT> >(self.outgoing_message)->sum_bits, sum_out_packed, "pack_sum_out"));
    unpack_count_out.reset(new packing_gadget<FieldT>(self.pb, std::dynamic_pointer_cast<tally_pcd_message_variable<FieldT> >(self.outgoing_message)->count_bits, count_out_packed, "pack_count_out"));

    for i in 0..max_arity
    {
        pack_sum_in.push(packing_gadget<FieldT>(self.pb, std::dynamic_pointer_cast<tally_pcd_message_variable<FieldT> >(self.incoming_messages[i])->sum_bits, sum_in_packed[i], FMT("", "pack_sum_in_{}", i)));
        pack_count_in.push(packing_gadget<FieldT>(self.pb, std::dynamic_pointer_cast<tally_pcd_message_variable<FieldT> >(self.incoming_messages[i])->sum_bits, count_in_packed[i], FMT("", "pack_count_in_{}", i)));
    }

    arity_indicators.allocate(self.pb, max_arity+1, "arity_indicators");
}

template<typename FieldT>
void tally_cp_handler<FieldT>::generate_r1cs_constraints()
{
    unpack_sum_out->generate_r1cs_constraints(true);
    unpack_count_out->generate_r1cs_constraints(true);

    for i in 0..self.max_arity
    {
        pack_sum_in[i].generate_r1cs_constraints(true);
        pack_count_in[i].generate_r1cs_constraints(true);
    }

    for i in 0..self.max_arity
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(incoming_types[i], sum_in_packed_aux[i], sum_in_packed[i]), FMT("", "initial_sum_%zu_is_zero", i));
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(incoming_types[i], count_in_packed_aux[i], count_in_packed[i]), FMT("", "initial_sum_%zu_is_zero", i));
    }

    /* constrain arity indicator variables so that arity_indicators[arity] = 1 and arity_indicators[i] = 0 for any other i */
    for i in 0..self.max_arity
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(self.arity - FieldT(i), arity_indicators[i], 0), FMT("", "arity_indicators_{}", i));
    }

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, pb_sum<FieldT>(arity_indicators), 1), "arity_indicators");

    /* require that types of messages that are past arity (i.e. unbound wires) carry 0 */
    for i in 0..self.max_arity
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(0 + pb_sum<FieldT>(pb_variable_array<FieldT>(arity_indicators.begin(), arity_indicators.begin() + i)), incoming_types[i], 0), FMT("", "unbound_types_{}", i));
    }

    /* sum_out = local_data + \sum_i type[i] * sum_in[i] */
    compute_type_val_inner_product->generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, type_val_inner_product + std::dynamic_pointer_cast<tally_pcd_local_data_variable<FieldT> >(self.local_data)->summand, sum_out_packed), "update_sum");

    /* count_out = 1 + \sum_i count_in[i] */
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, 1 + pb_sum<FieldT>(count_in_packed), count_out_packed), "update_count");
}

template<typename FieldT>
void tally_cp_handler<FieldT>::generate_r1cs_witness(const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_messages,
                                                     const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data)
{
    base_handler::generate_r1cs_witness(incoming_messages, local_data);

    for i in 0..self.max_arity
    {
        pack_sum_in[i].generate_r1cs_witness_from_bits();
        pack_count_in[i].generate_r1cs_witness_from_bits();

        if !self.pb.val(incoming_types[i]).is_zero()
        {
            self.pb.val(sum_in_packed_aux[i]) = self.pb.val(sum_in_packed[i]) * self.pb.val(incoming_types[i]).inverse();
            self.pb.val(count_in_packed_aux[i]) = self.pb.val(count_in_packed[i]) * self.pb.val(incoming_types[i]).inverse();
        }
    }

    for i in 0..self.max_arity + 1
    {
        self.pb.val(arity_indicators[i]) = (incoming_messages.size() == i ? FieldT::one() : FieldT::zero());
    }

    compute_type_val_inner_product->generate_r1cs_witness();
    self.pb.val(sum_out_packed) = self.pb.val(std::dynamic_pointer_cast<tally_pcd_local_data_variable<FieldT> >(self.local_data)->summand) + self.pb.val(type_val_inner_product);

    self.pb.val(count_out_packed) = FieldT::one();
    for i in 0..self.max_arity
    {
        self.pb.val(count_out_packed) += self.pb.val(count_in_packed[i]);
    }

    unpack_sum_out->generate_r1cs_witness_from_packed();
    unpack_count_out->generate_r1cs_witness_from_packed();
}

template<typename FieldT>
std::shared_ptr<r1cs_pcd_message<FieldT> > tally_cp_handler<FieldT>::get_base_case_message() const
{
    const size_t type = 0;
    const size_t sum = 0;
    const size_t count = 0;

    std::shared_ptr<r1cs_pcd_message<FieldT> > result;
    result.reset(new tally_pcd_message<FieldT>(type, wordsize, sum, count));

    return result;
}



//#endif // TALLY_CP_TCC_
