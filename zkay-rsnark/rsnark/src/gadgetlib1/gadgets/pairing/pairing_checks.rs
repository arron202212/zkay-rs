/** @file
 *****************************************************************************

 Declaration of interfaces for pairing-check gadgets.

 Given that e(.,.) denotes a pairing,
 - the gadget "check_e_equals_e_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)"; and
 - the gadget "check_e_equals_ee_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)*e(P3,Q3)".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PAIRING_CHECKS_HPP_
// #define PAIRING_CHECKS_HPP_

use  <memory>

use crate::gadgetlib1::gadgets::pairing::pairing_params;
use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation;
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop;



template<typename ppT>
class check_e_equals_e_gadget : public gadget<ffec::Fr<ppT> > {


    type ffec::Fr<ppT> FieldT;

    std::shared_ptr<Fqk_variable<ppT> > ratio;
    std::shared_ptr<e_over_e_miller_loop_gadget<ppT> > compute_ratio;
    std::shared_ptr<final_exp_gadget<ppT> > check_finexp;

    G1_precomputation<ppT> lhs_G1;
    G2_precomputation<ppT> lhs_G2;
    G1_precomputation<ppT> rhs_G1;
    G2_precomputation<ppT> rhs_G2;

    pb_variable<FieldT> result;

    check_e_equals_e_gadget(protoboard<FieldT> &pb,
                            const G1_precomputation<ppT> &lhs_G1,
                            const G2_precomputation<ppT> &lhs_G2,
                            const G1_precomputation<ppT> &rhs_G1,
                            const G2_precomputation<ppT> &rhs_G2,
                            const pb_variable<FieldT> &result,
                            const std::string &annotation_prefix);

    void generate_r1cs_constraints();

    void generate_r1cs_witness();
};

template<typename ppT>
class check_e_equals_ee_gadget : public gadget<ffec::Fr<ppT> > {


    type ffec::Fr<ppT> FieldT;

    std::shared_ptr<Fqk_variable<ppT> > ratio;
    std::shared_ptr<e_times_e_over_e_miller_loop_gadget<ppT> > compute_ratio;
    std::shared_ptr<final_exp_gadget<ppT> > check_finexp;

    G1_precomputation<ppT> lhs_G1;
    G2_precomputation<ppT> lhs_G2;
    G1_precomputation<ppT> rhs1_G1;
    G2_precomputation<ppT> rhs1_G2;
    G1_precomputation<ppT> rhs2_G1;
    G2_precomputation<ppT> rhs2_G2;

    pb_variable<FieldT> result;

    check_e_equals_ee_gadget(protoboard<FieldT> &pb,
                             const G1_precomputation<ppT> &lhs_G1,
                             const G2_precomputation<ppT> &lhs_G2,
                             const G1_precomputation<ppT> &rhs1_G1,
                             const G2_precomputation<ppT> &rhs1_G2,
                             const G1_precomputation<ppT> &rhs2_G1,
                             const G2_precomputation<ppT> &rhs2_G2,
                             const pb_variable<FieldT> &result,
                             const std::string &annotation_prefix);

    void generate_r1cs_constraints();

    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets::pairing::pairing_checks;

//#endif // PAIRING_CHECKS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for pairing-check gadgets.

 See pairing_checks.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PAIRING_CHECKS_TCC_
// #define PAIRING_CHECKS_TCC_



template<typename ppT>
check_e_equals_e_gadget<ppT>::check_e_equals_e_gadget(protoboard<FieldT> &pb,
                                                      const G1_precomputation<ppT> &lhs_G1,
                                                      const G2_precomputation<ppT> &lhs_G2,
                                                      const G1_precomputation<ppT> &rhs_G1,
                                                      const G2_precomputation<ppT> &rhs_G2,
                                                      const pb_variable<FieldT> &result,
                                                      const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    lhs_G1(lhs_G1),
    lhs_G2(lhs_G2),
    rhs_G1(rhs_G1),
    rhs_G2(rhs_G2),
    result(result)
{
    ratio.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " ratio")));
    compute_ratio.reset(new e_over_e_miller_loop_gadget<ppT>(pb, lhs_G1, lhs_G2, rhs_G1, rhs_G2, *ratio, FMT(annotation_prefix, " compute_ratio")));
    check_finexp.reset(new final_exp_gadget<ppT>(pb, *ratio, result, FMT(annotation_prefix, " check_finexp")));
}

template<typename ppT>
void check_e_equals_e_gadget<ppT>::generate_r1cs_constraints()
{
    compute_ratio->generate_r1cs_constraints();
    check_finexp->generate_r1cs_constraints();
}

template<typename ppT>
void check_e_equals_e_gadget<ppT>::generate_r1cs_witness()
{
    compute_ratio->generate_r1cs_witness();
    check_finexp->generate_r1cs_witness();
}

template<typename ppT>
check_e_equals_ee_gadget<ppT>::check_e_equals_ee_gadget(protoboard<FieldT> &pb,
                                                        const G1_precomputation<ppT> &lhs_G1,
                                                        const G2_precomputation<ppT> &lhs_G2,
                                                        const G1_precomputation<ppT> &rhs1_G1,
                                                        const G2_precomputation<ppT> &rhs1_G2,
                                                        const G1_precomputation<ppT> &rhs2_G1,
                                                        const G2_precomputation<ppT> &rhs2_G2,
                                                        const pb_variable<FieldT> &result,
                                                        const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    lhs_G1(lhs_G1),
    lhs_G2(lhs_G2),
    rhs1_G1(rhs1_G1),
    rhs1_G2(rhs1_G2),
    rhs2_G1(rhs2_G1),
    rhs2_G2(rhs2_G2),
    result(result)
{
    ratio.reset(new Fqk_variable<ppT>(pb, FMT(annotation_prefix, " ratio")));
    compute_ratio.reset(new e_times_e_over_e_miller_loop_gadget<ppT>(pb, rhs1_G1, rhs1_G2, rhs2_G1, rhs2_G2, lhs_G1, lhs_G2, *ratio, FMT(annotation_prefix, " compute_ratio")));
    check_finexp.reset(new final_exp_gadget<ppT>(pb, *ratio, result, FMT(annotation_prefix, " check_finexp")));
}

template<typename ppT>
void check_e_equals_ee_gadget<ppT>::generate_r1cs_constraints()
{
    compute_ratio->generate_r1cs_constraints();
    check_finexp->generate_r1cs_constraints();
}

template<typename ppT>
void check_e_equals_ee_gadget<ppT>::generate_r1cs_witness()
{
    compute_ratio->generate_r1cs_witness();
    check_finexp->generate_r1cs_witness();
}



//#endif // PAIRING_CHECKS_TCC_
