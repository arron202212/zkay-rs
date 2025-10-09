/** @file
 *****************************************************************************

 Declaration of interfaces for Fp2 gadgets.

 The gadgets verify field arithmetic in Fp2 = Fp[U]/(U^2-non_residue),
 where non_residue is in Fp.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_GADGETS_HPP_
// #define FP2_GADGETS_HPP_

use  <memory>

use libsnark/gadgetlib1/gadget;



/**
 * Gadget that represents an Fp2 variable.
 */
template<typename Fp2T>
class Fp2_variable : public gadget<typename Fp2T::my_Fp> {
public:
    type typename Fp2T::my_Fp FieldT;

    pb_linear_combination<FieldT> c0;
    pb_linear_combination<FieldT> c1;

    pb_linear_combination_array<FieldT> all_vars;

    Fp2_variable(protoboard<FieldT> &pb,
                 const std::string &annotation_prefix);
    Fp2_variable(protoboard<FieldT> &pb,
                 const Fp2T &el,
                 const std::string &annotation_prefix);
    Fp2_variable(protoboard<FieldT> &pb,
                 const Fp2T &el,
                 const pb_linear_combination<FieldT> &coeff,
                 const std::string &annotation_prefix);
    Fp2_variable(protoboard<FieldT> &pb,
                 const pb_linear_combination<FieldT> &c0,
                 const pb_linear_combination<FieldT> &c1,
                 const std::string &annotation_prefix);

    void generate_r1cs_equals_const_constraints(const Fp2T &el);
    void generate_r1cs_witness(const Fp2T &el);
    Fp2T get_element();

    Fp2_variable<Fp2T> operator*(const FieldT &coeff) const;
    Fp2_variable<Fp2T> operator+(const Fp2_variable<Fp2T> &other) const;
    Fp2_variable<Fp2T> operator+(const Fp2T &other) const;
    Fp2_variable<Fp2T> mul_by_X() const;
    void evaluate() const;
    bool is_constant() const;

    static size_t size_in_bits();
    static size_t num_variables();
};

/**
 * Gadget that creates constraints for Fp2 by Fp2 multiplication.
 */
template<typename Fp2T>
class Fp2_mul_gadget : public gadget<typename Fp2T::my_Fp> {
public:
    type typename Fp2T::my_Fp FieldT;

    Fp2_variable<Fp2T> A;
    Fp2_variable<Fp2T> B;
    Fp2_variable<Fp2T> result;

    pb_variable<FieldT> v1;

    Fp2_mul_gadget(protoboard<FieldT> &pb,
                   const Fp2_variable<Fp2T> &A,
                   const Fp2_variable<Fp2T> &B,
                   const Fp2_variable<Fp2T> &result,
                   const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that creates constraints for Fp2 multiplication by a linear combination.
 */
template<typename Fp2T>
class Fp2_mul_by_lc_gadget : public gadget<typename Fp2T::my_Fp> {
public:
    type typename Fp2T::my_Fp FieldT;

    Fp2_variable<Fp2T> A;
    pb_linear_combination<FieldT> lc;
    Fp2_variable<Fp2T> result;

    Fp2_mul_by_lc_gadget(protoboard<FieldT> &pb,
                         const Fp2_variable<Fp2T> &A,
                         const pb_linear_combination<FieldT> &lc,
                         const Fp2_variable<Fp2T> &result,
                         const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that creates constraints for Fp2 squaring.
 */
template<typename Fp2T>
class Fp2_sqr_gadget : public gadget<typename Fp2T::my_Fp> {
public:
    type typename Fp2T::my_Fp FieldT;

    Fp2_variable<Fp2T> A;
    Fp2_variable<Fp2T> result;

    Fp2_sqr_gadget(protoboard<FieldT> &pb,
                   const Fp2_variable<Fp2T> &A,
                   const Fp2_variable<Fp2T> &result,
                   const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use libsnark/gadgetlib1/gadgets/fields/fp2_gadgets;

//#endif // FP2_GADGETS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for Fp2 gadgets.

 See fp2_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FP2_GADGETS_TCC_
// #define FP2_GADGETS_TCC_



template<typename Fp2T>
Fp2_variable<Fp2T>::Fp2_variable(protoboard<FieldT> &pb,
                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    pb_variable<FieldT> c0_var, c1_var;
    c0_var.allocate(pb, FMT(annotation_prefix, " c0"));
    c1_var.allocate(pb, FMT(annotation_prefix, " c1"));

    c0 = pb_linear_combination<FieldT>(c0_var);
    c1 = pb_linear_combination<FieldT>(c1_var);

    all_vars.push(c0);
    all_vars.push(c1);
}

template<typename Fp2T>
Fp2_variable<Fp2T>::Fp2_variable(protoboard<FieldT> &pb,
                                 const Fp2T &el,
                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    c0.assign(pb, el.c0);
    c1.assign(pb, el.c1);

    c0.evaluate(pb);
    c1.evaluate(pb);

    all_vars.push(c0);
    all_vars.push(c1);
}

template<typename Fp2T>
Fp2_variable<Fp2T>::Fp2_variable(protoboard<FieldT> &pb,
                                 const Fp2T &el,
                                 const pb_linear_combination<FieldT> &coeff,
                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    c0.assign(pb, el.c0 * coeff);
    c1.assign(pb, el.c1 * coeff);

    all_vars.push(c0);
    all_vars.push(c1);
}

template<typename Fp2T>
Fp2_variable<Fp2T>::Fp2_variable(protoboard<FieldT> &pb,
                                 const pb_linear_combination<FieldT> &c0,
                                 const pb_linear_combination<FieldT> &c1,
                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), c0(c0), c1(c1)
{
    all_vars.push(c0);
    all_vars.push(c1);
}

template<typename Fp2T>
void Fp2_variable<Fp2T>::generate_r1cs_equals_const_constraints(const Fp2T &el)
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, el.c0, c0),
                                 FMT(self.annotation_prefix, " c0"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, el.c1, c1),
                                 FMT(self.annotation_prefix, " c1"));
}

template<typename Fp2T>
void Fp2_variable<Fp2T>::generate_r1cs_witness(const Fp2T &el)
{
    self.pb.lc_val(c0) = el.c0;
    self.pb.lc_val(c1) = el.c1;
}

template<typename Fp2T>
Fp2T Fp2_variable<Fp2T>::get_element()
{
    Fp2T el;
    el.c0 = self.pb.lc_val(c0);
    el.c1 = self.pb.lc_val(c1);
    return el;
}

template<typename Fp2T>
Fp2_variable<Fp2T> Fp2_variable<Fp2T>::operator*(const FieldT &coeff) const
{
    pb_linear_combination<FieldT> new_c0, new_c1;
    new_c0.assign(self.pb, self.c0 * coeff);
    new_c1.assign(self.pb, self.c1 * coeff);
    return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator*"));
}

template<typename Fp2T>
Fp2_variable<Fp2T> Fp2_variable<Fp2T>::operator+(const Fp2_variable<Fp2T> &other) const
{
    pb_linear_combination<FieldT> new_c0, new_c1;
    new_c0.assign(self.pb, self.c0 + other.c0);
    new_c1.assign(self.pb, self.c1 + other.c1);
    return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator+"));
}

template<typename Fp2T>
Fp2_variable<Fp2T> Fp2_variable<Fp2T>::operator+(const Fp2T &other) const
{
    pb_linear_combination<FieldT> new_c0, new_c1;
    new_c0.assign(self.pb, self.c0 + other.c0);
    new_c1.assign(self.pb, self.c1 + other.c1);
    return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " operator+"));
}

template<typename Fp2T>
Fp2_variable<Fp2T> Fp2_variable<Fp2T>::mul_by_X() const
{
    pb_linear_combination<FieldT> new_c0, new_c1;
    new_c0.assign(self.pb, self.c1 * Fp2T::non_residue);
    new_c1.assign(self.pb, self.c0);
    return Fp2_variable<Fp2T>(self.pb, new_c0, new_c1, FMT(self.annotation_prefix, " mul_by_X"));
}

template<typename Fp2T>
void Fp2_variable<Fp2T>::evaluate() const
{
    c0.evaluate(self.pb);
    c1.evaluate(self.pb);
}

template<typename Fp2T>
bool Fp2_variable<Fp2T>::is_constant() const
{
    return (c0.is_constant() && c1.is_constant());
}

template<typename Fp2T>
size_t Fp2_variable<Fp2T>::size_in_bits()
{
    return 2 * FieldT::size_in_bits();
}

template<typename Fp2T>
size_t Fp2_variable<Fp2T>::num_variables()
{
    return 2;
}

template<typename Fp2T>
Fp2_mul_gadget<Fp2T>::Fp2_mul_gadget(protoboard<FieldT> &pb,
                                     const Fp2_variable<Fp2T> &A,
                                     const Fp2_variable<Fp2T> &B,
                                     const Fp2_variable<Fp2T> &result,
                                     const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), A(A), B(B), result(result)
{
    v1.allocate(pb, FMT(annotation_prefix, " v1"));
}

template<typename Fp2T>
void Fp2_mul_gadget<Fp2T>::generate_r1cs_constraints()
{
/*
    Karatsuba multiplication for Fp2:
        v0 = A.c0 * B.c0
        v1 = A.c1 * B.c1
        result.c0 = v0 + non_residue * v1
        result.c1 = (A.c0 + A.c1) * (B.c0 + B.c1) - v0 - v1

    Enforced with 3 constraints:
        A.c1 * B.c1 = v1
        A.c0 * B.c0 = result.c0 - non_residue * v1
        (A.c0+A.c1)*(B.c0+B.c1) = result.c1 + result.c0 + (1 - non_residue) * v1

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab
*/
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c1, B.c1, v1),
                                 FMT(self.annotation_prefix, " v1"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c0, B.c0, result.c0 + v1 * (-Fp2T::non_residue)),
                                 FMT(self.annotation_prefix, " result.c0"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c0 + A.c1, B.c0 + B.c1,
                                                         result.c1 + result.c0 + v1 * (FieldT::one() - Fp2T::non_residue)),
                                 FMT(self.annotation_prefix, " result.c1"));
}

template<typename Fp2T>
void Fp2_mul_gadget<Fp2T>::generate_r1cs_witness()
{
    const FieldT aA = self.pb.lc_val(A.c0) * self.pb.lc_val(B.c0);
    self.pb.val(v1) = self.pb.lc_val(A.c1) * self.pb.lc_val(B.c1);
    self.pb.lc_val(result.c0) = aA + Fp2T::non_residue * self.pb.val(v1);
    self.pb.lc_val(result.c1) = (self.pb.lc_val(A.c0) + self.pb.lc_val(A.c1)) * (self.pb.lc_val(B.c0) + self.pb.lc_val(B.c1)) - aA - self.pb.lc_val(v1);
}

template<typename Fp2T>
Fp2_mul_by_lc_gadget<Fp2T>::Fp2_mul_by_lc_gadget(protoboard<FieldT> &pb,
                                                 const Fp2_variable<Fp2T> &A,
                                                 const pb_linear_combination<FieldT> &lc,
                                                 const Fp2_variable<Fp2T> &result,
                                                 const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), A(A), lc(lc), result(result)
{
}

template<typename Fp2T>
void Fp2_mul_by_lc_gadget<Fp2T>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c0, lc, result.c0),
                                 FMT(self.annotation_prefix, " result.c0"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c1, lc, result.c1),
                                 FMT(self.annotation_prefix, " result.c1"));
}

template<typename Fp2T>
void Fp2_mul_by_lc_gadget<Fp2T>::generate_r1cs_witness()
{
    self.pb.lc_val(result.c0) = self.pb.lc_val(A.c0) * self.pb.lc_val(lc);
    self.pb.lc_val(result.c1) = self.pb.lc_val(A.c1) * self.pb.lc_val(lc);
}

template<typename Fp2T>
Fp2_sqr_gadget<Fp2T>::Fp2_sqr_gadget(protoboard<FieldT> &pb,
                                     const Fp2_variable<Fp2T> &A,
                                     const Fp2_variable<Fp2T> &result,
                                     const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), A(A), result(result)
{
}

template<typename Fp2T>
void Fp2_sqr_gadget<Fp2T>::generate_r1cs_constraints()
{
/*
    Complex multiplication for Fp2:
        v0 = A.c0 * A.c1
        result.c0 = (A.c0 + A.c1) * (A.c0 + non_residue * A.c1) - (1 + non_residue) * v0
        result.c1 = 2 * v0

    Enforced with 2 constraints:
        (2*A.c0) * A.c1 = result.c1
        (A.c0 + A.c1) * (A.c0 + non_residue * A.c1) = result.c0 + result.c1 * (1 + non_residue)/2

    Reference:
        "Multiplication and Squaring on Pairing-Friendly Fields"
        Devegili, OhEigeartaigh, Scott, Dahab
*/
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(2 * A.c0, A.c1, result.c1),
                                 FMT(self.annotation_prefix, " result.c1"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(A.c0 + A.c1,
                                                         A.c0 + Fp2T::non_residue * A.c1,
                                                         result.c0 + result.c1 * (FieldT::one() + Fp2T::non_residue) * FieldT(2).inverse()),
                                 FMT(self.annotation_prefix, " result.c0"));
}

template<typename Fp2T>
void Fp2_sqr_gadget<Fp2T>::generate_r1cs_witness()
{
    const FieldT a = self.pb.lc_val(A.c0);
    const FieldT b = self.pb.lc_val(A.c1);
    self.pb.lc_val(result.c1) = FieldT(2) * a * b;
    self.pb.lc_val(result.c0) = (a + b) * (a + Fp2T::non_residue * b) - a*b - Fp2T::non_residue * a* b;
}



//#endif // FP2_GADGETS_TCC_
