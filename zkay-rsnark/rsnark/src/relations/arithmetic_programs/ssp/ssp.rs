/** @file
 *****************************************************************************

 Declaration of interfaces for a SSP ("Square Span Program").

 SSPs are defined in \[DFGK14].

 References:

 \[DFGK14]:
 "Square Span Programs with Applications to Succinct NIZK Arguments"
 George Danezis, Cedric Fournet, Jens Groth, Markulf Kohlweiss,
 ASIACRYPT 2014,
 <http://eprint.iacr.org/2014/718>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef SSP_HPP_
#define SSP_HPP_

use  <map>
use  <memory>

use  <libfqfft/evaluation_domain/evaluation_domain.hpp>

namespace libsnark {

/* forward declaration */
template<typename FieldT>
class ssp_witness;

/**
 * A SSP instance.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs; and
 * - coefficients of the V polynomials in the Lagrange basis.
 *
 * There is no need to store the Z polynomial because it is uniquely
 * determined by the domain (as Z is its vanishing polynomial).
 */
template<typename FieldT>
class ssp_instance {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;

public:
    std::shared_ptr<libfqfft::evaluation_domain<FieldT> > domain;

    std::vector<std::map<size_t, FieldT> > V_in_Lagrange_basis;

    ssp_instance(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                 const size_t num_variables,
                 const size_t degree,
                 const size_t num_inputs,
                 const std::vector<std::map<size_t, FieldT> > &V_in_Lagrange_basis);
    ssp_instance(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                 const size_t num_variables,
                 const size_t degree,
                 const size_t num_inputs,
                 std::vector<std::map<size_t, FieldT> > &&V_in_Lagrange_basis);

    ssp_instance(const ssp_instance<FieldT> &other) = default;
    ssp_instance(ssp_instance<FieldT> &&other) = default;
    ssp_instance& operator=(const ssp_instance<FieldT> &other) = default;
    ssp_instance& operator=(ssp_instance<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;

    bool is_satisfied(const ssp_witness<FieldT> &witness) const;
};


/**
 * A SSP instance evaluation is a SSP instance that is evaluated at a field element t.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs;
 * - a field element t;
 * - evaluations of the V (and Z) polynomials at t;
 * - evaluations of all monomials of t.
 */
template<typename FieldT>
class ssp_instance_evaluation {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;

public:
    std::shared_ptr<libfqfft::evaluation_domain<FieldT> > domain;

    FieldT t;

    std::vector<FieldT> Vt, Ht;

    FieldT Zt;

    ssp_instance_evaluation(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                            const size_t num_variables,
                            const size_t degree,
                            const size_t num_inputs,
                            const FieldT &t,
                            const std::vector<FieldT> &Vt,
                            const std::vector<FieldT> &Ht,
                            const FieldT &Zt);
    ssp_instance_evaluation(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                            const size_t num_variables,
                            const size_t degree,
                            const size_t num_inputs,
                            const FieldT &t,
                            std::vector<FieldT> &&Vt,
                            std::vector<FieldT> &&Ht,
                            const FieldT &Zt);

    ssp_instance_evaluation(const ssp_instance_evaluation<FieldT> &other) = default;
    ssp_instance_evaluation(ssp_instance_evaluation<FieldT> &&other) = default;
    ssp_instance_evaluation& operator=(const ssp_instance_evaluation<FieldT> &other) = default;
    ssp_instance_evaluation& operator=(ssp_instance_evaluation<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;

    bool is_satisfied(const ssp_witness<FieldT> &witness) const;
};

/**
 * A SSP witness.
 */
template<typename FieldT>
class ssp_witness {
private:
    size_t num_variables_;
    size_t degree_;
    size_t num_inputs_;

public:
    FieldT d;

    std::vector<FieldT> coefficients_for_Vs;
    std::vector<FieldT> coefficients_for_H;

    ssp_witness(const size_t num_variables,
                const size_t degree,
                const size_t num_inputs,
                const FieldT &d,
                const std::vector<FieldT> &coefficients_for_Vs,
                const std::vector<FieldT> &coefficients_for_H);
    ssp_witness(const size_t num_variables,
                const size_t degree,
                const size_t num_inputs,
                const FieldT &d,
                const std::vector<FieldT> &coefficients_for_Vs,
                std::vector<FieldT> &&coefficients_for_H);

    ssp_witness(const ssp_witness<FieldT> &other) = default;
    ssp_witness(ssp_witness<FieldT> &&other) = default;
    ssp_witness& operator=(const ssp_witness<FieldT> &other) = default;
    ssp_witness& operator=(ssp_witness<FieldT> &&other) = default;

    size_t num_variables() const;
    size_t degree() const;
    size_t num_inputs() const;
};

} // libsnark

use  <libsnark/relations/arithmetic_programs/ssp/ssp.tcc>

#endif // SSP_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a SSP ("Square Span Program").

 See ssp.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef SSP_TCC_
#define SSP_TCC_

use  <libff/algebra/scalar_multiplication/multiexp.hpp>
use  <libff/common/profiling.hpp>
use  <libff/common/utils.hpp>
use  <libfqfft/evaluation_domain/evaluation_domain.hpp>

namespace libsnark {

template<typename FieldT>
ssp_instance<FieldT>::ssp_instance(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                                   const size_t num_variables,
                                   const size_t degree,
                                   const size_t num_inputs,
                                   const std::vector<std::map<size_t, FieldT> > &V_in_Lagrange_basis) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    V_in_Lagrange_basis(V_in_Lagrange_basis)
{
}

template<typename FieldT>
ssp_instance<FieldT>::ssp_instance(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                                   const size_t num_variables,
                                   const size_t degree,
                                   const size_t num_inputs,
                                   std::vector<std::map<size_t, FieldT> > &&V_in_Lagrange_basis) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    V_in_Lagrange_basis(std::move(V_in_Lagrange_basis))
{
}

template<typename FieldT>
size_t ssp_instance<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t ssp_instance<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t ssp_instance<FieldT>::num_inputs() const
{
    return num_inputs_;
}

template<typename FieldT>
bool ssp_instance<FieldT>::is_satisfied(const ssp_witness<FieldT> &witness) const
{
    const FieldT t = FieldT::random_element();;
    std::vector<FieldT> Vt(this->num_variables()+1, FieldT::zero());
    std::vector<FieldT> Ht(this->degree()+1);

    const FieldT Zt = this->domain->compute_vanishing_polynomial(t);

    const std::vector<FieldT> u = this->domain->evaluate_all_lagrange_polynomials(t);

    for (size_t i = 0; i < this->num_variables()+1; ++i)
    {
        for (auto &el : V_in_Lagrange_basis[i])
        {
            Vt[i] += u[el.first] * el.second;
        }
    }

    FieldT ti = FieldT::one();
    for (size_t i = 0; i < this->degree()+1; ++i)
    {
        Ht[i] = ti;
        ti *= t;
    }

    const ssp_instance_evaluation<FieldT> eval_ssp_inst(this->domain,
                                                        this->num_variables(),
                                                        this->degree(),
                                                        this->num_inputs(),
                                                        t,
                                                        std::move(Vt),
                                                        std::move(Ht),
                                                        Zt);
    return eval_ssp_inst.is_satisfied(witness);
}

template<typename FieldT>
ssp_instance_evaluation<FieldT>::ssp_instance_evaluation(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                                                         const size_t num_variables,
                                                         const size_t degree,
                                                         const size_t num_inputs,
                                                         const FieldT &t,
                                                         const std::vector<FieldT> &Vt,
                                                         const std::vector<FieldT> &Ht,
                                                         const FieldT &Zt) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    t(t),
    Vt(Vt),
    Ht(Ht),
    Zt(Zt)
{
}

template<typename FieldT>
ssp_instance_evaluation<FieldT>::ssp_instance_evaluation(const std::shared_ptr<libfqfft::evaluation_domain<FieldT> > &domain,
                                                         const size_t num_variables,
                                                         const size_t degree,
                                                         const size_t num_inputs,
                                                         const FieldT &t,
                                                         std::vector<FieldT> &&Vt,
                                                         std::vector<FieldT> &&Ht,
                                                         const FieldT &Zt) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    domain(domain),
    t(t),
    Vt(std::move(Vt)),
    Ht(std::move(Ht)),
    Zt(Zt)
{
}

template<typename FieldT>
size_t ssp_instance_evaluation<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t ssp_instance_evaluation<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t ssp_instance_evaluation<FieldT>::num_inputs() const
{
    return num_inputs_;
}

template<typename FieldT>
bool ssp_instance_evaluation<FieldT>::is_satisfied(const ssp_witness<FieldT> &witness) const
{

    if (this->num_variables() != witness.num_variables())
    {
        return false;
    }

    if (this->degree() != witness.degree())
    {
        return false;
    }

    if (this->num_inputs() != witness.num_inputs())
    {
        return false;
    }

    if (this->num_variables() != witness.coefficients_for_Vs.size())
    {
        return false;
    }

    if (this->degree()+1 != witness.coefficients_for_H.size())
    {
        return false;
    }

    if (this->Vt.size() != this->num_variables()+1)
    {
        return false;
    }

    if (this->Ht.size() != this->degree()+1)
    {
        return false;
    }

    if (this->Zt != this->domain->compute_vanishing_polynomial(this->t))
    {
        return false;
    }

    FieldT ans_V = this->Vt[0] + witness.d*this->Zt;
    FieldT ans_H = FieldT::zero();

    ans_V = ans_V + libff::inner_product<FieldT>(this->Vt.begin()+1,
                                                 this->Vt.begin()+1+this->num_variables(),
                                                 witness.coefficients_for_Vs.begin(),
                                                 witness.coefficients_for_Vs.begin()+this->num_variables());
    ans_H = ans_H + libff::inner_product<FieldT>(this->Ht.begin(),
                                                 this->Ht.begin()+this->degree()+1,
                                                 witness.coefficients_for_H.begin(),
                                                 witness.coefficients_for_H.begin()+this->degree()+1);

    if (ans_V.squared() - FieldT::one() != ans_H * this->Zt)
    {
        return false;
    }

    return true;
}

template<typename FieldT>
ssp_witness<FieldT>::ssp_witness(const size_t num_variables,
                                 const size_t degree,
                                 const size_t num_inputs,
                                 const FieldT &d,
                                 const std::vector<FieldT> &coefficients_for_Vs,
                                 const std::vector<FieldT> &coefficients_for_H) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    d(d),
    coefficients_for_Vs(coefficients_for_Vs),
    coefficients_for_H(coefficients_for_H)
{
}

template<typename FieldT>
ssp_witness<FieldT>::ssp_witness(const size_t num_variables,
                                 const size_t degree,
                                 const size_t num_inputs,
                                 const FieldT &d,
                                 const std::vector<FieldT> &coefficients_for_Vs,
                                 std::vector<FieldT> &&coefficients_for_H) :
    num_variables_(num_variables),
    degree_(degree),
    num_inputs_(num_inputs),
    d(d),
    coefficients_for_Vs(coefficients_for_Vs),
    coefficients_for_H(std::move(coefficients_for_H))
{
}

template<typename FieldT>
size_t ssp_witness<FieldT>::num_variables() const
{
    return num_variables_;
}

template<typename FieldT>
size_t ssp_witness<FieldT>::degree() const
{
    return degree_;
}

template<typename FieldT>
size_t ssp_witness<FieldT>::num_inputs() const
{
    return num_inputs_;
}

} // libsnark

#endif // SSP_TCC_
