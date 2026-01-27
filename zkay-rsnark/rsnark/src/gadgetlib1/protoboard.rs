use crate::gadgetlib1::pb_variable::{lc_index_t, pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
    r1cs_variable_assignment,
};
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, var_index_t, variable,
};
use ffec::FieldTConfig;
use ffec::common::utils;
use ffec::scalar_multiplication::multiexp::KCConfig;
use rccell::RcCell;

pub trait PBConfig: Default + Clone + Sized {
    fn new_with_ap<APT: ArchitectureParamsTypeConfig>(ap: APT) -> Self {
        Default::default()
    }
}

pub trait ProtoboardConfig: Default + Clone + Sized {
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    fn clear_values(&mut self);
    fn val_ref(&mut self, var: &variable<Self::FieldT, pb_variable>) -> &mut Self::FieldT;
    fn val(&self, var: &variable<Self::FieldT, pb_variable>) -> Self::FieldT;
    fn lc_val_ref(
        &mut self,
        lc: &linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
    ) -> &mut Self::FieldT;
    fn lc_val(
        &self,
        lc: &linear_combination<Self::FieldT, pb_variable, pb_linear_combination>,
    ) -> Self::FieldT;
    fn num_variables(&self) -> usize;
    fn full_variable_assignment(&self) -> Vec<Self::FieldT>;
    fn get_constraint_system(
        &self,
    ) -> r1cs_constraint_system<Self::FieldT, pb_variable, pb_linear_combination>;
    fn add_r1cs_constraint(
        &mut self,
        constr: r1cs_constraint<Self::FieldT, pb_variable, pb_linear_combination>,
        annotation: String,
    );
    fn num_constraints(&self) -> usize;
    fn set_input_sizes(&mut self, primary_input_size: usize);

    fn primary_input(&self) -> r1cs_primary_input<Self::FieldT>;

    fn auxiliary_input(&self) -> r1cs_auxiliary_input<Self::FieldT>;
    fn into_p(self) -> RcCell<protoboard<Self::FieldT, Self::PB>> {
        RcCell::new(Default::default())
    }
    fn new_with_ap<APT: ArchitectureParamsTypeConfig>(ap: APT) -> Self;
    fn ap<APT: ArchitectureParamsTypeConfig>(&self) -> APT {
        Default::default()
    }
}

#[derive(Clone)]
pub struct protoboard<FieldT: FieldTConfig, T: PBConfig> {
    pub constant_term: FieldT, /* only here, because pb.borrow().val(&) needs to be able to return reference to the constant 1 term */
    pub values: r1cs_variable_assignment<FieldT>, /* values[0] will hold the value of the first allocated variable of the protoboard, *NOT* constant 1 */
    pub next_free_var: var_index_t,
    pub next_free_lc: lc_index_t,
    pub lc_values: Vec<FieldT>,
    pub constraint_system: r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
    pub t: T,
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

use crate::common::data_structures::merkle_tree::PConfig;
use ffec::common::profiling;
impl<FieldT: FieldTConfig, T: PBConfig> PConfig for protoboard<FieldT, T> {}

impl<FieldT: FieldTConfig, T: PBConfig> Default for protoboard<FieldT, T> {
    fn default() -> Self {
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
            t: T::default(),
        }
    }
}
// impl<FieldT: FieldTConfig, T: PBConfig,P:ProtoboardConfig> From<RcCell<P>> for  RcCell<protoboard<FieldT, T>> {
//     fn from(rhs:P)->Self{
//        RcCell::new (Default::default())
//     }
//
impl<FieldT: FieldTConfig, T: PBConfig> protoboard<FieldT, T> {
    pub fn new(t: T) -> Self {
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
            t,
        }
    }

    pub fn allocate_var_index(&mut self, annotation: String) -> var_index_t {
        // #ifdef DEBUG
        assert!(!annotation.is_empty());
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

    pub fn num_inputs(&self) -> usize {
        self.constraint_system.num_inputs()
    }
}
impl<FieldT: FieldTConfig, T: PBConfig> ProtoboardConfig for protoboard<FieldT, T> {
    type FieldT = FieldT;
    type PB = T;
    fn set_input_sizes(&mut self, primary_input_size: usize) {
        assert!(self.constraint_system.primary_input_size <= self.num_variables());
        self.constraint_system.primary_input_size = primary_input_size;
        self.constraint_system.auxiliary_input_size = self.num_variables() - primary_input_size;
    }

    fn primary_input(&self) -> r1cs_primary_input<FieldT> {
        self.values[..self.num_inputs()].to_vec()
    }

    fn auxiliary_input(&self) -> r1cs_auxiliary_input<FieldT> {
        self.values[self.num_inputs()..].to_vec()
    }
    fn lc_val_ref(
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

    fn lc_val(
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
    fn new_with_ap<APT: ArchitectureParamsTypeConfig>(ap: APT) -> Self {
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
            t: T::new_with_ap(ap),
        }
    }
    fn clear_values(&mut self) {
        self.values.fill(FieldT::zero());
    }
    fn val_ref(&mut self, var: &variable<FieldT, pb_variable>) -> &mut FieldT {
        assert!(var.index <= self.values.len());
        if var.index == 0 {
            &mut self.constant_term
        } else {
            &mut self.values[var.index - 1]
        }
    }

    fn val(&self, var: &variable<FieldT, pb_variable>) -> FieldT {
        assert!(var.index <= self.values.len());
        return if var.index == 0 {
            self.constant_term.clone()
        } else {
            self.values[var.index - 1].clone()
        };
    }

    fn num_variables(&self) -> usize {
        self.next_free_var - 1
    }
    fn full_variable_assignment(&self) -> r1cs_variable_assignment<FieldT> {
        self.values.clone()
    }
    fn get_constraint_system(
        &self,
    ) -> r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination> {
        self.constraint_system.clone()
    }
    fn add_r1cs_constraint(
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
    fn num_constraints(&self) -> usize {
        self.constraint_system.num_constraints()
    }
}
