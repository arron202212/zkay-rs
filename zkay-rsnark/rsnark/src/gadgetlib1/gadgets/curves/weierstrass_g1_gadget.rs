/** @file
 *****************************************************************************

 Declaration of interfaces for G1 gadgets.

 The gadgets verify curve arithmetic in G1 = E(F) where E/F: y^2 = x^3 + A * X + B
 is an elliptic curve over F in short Weierstrass form.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G1_GADGET_HPP_
// #define WEIERSTRASS_G1_GADGET_HPP_

use ffec::algebra::curves::public_params;

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params;



/**
 * Gadget that represents a G1 variable.
 */
template<typename ppT>
class G1_variable : public gadget<ffec::Fr<ppT> > {

    type ffec::Fr<ppT> FieldT;

    pb_linear_combination<FieldT> X;
    pb_linear_combination<FieldT> Y;

    pb_linear_combination_array<FieldT> all_vars;

    G1_variable(protoboard<FieldT> &pb,
                const std::string &annotation_prefix);
    G1_variable(protoboard<FieldT> &pb,
                const ffec::G1<other_curve<ppT> > &P,
                const std::string &annotation_prefix);

    void generate_r1cs_witness(const ffec::G1<other_curve<ppT> > &elt);

    // (See a comment in r1cs_ppzksnark_verifier_gadget.hpp about why
    // we mark this function noinline.) TODO: remove later
    static size_t __attribute__((noinline)) size_in_bits();
    static size_t num_variables();
};

/**
 * Gadget that creates constraints for the validity of a G1 variable.
 */
template<typename ppT>
class G1_checker_gadget : public gadget<ffec::Fr<ppT> > {

    type ffec::Fr<ppT> FieldT;

    G1_variable<ppT> P;
    pb_variable<FieldT> P_X_squared;
    pb_variable<FieldT> P_Y_squared;

    G1_checker_gadget(protoboard<FieldT> &pb,
                      const G1_variable<ppT> &P,
                      const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that creates constraints for G1 addition.
 */
template<typename ppT>
class G1_add_gadget : public gadget<ffec::Fr<ppT> > {

    type ffec::Fr<ppT> FieldT;

    pb_variable<FieldT> lambda;
    pb_variable<FieldT> inv;

    G1_variable<ppT> A;
    G1_variable<ppT> B;
    G1_variable<ppT> C;

    G1_add_gadget(protoboard<FieldT> &pb,
                  const G1_variable<ppT> &A,
                  const G1_variable<ppT> &B,
                  const G1_variable<ppT> &C,
                  const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that creates constraints for G1 doubling.
 */
template<typename ppT>
class G1_dbl_gadget : public gadget<ffec::Fr<ppT> > {

    type ffec::Fr<ppT> FieldT;

    pb_variable<FieldT> Xsquared;
    pb_variable<FieldT> lambda;

    G1_variable<ppT> A;
    G1_variable<ppT> B;

    G1_dbl_gadget(protoboard<FieldT> &pb,
                  const G1_variable<ppT> &A,
                  const G1_variable<ppT> &B,
                  const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

/**
 * Gadget that creates constraints for G1 multi-scalar multiplication.
 */
template<typename ppT>
class G1_multiscalar_mul_gadget : public gadget<ffec::Fr<ppT> > {

    type ffec::Fr<ppT> FieldT;

    std::vector<G1_variable<ppT> > computed_results;
    std::vector<G1_variable<ppT> > chosen_results;
    std::vector<G1_add_gadget<ppT> > adders;
    std::vector<G1_dbl_gadget<ppT> > doublers;

    G1_variable<ppT> base;
    pb_variable_array<FieldT> scalars;
    std::vector<G1_variable<ppT> > points;
    std::vector<G1_variable<ppT> > points_and_powers;
    G1_variable<ppT> result;

    const size_t elt_size;
    const size_t num_points;
    const size_t scalar_size;

    G1_multiscalar_mul_gadget(protoboard<FieldT> &pb,
                              const G1_variable<ppT> &base,
                              const pb_variable_array<FieldT> &scalars,
                              const size_t elt_size,
                              const std::vector<G1_variable<ppT> > &points,
                              const G1_variable<ppT> &result,
                              const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets::curves/weierstrass_g1_gadget;

//#endif // WEIERSTRASS_G1_GADGET_TCC_
/** @file
 *****************************************************************************

 Implementation of interfaces for G1 gadgets.

 See weierstrass_g1_gadgets.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WEIERSTRASS_G1_GADGET_TCC_
// #define WEIERSTRASS_G1_GADGET_TCC_



template<typename ppT>
G1_variable<ppT>::G1_variable(protoboard<FieldT> &pb,
                              const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    pb_variable<FieldT> X_var, Y_var;

    X_var.allocate(pb, FMT(annotation_prefix, " X"));
    Y_var.allocate(pb, FMT(annotation_prefix, " Y"));

    X = pb_linear_combination<FieldT>(X_var);
    Y = pb_linear_combination<FieldT>(Y_var);

    all_vars.push(X);
    all_vars.push(Y);
}

template<typename ppT>
G1_variable<ppT>::G1_variable(protoboard<FieldT> &pb,
                              const ffec::G1<other_curve<ppT> > &P,
                              const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix)
{
    ffec::G1<other_curve<ppT> > Pcopy = P;
    Pcopy.to_affine_coordinates();

    X.assign(pb, Pcopy.X());
    Y.assign(pb, Pcopy.Y());
    X.evaluate(pb);
    Y.evaluate(pb);
    all_vars.push(X);
    all_vars.push(Y);
}

template<typename ppT>
void G1_variable<ppT>::generate_r1cs_witness(const ffec::G1<other_curve<ppT> > &el)
{
    ffec::G1<other_curve<ppT> > el_normalized = el;
    el_normalized.to_affine_coordinates();

    self.pb.lc_val(X) = el_normalized.X();
    self.pb.lc_val(Y) = el_normalized.Y();
}

template<typename ppT>
size_t G1_variable<ppT>::size_in_bits()
{
    return 2 * FieldT::size_in_bits();
}

template<typename ppT>
size_t G1_variable<ppT>::num_variables()
{
    return 2;
}

template<typename ppT>
G1_checker_gadget<ppT>::G1_checker_gadget(protoboard<FieldT> &pb, const G1_variable<ppT> &P, const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix), P(P)
{
    P_X_squared.allocate(pb, FMT(annotation_prefix, " P_X_squared"));
    P_Y_squared.allocate(pb, FMT(annotation_prefix, " P_Y_squared"));
}

template<typename ppT>
void G1_checker_gadget<ppT>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { P.X },
        { P.X },
        { P_X_squared }),
        FMT(self.annotation_prefix, " P_X_squared"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { P.Y },
        { P.Y },
        { P_Y_squared }),
        FMT(self.annotation_prefix, " P_Y_squared"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { P.X },
        { P_X_squared, ONE * ffec::G1<other_curve<ppT> >::coeff_a },
        { P_Y_squared, ONE * (-ffec::G1<other_curve<ppT> >::coeff_b) }),
        FMT(self.annotation_prefix, " curve_equation"));
}

template<typename ppT>
void G1_checker_gadget<ppT>::generate_r1cs_witness()
{
    self.pb.val(P_X_squared) = self.pb.lc_val(P.X).squared();
    self.pb.val(P_Y_squared) = self.pb.lc_val(P.Y).squared();
}

template<typename ppT>
G1_add_gadget<ppT>::G1_add_gadget(protoboard<FieldT> &pb,
                                  const G1_variable<ppT> &A,
                                  const G1_variable<ppT> &B,
                                  const G1_variable<ppT> &C,
                                  const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    A(A),
    B(B),
    C(C)
{
    /*
      lambda = (B.y - A.y)/(B.x - A.x)
      C.x = lambda^2 - A.x - B.x
      C.y = lambda(A.x - C.x) - A.y

      Special cases:

      doubling: if B.y = A.y and B.x = A.x then lambda is unbound and
      C = (lambda^2, lambda^3)

      addition of negative point: if B.y = -A.y and B.x = A.x then no
      lambda can satisfy the first equation unless B.y - A.y = 0. But
      then this reduces to doubling.

      So we need to check that A.x - B.x != 0, which can be done by
      enforcing I * (B.x - A.x) = 1
    */
    lambda.allocate(pb, FMT(annotation_prefix, " lambda"));
    inv.allocate(pb, FMT(annotation_prefix, " inv"));
}

template<typename ppT>
void G1_add_gadget<ppT>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda },
        { B.X, A.X * (-1) },
        { B.Y, A.Y * (-1) }),
        FMT(self.annotation_prefix, " calc_lambda"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda },
        { lambda },
        { C.X, A.X, B.X }),
        FMT(self.annotation_prefix, " calc_X"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda },
        { A.X, C.X * (-1) },
        { C.Y, A.Y }),
        FMT(self.annotation_prefix, " calc_Y"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { inv },
        { B.X, A.X * (-1) },
        { ONE }),
        FMT(self.annotation_prefix, " no_special_cases"));
}

template<typename ppT>
void G1_add_gadget<ppT>::generate_r1cs_witness()
{
    self.pb.val(inv) = (self.pb.lc_val(B.X) - self.pb.lc_val(A.X)).inverse();
    self.pb.val(lambda) = (self.pb.lc_val(B.Y) - self.pb.lc_val(A.Y)) * self.pb.val(inv);
    self.pb.lc_val(C.X) = self.pb.val(lambda).squared() - self.pb.lc_val(A.X) - self.pb.lc_val(B.X);
    self.pb.lc_val(C.Y) = self.pb.val(lambda) * (self.pb.lc_val(A.X) - self.pb.lc_val(C.X)) - self.pb.lc_val(A.Y);
}

template<typename ppT>
G1_dbl_gadget<ppT>::G1_dbl_gadget(protoboard<FieldT> &pb,
                                  const G1_variable<ppT> &A,
                                  const G1_variable<ppT> &B,
                                  const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    A(A),
    B(B)
{
    Xsquared.allocate(pb, FMT(annotation_prefix, " X_squared"));
    lambda.allocate(pb, FMT(annotation_prefix, " lambda"));
}

template<typename ppT>
void G1_dbl_gadget<ppT>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { A.X },
        { A.X },
        { Xsquared }),
        FMT(self.annotation_prefix, " calc_Xsquared"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda * 2 },
        { A.Y },
        { Xsquared * 3, ONE * ffec::G1<other_curve<ppT> >::coeff_a }),
        FMT(self.annotation_prefix, " calc_lambda"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda },
        { lambda },
        { B.X, A.X * 2 }),
        FMT(self.annotation_prefix, " calc_X"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(
        { lambda },
        { A.X, B.X * (-1) },
        { B.Y, A.Y }),
        FMT(self.annotation_prefix, " calc_Y"));
}

template<typename ppT>
void G1_dbl_gadget<ppT>::generate_r1cs_witness()
{
    self.pb.val(Xsquared) = self.pb.lc_val(A.X).squared();
    self.pb.val(lambda) = (FieldT(3) * self.pb.val(Xsquared) + ffec::G1<other_curve<ppT> >::coeff_a) * (FieldT(2) * self.pb.lc_val(A.Y)).inverse();
    self.pb.lc_val(B.X) = self.pb.val(lambda).squared() - FieldT(2) * self.pb.lc_val(A.X);
    self.pb.lc_val(B.Y) = self.pb.val(lambda) * (self.pb.lc_val(A.X) - self.pb.lc_val(B.X)) - self.pb.lc_val(A.Y);
}

template<typename ppT>
G1_multiscalar_mul_gadget<ppT>::G1_multiscalar_mul_gadget(protoboard<FieldT> &pb,
                                                          const G1_variable<ppT> &base,
                                                          const pb_variable_array<FieldT> &scalars,
                                                          const size_t elt_size,
                                                          const std::vector<G1_variable<ppT> > &points,
                                                          const G1_variable<ppT>&result,
                                                          const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    base(base),
    scalars(scalars),
    points(points),
    result(result),
    elt_size(elt_size),
    num_points(points.len()),
    scalar_size(scalars.len())
{
    assert!(num_points >= 1);
    assert!(num_points * elt_size == scalar_size);

    for i in 0..num_points
    {
        points_and_powers.push(points[i]);
        for j in 0..elt_size - 1
        {
            points_and_powers.push(G1_variable<ppT>(pb, FMT(annotation_prefix, " points_%zu_times_2_to_{}", i, j+1)));
            doublers.push(G1_dbl_gadget<ppT>(pb, points_and_powers[i*elt_size + j], points_and_powers[i*elt_size + j + 1], FMT(annotation_prefix, " double_%zu_to_2_to_{}", i, j+1)));
        }
    }

    chosen_results.push(base);
    for i in 0..scalar_size
    {
        computed_results.push(G1_variable<ppT>(pb, FMT(annotation_prefix, " computed_results_{}")));
        if i < scalar_size-1
        {
            chosen_results.push(G1_variable<ppT>(pb, FMT(annotation_prefix, " chosen_results_{}")));
        }
        else
        {
            chosen_results.push(result);
        }

        adders.push(G1_add_gadget<ppT>(pb, chosen_results[i], points_and_powers[i], computed_results[i], FMT(annotation_prefix, " adders_{}")));
    }
}

template<typename ppT>
void G1_multiscalar_mul_gadget<ppT>::generate_r1cs_constraints()
{
    const size_t num_constraints_before = self.pb.num_constraints();

    for i in 0..scalar_size - num_points
    {
        doublers[i].generate_r1cs_constraints();
    }

    for i in 0..scalar_size
    {
        adders[i].generate_r1cs_constraints();

        /*
          chosen_results[i+1].X = scalars[i] * computed_results[i].X + (1-scalars[i]) *  chosen_results[i].X
          chosen_results[i+1].X - chosen_results[i].X = scalars[i] * (computed_results[i].X - chosen_results[i].X)
        */
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(scalars[i],
                                                             computed_results[i].X - chosen_results[i].X,
                                                             chosen_results[i+1].X - chosen_results[i].X),
                                     FMT(self.annotation_prefix, " chosen_results_%zu_X", i+1));
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(scalars[i],
                                                             computed_results[i].Y - chosen_results[i].Y,
                                                             chosen_results[i+1].Y - chosen_results[i].Y),
                                     FMT(self.annotation_prefix, " chosen_results_%zu_Y", i+1));
    }

    const size_t num_constraints_after = self.pb.num_constraints();
    assert!(num_constraints_after - num_constraints_before == 4 * (scalar_size-num_points) + (4 + 2) * scalar_size);
}

template<typename ppT>
void G1_multiscalar_mul_gadget<ppT>::generate_r1cs_witness()
{
    for i in 0..scalar_size - num_points
    {
        doublers[i].generate_r1cs_witness();
    }

    for i in 0..scalar_size
    {
        adders[i].generate_r1cs_witness();
        self.pb.lc_val(chosen_results[i+1].X) = if self.pb.val(scalars[i]) == ffec::Fr<ppT>::zero() {self.pb.lc_val(chosen_results[i].X)} else{self.pb.lc_val(computed_results[i].X)};
        self.pb.lc_val(chosen_results[i+1].Y) = if self.pb.val(scalars[i]) == ffec::Fr<ppT>::zero() {self.pb.lc_val(chosen_results[i].Y)} else{self.pb.lc_val(computed_results[i].Y)};
    }
}



//#endif // WEIERSTRASS_G1_GADGET_TCC_
