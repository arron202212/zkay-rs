/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_GADGETS_HPP_
// #define BASIC_GADGETS_HPP_

use  <cassert>
use  <memory>

use libsnark/gadgetlib1/gadget;



/* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
template<typename FieldT>
void generate_boolean_r1cs_constraint(protoboard<FieldT> &pb, const pb_linear_combination<FieldT> &lc, const std::string &annotation_prefix="");

template<typename FieldT>
void generate_r1cs_equals_const_constraint(protoboard<FieldT> &pb, const pb_linear_combination<FieldT> &lc, const FieldT& c, const std::string &annotation_prefix="");

template<typename FieldT>
class packing_gadget : public gadget<FieldT> {
private:
    /* no internal variables */
public:
    const pb_linear_combination_array<FieldT> bits;
    const pb_linear_combination<FieldT> packed;

    packing_gadget(protoboard<FieldT> &pb,
                   const pb_linear_combination_array<FieldT> &bits,
                   const pb_linear_combination<FieldT> &packed,
                   const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), bits(bits), packed(packed) {}

    void generate_r1cs_constraints(const bool enforce_bitness);
    /* adds constraint result = \sum  bits[i] * 2^i */

    void generate_r1cs_witness_from_packed();
    void generate_r1cs_witness_from_bits();
};

template<typename FieldT>
class multipacking_gadget : public gadget<FieldT> {
private:
    std::vector<packing_gadget<FieldT> > packers;
public:
    const pb_linear_combination_array<FieldT> bits;
    const pb_linear_combination_array<FieldT> packed_vars;

    const size_t chunk_size;
    const size_t num_chunks;
    // const size_t last_chunk_size;

    multipacking_gadget(protoboard<FieldT> &pb,
                        const pb_linear_combination_array<FieldT> &bits,
                        const pb_linear_combination_array<FieldT> &packed_vars,
                        const size_t chunk_size,
                        const std::string &annotation_prefix="");
    void generate_r1cs_constraints(const bool enforce_bitness);
    void generate_r1cs_witness_from_packed();
    void generate_r1cs_witness_from_bits();
};

template<typename FieldT>
class field_vector_copy_gadget : public gadget<FieldT> {
public:
    const pb_variable_array<FieldT> source;
    const pb_variable_array<FieldT> target;
    const pb_linear_combination<FieldT> do_copy;

    field_vector_copy_gadget(protoboard<FieldT> &pb,
                             const pb_variable_array<FieldT> &source,
                             const pb_variable_array<FieldT> &target,
                             const pb_linear_combination<FieldT> &do_copy,
                             const std::string &annotation_prefix="");
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
class bit_vector_copy_gadget : public gadget<FieldT> {
public:
    const pb_variable_array<FieldT> source_bits;
    const pb_variable_array<FieldT> target_bits;
    const pb_linear_combination<FieldT> do_copy;

    pb_variable_array<FieldT> packed_source;
    pb_variable_array<FieldT> packed_target;

    std::shared_ptr<multipacking_gadget<FieldT> > pack_source;
    std::shared_ptr<multipacking_gadget<FieldT> > pack_target;
    std::shared_ptr<field_vector_copy_gadget<FieldT> > copier;

    const size_t chunk_size;
    const size_t num_chunks;

    bit_vector_copy_gadget(protoboard<FieldT> &pb,
                           const pb_variable_array<FieldT> &source_bits,
                           const pb_variable_array<FieldT> &target_bits,
                           const pb_linear_combination<FieldT> &do_copy,
                           const size_t chunk_size,
                           const std::string &annotation_prefix="");
    void generate_r1cs_constraints(const bool enforce_source_bitness, const bool enforce_target_bitness);
    void generate_r1cs_witness();
};

template<typename FieldT>
class dual_variable_gadget : public gadget<FieldT> {
private:
    std::shared_ptr<packing_gadget<FieldT> > consistency_check;
public:
    pb_variable<FieldT> packed;
    pb_variable_array<FieldT> bits;

    dual_variable_gadget(protoboard<FieldT> &pb,
                         const size_t width,
                         const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix)
    {
        packed.allocate(pb, FMT(self.annotation_prefix, " packed"));
        bits.allocate(pb, width, FMT(self.annotation_prefix, " bits"));
        consistency_check.reset(new packing_gadget<FieldT>(pb,
                                                           bits,
                                                           packed,
                                                           FMT(self.annotation_prefix, " consistency_check")));
    }

    dual_variable_gadget(protoboard<FieldT> &pb,
                         const pb_variable_array<FieldT> &bits,
                         const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), bits(bits)
    {
        packed.allocate(pb, FMT(self.annotation_prefix, " packed"));
        consistency_check.reset(new packing_gadget<FieldT>(pb,
                                                           bits,
                                                           packed,
                                                           FMT(self.annotation_prefix, " consistency_check")));
    }

    dual_variable_gadget(protoboard<FieldT> &pb,
                         const pb_variable<FieldT> &packed,
                         const size_t width,
                         const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), packed(packed)
    {
        bits.allocate(pb, width, FMT(self.annotation_prefix, " bits"));
        consistency_check.reset(new packing_gadget<FieldT>(pb,
                                                           bits,
                                                           packed,
                                                           FMT(self.annotation_prefix, " consistency_check")));
    }

    void generate_r1cs_constraints(const bool enforce_bitness);
    void generate_r1cs_witness_from_packed();
    void generate_r1cs_witness_from_bits();
};

/*
  the gadgets below are Fp specific:
  I * X = R
  (1-R) * X = 0

  if X = 0 then R = 0
  if X != 0 then R = 1 and I = X^{-1}
*/

template<typename FieldT>
class disjunction_gadget : public gadget<FieldT> {
private:
    pb_variable<FieldT> inv;
public:
    const pb_variable_array<FieldT> inputs;
    const pb_variable<FieldT> output;

    disjunction_gadget(protoboard<FieldT>& pb,
                       const pb_variable_array<FieldT> &inputs,
                       const pb_variable<FieldT> &output,
                       const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), inputs(inputs), output(output)
    {
        assert!(inputs.size() >= 1);
        inv.allocate(pb, FMT(self.annotation_prefix, " inv"));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_disjunction_gadget(const size_t n);

template<typename FieldT>
class conjunction_gadget : public gadget<FieldT> {
private:
    pb_variable<FieldT> inv;
public:
    const pb_variable_array<FieldT> inputs;
    const pb_variable<FieldT> output;

    conjunction_gadget(protoboard<FieldT>& pb,
                       const pb_variable_array<FieldT> &inputs,
                       const pb_variable<FieldT> &output,
                       const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), inputs(inputs), output(output)
    {
        assert!(inputs.size() >= 1);
        inv.allocate(pb, FMT(self.annotation_prefix, " inv"));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_conjunction_gadget(const size_t n);

template<typename FieldT>
class comparison_gadget : public gadget<FieldT> {
private:
    pb_variable_array<FieldT> alpha;
    pb_variable<FieldT> alpha_packed;
    std::shared_ptr<packing_gadget<FieldT> > pack_alpha;

    std::shared_ptr<disjunction_gadget<FieldT> > all_zeros_test;
    pb_variable<FieldT> not_all_zeros;
public:
    const size_t n;
    const pb_linear_combination<FieldT> A;
    const pb_linear_combination<FieldT> B;
    const pb_variable<FieldT> less;
    const pb_variable<FieldT> less_or_eq;

    comparison_gadget(protoboard<FieldT>& pb,
                      const size_t n,
                      const pb_linear_combination<FieldT> &A,
                      const pb_linear_combination<FieldT> &B,
                      const pb_variable<FieldT> &less,
                      const pb_variable<FieldT> &less_or_eq,
                      const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), n(n), A(A), B(B), less(less), less_or_eq(less_or_eq)
    {
        alpha.allocate(pb, n, FMT(self.annotation_prefix, " alpha"));
        alpha.push(less_or_eq); // alpha[n] is less_or_eq

        alpha_packed.allocate(pb, FMT(self.annotation_prefix, " alpha_packed"));
        not_all_zeros.allocate(pb, FMT(self.annotation_prefix, " not_all_zeros"));

        pack_alpha.reset(new packing_gadget<FieldT>(pb, alpha, alpha_packed,
                                                    FMT(self.annotation_prefix, " pack_alpha")));

        all_zeros_test.reset(new disjunction_gadget<FieldT>(pb,
                                                            pb_variable_array<FieldT>(alpha.begin(), alpha.begin() + n),
                                                            not_all_zeros,
                                                            FMT(self.annotation_prefix, " all_zeros_test")));
    };

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_comparison_gadget(const size_t n);

template<typename FieldT>
class inner_product_gadget : public gadget<FieldT> {
private:
    /* S_i = \sum_{k=0}^{i+1} A[i] * B[i] */
    pb_variable_array<FieldT> S;
public:
    const pb_linear_combination_array<FieldT> A;
    const pb_linear_combination_array<FieldT> B;
    const pb_variable<FieldT> result;

    inner_product_gadget(protoboard<FieldT>& pb,
                         const pb_linear_combination_array<FieldT> &A,
                         const pb_linear_combination_array<FieldT> &B,
                         const pb_variable<FieldT> &result,
                         const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), A(A), B(B), result(result)
    {
        assert!(A.size() >= 1);
        assert!(A.size() == B.size());

        S.allocate(pb, A.size()-1, FMT(self.annotation_prefix, " S"));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_inner_product_gadget(const size_t n);

template<typename FieldT>
class loose_multiplexing_gadget : public gadget<FieldT> {
/*
  this implements loose multiplexer:
  index not in bounds -> success_flag = 0
  index in bounds && success_flag = 1 -> result is correct
  however if index is in bounds we can also set success_flag to 0 (and then result will be forced to be 0)
*/
public:
    pb_variable_array<FieldT> alpha;
private:
    std::shared_ptr<inner_product_gadget<FieldT> > compute_result;
public:
    const pb_linear_combination_array<FieldT> arr;
    const pb_variable<FieldT> index;
    const pb_variable<FieldT> result;
    const pb_variable<FieldT> success_flag;

    loose_multiplexing_gadget(protoboard<FieldT>& pb,
                              const pb_linear_combination_array<FieldT> &arr,
                              const pb_variable<FieldT> &index,
                              const pb_variable<FieldT> &result,
                              const pb_variable<FieldT> &success_flag,
                              const std::string &annotation_prefix="") :
        gadget<FieldT>(pb, annotation_prefix), arr(arr), index(index), result(result), success_flag(success_flag)
    {
        alpha.allocate(pb, arr.size(), FMT(self.annotation_prefix, " alpha"));
        compute_result.reset(new inner_product_gadget<FieldT>(pb, alpha, arr, result, FMT(self.annotation_prefix, " compute_result")));
    };

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_loose_multiplexing_gadget(const size_t n);

template<typename FieldT, typename VarT>
void create_linear_combination_constraints(protoboard<FieldT> &pb,
                                           const std::vector<FieldT> &base,
                                           const std::vector<std::pair<VarT, FieldT> > &v,
                                           const VarT &target,
                                           const std::string &annotation_prefix);

template<typename FieldT, typename VarT>
void create_linear_combination_witness(protoboard<FieldT> &pb,
                                       const std::vector<FieldT> &base,
                                       const std::vector<std::pair<VarT, FieldT> > &v,
                                       const VarT &target);


use libsnark/gadgetlib1/gadgets/basic_gadgets;

//#endif // BASIC_GADGETS_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_GADGETS_TCC_
// #define BASIC_GADGETS_TCC_

use ffec::common::profiling;
use ffec::common::utils;



template<typename FieldT>
void generate_boolean_r1cs_constraint(protoboard<FieldT> &pb, const pb_linear_combination<FieldT> &lc, const std::string &annotation_prefix)
/* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
{
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(lc, 1-lc, 0),
                           FMT(annotation_prefix, " boolean_r1cs_constraint"));
}

template<typename FieldT>
void generate_r1cs_equals_const_constraint(protoboard<FieldT> &pb, const pb_linear_combination<FieldT> &lc, const FieldT& c, const std::string &annotation_prefix)
{
    pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, lc, c),
                           FMT(annotation_prefix, " constness_constraint"));
}

template<typename FieldT>
void packing_gadget<FieldT>::generate_r1cs_constraints(const bool enforce_bitness)
/* adds constraint result = \sum  bits[i] * 2^i */
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, pb_packing_sum<FieldT>(bits), packed), FMT(self.annotation_prefix, " packing_constraint"));

    if (enforce_bitness)
    {
        for (size_t i = 0; i < bits.size(); ++i)
        {
            generate_boolean_r1cs_constraint<FieldT>(self.pb, bits[i], FMT(self.annotation_prefix, " bitness_{}", i));
        }
    }
}

template<typename FieldT>
void packing_gadget<FieldT>::generate_r1cs_witness_from_packed()
{
    packed.evaluate(self.pb);
    assert!(self.pb.lc_val(packed).as_bigint().num_bits() <= bits.size()); // `bits` is large enough to represent this packed value
    bits.fill_with_bits_of_field_element(self.pb, self.pb.lc_val(packed));
}

template<typename FieldT>
void packing_gadget<FieldT>::generate_r1cs_witness_from_bits()
{
    bits.evaluate(self.pb);
    self.pb.lc_val(packed) = bits.get_field_element_from_bits(self.pb);
}

template<typename FieldT>
multipacking_gadget<FieldT>::multipacking_gadget(protoboard<FieldT> &pb,
                                                 const pb_linear_combination_array<FieldT> &bits,
                                                 const pb_linear_combination_array<FieldT> &packed_vars,
                                                 const size_t chunk_size,
                                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), bits(bits), packed_vars(packed_vars),
    chunk_size(chunk_size),
    num_chunks(ffec::div_ceil(bits.size(), chunk_size))
    // last_chunk_size(bits.size() - (num_chunks-1) * chunk_size)
{
    assert!(packed_vars.size() == num_chunks);
    for (size_t i = 0; i < num_chunks; ++i)
    {
        packers.push(packing_gadget<FieldT>(self.pb, pb_linear_combination_array<FieldT>(bits.begin() + i * chunk_size,
                                                                                                  bits.begin() + std::min((i+1) * chunk_size, bits.size())),
                                                    packed_vars[i], FMT(self.annotation_prefix, " packers_{}", i)));
    }
}

template<typename FieldT>
void multipacking_gadget<FieldT>::generate_r1cs_constraints(const bool enforce_bitness)
{
    for (size_t i = 0; i < num_chunks; ++i)
    {
        packers[i].generate_r1cs_constraints(enforce_bitness);
    }
}

template<typename FieldT>
void multipacking_gadget<FieldT>::generate_r1cs_witness_from_packed()
{
    for (size_t i = 0; i < num_chunks; ++i)
    {
        packers[i].generate_r1cs_witness_from_packed();
    }
}

template<typename FieldT>
void multipacking_gadget<FieldT>::generate_r1cs_witness_from_bits()
{
    for (size_t i = 0; i < num_chunks; ++i)
    {
        packers[i].generate_r1cs_witness_from_bits();
    }
}

template<typename FieldT>
size_t multipacking_num_chunks(const size_t num_bits)
{
    return ffec::div_ceil(num_bits, FieldT::capacity());
}

template<typename FieldT>
field_vector_copy_gadget<FieldT>::field_vector_copy_gadget(protoboard<FieldT> &pb,
                                                           const pb_variable_array<FieldT> &source,
                                                           const pb_variable_array<FieldT> &target,
                                                           const pb_linear_combination<FieldT> &do_copy,
                                                           const std::string &annotation_prefix) :
gadget<FieldT>(pb, annotation_prefix), source(source), target(target), do_copy(do_copy)
{
    assert!(source.size() == target.size());
}

template<typename FieldT>
void field_vector_copy_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < source.size(); ++i)
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(do_copy, source[i] - target[i], 0),
                                     FMT(self.annotation_prefix, " copying_check_{}", i));
    }
}

template<typename FieldT>
void field_vector_copy_gadget<FieldT>::generate_r1cs_witness()
{
    do_copy.evaluate(self.pb);
    assert!(self.pb.lc_val(do_copy) == FieldT::one() || self.pb.lc_val(do_copy) == FieldT::zero());
    if (self.pb.lc_val(do_copy) != FieldT::zero())
    {
        for (size_t i = 0; i < source.size(); ++i)
        {
            self.pb.val(target[i]) = self.pb.val(source[i]);
        }
    }
}

template<typename FieldT>
bit_vector_copy_gadget<FieldT>::bit_vector_copy_gadget(protoboard<FieldT> &pb,
                                                       const pb_variable_array<FieldT> &source_bits,
                                                       const pb_variable_array<FieldT> &target_bits,
                                                       const pb_linear_combination<FieldT> &do_copy,
                                                       const size_t chunk_size,
                                                       const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), source_bits(source_bits), target_bits(target_bits), do_copy(do_copy),
    chunk_size(chunk_size), num_chunks(ffec::div_ceil(source_bits.size(), chunk_size))
{
    assert!(source_bits.size() == target_bits.size());

    packed_source.allocate(pb, num_chunks, FMT(annotation_prefix, " packed_source"));
    pack_source.reset(new multipacking_gadget<FieldT>(pb, source_bits, packed_source, chunk_size, FMT(annotation_prefix, " pack_source")));

    packed_target.allocate(pb, num_chunks, FMT(annotation_prefix, " packed_target"));
    pack_target.reset(new multipacking_gadget<FieldT>(pb, target_bits, packed_target, chunk_size, FMT(annotation_prefix, " pack_target")));

    copier.reset(new field_vector_copy_gadget<FieldT>(pb, packed_source, packed_target, do_copy, FMT(annotation_prefix, " copier")));
}

template<typename FieldT>
void bit_vector_copy_gadget<FieldT>::generate_r1cs_constraints(const bool enforce_source_bitness, const bool enforce_target_bitness)
{
    pack_source->generate_r1cs_constraints(enforce_source_bitness);
    pack_target->generate_r1cs_constraints(enforce_target_bitness);

    copier->generate_r1cs_constraints();
}

template<typename FieldT>
void bit_vector_copy_gadget<FieldT>::generate_r1cs_witness()
{
    do_copy.evaluate(self.pb);
    assert!(self.pb.lc_val(do_copy) == FieldT::zero() || self.pb.lc_val(do_copy) == FieldT::one());
    if (self.pb.lc_val(do_copy) == FieldT::one())
    {
        for (size_t i = 0; i < source_bits.size(); ++i)
        {
            self.pb.val(target_bits[i]) = self.pb.val(source_bits[i]);
        }
    }

    pack_source->generate_r1cs_witness_from_bits();
    pack_target->generate_r1cs_witness_from_bits();
}

template<typename FieldT>
void dual_variable_gadget<FieldT>::generate_r1cs_constraints(const bool enforce_bitness)
{
    consistency_check->generate_r1cs_constraints(enforce_bitness);
}

template<typename FieldT>
void dual_variable_gadget<FieldT>::generate_r1cs_witness_from_packed()
{
    consistency_check->generate_r1cs_witness_from_packed();
}

template<typename FieldT>
void dual_variable_gadget<FieldT>::generate_r1cs_witness_from_bits()
{
    consistency_check->generate_r1cs_witness_from_bits();
}

template<typename FieldT>
void disjunction_gadget<FieldT>::generate_r1cs_constraints()
{
    /* inv * sum = output */
    linear_combination<FieldT> a1, b1, c1;
    a1.add_term(inv);
    for (size_t i = 0; i < inputs.size(); ++i)
    {
        b1.add_term(inputs[i]);
    }
    c1.add_term(output);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a1, b1, c1), FMT(self.annotation_prefix, " inv*sum=output"));

    /* (1-output) * sum = 0 */
    linear_combination<FieldT> a2, b2, c2;
    a2.add_term(ONE);
    a2.add_term(output, -1);
    for (size_t i = 0; i < inputs.size(); ++i)
    {
        b2.add_term(inputs[i]);
    }
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a2, b2, c2), FMT(self.annotation_prefix, " (1-output)*sum=0"));
}

template<typename FieldT>
void disjunction_gadget<FieldT>::generate_r1cs_witness()
{
    FieldT sum = FieldT::zero();

    for (size_t i = 0; i < inputs.size(); ++i)
    {
        sum += self.pb.val(inputs[i]);
    }

    if (sum.is_zero())
    {
        self.pb.val(inv) = FieldT::zero();
        self.pb.val(output) = FieldT::zero();
    }
    else
    {
        self.pb.val(inv) = sum.inverse();
        self.pb.val(output) = FieldT::one();
    }
}

template<typename FieldT>
void test_disjunction_gadget(const size_t n)
{
    print!("testing disjunction_gadget on all {} bit strings\n", n);

    protoboard<FieldT> pb;
    pb_variable_array<FieldT> inputs;
    inputs.allocate(pb, n, "inputs");

    pb_variable<FieldT> output;
    output.allocate(pb, "output");

    disjunction_gadget<FieldT> d(pb, inputs, output, "d");
    d.generate_r1cs_constraints();

    for (size_t w = 0; w < 1ul<<n; ++w)
    {
        for (size_t j = 0; j < n; ++j)
        {
            pb.val(inputs[j]) = FieldT((w & (1ul<<j)) ? 1 : 0);
        }

        d.generate_r1cs_witness();

// #ifdef DEBUG
        print!("positive test for {}\n", w);
//#endif
        assert!(pb.val(output) == (w ? FieldT::one() : FieldT::zero()));
        assert!(pb.is_satisfied());

// #ifdef DEBUG
        print!("negative test for {}\n", w);
//#endif
        pb.val(output) = (w ? FieldT::zero() : FieldT::one());
        assert!(!pb.is_satisfied());
    }

    ffec::print_time("disjunction tests successful");
}

template<typename FieldT>
void conjunction_gadget<FieldT>::generate_r1cs_constraints()
{
    /* inv * (n-sum) = 1-output */
    linear_combination<FieldT> a1, b1, c1;
    a1.add_term(inv);
    b1.add_term(ONE, inputs.size());
    for (size_t i = 0; i < inputs.size(); ++i)
    {
        b1.add_term(inputs[i], -1);
    }
    c1.add_term(ONE);
    c1.add_term(output, -1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a1, b1, c1), FMT(self.annotation_prefix, " inv*(n-sum)=(1-output)"));

    /* output * (n-sum) = 0 */
    linear_combination<FieldT> a2, b2, c2;
    a2.add_term(output);
    b2.add_term(ONE, inputs.size());
    for (size_t i = 0; i < inputs.size(); ++i)
    {
        b2.add_term(inputs[i], -1);
    }
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a2, b2, c2), FMT(self.annotation_prefix, " output*(n-sum)=0"));
}

template<typename FieldT>
void conjunction_gadget<FieldT>::generate_r1cs_witness()
{
    FieldT sum = FieldT(inputs.size());

    for (size_t i = 0; i < inputs.size(); ++i)
    {
        sum -= self.pb.val(inputs[i]);
    }

    if (sum.is_zero())
    {
        self.pb.val(inv) = FieldT::zero();
        self.pb.val(output) = FieldT::one();
    }
    else
    {
        self.pb.val(inv) = sum.inverse();
        self.pb.val(output) = FieldT::zero();
    }
}

template<typename FieldT>
void test_conjunction_gadget(const size_t n)
{
    print!("testing conjunction_gadget on all {} bit strings\n", n);

    protoboard<FieldT> pb;
    pb_variable_array<FieldT> inputs;
    inputs.allocate(pb, n, "inputs");

    pb_variable<FieldT> output;
    output.allocate(pb, "output");

    conjunction_gadget<FieldT> c(pb, inputs, output, "c");
    c.generate_r1cs_constraints();

    for (size_t w = 0; w < 1ul<<n; ++w)
    {
        for (size_t j = 0; j < n; ++j)
        {
            pb.val(inputs[j]) = (w & (1ul<<j)) ? FieldT::one() : FieldT::zero();
        }

        c.generate_r1cs_witness();

// #ifdef DEBUG
        print!("positive test for {}\n", w);
//#endif
        assert!(pb.val(output) == (w == (1ul<<n) - 1 ? FieldT::one() : FieldT::zero()));
        assert!(pb.is_satisfied());

// #ifdef DEBUG
        print!("negative test for {}\n", w);
//#endif
        pb.val(output) = (w == (1ul<<n) - 1 ? FieldT::zero() : FieldT::one());
        assert!(!pb.is_satisfied());
    }

    ffec::print_time("conjunction tests successful");
}

template<typename FieldT>
void comparison_gadget<FieldT>::generate_r1cs_constraints()
{
    /*
      packed(alpha) = 2^n + B - A

      not_all_zeros = \bigvee_{i=0}^{n-1} alpha_i

      if B - A > 0, then 2^n + B - A > 2^n,
          so alpha_n = 1 and not_all_zeros = 1
      if B - A = 0, then 2^n + B - A = 2^n,
          so alpha_n = 1 and not_all_zeros = 0
      if B - A < 0, then 2^n + B - A \in {0, 1, \ldots, 2^n-1},
          so alpha_n = 0

      therefore alpha_n = less_or_eq and alpha_n * not_all_zeros = less
     */

    /* not_all_zeros to be Boolean, alpha_i are Boolean by packing gadget */
    generate_boolean_r1cs_constraint<FieldT>(self.pb, not_all_zeros,
                                     FMT(self.annotation_prefix, " not_all_zeros"));

    /* constraints for packed(alpha) = 2^n + B - A */
    pack_alpha->generate_r1cs_constraints(true);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, (FieldT(2)^n) + B - A, alpha_packed), FMT(self.annotation_prefix, " main_constraint"));

    /* compute result */
    all_zeros_test->generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(less_or_eq, not_all_zeros, less),
                                 FMT(self.annotation_prefix, " less"));
}

template<typename FieldT>
void comparison_gadget<FieldT>::generate_r1cs_witness()
{
    A.evaluate(self.pb);
    B.evaluate(self.pb);

    /* unpack 2^n + B - A into alpha_packed */
    self.pb.val(alpha_packed) = (FieldT(2)^n) + self.pb.lc_val(B) - self.pb.lc_val(A);
    pack_alpha->generate_r1cs_witness_from_packed();

    /* compute result */
    all_zeros_test->generate_r1cs_witness();
    self.pb.val(less) = self.pb.val(less_or_eq) * self.pb.val(not_all_zeros);
}

template<typename FieldT>
void test_comparison_gadget(const size_t n)
{
    print!("testing comparison_gadget on all {} bit inputs\n", n);

    protoboard<FieldT> pb;

    pb_variable<FieldT> A, B, less, less_or_eq;
    A.allocate(pb, "A");
    B.allocate(pb, "B");
    less.allocate(pb, "less");
    less_or_eq.allocate(pb, "less_or_eq");

    comparison_gadget<FieldT> cmp(pb, n, A, B, less, less_or_eq, "cmp");
    cmp.generate_r1cs_constraints();

    for (size_t a = 0; a < 1ul<<n; ++a)
    {
        for (size_t b = 0; b < 1ul<<n; ++b)
        {
            pb.val(A) = FieldT(a);
            pb.val(B) = FieldT(b);

            cmp.generate_r1cs_witness();

// #ifdef DEBUG
            print!("positive test for {} < {}\n", a, b);
//#endif
            assert!(pb.val(less) == (a < b ? FieldT::one() : FieldT::zero()));
            assert!(pb.val(less_or_eq) == (a <= b ? FieldT::one() : FieldT::zero()));
            assert!(pb.is_satisfied());
        }
    }

    ffec::print_time("comparison tests successful");
}

template<typename FieldT>
void inner_product_gadget<FieldT>::generate_r1cs_constraints()
{
    /*
      S_i = \sum_{k=0}^{i+1} A[i] * B[i]
      S[0] = A[0] * B[0]
      S[i+1] - S[i] = A[i] * B[i]
    */
    for (size_t i = 0; i < A.size(); ++i)
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(A[i], B[i],
                                    (i == A.size()-1 ? result : S[i]) + (i == 0 ? 0 * ONE : -S[i-1])),
            FMT(self.annotation_prefix, " S_{}", i));
    }
}

template<typename FieldT>
void inner_product_gadget<FieldT>::generate_r1cs_witness()
{
    FieldT total = FieldT::zero();
    for (size_t i = 0; i < A.size(); ++i)
    {
        A[i].evaluate(self.pb);
        B[i].evaluate(self.pb);

        total += self.pb.lc_val(A[i]) * self.pb.lc_val(B[i]);
        self.pb.val(i == A.size()-1 ? result : S[i]) = total;
    }
}

template<typename FieldT>
void test_inner_product_gadget(const size_t n)
{
    print!("testing inner_product_gadget on all {} bit strings\n", n);

    protoboard<FieldT> pb;
    pb_variable_array<FieldT> A;
    A.allocate(pb, n, "A");
    pb_variable_array<FieldT> B;
    B.allocate(pb, n, "B");

    pb_variable<FieldT> result;
    result.allocate(pb, "result");

    inner_product_gadget<FieldT> g(pb, A, B, result, "g");
    g.generate_r1cs_constraints();

    for (size_t i = 0; i < 1ul<<n; ++i)
    {
        for (size_t j = 0; j < 1ul<<n; ++j)
        {
            size_t correct = 0;
            for (size_t k = 0; k < n; ++k)
            {
                pb.val(A[k]) = (i & (1ul<<k) ? FieldT::one() : FieldT::zero());
                pb.val(B[k]) = (j & (1ul<<k) ? FieldT::one() : FieldT::zero());
                correct += ((i & (1ul<<k)) && (j & (1ul<<k)) ? 1 : 0);
            }

            g.generate_r1cs_witness();
// #ifdef DEBUG
            print!("positive test for ({}, {})\n", i, j);
//#endif
            assert!(pb.val(result) == FieldT(correct));
            assert!(pb.is_satisfied());

// #ifdef DEBUG
            print!("negative test for ({}, {})\n", i, j);
//#endif
            pb.val(result) = FieldT(100*n+19);
            assert!(!pb.is_satisfied());
        }
    }

    ffec::print_time("inner_product_gadget tests successful");
}

template<typename FieldT>
void loose_multiplexing_gadget<FieldT>::generate_r1cs_constraints()
{
    /* \alpha_i (index - i) = 0 */
    for (size_t i = 0; i < arr.size(); ++i)
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(alpha[i], index - i, 0),
            FMT(self.annotation_prefix, " alpha_{}", i));
    }

    /* 1 * (\sum \alpha_i) = success_flag */
    linear_combination<FieldT> a, b, c;
    a.add_term(ONE);
    for (size_t i = 0; i < arr.size(); ++i)
    {
        b.add_term(alpha[i]);
    }
    c.add_term(success_flag);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), FMT(self.annotation_prefix, " main_constraint"));

    /* now success_flag is constrained to either 0 (if index is out of
       range) or \alpha_i. constrain it and \alpha_i to zero */
    generate_boolean_r1cs_constraint<FieldT>(self.pb, success_flag, FMT(self.annotation_prefix, " success_flag"));

    /* compute result */
    compute_result->generate_r1cs_constraints();
}

template<typename FieldT>
void loose_multiplexing_gadget<FieldT>::generate_r1cs_witness()
{
    /* assumes that idx can be fit in ulong; true for our purposes for now */
    const ffec::bigint<FieldT::num_limbs> valint = self.pb.val(index).as_bigint();
    unsigned long idx = valint.as_ulong();
    const ffec::bigint<FieldT::num_limbs> arrsize(arr.size());

    if (idx >= arr.size() || mpn_cmp(valint.data, arrsize.data, FieldT::num_limbs) >= 0)
    {
        for (size_t i = 0; i < arr.size(); ++i)
        {
            self.pb.val(alpha[i]) = FieldT::zero();
        }

        self.pb.val(success_flag) = FieldT::zero();
    }
    else
    {
        for (size_t i = 0; i < arr.size(); ++i)
        {
            self.pb.val(alpha[i]) = (i == idx ? FieldT::one() : FieldT::zero());
        }

        self.pb.val(success_flag) = FieldT::one();
    }

    compute_result->generate_r1cs_witness();
}

template<typename FieldT>
void test_loose_multiplexing_gadget(const size_t n)
{
    print!("testing loose_multiplexing_gadget on 2**{} pb_variable<FieldT> array inputs\n", n);
    protoboard<FieldT> pb;

    pb_variable_array<FieldT> arr;
    arr.allocate(pb, 1ul<<n, "arr");
    pb_variable<FieldT> index, result, success_flag;
    index.allocate(pb, "index");
    result.allocate(pb, "result");
    success_flag.allocate(pb, "success_flag");

    loose_multiplexing_gadget<FieldT> g(pb, arr, index, result, success_flag, "g");
    g.generate_r1cs_constraints();

    for (size_t i = 0; i < 1ul<<n; ++i)
    {
        pb.val(arr[i]) = FieldT((19*i) % (1ul<<n));
    }

    for (int idx = -1; idx <= (int)(1ul<<n); ++idx)
    {
        pb.val(index) = FieldT(idx);
        g.generate_r1cs_witness();

        if (0 <= idx && idx <= (int)(1ul<<n) - 1)
        {
            print!("demuxing element %d (in bounds)\n", idx);
            assert!(pb.val(result) == FieldT((19*idx) % (1ul<<n)));
            assert!(pb.val(success_flag) == FieldT::one());
            assert!(pb.is_satisfied());
            pb.val(result) -= FieldT::one();
            assert!(!pb.is_satisfied());
        }
        else
        {
            print!("demuxing element %d (out of bounds)\n", idx);
            assert!(pb.val(success_flag) == FieldT::zero());
            assert!(pb.is_satisfied());
            pb.val(success_flag) = FieldT::one();
            assert!(!pb.is_satisfied());
        }
    }
    print!("loose_multiplexing_gadget tests successful\n");
}

template<typename FieldT, typename VarT>
void create_linear_combination_constraints(protoboard<FieldT> &pb,
                                           const std::vector<FieldT> &base,
                                           const std::vector<std::pair<VarT, FieldT> > &v,
                                           const VarT &target,
                                           const std::string &annotation_prefix)
{
    for (size_t i = 0; i < base.size(); ++i)
    {
        linear_combination<FieldT> a, b, c;

        a.add_term(ONE);
        b.add_term(ONE, base[i]);

        for (auto &p : v)
        {
            b.add_term(p.first.all_vars[i], p.second);
        }

        c.add_term(target.all_vars[i]);

        pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), FMT(annotation_prefix, " linear_combination_{}", i));
    }
}

template<typename FieldT, typename VarT>
void create_linear_combination_witness(protoboard<FieldT> &pb,
                                       const std::vector<FieldT> &base,
                                       const std::vector<std::pair<VarT, FieldT> > &v,
                                       const VarT &target)
{
    for (size_t i = 0; i < base.size(); ++i)
    {
        pb.val(target.all_vars[i]) = base[i];

        for (auto &p : v)
        {
            pb.val(target.all_vars[i]) += p.second * pb.val(p.first.all_vars[i]);
        }
    }
}


//#endif // BASIC_GADGETS_TCC_
