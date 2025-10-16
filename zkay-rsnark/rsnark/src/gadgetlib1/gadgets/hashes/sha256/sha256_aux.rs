/** @file
 *****************************************************************************

 Declaration of interfaces for auxiliary gadgets for the SHA256 gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_AUX_HPP_
// #define SHA256_AUX_HPP_

use crate::gadgetlib1::gadgets/basic_gadgets;



template<typename FieldT>
class lastbits_gadget : public gadget<FieldT> {
public:
    pb_variable<FieldT> X;
    size_t X_bits;
    pb_variable<FieldT> result;
    pb_linear_combination_array<FieldT> result_bits;

    pb_linear_combination_array<FieldT> full_bits;
    std::shared_ptr<packing_gadget<FieldT> > unpack_bits;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;

    lastbits_gadget(protoboard<FieldT> &pb,
                    const pb_variable<FieldT> &X,
                    const size_t X_bits,
                    const pb_variable<FieldT> &result,
                    const pb_linear_combination_array<FieldT> &result_bits,
                    const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
class XOR3_gadget : public gadget<FieldT> {
private:
    pb_variable<FieldT> tmp;
public:
    pb_linear_combination<FieldT> A;
    pb_linear_combination<FieldT> B;
    pb_linear_combination<FieldT> C;
    bool assume_C_is_zero;
    pb_linear_combination<FieldT> out;

    XOR3_gadget(protoboard<FieldT> &pb,
                const pb_linear_combination<FieldT> &A,
                const pb_linear_combination<FieldT> &B,
                const pb_linear_combination<FieldT> &C,
                const bool assume_C_is_zero,
                const pb_linear_combination<FieldT> &out,
                const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
class small_sigma_gadget : public gadget<FieldT> {
private:
    pb_variable_array<FieldT> W;
    pb_variable<FieldT> result;
public:
    pb_variable_array<FieldT> result_bits;
    std::vector<std::shared_ptr<XOR3_gadget<FieldT> > > compute_bits;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;

    small_sigma_gadget(protoboard<FieldT> &pb,
                       const pb_variable_array<FieldT> &W,
                       const pb_variable<FieldT> &result,
                       const size_t rot1,
                       const size_t rot2,
                       const size_t shift,
                       const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
class big_sigma_gadget : public gadget<FieldT> {
private:
    pb_linear_combination_array<FieldT> W;
    pb_variable<FieldT> result;
public:
    pb_variable_array<FieldT> result_bits;
    std::vector<std::shared_ptr<XOR3_gadget<FieldT> > > compute_bits;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;

    big_sigma_gadget(protoboard<FieldT> &pb,
                     const pb_linear_combination_array<FieldT> &W,
                     const pb_variable<FieldT> &result,
                     const size_t rot1,
                     const size_t rot2,
                     const size_t rot3,
                     const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
class choice_gadget : public gadget<FieldT> {
private:
    pb_variable_array<FieldT> result_bits;
public:
    pb_linear_combination_array<FieldT> X;
    pb_linear_combination_array<FieldT> Y;
    pb_linear_combination_array<FieldT> Z;
    pb_variable<FieldT> result;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;

    choice_gadget(protoboard<FieldT> &pb,
                  const pb_linear_combination_array<FieldT> &X,
                  const pb_linear_combination_array<FieldT> &Y,
                  const pb_linear_combination_array<FieldT> &Z,
                  const pb_variable<FieldT> &result, const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
class majority_gadget : public gadget<FieldT> {
private:
    pb_variable_array<FieldT> result_bits;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;
public:
    pb_linear_combination_array<FieldT> X;
    pb_linear_combination_array<FieldT> Y;
    pb_linear_combination_array<FieldT> Z;
    pb_variable<FieldT> result;

    majority_gadget(protoboard<FieldT> &pb,
                    const pb_linear_combination_array<FieldT> &X,
                    const pb_linear_combination_array<FieldT> &Y,
                    const pb_linear_combination_array<FieldT> &Z,
                    const pb_variable<FieldT> &result,
                    const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets::hashes::sha256/sha256_aux;

//#endif // SHA256_AUX_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for auxiliary gadgets for the SHA256 gadget.

 See sha256_aux.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SHA256_AUX_TCC_
// #define SHA256_AUX_TCC_



template<typename FieldT>
lastbits_gadget<FieldT>::lastbits_gadget(protoboard<FieldT> &pb,
                                         const pb_variable<FieldT> &X,
                                         const size_t X_bits,
                                         const pb_variable<FieldT> &result,
                                         const pb_linear_combination_array<FieldT> &result_bits,
                                         const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    X(X),
    X_bits(X_bits),
    result(result),
    result_bits(result_bits)
{
    full_bits = result_bits;
    for i in result_bits.size()..X_bits
    {
        pb_variable<FieldT> full_bits_overflow;
        full_bits_overflow.allocate(pb, FMT(self.annotation_prefix, " full_bits_{}", i));
        full_bits.push(full_bits_overflow);
    }

    unpack_bits.reset(new packing_gadget<FieldT>(pb, full_bits, X, FMT(self.annotation_prefix, " unpack_bits")));
    pack_result.reset(new packing_gadget<FieldT>(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
}

template<typename FieldT>
void lastbits_gadget<FieldT>::generate_r1cs_constraints()
{
    unpack_bits->generate_r1cs_constraints(true);
    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void lastbits_gadget<FieldT>::generate_r1cs_witness()
{
    unpack_bits->generate_r1cs_witness_from_packed();
    pack_result->generate_r1cs_witness_from_bits();
}

template<typename FieldT>
XOR3_gadget<FieldT>::XOR3_gadget(protoboard<FieldT> &pb,
                                 const pb_linear_combination<FieldT> &A,
                                 const pb_linear_combination<FieldT> &B,
                                 const pb_linear_combination<FieldT> &C,
                                 const bool assume_C_is_zero,
                                 const pb_linear_combination<FieldT> &out,
                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    A(A),
    B(B),
    C(C),
    assume_C_is_zero(assume_C_is_zero),
    out(out)
{
    if !assume_C_is_zero
    {
        tmp.allocate(pb, FMT(self.annotation_prefix, " tmp"));
    }
}

template<typename FieldT>
void XOR3_gadget<FieldT>::generate_r1cs_constraints()
{
    /*
      tmp = A + B - 2AB i.e. tmp = A xor B
      out = tmp + C - 2tmp C i.e. out = tmp xor C
    */
    if assume_C_is_zero
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(2*A, B, A + B - out), FMT(self.annotation_prefix, " implicit_tmp_equals_out"));
    }
    else
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(2*A, B, A + B - tmp), FMT(self.annotation_prefix, " tmp"));
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(2 * tmp, C, tmp + C - out), FMT(self.annotation_prefix, " out"));
    }
}

template<typename FieldT>
void XOR3_gadget<FieldT>::generate_r1cs_witness()
{
    if assume_C_is_zero
    {
        self.pb.lc_val(out) = self.pb.lc_val(A) + self.pb.lc_val(B) - FieldT(2) * self.pb.lc_val(A) * self.pb.lc_val(B);
    }
    else
    {
        self.pb.val(tmp) = self.pb.lc_val(A) + self.pb.lc_val(B) - FieldT(2) * self.pb.lc_val(A) * self.pb.lc_val(B);
        self.pb.lc_val(out) = self.pb.val(tmp) + self.pb.lc_val(C) - FieldT(2) * self.pb.val(tmp) * self.pb.lc_val(C);
    }
}

// #define SHA256_GADGET_ROTR(A, i, k) A[((i)+(k)) % 32]

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
small_sigma_gadget<FieldT>::small_sigma_gadget(protoboard<FieldT> &pb,
                                               const pb_variable_array<FieldT> &W,
                                               const pb_variable<FieldT> &result,
                                               const size_t rot1,
                                               const size_t rot2,
                                               const size_t shift,
                                               const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    W(W),
    result(result)
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    compute_bits.resize(32);
    for i in 0..32
    {
        compute_bits[i].reset(new XOR3_gadget<FieldT>(pb, SHA256_GADGET_ROTR(W, i, rot1), SHA256_GADGET_ROTR(W, i, rot2),
                                              (i + shift < 32 ? W[i+shift] : ONE),
                                              (i + shift >= 32), result_bits[i],
                                              FMT(self.annotation_prefix, " compute_bits_{}", i)));
    }
    pack_result.reset(new packing_gadget<FieldT>(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
}

template<typename FieldT>
void small_sigma_gadget<FieldT>::generate_r1cs_constraints()
{
    for i in 0..32
    {
        compute_bits[i]->generate_r1cs_constraints();
    }

    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void small_sigma_gadget<FieldT>::generate_r1cs_witness()
{
    for i in 0..32
    {
        compute_bits[i]->generate_r1cs_witness();
    }

    pack_result->generate_r1cs_witness_from_bits();
}

template<typename FieldT>
big_sigma_gadget<FieldT>::big_sigma_gadget(protoboard<FieldT> &pb,
                                           const pb_linear_combination_array<FieldT> &W,
                                           const pb_variable<FieldT> &result,
                                           const size_t rot1,
                                           const size_t rot2,
                                           const size_t rot3,
                                           const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    W(W),
    result(result)
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    compute_bits.resize(32);
    for i in 0..32
    {
        compute_bits[i].reset(new XOR3_gadget<FieldT>(pb, SHA256_GADGET_ROTR(W, i, rot1), SHA256_GADGET_ROTR(W, i, rot2), SHA256_GADGET_ROTR(W, i, rot3), false, result_bits[i],
                                                      FMT(self.annotation_prefix, " compute_bits_{}", i)));
    }

    pack_result.reset(new packing_gadget<FieldT>(pb, result_bits, result, FMT(self.annotation_prefix, " pack_result")));
}

template<typename FieldT>
void big_sigma_gadget<FieldT>::generate_r1cs_constraints()
{
    for i in 0..32
    {
        compute_bits[i]->generate_r1cs_constraints();
    }

    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void big_sigma_gadget<FieldT>::generate_r1cs_witness()
{
    for i in 0..32
    {
        compute_bits[i]->generate_r1cs_witness();
    }

    pack_result->generate_r1cs_witness_from_bits();
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
choice_gadget<FieldT>::choice_gadget(protoboard<FieldT> &pb,
                                     const pb_linear_combination_array<FieldT> &X,
                                     const pb_linear_combination_array<FieldT> &Y,
                                     const pb_linear_combination_array<FieldT> &Z,
                                     const pb_variable<FieldT> &result, const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    X(X),
    Y(Y),
    Z(Z),
    result(result)
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    pack_result.reset(new packing_gadget<FieldT>(pb, result_bits, result, FMT(self.annotation_prefix, " result")));
}

template<typename FieldT>
void choice_gadget<FieldT>::generate_r1cs_constraints()
{
    for i in 0..32
    {
        /*
          result = x * y + (1-x) * z
          result - z = x * (y - z)
        */
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(X[i], Y[i] - Z[i], result_bits[i] - Z[i]), FMT(self.annotation_prefix, " result_bits_{}", i));
    }
    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void choice_gadget<FieldT>::generate_r1cs_witness()
{
    for i in 0..32
    {
        self.pb.val(result_bits[i]) = self.pb.lc_val(X[i]) * self.pb.lc_val(Y[i]) + (FieldT::one() - self.pb.lc_val(X[i])) * self.pb.lc_val(Z[i]);
    }
    pack_result->generate_r1cs_witness_from_bits();
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
template<typename FieldT>
majority_gadget<FieldT>::majority_gadget(protoboard<FieldT> &pb,
                                         const pb_linear_combination_array<FieldT> &X,
                                         const pb_linear_combination_array<FieldT> &Y,
                                         const pb_linear_combination_array<FieldT> &Z,
                                         const pb_variable<FieldT> &result,
                                         const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    X(X),
    Y(Y),
    Z(Z),
    result(result)
{
    result_bits.allocate(pb, 32, FMT(self.annotation_prefix, " result_bits"));
    pack_result.reset(new packing_gadget<FieldT>(pb, result_bits, result, FMT(self.annotation_prefix, " result")));
}

template<typename FieldT>
void majority_gadget<FieldT>::generate_r1cs_constraints()
{
    for i in 0..32
    {
        /*
          2*result + aux = x + y + z
          x, y, z, aux -- bits
          aux = x + y + z - 2*result
        */
        generate_boolean_r1cs_constraint<FieldT>(self.pb, result_bits[i], FMT(self.annotation_prefix, " result_{}", i));
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(X[i] + Y[i] + Z[i] - 2 * result_bits[i],
                                                             1 - (X[i] + Y[i] + Z[i] -  2 * result_bits[i]),
                                                             0),
                                     FMT(self.annotation_prefix, " result_bits_{}", i));
    }
    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void majority_gadget<FieldT>::generate_r1cs_witness()
{
    for i in 0..32
    {
        const long v = (self.pb.lc_val(X[i]) + self.pb.lc_val(Y[i]) + self.pb.lc_val(Z[i])).as_ulong();
        self.pb.val(result_bits[i]) = FieldT(v / 2);
    }

    pack_result->generate_r1cs_witness_from_bits();
}



//#endif // SHA256_AUX_TCC_
