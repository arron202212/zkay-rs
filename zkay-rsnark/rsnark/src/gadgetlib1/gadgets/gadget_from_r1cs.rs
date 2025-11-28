// Declaration of interfaces for a gadget that can be created from an R1CS constraint system.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, linear_term, var_index_t,
    variable,
};
use rccell::RcCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub struct gadget_from_r1cs<
    FieldT: FieldTConfig,
    PB: PBConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    //gadget<FieldT>
    vars: Vec<pb_variable_array<FieldT, PB>>,
    cs: r1cs_constraint_system<FieldT, SV, SLC>,
    cs_to_vars: BTreeMap<usize, usize>,
}

// use crate::gadgetlib1::gadgets::gadget_from_r1cs;

//#endif // GADGET_FROM_R1CS_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a gadget that can be created from an R1CS constraint system.

See gadget_from_r1cs.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef GADGET_FROM_R1CS_TCC_
// #define GADGET_FROM_R1CS_TCC_
use crate::prefix_format;
impl<FieldT: FieldTConfig, PB: PBConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    gadget_from_r1cs<FieldT, PB, SV, SLC>
{
    pub fn new(
        mut pb: RcCell<protoboard<FieldT, PB>>,
        vars: Vec<pb_variable_array<FieldT, PB>>,
        cs: r1cs_constraint_system<FieldT, SV, SLC>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut cs_to_vars = BTreeMap::from([(0, 0)]); // constant term maps to constant term 

        let mut cs_var_idx = 1;
        for va in &vars {
            // #ifdef DEBUG
            print!(
                "gadget_from_r1cs: translating a block of variables with length {}\n",
                va.len()
            );
            //#endif
            for v in va.iter() {
                cs_to_vars.insert(cs_var_idx, v.index);

                // #ifdef DEBUG
                if v.index != 0 {
                    // handle annotations, except for re-annotating constant term
                    let mut annotation =
                        prefix_format!(annotation_prefix, " variable_{}", cs_var_idx);
                    if let Some(it) = cs.variable_annotations.get(&cs_var_idx) {
                        annotation = format!("{} {}", annotation_prefix, it);
                    }

                    pb.borrow_mut().augment_variable_annotation(&v, annotation);
                }
                //#endif
                cs_var_idx += 1;
            }
        }

        // #ifdef DEBUG
        print!(
            "gadget_from_r1cs: sum of all block lengths: {}\n",
            cs_var_idx - 1
        );
        print!(
            "gadget_from_r1cs: cs.num_variables(): {}\n",
            cs.num_variables()
        );
        //#endif

        assert!(cs_var_idx - 1 == cs.num_variables());
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                vars,
                cs,
                cs_to_vars,
            },
        )
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    gadget<FieldT, PB, gadget_from_r1cs<FieldT, PB, SV, SLC>>
{
    pub fn generate_r1cs_constraints(&mut self) {
        for i in 0..self.t.cs.num_constraints() {
            let constr = &self.t.cs.constraints[i];
            let mut translated_constr =
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::default();

            for t in &constr.a.terms {
                translated_constr
                    .a
                    .terms
                    .push(linear_term::<FieldT, pb_variable>::new_with_field(
                        variable::<FieldT, pb_variable>::from(self.t.cs_to_vars[&t.index.index]),
                        t.coeff.clone(),
                    ));
            }

            for t in &constr.b.terms {
                translated_constr
                    .b
                    .terms
                    .push(linear_term::<FieldT, pb_variable>::new_with_field(
                        variable::<FieldT, pb_variable>::from(self.t.cs_to_vars[&t.index.index]),
                        t.coeff.clone(),
                    ));
            }

            for t in &constr.c.terms {
                translated_constr
                    .c
                    .terms
                    .push(linear_term::<FieldT, pb_variable>::new_with_field(
                        variable::<FieldT, pb_variable>::from(self.t.cs_to_vars[&t.index.index]),
                        t.coeff.clone(),
                    ));
            }

            let mut annotation = prefix_format!(&self.annotation_prefix, " constraint_{}", i);

            // #ifdef DEBUG
            if let Some(it) = self.t.cs.constraint_annotations.get(&i) {
                annotation = format!("{} {}", self.annotation_prefix, it);
            }
            //#endif
            self.pb
                .borrow_mut()
                .add_r1cs_constraint(translated_constr, annotation);
        }
    }

    pub fn generate_r1cs_witness(
        &self,
        primary_input: &r1cs_primary_input<FieldT>,
        auxiliary_input: &r1cs_auxiliary_input<FieldT>,
    ) {
        assert!(self.t.cs.num_inputs() == primary_input.len());
        assert!(self.t.cs.num_variables() == primary_input.len() + auxiliary_input.len());

        for i in 0..primary_input.len() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&variable::<FieldT, pb_variable>::from(
                    self.t.cs_to_vars[&(i + 1)],
                )) = primary_input[i].clone();
        }

        for i in 0..auxiliary_input.len() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&variable::<FieldT, pb_variable>::from(
                    self.t.cs_to_vars[&(primary_input.len() + i + 1)],
                )) = auxiliary_input[i].clone();
        }
    }
}

//#endif // GADGET_FROM_R1CS_TCC_
