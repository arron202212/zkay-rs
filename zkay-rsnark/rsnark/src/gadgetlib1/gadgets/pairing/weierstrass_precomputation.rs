/** @file
 *****************************************************************************

 Declaration of interfaces for pairing precomputation gadgets.

 The gadgets verify correct precomputation of values for the G1 and G2 variables.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_PRECOMPUTATION_HPP_
// #define WEIERSTRASS_PRECOMPUTATION_HPP_

use  <memory>

use ffec::algebra::curves::mnt/mnt4/mnt4_init;
use ffec::algebra::curves::mnt/mnt6/mnt6_init;

use libsnark/gadgetlib1/gadgets/curves/weierstrass_g1_gadget;
use libsnark/gadgetlib1/gadgets/curves/weierstrass_g2_gadget;
use libsnark/gadgetlib1/gadgets/pairing/pairing_params;



/**************************** G1 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */
template<typename ppT>
class G1_precomputation {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    std::shared_ptr<G1_variable<ppT> > P;
    std::shared_ptr<Fqe_variable<ppT> > PY_twist_squared;

    G1_precomputation();
    G1_precomputation(protoboard<FieldT> &pb,
                      const ffec::G1<other_curve<ppT> > &P,
                      const std::string &annotation_prefix);
};

/**
 * Gadget that verifies correct precomputation of the G1 variable.
 */
template<typename ppT>
class precompute_G1_gadget : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    G1_precomputation<ppT> &precomp; // must be a reference.

    /* two possible pre-computations one for mnt4 and one for mnt6 */
    template<typename FieldT>
    precompute_G1_gadget(protoboard<FieldT> &pb,
                         const G1_variable<ppT> &P,
                         G1_precomputation<ppT> &precomp, // will allocate this inside
                         const std::string &annotation_prefix,
                         const typename std::enable_if<ffec::Fqk<other_curve<ppT> >::extension_degree() == 4, FieldT>::type& = FieldT()) :
            gadget<FieldT>(pb, annotation_prefix),
            precomp(precomp)
    {
        pb_linear_combination<FieldT> c0, c1;
        c0.assign(pb, P.Y * ((ffec::mnt4_twist).squared().c0));
        c1.assign(pb, P.Y * ((ffec::mnt4_twist).squared().c1));

        precomp.P.reset(new G1_variable<ppT>(P));
        precomp.PY_twist_squared.reset(new Fqe_variable<ppT>(pb, c0, c1, FMT(annotation_prefix, " PY_twist_squared")));
    }

    template<typename FieldT>
    precompute_G1_gadget(protoboard<FieldT> &pb,
                         const G1_variable<ppT> &P,
                         G1_precomputation<ppT> &precomp, // will allocate this inside
                         const std::string &annotation_prefix,
                         const typename std::enable_if<ffec::Fqk<other_curve<ppT> >::extension_degree() == 6, FieldT>::type& = FieldT()) :
        gadget<FieldT>(pb, annotation_prefix),
            precomp(precomp)
    {
        pb_linear_combination<FieldT> c0, c1, c2;
        c0.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c0));
        c1.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c1));
        c2.assign(pb, P.Y * ((ffec::mnt6_twist).squared().c2));

        precomp.P.reset(new G1_variable<ppT>(P));
        precomp.PY_twist_squared.reset(new Fqe_variable<ppT>(pb, c0, c1, c2, FMT(annotation_prefix, " PY_twist_squared")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename ppT>
void test_G1_variable_precomp(const std::string &annotation);


/**************************** G2 Precomputation ******************************/

/**
 * Not a gadget. It only holds values.
 */
template<typename ppT>
class precompute_G2_gadget_coeffs {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    std::shared_ptr<Fqe_variable<ppT> > RX;
    std::shared_ptr<Fqe_variable<ppT> > RY;
    std::shared_ptr<Fqe_variable<ppT> > gamma;
    std::shared_ptr<Fqe_variable<ppT> > gamma_X;

    precompute_G2_gadget_coeffs();
    precompute_G2_gadget_coeffs(protoboard<FieldT> &pb,
                                const std::string &annotation_prefix);
    precompute_G2_gadget_coeffs(protoboard<FieldT> &pb,
                                const G2_variable<ppT> &Q,
                                const std::string &annotation_prefix);
};

/**
 * Not a gadget. It only holds values.
 */
template<typename ppT>
class G2_precomputation {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    std::shared_ptr<G2_variable<ppT> > Q;

    std::vector<std::shared_ptr<precompute_G2_gadget_coeffs<ppT> > > coeffs;

    G2_precomputation();
    G2_precomputation(protoboard<FieldT> &pb,
                      const ffec::G2<other_curve<ppT> > &Q_val,
                      const std::string &annotation_prefix);
};

/**
 * Technical note:
 *
 * QX and QY -- X and Y coordinates of Q
 *
 * initialization:
 * coeffs[0].RX = QX
 * coeffs[0].RY = QY
 *
 * G2_precompute_doubling_step relates coeffs[i] and coeffs[i+1] as follows
 *
 * coeffs[i]
 * gamma = (3 * RX^2 + twist_coeff_a) * (2*RY).inverse()
 * gamma_X = gamma * RX
 *
 * coeffs[i+1]
 * RX = prev_gamma^2 - (2*prev_RX)
 * RY = prev_gamma * (prev_RX - RX) - prev_RY
 */
template<typename ppT>
class precompute_G2_gadget_doubling_step : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    precompute_G2_gadget_coeffs<ppT> cur;
    precompute_G2_gadget_coeffs<ppT> next;

    std::shared_ptr<Fqe_variable<ppT> > RXsquared;
    std::shared_ptr<Fqe_sqr_gadget<ppT> > compute_RXsquared;
    std::shared_ptr<Fqe_variable<ppT> > three_RXsquared_plus_a;
    std::shared_ptr<Fqe_variable<ppT> > two_RY;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_gamma;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_gamma_X;

    std::shared_ptr<Fqe_variable<ppT> > next_RX_plus_two_RX;
    std::shared_ptr<Fqe_sqr_gadget<ppT> > compute_next_RX;

    std::shared_ptr<Fqe_variable<ppT> > RX_minus_next_RX;
    std::shared_ptr<Fqe_variable<ppT> > RY_plus_next_RY;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_next_RY;

    precompute_G2_gadget_doubling_step(protoboard<FieldT> &pb,
                                       const precompute_G2_gadget_coeffs<ppT> &cur,
                                       const precompute_G2_gadget_coeffs<ppT> &next,
                                       const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Technical note:
 *
 * G2_precompute_addition_step relates coeffs[i] and coeffs[i+1] as follows
 *
 * coeffs[i]
 * gamma = (RY - QY) * (RX - QX).inverse()
 * gamma_X = gamma * QX
 *
 * coeffs[i+1]
 * RX = prev_gamma^2 + (prev_RX + QX)
 * RY = prev_gamma * (prev_RX - RX) - prev_RY
 *
 * (where prev_ in [i+1] refer to things from [i])
 *
 * If invert_Q is set to true: use -QY in place of QY everywhere above.
 */
template<typename ppT>
class precompute_G2_gadget_addition_step : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    bool invert_Q;
    precompute_G2_gadget_coeffs<ppT> cur;
    precompute_G2_gadget_coeffs<ppT> next;
    G2_variable<ppT> Q;

    std::shared_ptr<Fqe_variable<ppT> > RY_minus_QY;
    std::shared_ptr<Fqe_variable<ppT> > RX_minus_QX;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_gamma;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_gamma_X;

    std::shared_ptr<Fqe_variable<ppT> > next_RX_plus_RX_plus_QX;
    std::shared_ptr<Fqe_sqr_gadget<ppT> > compute_next_RX;

    std::shared_ptr<Fqe_variable<ppT> > RX_minus_next_RX;
    std::shared_ptr<Fqe_variable<ppT> > RY_plus_next_RY;
    std::shared_ptr<Fqe_mul_gadget<ppT> > compute_next_RY;

    precompute_G2_gadget_addition_step(protoboard<FieldT> &pb,
                                       const bool invert_Q,
                                       const precompute_G2_gadget_coeffs<ppT> &cur,
                                       const precompute_G2_gadget_coeffs<ppT> &next,
                                       const G2_variable<ppT> &Q,
                                       const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that verifies correct precomputation of the G2 variable.
 */
template<typename ppT>
class precompute_G2_gadget : public gadget<ffec::Fr<ppT> > {
public:
    type ffec::Fr<ppT> FieldT;
    type ffec::Fqe<other_curve<ppT> > FqeT;
    type ffec::Fqk<other_curve<ppT> > FqkT;

    std::vector<std::shared_ptr<precompute_G2_gadget_addition_step<ppT> > > addition_steps;
    std::vector<std::shared_ptr<precompute_G2_gadget_doubling_step<ppT> > > doubling_steps;

    size_t add_count;
    size_t dbl_count;

    G2_precomputation<ppT> &precomp; // important to have a reference here

    precompute_G2_gadget(protoboard<FieldT> &pb,
                         const G2_variable<ppT> &Q,
                         G2_precomputation<ppT> &precomp,  // will allocate this inside
                         const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename ppT>
void test_G2_variable_precomp(const std::string &annotation);



use libsnark/gadgetlib1/gadgets/pairing/weierstrass_precomputation;

//#endif // WEIERSTRASS_PRECOMPUTATION_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for pairing precomputation gadgets.

 See weierstrass_precomputation.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_PRECOMPUTATION_TCC_
// #define WEIERSTRASS_PRECOMPUTATION_TCC_

use  <type_traits>

use libsnark/gadgetlib1/gadgets/pairing/mnt_pairing_params;



template<typename ppT>
G1_precomputation<ppT>::G1_precomputation()
{
    // will be filled in precompute_G1_gadget, so do nothing here
}

template<typename ppT>
G1_precomputation<ppT>::G1_precomputation(protoboard<FieldT> &pb,
                                          const ffec::G1<other_curve<ppT> > &P_val,
                                          const std::string &annotation_prefix)
{
    ffec::G1<other_curve<ppT> > P_val_copy = P_val;
    P_val_copy.to_affine_coordinates();
    P.reset(new G1_variable<ppT>(pb, P_val_copy, FMT(annotation_prefix, " P")));
    PY_twist_squared.reset(new Fqe_variable<ppT>(pb, P_val_copy.Y() * ffec::G2<other_curve<ppT> >::twist.squared(), " PY_twist_squared"));
}

template<typename ppT>
void precompute_G1_gadget<ppT>::generate_r1cs_constraints()
{
    /* the same for neither ppT = mnt4 nor ppT = mnt6 */
}

template<typename ppT>
void precompute_G1_gadget<ppT>::generate_r1cs_witness()
{
    precomp.PY_twist_squared->evaluate(); /* the same for both ppT = mnt4 and ppT = mnt6 */
}

template<typename ppT>
void test_G1_variable_precomp(const std::string &annotation)
{
    protoboard<ffec::Fr<ppT> > pb;
    ffec::G1<other_curve<ppT> > g_val = ffec::Fr<other_curve<ppT> >::random_element() * ffec::G1<other_curve<ppT> >::one();

    G1_variable<ppT> g(pb, "g");
    G1_precomputation<ppT> precomp;
    precompute_G1_gadget<ppT> do_precomp(pb, g, precomp, "do_precomp");
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    G1_precomputation<ppT> const_precomp(pb, g_val, "const_precomp");

    ffec::affine_ate_G1_precomp<other_curve<ppT> > native_precomp = other_curve<ppT>::affine_ate_precompute_G1(g_val);
    assert!(precomp.PY_twist_squared->get_element() == native_precomp.PY_twist_squared);
    assert!(const_precomp.PY_twist_squared->get_element() == native_precomp.PY_twist_squared);

    print!("number of constraints for G1 precomp (Fr is %s)  = {}\n", annotation.c_str(), pb.num_constraints());
}

template<typename ppT>
G2_precomputation<ppT>::G2_precomputation()
{
}

template<typename ppT>
G2_precomputation<ppT>::G2_precomputation(protoboard<FieldT> &pb,
                                          const ffec::G2<other_curve<ppT> > &Q_val,
                                          const std::string &annotation_prefix)
{
    Q.reset(new G2_variable<ppT>(pb, Q_val, FMT(annotation_prefix, " Q")));
    const ffec::affine_ate_G2_precomp<other_curve<ppT> > native_precomp = other_curve<ppT>::affine_ate_precompute_G2(Q_val);

    coeffs.resize(native_precomp.coeffs.size() + 1); // the last precomp remains for convenient programming
    for (size_t i = 0; i < native_precomp.coeffs.size(); ++i)
    {
        coeffs[i].reset(new precompute_G2_gadget_coeffs<ppT>());
        coeffs[i]->RX.reset(new Fqe_variable<ppT>(pb, native_precomp.coeffs[i].old_RX, FMT(annotation_prefix, " RX")));
        coeffs[i]->RY.reset(new Fqe_variable<ppT>(pb, native_precomp.coeffs[i].old_RY, FMT(annotation_prefix, " RY")));
        coeffs[i]->gamma.reset(new Fqe_variable<ppT>(pb, native_precomp.coeffs[i].gamma, FMT(annotation_prefix, " gamma")));
        coeffs[i]->gamma_X.reset(new Fqe_variable<ppT>(pb, native_precomp.coeffs[i].gamma_X, FMT(annotation_prefix, " gamma_X")));
    }
}

template<typename ppT>
precompute_G2_gadget_coeffs<ppT>::precompute_G2_gadget_coeffs()
{
    // we will be filled in precomputed case of precompute_G2_gadget, so do nothing here
}

template<typename ppT>
precompute_G2_gadget_coeffs<ppT>::precompute_G2_gadget_coeffs(protoboard<FieldT> &pb,
                                                              const std::string &annotation_prefix)
{
    RX.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " RX")));
    RY.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " RY")));
    gamma.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " gamma")));
    gamma_X.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " gamma_X")));
}

template<typename ppT>
precompute_G2_gadget_coeffs<ppT>::precompute_G2_gadget_coeffs(protoboard<FieldT> &pb,
                                                              const G2_variable<ppT> &Q,
                                                              const std::string &annotation_prefix)
{
    RX.reset(new Fqe_variable<ppT>(*(Q.X)));
    RY.reset(new Fqe_variable<ppT>(*(Q.Y)));
    gamma.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " gamma")));
    gamma_X.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " gamma_X")));
}

/*
 QX and QY -- X and Y coordinates of Q

 initialization:
 coeffs[0].RX = QX
 coeffs[0].RY = QY

 G2_precompute_doubling_step relates coeffs[i] and coeffs[i+1] as follows

 coeffs[i]
 gamma = (3 * RX^2 + twist_coeff_a) * (2*RY).inverse()
 gamma_X = gamma * RX

 coeffs[i+1]
 RX = prev_gamma^2 - (2*prev_RX)
 RY = prev_gamma * (prev_RX - RX) - prev_RY
 */

template<typename ppT>
precompute_G2_gadget_doubling_step<ppT>::precompute_G2_gadget_doubling_step(protoboard<FieldT> &pb,
                                                                            const precompute_G2_gadget_coeffs<ppT> &cur,
                                                                            const precompute_G2_gadget_coeffs<ppT> &next,
                                                                            const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    cur(cur),
    next(next)
{
    RXsquared.reset(new Fqe_variable<ppT>(pb, FMT(annotation_prefix, " RXsquared")));
    compute_RXsquared.reset(new Fqe_sqr_gadget<ppT>(pb, *(cur.RX), *RXsquared, FMT(annotation_prefix, " compute_RXsquared")));
    three_RXsquared_plus_a.reset(new Fqe_variable<ppT>((*RXsquared) * FieldT(3) + ffec::G2<other_curve<ppT> >::coeff_a));
    two_RY.reset(new Fqe_variable<ppT>(*(cur.RY) * FieldT(2)));

    compute_gamma.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *two_RY, *three_RXsquared_plus_a, FMT(annotation_prefix, " compute_gamma")));
    compute_gamma_X.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *(cur.RX), *(cur.gamma_X), FMT(annotation_prefix, " compute_gamma_X")));

    next_RX_plus_two_RX.reset(new Fqe_variable<ppT>(*(next.RX) + *(cur.RX) * FieldT(2)));
    compute_next_RX.reset(new Fqe_sqr_gadget<ppT>(pb, *(cur.gamma), *next_RX_plus_two_RX, FMT(annotation_prefix, " compute_next_RX")));

    RX_minus_next_RX.reset(new Fqe_variable<ppT>(*(cur.RX) + *(next.RX) * (-FieldT::one())));
    RY_plus_next_RY.reset(new Fqe_variable<ppT>(*(cur.RY) + *(next.RY)));
    compute_next_RY.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *RX_minus_next_RX, *RY_plus_next_RY, FMT(annotation_prefix, " compute_next_RY")));
}

template<typename ppT>
void precompute_G2_gadget_doubling_step<ppT>::generate_r1cs_constraints()
{
    compute_RXsquared->generate_r1cs_constraints();
    compute_gamma->generate_r1cs_constraints();
    compute_gamma_X->generate_r1cs_constraints();
    compute_next_RX->generate_r1cs_constraints();
    compute_next_RY->generate_r1cs_constraints();
}

template<typename ppT>
void precompute_G2_gadget_doubling_step<ppT>::generate_r1cs_witness()
{
    compute_RXsquared->generate_r1cs_witness();
    two_RY->evaluate();
    three_RXsquared_plus_a->evaluate();

    const FqeT three_RXsquared_plus_a_val = three_RXsquared_plus_a->get_element();
    const FqeT two_RY_val = two_RY->get_element();
    const FqeT gamma_val = three_RXsquared_plus_a_val * two_RY_val.inverse();
    cur.gamma->generate_r1cs_witness(gamma_val);

    compute_gamma->generate_r1cs_witness();
    compute_gamma_X->generate_r1cs_witness();

    const FqeT RX_val = cur.RX->get_element();
    const FqeT RY_val = cur.RY->get_element();
    const FqeT next_RX_val = gamma_val.squared() - RX_val - RX_val;
    const FqeT next_RY_val = gamma_val * (RX_val - next_RX_val) - RY_val;

    next.RX->generate_r1cs_witness(next_RX_val);
    next.RY->generate_r1cs_witness(next_RY_val);

    RX_minus_next_RX->evaluate();
    RY_plus_next_RY->evaluate();

    compute_next_RX->generate_r1cs_witness();
    compute_next_RY->generate_r1cs_witness();
}

/*
 G2_precompute_addition_step relates coeffs[i] and coeffs[i+1] as follows

 coeffs[i]
 gamma = (RY - QY) * (RX - QX).inverse()
 gamma_X = gamma * QX

 coeffs[i+1]
 RX = prev_gamma^2 - (prev_RX + QX)
 RY = prev_gamma * (prev_RX - RX) - prev_RY

 (where prev_ in [i+1] refer to things from [i])

 If invert_Q is set to true: use -QY in place of QY everywhere above.
 */
template<typename ppT>
precompute_G2_gadget_addition_step<ppT>::precompute_G2_gadget_addition_step(protoboard<FieldT> &pb,
                                                                            const bool invert_Q,
                                                                            const precompute_G2_gadget_coeffs<ppT> &cur,
                                                                            const precompute_G2_gadget_coeffs<ppT> &next,
                                                                            const G2_variable<ppT> &Q,
                                                                            const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    invert_Q(invert_Q),
    cur(cur),
    next(next),
    Q(Q)
{
    RY_minus_QY.reset(new Fqe_variable<ppT>(*(cur.RY) + *(Q.Y) * (!invert_Q ? -FieldT::one() : FieldT::one())));

    RX_minus_QX.reset(new Fqe_variable<ppT>(*(cur.RX) + *(Q.X) * (-FieldT::one())));
    compute_gamma.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *RX_minus_QX, *RY_minus_QY, FMT(annotation_prefix, " compute_gamma")));
    compute_gamma_X.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *(Q.X), *(cur.gamma_X), FMT(annotation_prefix, " compute_gamma_X")));

    next_RX_plus_RX_plus_QX.reset(new Fqe_variable<ppT>(*(next.RX) + *(cur.RX) + *(Q.X)));
    compute_next_RX.reset(new Fqe_sqr_gadget<ppT>(pb, *(cur.gamma), *next_RX_plus_RX_plus_QX, FMT(annotation_prefix, " compute_next_RX")));

    RX_minus_next_RX.reset(new Fqe_variable<ppT>(*(cur.RX) + *(next.RX) * (-FieldT::one())));
    RY_plus_next_RY.reset(new Fqe_variable<ppT>(*(cur.RY) + *(next.RY)));
    compute_next_RY.reset(new Fqe_mul_gadget<ppT>(pb, *(cur.gamma), *RX_minus_next_RX, *RY_plus_next_RY, FMT(annotation_prefix, " compute_next_RY")));
}

template<typename ppT>
void precompute_G2_gadget_addition_step<ppT>::generate_r1cs_constraints()
{
    compute_gamma->generate_r1cs_constraints();
    compute_gamma_X->generate_r1cs_constraints();
    compute_next_RX->generate_r1cs_constraints();
    compute_next_RY->generate_r1cs_constraints();
}

template<typename ppT>
void precompute_G2_gadget_addition_step<ppT>::generate_r1cs_witness()
{
    RY_minus_QY->evaluate();
    RX_minus_QX->evaluate();

    const FqeT RY_minus_QY_val = RY_minus_QY->get_element();
    const FqeT RX_minus_QX_val = RX_minus_QX->get_element();
    const FqeT gamma_val = RY_minus_QY_val * RX_minus_QX_val.inverse();
    cur.gamma->generate_r1cs_witness(gamma_val);

    compute_gamma->generate_r1cs_witness();
    compute_gamma_X->generate_r1cs_witness();

    const FqeT RX_val = cur.RX->get_element();
    const FqeT RY_val = cur.RY->get_element();
    const FqeT QX_val = Q.X->get_element();
    const FqeT next_RX_val = gamma_val.squared() - RX_val - QX_val;
    const FqeT next_RY_val = gamma_val * (RX_val - next_RX_val) - RY_val;

    next.RX->generate_r1cs_witness(next_RX_val);
    next.RY->generate_r1cs_witness(next_RY_val);

    next_RX_plus_RX_plus_QX->evaluate();
    RX_minus_next_RX->evaluate();
    RY_plus_next_RY->evaluate();

    compute_next_RX->generate_r1cs_witness();
    compute_next_RY->generate_r1cs_witness();
}

template<typename ppT>
precompute_G2_gadget<ppT>::precompute_G2_gadget(protoboard<FieldT> &pb,
                                                const G2_variable<ppT> &Q,
                                                G2_precomputation<ppT> &precomp,  // will allocate this inside
                                                const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    precomp(precomp)
{
    precomp.Q.reset(new G2_variable<ppT>(Q));

    const auto &loop_count = pairing_selector<ppT>::pairing_loop_count;
    size_t coeff_count = 1; // the last RX/RY are unused in Miller loop, but will need to get allocated somehow
    self.add_count = 0;
    self.dbl_count = 0;

    bool found_nonzero = false;
    std::vector<long> NAF = find_wnaf(1, loop_count);
    for (long i = NAF.size()-1; i >= 0; --i)
    {
        if (!found_nonzero)
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        dbl_count+=1;
        coeff_count+=1;

        if (NAF[i] != 0)
        {
            add_count+=1;
            coeff_count+=1;
        }
    }

    precomp.coeffs.resize(coeff_count);
    addition_steps.resize(add_count);
    doubling_steps.resize(dbl_count);

    precomp.coeffs[0].reset(new precompute_G2_gadget_coeffs<ppT>(pb, Q, FMT(annotation_prefix, " coeffs_0")));
    for (size_t i = 1; i < coeff_count; ++i)
    {
        precomp.coeffs[i].reset(new precompute_G2_gadget_coeffs<ppT>(pb, FMT(annotation_prefix, " coeffs_{}", i)));
    }

    size_t add_id = 0;
    size_t dbl_id = 0;
    size_t coeff_id = 0;

    found_nonzero = false;
    for (long i = NAF.size()-1; i >= 0; --i)
    {
        if (!found_nonzero)
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id].reset(new precompute_G2_gadget_doubling_step<ppT>(pb, *(precomp.coeffs[coeff_id]), *(precomp.coeffs[coeff_id+1]),
                                                                                 FMT(annotation_prefix, " doubling_steps_{}", dbl_id)));
        dbl_id+=1;
        coeff_id+=1;

        if (NAF[i] != 0)
        {
            addition_steps[add_id].reset(new precompute_G2_gadget_addition_step<ppT>(pb, NAF[i] < 0, *(precomp.coeffs[coeff_id]), *(precomp.coeffs[coeff_id+1]), Q,
                                                                                     FMT(annotation_prefix, " addition_steps_{}", add_id)));
            add_id+=1;
            coeff_id+=1;
        }
    }
}

template<typename ppT>
void precompute_G2_gadget<ppT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < dbl_count; ++i)
    {
        doubling_steps[i]->generate_r1cs_constraints();
    }

    for (size_t i = 0; i < add_count; ++i)
    {
        addition_steps[i]->generate_r1cs_constraints();
    }
}

template<typename ppT>
void precompute_G2_gadget<ppT>::generate_r1cs_witness()
{
    precomp.coeffs[0]->RX->generate_r1cs_witness(precomp.Q->X->get_element());
    precomp.coeffs[0]->RY->generate_r1cs_witness(precomp.Q->Y->get_element());

    const auto &loop_count = pairing_selector<ppT>::pairing_loop_count;

    size_t add_id = 0;
    size_t dbl_id = 0;

    bool found_nonzero = false;
    std::vector<long> NAF = find_wnaf(1, loop_count);
    for (long i = NAF.size()-1; i >= 0; --i)
    {
        if (!found_nonzero)
        {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        doubling_steps[dbl_id]->generate_r1cs_witness();
        dbl_id+=1;

        if (NAF[i] != 0)
        {
            addition_steps[add_id]->generate_r1cs_witness();
            add_id+=1;
        }
    }
}

template<typename ppT>
void test_G2_variable_precomp(const std::string &annotation)
{
    protoboard<ffec::Fr<ppT> > pb;
    ffec::G2<other_curve<ppT> > g_val = ffec::Fr<other_curve<ppT> >::random_element() * ffec::G2<other_curve<ppT> >::one();

    G2_variable<ppT> g(pb, "g");
    G2_precomputation<ppT> precomp;
    precompute_G2_gadget<ppT> do_precomp(pb, g, precomp, "do_precomp");
    do_precomp.generate_r1cs_constraints();

    g.generate_r1cs_witness(g_val);
    do_precomp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    ffec::affine_ate_G2_precomp<other_curve<ppT> > native_precomp = other_curve<ppT>::affine_ate_precompute_G2(g_val);

    assert!(precomp.coeffs.size() - 1 == native_precomp.coeffs.size()); // the last precomp is unused, but remains for convenient programming
    for (size_t i = 0; i < native_precomp.coeffs.size(); ++i)
    {
        assert!(precomp.coeffs[i]->RX->get_element() == native_precomp.coeffs[i].old_RX);
        assert!(precomp.coeffs[i]->RY->get_element() == native_precomp.coeffs[i].old_RY);
        assert!(precomp.coeffs[i]->gamma->get_element() == native_precomp.coeffs[i].gamma);
        assert!(precomp.coeffs[i]->gamma_X->get_element() == native_precomp.coeffs[i].gamma_X);
    }

    print!("number of constraints for G2 precomp (Fr is %s)  = {}\n", annotation.c_str(), pb.num_constraints());
}



//#endif // WEIERSTRASS_PRECOMPUTATION_TCC_
