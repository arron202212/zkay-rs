use crate::gadgetlib1::pb_variable::{lc_index_t, pb_linear_combination, pb_variable};
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
    r1cs_variable_assignment,
};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, var_index_t, variable,
};
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
//
use ffec::common::utils;
//
// pub struct r1cs_constraint;

//
// pub struct r1cs_constraint_system;

//
pub struct protoboard<FieldT: FieldTConfig> {
    //
    pub constant_term: FieldT, /* only here, because pb.val() needs to be able to return reference to the constant 1 term */
    pub values: r1cs_variable_assignment<FieldT>, /* values[0] will hold the value of the first allocated variable of the protoboard, *NOT* constant 1 */
    pub next_free_var: var_index_t,
    pub next_free_lc: lc_index_t,
    pub lc_values: Vec<FieldT>,
    pub constraint_system: r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
}

//
//     protoboard();

//     pub fn  clear_values();

//     FieldT& val(var:&variable<FieldT,pb_variable>);
//     FieldT val(var:&variable<FieldT,pb_variable>) const;

//     FieldT& lc_val(lc:&linear_combination<FieldT,pb_linear_combination>);
//     FieldT lc_val(lc:&linear_combination<FieldT,pb_linear_combination>) const;

//     pub fn  add_r1cs_constraint(constr:&r1cs_constraint<FieldT,SLC>, annotation:&String="");
//     pub fn  augment_variable_annotation(v:&variable<FieldT,pb_variable>, postfix:&String);
//     bool is_satisfied() const;
//     pub fn  dump_variables() const;

//     usize num_constraints() const;
//     usize num_inputs() const;
//     usize num_variables() const;

//     pub fn  set_input_sizes(primary_input_size:usize);

//     r1cs_variable_assignment<FieldT> full_variable_assignment() const;
//     r1cs_primary_input<FieldT> primary_input() const;
//     r1cs_auxiliary_input<FieldT> auxiliary_input() const;
//     r1cs_constraint_system<FieldT> get_constraint_system() const;

//     friend pub struct variable<FieldT,pb_variable>;
//     friend pub struct linear_combination<FieldT,pb_linear_combination>;

//
//     var_index_t allocate_var_index(annotation:&String="");
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

impl<FieldT: FieldTConfig> protoboard<FieldT> {
    pub fn new() -> Self {
        let mut constraint_system =
            r1cs_constraint_system::<FieldT, pb_variable, pb_linear_combination>::default();
        // #ifdef DEBUG
        constraint_system
            .variable_annotations
            .insert(0, "ONE".to_owned());
        //#endif

        // to account for constant 1 term

        Self {
            constant_term: FieldT::one(),
            values: r1cs_variable_assignment::<FieldT>::default(),
            next_free_var: 1,
            next_free_lc: 0,
            lc_values: vec![],
            constraint_system,
        }
    }

    pub fn clear_values(&mut self) {
        self.values.fill(FieldT::zero());
    }

    pub fn allocate_var_index(&mut self, annotation: String) -> var_index_t {
        // #ifdef DEBUG
        assert!(annotation != "");
        self.constraint_system
            .variable_annotations
            .insert(self.next_free_var, annotation);
        // #else
        // //ffec::UNUSED(annotation);
        //#endif
        self.constraint_system.auxiliary_input_size += 1;
        self.values.push(FieldT::zero());
        self.next_free_var += 1;
        self.next_free_var
    }

    pub fn allocate_lc_index(&mut self) -> lc_index_t {
        self.lc_values.push(FieldT::zero());
        self.next_free_lc += 1;
        self.next_free_lc
    }

    pub fn val_ref(&mut self, var: &variable<FieldT, pb_variable>) -> &mut FieldT {
        assert!(var.index <= self.values.len());
        if var.index == 0 {
            &mut self.constant_term
        } else {
            &mut self.values[var.index - 1]
        }
    }

    pub fn val(&self, var: &variable<FieldT, pb_variable>) -> FieldT {
        assert!(var.index <= self.values.len());
        return if var.index == 0 {
            self.constant_term.clone()
        } else {
            self.values[var.index - 1].clone()
        };
    }

    pub fn lc_val_ref(
        &mut self,
        lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    ) -> &mut FieldT {
        if lc.t.is_variable {
            self.val_ref(&variable::<FieldT, pb_variable>::new(
                lc.t.index,
                pb_variable::default(),
            ))
        } else {
            assert!(lc.t.index < self.lc_values.len());
            &mut self.lc_values[lc.t.index]
        }
    }

    pub fn lc_val(
        &self,
        lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    ) -> FieldT {
        if lc.t.is_variable {
            self.val(&variable::<FieldT, pb_variable>::new(
                lc.t.index,
                pb_variable,
            ))
        } else {
            assert!(lc.t.index < self.lc_values.len());
            self.lc_values[lc.t.index].clone()
        }
    }

    pub fn add_r1cs_constraint(
        &mut self,
        constr: r1cs_constraint<FieldT, pb_variable, pb_linear_combination>,
        annotation: String,
    ) {
        // #ifdef DEBUG
        assert!(!annotation.is_empty());
        self.constraint_system
            .constraint_annotations
            .insert(self.constraint_system.constraints.len(), annotation);
        // #else
        //     //ffec::UNUSED(annotation);
        //#endif
        self.constraint_system.constraints.push(constr);
    }

    pub fn augment_variable_annotation(
        &mut self,
        v: &variable<FieldT, pb_variable>,
        postfix: String,
    ) {
        // #ifdef DEBUG
        self.constraint_system.variable_annotations.insert(
            v.index,
            if let Some(it) = self.constraint_system.variable_annotations.get(&v.index) {
                format!("{it} {postfix}")
            } else {
                postfix
            },
        );
        //#endif
    }

    pub fn is_satisfied(&self) -> bool {
        self.constraint_system
            .is_satisfied(&self.primary_input(), &self.auxiliary_input())
    }

    pub fn dump_variables<const N: usize>(&self) {
        // #ifdef DEBUG
        for i in 0..self.constraint_system.num_variables() {
            print!(
                "{:<40} --> ",
                self.constraint_system.variable_annotations[&i].to_string()
            ); //%-40s
            self.values[i].as_bigint::<N>().print_hex();
        }
        //#endif
    }

    pub fn num_constraints(&self) -> usize {
        self.constraint_system.num_constraints()
    }

    pub fn num_inputs(&self) -> usize {
        self.constraint_system.num_inputs()
    }

    pub fn num_variables(&self) -> usize {
        self.next_free_var - 1
    }

    pub fn set_input_sizes(&mut self, primary_input_size: usize) {
        assert!(self.constraint_system.primary_input_size <= self.num_variables());
        self.constraint_system.primary_input_size = primary_input_size;
        self.constraint_system.auxiliary_input_size = self.num_variables() - primary_input_size;
    }

    pub fn full_variable_assignment(&self) -> r1cs_variable_assignment<FieldT> {
        self.values.clone()
    }

    pub fn primary_input(&self) -> r1cs_primary_input<FieldT> {
        self.values[..self.num_inputs()].to_vec()
    }

    pub fn auxiliary_input(&self) -> r1cs_auxiliary_input<FieldT> {
        self.values[self.num_inputs()..].to_vec()
    }

    pub fn get_constraint_system(
        &self,
    ) -> r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination> {
        self.constraint_system.clone()
    }
}

//#endif // PROTOBOARD_TCC_
