/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PROTOBOARD_HPP_
// #define PROTOBOARD_HPP_

// use  <algorithm>
// use  <cassert>
// use  <cstdio>
// use  <string>
// use  <vector>

use ffec::common::utils;

use crate::gadgetlib1::pb_variable;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



// 
// class r1cs_constraint;

// 
// class r1cs_constraint_system;

// 
pub struct protoboard<FieldT> {
// private:
constant_term:    FieldT, /* only here, because pb.val() needs to be able to return reference to the constant 1 term */
values:    r1cs_variable_assignment<FieldT>, /* values[0] will hold the value of the first allocated variable of the protoboard, *NOT* constant 1 */
next_free_var:    var_index_t,
next_free_lc:    lc_index_t,
lc_values:    std::vector<FieldT>,
constraint_system:    r1cs_constraint_system<FieldT>,
}

// 
//     protoboard();

//     void clear_values();

//     FieldT& val(var:&pb_variable<FieldT>);
//     FieldT val(var:&pb_variable<FieldT>) const;

//     FieldT& lc_val(lc:&pb_linear_combination<FieldT>);
//     FieldT lc_val(lc:&pb_linear_combination<FieldT>) const;

//     void add_r1cs_constraint(constr:&r1cs_constraint<FieldT>, annotation:&std::string="");
//     void augment_variable_annotation(v:&pb_variable<FieldT>, postfix:&std::string);
//     bool is_satisfied() const;
//     void dump_variables() const;

//     size_t num_constraints() const;
//     size_t num_inputs() const;
//     size_t num_variables() const;

//     void set_input_sizes(primary_input_size:size_t);

//     r1cs_variable_assignment<FieldT> full_variable_assignment() const;
//     r1cs_primary_input<FieldT> primary_input() const;
//     r1cs_auxiliary_input<FieldT> auxiliary_input() const;
//     r1cs_constraint_system<FieldT> get_constraint_system() const;

//     friend class pb_variable<FieldT>;
//     friend class pb_linear_combination<FieldT>;

// private:
//     var_index_t allocate_var_index(annotation:&std::string="");
//     lc_index_t allocate_lc_index();
// };


// use crate::gadgetlib1::protoboard;
//#endif // PROTOBOARD_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PROTOBOARD_TCC_
// #define PROTOBOARD_TCC_

// use  <cstdarg>
// use  <cstdio>

use ffec::common::profiling;


impl protoboard<FieldT>{

pub fn new()->Self
{
    constant_term = FieldT::one();

// #ifdef DEBUG
    constraint_system.variable_annotations[0] = "ONE";
//#endif

    next_free_var = 1; /* to account for constant 1 term */
    next_free_lc = 0;
}


pub fn clear_values()
{
    values.fill(FieldT::zero());
}


 pub fn allocate_var_index(annotation:&std::string)->var_index_t
{
// #ifdef DEBUG
//     assert!(annotation != "");
//     constraint_system.variable_annotations[next_free_var] = annotation;
// #else
    // ffec::UNUSED(annotation);
//#endif
    constraint_system.auxiliary_input_size+=1;
    values.push(FieldT::zero());
     next_free_var+=1;
    next_free_var
}


 pub fn allocate_lc_index()->lc_index_t
{
    lc_values.push(FieldT::zero());
     next_free_lc+=1;
     next_free_lc
}


 pub fn val(var:&pb_variable<FieldT>)->&FieldT
{
    assert!(var.index <= values.len());
    return if var.index == 0 {constant_term} else{values[var.index-1]};
}


 pub fn val(var:&pb_variable<FieldT>) ->FieldT
{
    assert!(var.index <= values.len());
    return if var.index == 0 {constant_term} else{values[var.index-1]};
}


 pub fn lc_val(lc:&pb_linear_combination<FieldT>)->&FieldT
{
    if lc.is_variable
    {
        return self.val(pb_variable::<FieldT>(lc.index));
    }
    else
    {
        assert!(lc.index < lc_values.len());
        return lc_values[lc.index];
    }
}


 pub fn lc_val(lc:&pb_linear_combination<FieldT>) ->FieldT
{
    if lc.is_variable
    {
        return self.val(pb_variable::<FieldT>(lc.index));
    }
    else
    {
        assert!(lc.index < lc_values.len());
        return lc_values[lc.index];
    }
}


pub fn add_r1cs_constraint(constr:&r1cs_constraint<FieldT>, annotation:&std::string)
{
// #ifdef DEBUG
//     assert!(annotation != "");
//     constraint_system.constraint_annotations[constraint_system.constraints.len()] = annotation;
// #else
//     ffec::UNUSED(annotation);
//#endif
    constraint_system.constraints.push(constr);
}


pub fn augment_variable_annotation(v:&pb_variable<FieldT>, postfix:&std::string)
{
// #ifdef DEBUG
    constraint_system.variable_annotations[v.index] = if let Some(it) = constraint_system.variable_annotations.find(v.index){it.1.to_string()+" "} else {""}  + postfix;
//#endif
}


 pub fn is_satisfied() ->bool
{
    return constraint_system.is_satisfied(primary_input(), auxiliary_input());
}


pub fn dump_variables() 
{
// #ifdef DEBUG
    for i in 0..constraint_system.num_variables
    {
        print!("{:<40} --> ", constraint_system.variable_annotations[i].to_string());//%-40s
        values[i].as_bigint().print_hex();
    }
//#endif
}


 pub fn num_constraints() ->usize
{
    return constraint_system.num_constraints();
}


 pub fn num_inputs() ->usize
{
    return constraint_system.num_inputs();
}


 pub fn num_variables() ->usize
{
    return next_free_var - 1;
}


pub fn set_input_sizes(primary_input_size:size_t)
{
    assert!(primary_input_size <= num_variables());
    constraint_system.primary_input_size = primary_input_size;
    constraint_system.auxiliary_input_size = num_variables() - primary_input_size;
}


 pub fn full_variable_assignment() ->r1cs_variable_assignment<FieldT>
{
    return values;
}


pub fn primary_input() ->r1cs_primary_input<FieldT> 
{
    return r1cs_primary_input::<FieldT>::new(values[..num_inputs()]);
}


 pub fn auxiliary_input() ->r1cs_auxiliary_input<FieldT>
{
    return r1cs_auxiliary_input::<FieldT>(values[num_inputs()..]);
}


pub fn get_constraint_system() ->r1cs_constraint_system<FieldT> 
{
    return constraint_system;
}

}

//#endif // PROTOBOARD_TCC_
