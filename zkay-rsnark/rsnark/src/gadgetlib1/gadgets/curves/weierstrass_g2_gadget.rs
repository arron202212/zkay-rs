/** @file
 *****************************************************************************

 Declaration of interfaces for G2 gadgets.

 The gadgets verify curve arithmetic in G2 = E'(F) where E'/F^e: y^2 = x^3 + A' * X + B'
 is an elliptic curve over F^e in short Weierstrass form.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G2_GADGET_HPP_
// #define WEIERSTRASS_G2_GADGET_HPP_

use  <memory>

use ffec::algebra::curves::public_params;

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets/pairing/pairing_params;



/**
 * Gadget that represents a G2 variable.
 */
template<typename ppT>
class G2_variable : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    std::shared_ptr<Fqe_variable<ppT> > X;
    std::shared_ptr<Fqe_variable<ppT> > Y;

    pb_linear_combination_array<FieldT> all_vars;

    G2_variable(protoboard<FieldT> &pb,
                const std::string &annotation_prefix);
    G2_variable(protoboard<FieldT> &pb,
                const ffec::G2<other_curve<ppT> > &Q,
                const std::string &annotation_prefix);

    void generate_r1cs_witness(const ffec::G2<other_curve<ppT> > &Q);

    // (See a comment in r1cs_ppzksnark_verifier_gadget.hpp about why
    // we mark this function noinline.) TODO: remove later
    static size_t __attribute__((noinline)) size_in_bits();
    static size_t num_variables();
};

/**
 * Gadget that creates constraints for the validity of a G2 variable.
 */
template<typename ppT>
class G2_checker_gadget : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    G2_variable<ppT> Q;

    std::shared_ptr<Fqe_variable<ppT> > Xsquared;
    std::shared_ptr<Fqe_variable<ppT> > Ysquared;
    std::shared_ptr<Fqe_variable<ppT> > Xsquared_plus_a;
    std::shared_ptr<Fqe_variable<ppT> > Ysquared_minus_b;

    std::shared_ptr<Fqe_sqr_gadget<ppT> > compute_Xsquared;
    std::shared_ptr<Fqe_sqr_gadget<ppT> > compute_Ysquared;
    std::shared_ptr<Fqe_mul_gadget<ppT> > curve_equation;

    G2_checker_gadget(protoboard<FieldT> &pb,
                      const G2_variable<ppT> &Q,
                      const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets/curves/weierstrass_g2_gadget;

//#endif // WEIERSTRASS_G2_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for G2 gadgets.

 See weierstrass_g2_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G2_GADGET_TCC_
// #define WEIERSTRASS_G2_GADGET_TCC_

 use ffec::algebra::scalar_multiplication::wnaf;



template<typename ppT>
G2_variable<ppT>::G2_variable(protoboard<FieldT> &pb,
                              const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    X.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " X")));
    Y.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " Y")));

    all_vars.insert(all_vars.end(), X->all_vars.begin(), X->all_vars.end());
    all_vars.insert(all_vars.end(), Y->all_vars.begin(), Y->all_vars.end());
}

template<typename ppT>
G2_variable<ppT>::G2_variable(protoboard<FieldT> &pb,
                              const ffec::G2<other_curve<ppT> > &Q,
                              const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    ffec::G2<other_curve<ppT> > Q_copy = Q;
    Q_copy.to_affine_coordinates();

    X.reset(new Fqe_variable<ppT>(pb, Q_copy.X(), FMT(annotation_prefix, " X")));
    Y.reset(new Fqe_variable<ppT>(pb, Q_copy.Y(), FMT(annotation_prefix, " Y")));

    all_vars.insert(all_vars.end(), X->all_vars.begin(), X->all_vars.end());
    all_vars.insert(all_vars.end(), Y->all_vars.begin(), Y->all_vars.end());
}

template<typename ppT>
void G2_variable<ppT>::generate_r1cs_witness(const ffec::G2<other_curve<ppT> > &Q)
{
    ffec::G2<other_curve<ppT> > Qcopy = Q;
    Qcopy.to_affine_coordinates();

    X->generate_r1cs_witness(Qcopy.X());
    Y->generate_r1cs_witness(Qcopy.Y());
}

template<typename ppT>
size_t G2_variable<ppT>::size_in_bits()
{
    return 2 * Fqe_variable<ppT>::size_in_bits();
}

template<typename ppT>
size_t G2_variable<ppT>::num_variables()
{
    return 2 * Fqe_variable<ppT>::num_variables();
}

template<typename ppT>
G2_checker_gadget<ppT>::G2_checker_gadget(protoboard<FieldT> &pb,
                                          const G2_variable<ppT> &Q,
                                          const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    Q(Q)
{
    Xsquared.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " Xsquared")));
    Ysquared.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " Ysquared")));

    compute_Xsquared.reset(new Fqe_sqr_gadget<ppT>(pb, *(Q.X), *Xsquared, FMT(annotation_prefix, " compute_Xsquared")));
    compute_Ysquared.reset(new Fqe_sqr_gadget<ppT>(pb, *(Q.Y), *Ysquared, FMT(annotation_prefix, " compute_Ysquared")));

    Xsquared_plus_a.reset(new Fqe_variable<ppT>((*Xsquared) + ffec::G2<other_curve<ppT> >::coeff_a));
    Ysquared_minus_b.reset(new Fqe_variable<ppT>((*Ysquared) + (-ffec::G2<other_curve<ppT> >::coeff_b)));

    curve_equation.reset(new Fqe_mul_gadget<ppT>(pb, *(Q.X), *Xsquared_plus_a, *Ysquared_minus_b, FMT(annotation_prefix, " curve_equation")));
}

template<typename ppT>
void G2_checker_gadget<ppT>::generate_r1cs_constraints()
{
    compute_Xsquared->generate_r1cs_constraints();
    compute_Ysquared->generate_r1cs_constraints();
    curve_equation->generate_r1cs_constraints();
}

template<typename ppT>
void G2_checker_gadget<ppT>::generate_r1cs_witness()
{
    compute_Xsquared->generate_r1cs_witness();
    compute_Ysquared->generate_r1cs_witness();
    Xsquared_plus_a->evaluate();
    curve_equation->generate_r1cs_witness();
}

template<typename ppT>
void test_G2_checker_gadget(const std::string &annotation)
{
    protoboard<ffec::Fr<ppT> > pb;
    G2_variable<ppT> g(pb, "g");
    G2_checker_gadget<ppT> g_check(pb, g, "g_check");
    g_check.generate_r1cs_constraints();

    print!("positive test\n");
    g.generate_r1cs_witness(ffec::G2<other_curve<ppT> >::one());
    g_check.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    print!("negative test\n");
    g.generate_r1cs_witness(ffec::G2<other_curve<ppT> >::zero());
    g_check.generate_r1cs_witness();
    assert!(!pb.is_satisfied());

    print!("number of constraints for G2 checker (Fr is %s)  = {}\n", annotation.c_str(), pb.num_constraints());
}



//#endif // WEIERSTRASS_G2_GADGET_TCC_
