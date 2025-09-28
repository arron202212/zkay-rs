/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef PROTOBOARD_HPP_
#define PROTOBOARD_HPP_

use  <algorithm>
use  <cassert>
use  <cstdio>
use  <string>
use  <vector>

use  <libff/common/utils.hpp>

use  <libsnark/gadgetlib1/pb_variable.hpp>
use  <libsnark/relations/constraint_satisfaction_problems/r1cs/r1cs.hpp>

namespace libsnark {

template<typename FieldT>
class r1cs_constraint;

template<typename FieldT>
class r1cs_constraint_system;

template<typename FieldT>
class protoboard {
private:
    FieldT constant_term; /* only here, because pb.val() needs to be able to return reference to the constant 1 term */
    r1cs_variable_assignment<FieldT> values; /* values[0] will hold the value of the first allocated variable of the protoboard, *NOT* constant 1 */
    var_index_t next_free_var;
    lc_index_t next_free_lc;
    std::vector<FieldT> lc_values;
    r1cs_constraint_system<FieldT> constraint_system;

public:
    protoboard();

    void clear_values();

    FieldT& val(const pb_variable<FieldT> &var);
    FieldT val(const pb_variable<FieldT> &var) const;

    FieldT& lc_val(const pb_linear_combination<FieldT> &lc);
    FieldT lc_val(const pb_linear_combination<FieldT> &lc) const;

    void add_r1cs_constraint(const r1cs_constraint<FieldT> &constr, const std::string &annotation="");
    void augment_variable_annotation(const pb_variable<FieldT> &v, const std::string &postfix);
    bool is_satisfied() const;
    void dump_variables() const;

    size_t num_constraints() const;
    size_t num_inputs() const;
    size_t num_variables() const;

    void set_input_sizes(const size_t primary_input_size);

    r1cs_variable_assignment<FieldT> full_variable_assignment() const;
    r1cs_primary_input<FieldT> primary_input() const;
    r1cs_auxiliary_input<FieldT> auxiliary_input() const;
    r1cs_constraint_system<FieldT> get_constraint_system() const;

    friend class pb_variable<FieldT>;
    friend class pb_linear_combination<FieldT>;

private:
    var_index_t allocate_var_index(const std::string &annotation="");
    lc_index_t allocate_lc_index();
};

} // libsnark
use  <libsnark/gadgetlib1/protoboard.tcc>
#endif // PROTOBOARD_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef PROTOBOARD_TCC_
#define PROTOBOARD_TCC_

use  <cstdarg>
use  <cstdio>

use  <libff/common/profiling.hpp>

namespace libsnark {

template<typename FieldT>
protoboard<FieldT>::protoboard()
{
    constant_term = FieldT::one();

#ifdef DEBUG
    constraint_system.variable_annotations[0] = "ONE";
#endif

    next_free_var = 1; /* to account for constant 1 term */
    next_free_lc = 0;
}

template<typename FieldT>
void protoboard<FieldT>::clear_values()
{
    std::fill(values.begin(), values.end(), FieldT::zero());
}

template<typename FieldT>
var_index_t protoboard<FieldT>::allocate_var_index(const std::string &annotation)
{
#ifdef DEBUG
    assert(annotation != "");
    constraint_system.variable_annotations[next_free_var] = annotation;
#else
    libff::UNUSED(annotation);
#endif
    ++constraint_system.auxiliary_input_size;
    values.emplace_back(FieldT::zero());
    return next_free_var++;
}

template<typename FieldT>
lc_index_t protoboard<FieldT>::allocate_lc_index()
{
    lc_values.emplace_back(FieldT::zero());
    return next_free_lc++;
}

template<typename FieldT>
FieldT& protoboard<FieldT>::val(const pb_variable<FieldT> &var)
{
    assert(var.index <= values.size());
    return (var.index == 0 ? constant_term : values[var.index-1]);
}

template<typename FieldT>
FieldT protoboard<FieldT>::val(const pb_variable<FieldT> &var) const
{
    assert(var.index <= values.size());
    return (var.index == 0 ? constant_term : values[var.index-1]);
}

template<typename FieldT>
FieldT& protoboard<FieldT>::lc_val(const pb_linear_combination<FieldT> &lc)
{
    if (lc.is_variable)
    {
        return this->val(pb_variable<FieldT>(lc.index));
    }
    else
    {
        assert(lc.index < lc_values.size());
        return lc_values[lc.index];
    }
}

template<typename FieldT>
FieldT protoboard<FieldT>::lc_val(const pb_linear_combination<FieldT> &lc) const
{
    if (lc.is_variable)
    {
        return this->val(pb_variable<FieldT>(lc.index));
    }
    else
    {
        assert(lc.index < lc_values.size());
        return lc_values[lc.index];
    }
}

template<typename FieldT>
void protoboard<FieldT>::add_r1cs_constraint(const r1cs_constraint<FieldT> &constr, const std::string &annotation)
{
#ifdef DEBUG
    assert(annotation != "");
    constraint_system.constraint_annotations[constraint_system.constraints.size()] = annotation;
#else
    libff::UNUSED(annotation);
#endif
    constraint_system.constraints.emplace_back(constr);
}

template<typename FieldT>
void protoboard<FieldT>::augment_variable_annotation(const pb_variable<FieldT> &v, const std::string &postfix)
{
#ifdef DEBUG
    auto it = constraint_system.variable_annotations.find(v.index);
    constraint_system.variable_annotations[v.index] = (it == constraint_system.variable_annotations.end() ? "" : it->second + " ") + postfix;
#endif
}

template<typename FieldT>
bool protoboard<FieldT>::is_satisfied() const
{
    return constraint_system.is_satisfied(primary_input(), auxiliary_input());
}

template<typename FieldT>
void protoboard<FieldT>::dump_variables() const
{
#ifdef DEBUG
    for (size_t i = 0; i < constraint_system.num_variables; ++i)
    {
        printf("%-40s --> ", constraint_system.variable_annotations[i].c_str());
        values[i].as_bigint().print_hex();
    }
#endif
}

template<typename FieldT>
size_t protoboard<FieldT>::num_constraints() const
{
    return constraint_system.num_constraints();
}

template<typename FieldT>
size_t protoboard<FieldT>::num_inputs() const
{
    return constraint_system.num_inputs();
}

template<typename FieldT>
size_t protoboard<FieldT>::num_variables() const
{
    return next_free_var - 1;
}

template<typename FieldT>
void protoboard<FieldT>::set_input_sizes(const size_t primary_input_size)
{
    assert(primary_input_size <= num_variables());
    constraint_system.primary_input_size = primary_input_size;
    constraint_system.auxiliary_input_size = num_variables() - primary_input_size;
}

template<typename FieldT>
r1cs_variable_assignment<FieldT> protoboard<FieldT>::full_variable_assignment() const
{
    return values;
}

template<typename FieldT>
r1cs_primary_input<FieldT> protoboard<FieldT>::primary_input() const
{
    return r1cs_primary_input<FieldT>(values.begin(), values.begin() + num_inputs());
}

template<typename FieldT>
r1cs_auxiliary_input<FieldT> protoboard<FieldT>::auxiliary_input() const
{
    return r1cs_auxiliary_input<FieldT>(values.begin() + num_inputs(), values.end());
}

template<typename FieldT>
r1cs_constraint_system<FieldT> protoboard<FieldT>::get_constraint_system() const
{
    return constraint_system;
}

} // libsnark
#endif // PROTOBOARD_TCC_
