/** @file
 *****************************************************************************

 Declaration of interfaces for a QAP ("Quadratic Arithmetic Program").

 QAPs are defined in \[GGPR13].

 References:

 \[GGPR13]:
 "Quadratic span programs and succinct NIZKs without PCPs",
 Rosario Gennaro, Craig Gentry, Bryan Parno, Mariana Raykova,
 EUROCRYPT 2013,
 <http://eprint.iacr.org/2012/215>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef QAP_HPP_
#define QAP_HPP_

use  <map>
use  <memory>

use  <libfqfft/evaluation_domain/evaluation_domain.hpp>

namespace libsnark {

/* forward declaration */
template<typename FieldT>
class qap_witness;

/**
 * A QAP instance.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs; and
 * - coefficients of the A,B,C polynomials in the Lagrange basis.
 *
 * There is no need to store the Z polynomial because it is uniquely
 * determined by the domain (as Z is its vanishing polynomial).
 */
template<typename FieldT>
class qap_instance {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;


    std::shared_ptr<libfqfft::evaluation_domain<FieldT> > domain;

    std::vector<std::map<size_t, FieldT> > A_in_Lagrange_basis;
    std::vector<std::map<size_t, FieldT> > B_in_Lagrange_basis;
    std::vector<std::map<size_t, FieldT> > C_in_Lagrange_basis;

    qap_instance(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                 num_variables:size_t
                 degree:size_t
                 num_inputs:size_t
                 A_in_Lagrange_basis:&std::vector<std::map<size_t, FieldT> >
                 B_in_Lagrange_basis:&std::vector<std::map<size_t, FieldT> >
                 &C_in_Lagrange_basis:std::vector<std::map<size_t, FieldT> >);

    qap_instance(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                 num_variables:size_t
                 degree:size_t
                 num_inputs:size_t
                 std::vector<std::map<size_t, FieldT> > &&A_in_Lagrange_basis,
                 std::vector<std::map<size_t, FieldT> > &&B_in_Lagrange_basis,
                 std::vector<std::map<size_t, FieldT> > &&C_in_Lagrange_basis);

    qap_instance(&other:qap_instance<FieldT>) = default;
    qap_instance(qap_instance<FieldT> &&other) = default;
    qap_instance& operator=(&other:qap_instance<FieldT>) = default;
    qap_instance& operator=(qap_instance<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;

    bool is_satisfied(&witness:qap_witness<FieldT>) const;
};

/**
 * A QAP instance evaluation is a QAP instance that is evaluated at a field element t.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs;
 * - a field element t;
 * - evaluations of the A,B,C (and Z) polynomials at t;
 * - evaluations of all monomials of t;
 * - counts about how many of the above evaluations are in fact non-zero.
 */
template<typename FieldT>
class qap_instance_evaluation {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;

    std::shared_ptr<libfqfft::evaluation_domain<FieldT> > domain;

    FieldT t;

    std::vector<FieldT> At, Bt, Ct, Ht;

    FieldT Zt;

    qap_instance_evaluation(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                            num_variables:size_t
                            degree:size_t
                            num_inputs:size_t
                            t:&FieldT
                            At:&std::vector<FieldT>
                            Bt:&std::vector<FieldT>
                            Ct:&std::vector<FieldT>
                            Ht:&std::vector<FieldT>
                            &Zt:FieldT);
    qap_instance_evaluation(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                            num_variables:size_t
                            degree:size_t
                            num_inputs:size_t
                            t:&FieldT
                            std::vector<FieldT> &&At,
                            std::vector<FieldT> &&Bt,
                            std::vector<FieldT> &&Ct,
                            std::vector<FieldT> &&Ht,
                            &Zt:FieldT);

    qap_instance_evaluation(&other:qap_instance_evaluation<FieldT>) = default;
    qap_instance_evaluation(qap_instance_evaluation<FieldT> &&other) = default;
    qap_instance_evaluation& operator=(&other:qap_instance_evaluation<FieldT>) = default;
    qap_instance_evaluation& operator=(qap_instance_evaluation<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;

    bool is_satisfied(&witness:qap_witness<FieldT>) const;
};

/**
 * A QAP witness.
 */
template<typename FieldT>
class qap_witness {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;


    FieldT d1, d2, d3;

    std::vector<FieldT> coefficients_for_ABCs;
    std::vector<FieldT> coefficients_for_H;

    qap_witness(num_variables:size_t
                degree:size_t
                num_inputs:size_t
                d1:&FieldT
                d2:&FieldT
                d3:&FieldT
                coefficients_for_ABCs:&std::vector<FieldT>
                &coefficients_for_H:std::vector<FieldT>);

    qap_witness(num_variables:size_t
                degree:size_t
                num_inputs:size_t
                d1:&FieldT
                d2:&FieldT
                d3:&FieldT
                coefficients_for_ABCs:&std::vector<FieldT>
                std::vector<FieldT> &&coefficients_for_H);

    qap_witness(&other:qap_witness<FieldT>) = default;
    qap_witness(qap_witness<FieldT> &&other) = default;
    qap_witness& operator=(&other:qap_witness<FieldT>) = default;
    qap_witness& operator=(qap_witness<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;
};

} // libsnark

use  <libsnark/relations/arithmetic_programs/qap/qap.tcc>

#endif // QAP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a QAP ("Quadratic Arithmetic Program").

See qap.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

#ifndef QAP_TCC_
#define QAP_TCC_

use  <libff/algebra/scalar_multiplication/multiexp.hpp>
use  <libff/common/profiling.hpp>
use  <libff/common/utils.hpp>
use  <libfqfft/evaluation_domain/evaluation_domain.hpp>

namespace libsnark {

template<typename FieldT>
qap_instance<FieldT>::qap_instance(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                                   num_variables:size_t
                                   degree:size_t
                                   num_inputs:size_t
                                   A_in_Lagrange_basis:&std::vector<std::map<size_t, FieldT> >
                                   B_in_Lagrange_basis:&std::vector<std::map<size_t, FieldT> >
                                   &C_in_Lagrange_basis:std::vector<std::map<size_t, FieldT> >) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    A_in_Lagrange_basis(A_in_Lagrange_basis),
    B_in_Lagrange_basis(B_in_Lagrange_basis),
    C_in_Lagrange_basis(C_in_Lagrange_basis)
{
}

template<typename FieldT>
qap_instance<FieldT>::qap_instance(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                                   num_variables:size_t
                                   degree:size_t
                                   num_inputs:size_t
                                   std::vector<std::map<size_t, FieldT> > &&A_in_Lagrange_basis,
                                   std::vector<std::map<size_t, FieldT> > &&B_in_Lagrange_basis,
                                   std::vector<std::map<size_t, FieldT> > &&C_in_Lagrange_basis) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    A_in_Lagrange_basis(std::move(A_in_Lagrange_basis)),
    B_in_Lagrange_basis(std::move(B_in_Lagrange_basis)),
    C_in_Lagrange_basis(std::move(C_in_Lagrange_basis))
{
}

template<typename FieldT>
size_t qap_instance<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t qap_instance<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t qap_instance<FieldT>::num_inputs() const
{
    return num_inputs_;
}

template<typename FieldT>
bool qap_instance<FieldT>::is_satisfied(&witness:qap_witness<FieldT>) const
{
    FieldT::random_element(:FieldT t =);

    std::vector<FieldT> At(this->num_variables()+1, FieldT::zero());
    std::vector<FieldT> Bt(this->num_variables()+1, FieldT::zero());
    std::vector<FieldT> Ct(this->num_variables()+1, FieldT::zero());
    std::vector<FieldT> Ht(this->degree()+1);

    this->domain->compute_vanishing_polynomial(t:FieldT Zt =);

    this->domain->evaluate_all_lagrange_polynomials(t:std::vector<FieldT> u =);

    for i in 0..this->num_variables()+1
    {
        for (auto &el : A_in_Lagrange_basis[i])
        {
            At[i] += u[el.first] * el.second;
        }

        for (auto &el : B_in_Lagrange_basis[i])
        {
            Bt[i] += u[el.first] * el.second;
        }

        for (auto &el : C_in_Lagrange_basis[i])
        {
            Ct[i] += u[el.first] * el.second;
        }
    }

    FieldT ti = FieldT::one();
    for i in 0..this->degree()+1
    {
        Ht[i] = ti;
        ti *= t;
    }

    eval_qap_inst(this->domain:qap_instance_evaluation<FieldT>
                                                        this->num_variables(),
                                                        this->degree(),
                                                        this->num_inputs(),
                                                        t,
                                                        std::move(At),
                                                        std::move(Bt),
                                                        std::move(Ct),
                                                        std::move(Ht),
                                                        Zt);
    return eval_qap_inst.is_satisfied(witness);
}

template<typename FieldT>
qap_instance_evaluation<FieldT>::qap_instance_evaluation(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                                                         num_variables:size_t
                                                         degree:size_t
                                                         num_inputs:size_t
                                                         t:&FieldT
                                                         At:&std::vector<FieldT>
                                                         Bt:&std::vector<FieldT>
                                                         Ct:&std::vector<FieldT>
                                                         Ht:&std::vector<FieldT>
                                                         &Zt:FieldT) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    t(t),
    At(At),
    Bt(Bt),
    Ct(Ct),
    Ht(Ht),
    Zt(Zt)
{
}

template<typename FieldT>
qap_instance_evaluation<FieldT>::qap_instance_evaluation(domain:&std::shared_ptr<libfqfft::evaluation_domain<FieldT> >
                                                         num_variables:size_t
                                                         degree:size_t
                                                         num_inputs:size_t
                                                         t:&FieldT
                                                         std::vector<FieldT> &&At,
                                                         std::vector<FieldT> &&Bt,
                                                         std::vector<FieldT> &&Ct,
                                                         std::vector<FieldT> &&Ht,
                                                         &Zt:FieldT) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    t(t),
    At(std::move(At)),
    Bt(std::move(Bt)),
    Ct(std::move(Ct)),
    Ht(std::move(Ht)),
    Zt(Zt)
{
}

template<typename FieldT>
size_t qap_instance_evaluation<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t qap_instance_evaluation<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t qap_instance_evaluation<FieldT>::num_inputs() const
{
    return num_inputs_;
}

template<typename FieldT>
bool qap_instance_evaluation<FieldT>::is_satisfied(&witness:qap_witness<FieldT>) const
{

    if this->num_variables() != witness.num_variables()
    {
        return false;
    }

    if this->degree() != witness.degree()
    {
        return false;
    }

    if this->num_inputs() != witness.num_inputs()
    {
        return false;
    }

    if this->num_variables() != witness.coefficients_for_ABCs.size()
    {
        return false;
    }

    if this->degree()+1 != witness.coefficients_for_H.size()
    {
        return false;
    }

    if this->At.size() != this->num_variables()+1 || this->Bt.size() != this->num_variables()+1 || this->Ct.size() != this->num_variables()+1
    {
        return false;
    }

    if this->Ht.size() != this->degree()+1
    {
        return false;
    }

    if this->Zt != this->domain->compute_vanishing_polynomial(this->t)
    {
        return false;
    }

    FieldT ans_A = this->At[0] + witness.d1*this->Zt;
    FieldT ans_B = this->Bt[0] + witness.d2*this->Zt;
    FieldT ans_C = this->Ct[0] + witness.d3*this->Zt;
    FieldT ans_H = FieldT::zero();

    ans_A = ans_A + libff::inner_product<FieldT>(this->At.begin()+1,
                                                 this->At.begin()+1+this->num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+this->num_variables());
    ans_B = ans_B + libff::inner_product<FieldT>(this->Bt.begin()+1,
                                                 this->Bt.begin()+1+this->num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+this->num_variables());
    ans_C = ans_C + libff::inner_product<FieldT>(this->Ct.begin()+1,
                                                 this->Ct.begin()+1+this->num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+this->num_variables());
    ans_H = ans_H + libff::inner_product<FieldT>(this->Ht.begin(),
                                                 this->Ht.begin()+this->degree()+1,
                                                 witness.coefficients_for_H.begin(),
                                                 witness.coefficients_for_H.begin()+this->degree()+1);

    if ans_A * ans_B - ans_C != ans_H * this->Zt
    {
        return false;
    }

    return true;
}

template<typename FieldT>
qap_witness<FieldT>::qap_witness(num_variables:size_t
                                 degree:size_t
                                 num_inputs:size_t
                                 d1:&FieldT
                                 d2:&FieldT
                                 d3:&FieldT
                                 coefficients_for_ABCs:&std::vector<FieldT>
                                 &coefficients_for_H:std::vector<FieldT>) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    d1(d1),
    d2(d2),
    d3(d3),
    coefficients_for_ABCs(coefficients_for_ABCs),
    coefficients_for_H(coefficients_for_H)
{
}

template<typename FieldT>
qap_witness<FieldT>::qap_witness(num_variables:size_t
                                 degree:size_t
                                 num_inputs:size_t
                                 d1:&FieldT
                                 d2:&FieldT
                                 d3:&FieldT
                                 coefficients_for_ABCs:&std::vector<FieldT>
                                 std::vector<FieldT> &&coefficients_for_H) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    d1(d1),
    d2(d2),
    d3(d3),
    coefficients_for_ABCs(coefficients_for_ABCs),
    coefficients_for_H(std::move(coefficients_for_H))
{
}


template<typename FieldT>
size_t qap_witness<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t qap_witness<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t qap_witness<FieldT>::num_inputs() const
{
    return num_inputs_;
}


} // libsnark

#endif // QAP_TCC_
