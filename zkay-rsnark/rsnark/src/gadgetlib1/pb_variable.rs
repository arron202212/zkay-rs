use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, linear_term, variable,
};
use ffec::FieldTConfig;
use ffec::common::utils;
use ffec::common::utils::bit_vector;
use rccell::RcCell;
use std::borrow::Borrow;
use std::marker::PhantomData;

pub type lc_index_t = usize;

#[derive(Clone, Default, PartialEq)]
pub struct pb_variable;
// <FieldT> {
// public variable<FieldT>
// pb_variable(let index= 0)->Self variable<FieldT>(index) {};

// pub fn  allocate(pb:&RcCell<protoboard<FieldT,PB>>, annotation:&String="");
// }
// pub trait ContentsConfig {
//     type contents;
// }
// impl ContentsConfig for pb_variable_array<FieldT,PB> {
//     type contents = Vec<pb_variable<FieldT>>;
// }
impl SubVariableConfig for pb_variable {}
// : private Vec<pb_variable<FieldT> >
#[derive(Default, Clone)]
pub struct pb_variable_array<FieldT: FieldTConfig, PB: PBConfig> {
    pub contents: Vec<variable<FieldT, pb_variable>>,
    _pb: PhantomData<PB>,
    // pb_variable_array()->Self contents() {};
    // pb_variable_array(usize count, value:&pb_variable<FieldT>)->Self contents(count, value) {};
    // pb_variable_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_variable_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};
    // pub fn  allocate(pb:&RcCell<protoboard<FieldT,PB>>, n:usize, annotation_prefix:&String="");

    // pub fn  fill_with_field_elements(pb:&RcCell<protoboard<FieldT,PB>>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&RcCell<protoboard<FieldT,PB>>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&RcCell<protoboard<FieldT,PB>>, i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:&RcCell<protoboard<FieldT,PB>>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&RcCell<protoboard<FieldT,PB>>) ;
    // bit_vector get_bits(pb:&RcCell<protoboard<FieldT,PB>>) ;

    // FieldT get_field_element_from_bits(pb:&RcCell<protoboard<FieldT,PB>>) ;
}

/* index 0 corresponds to the constant term (used in legacy code) */
// pub const  ONE<FieldT:FieldTConfig>:variable<FieldT,pb_variable>= <variable::<FieldT,pb_variable> as std::default::Default>::default();
pub const ONE: usize = 0;

#[derive(Clone, Default, PartialEq)]
pub struct pb_linear_combination {
    // : public linear_combination<FieldT>
    pub is_variable: bool,
    pub index: lc_index_t,
    // pb_linear_combination();
    // pb_linear_combination(var:&pb_variable<FieldT>);

    // pub fn  assign(pb:&RcCell<protoboard<FieldT,PB>>, lc:&linear_combination<FieldT>);
    // pub fn  evaluate(pb:&RcCell<protoboard<FieldT,PB>>) ;

    // bool is_constant() ;
    // FieldT constant_term() ;
}
// impl ContentsConfig for pb_linear_combination_array<FieldT,PB> {
//     type contents = Vec<linear_combination<FieldT, pb_variable, pb_linear_combination>>;
// }
//
impl<FieldT: FieldTConfig, PB: PBConfig> From<pb_variable_array<FieldT, PB>>
    for pb_linear_combination_array<FieldT, PB>
{
    fn from(rhs: pb_variable_array<FieldT, PB>) -> Self {
        Self {
            contents: rhs
                .contents
                .iter()
                .cloned()
                .map(|v| linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(v))
                .collect(),
            _pb: PhantomData,
        }
    }
}
#[derive(Clone, Default)]
pub struct pb_linear_combination_array<FieldT: FieldTConfig, PB: PBConfig> {
    //: private
    pub contents: Vec<linear_combination<FieldT, pb_variable, pb_linear_combination>>,
    _pb: PhantomData<PB>,
    // pb_linear_combination_array()->Self contents() {};
    // pb_linear_combination_array(arr:&arr:&pb_variable_array<FieldT,PB>) { for v in self.push(linear_combination<FieldT, pb_variable, pb_linear_combination>(v)); };
    // pb_linear_combination_array(usize count)->Self contents(count) {};
    // pb_linear_combination_array(usize count, value:&linear_combination<FieldT, pb_variable, pb_linear_combination>)->Self contents(count, value) {};
    // pb_linear_combination_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_linear_combination_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};

    // pub fn  evaluate(pb:&RcCell<protoboard<FieldT,PB>>) ;

    // pub fn  fill_with_field_elements(pb:&RcCell<protoboard<FieldT,PB>>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&RcCell<protoboard<FieldT,PB>>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&RcCell<protoboard<FieldT,PB>>,  i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:RcCell<protoboard<FieldT,PB>>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&RcCell<protoboard<FieldT,PB>>) ;
    // bit_vector get_bits(pb:&RcCell<protoboard<FieldT,PB>>) ;

    // FieldT get_field_element_from_bits(pb:&RcCell<protoboard<FieldT,PB>>) ;
}

// linear_combination<FieldT> pb_sum(v:&pb_linear_combination_array<FieldT>);

// linear_combination<FieldT> pb_packing_sum(v:&pb_linear_combination_array<FieldT>);

// linear_combination<FieldT> pb_coeff_sum(v:&pb_linear_combination_array<FieldT>, coeffs:&Vec<FieldT>);

// use ffec::common::utils;

impl<FieldT: FieldTConfig> variable<FieldT, pb_variable> {
    pub fn allocate<PB: PBConfig, T: Borrow<str>>(
        &mut self,
        pb: &RcCell<protoboard<FieldT, PB>>,
        annotation: T,
    ) {
        self.index = pb
            .borrow_mut()
            .allocate_var_index(annotation.borrow().to_string());
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> IntoIterator for pb_variable_array<FieldT, PB> {
    type Item = variable<FieldT, pb_variable>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

use std::ops::{Index, IndexMut};
impl<FieldT: FieldTConfig, PB: PBConfig> Index<usize> for pb_variable_array<FieldT, PB> {
    type Output = variable<FieldT, pb_variable>;

    fn index(&self, index: usize) -> &Self::Output {
        self.contents.get(index).unwrap()
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> IndexMut<usize> for pb_variable_array<FieldT, PB> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.contents.get_mut(index).unwrap()
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> pb_variable_array<FieldT, PB> {
    pub fn len(&self) -> usize {
        self.contents.len()
    }
    pub fn iter(&self) -> std::slice::Iter<variable<FieldT, pb_variable>> {
        self.contents.iter()
    }
    /* allocates pb_variable<FieldT> array in MSB->LSB order */
    pub fn new(contents: Vec<variable<FieldT, pb_variable>>) -> Self {
        Self {
            contents,
            _pb: PhantomData,
        }
    }
    pub fn allocate<T: Borrow<str>>(
        &mut self,
        pb: &RcCell<protoboard<FieldT, PB>>,
        n: usize,
        annotation_prefix: T,
    ) {
        // #ifdef DEBUG
        assert!(!annotation_prefix.borrow().is_empty());
        //#endif
        self.contents
            .resize(n, variable::<FieldT, pb_variable>::default());

        for i in 0..n {
            self.contents[i].allocate(&pb, format!("{}_{}", annotation_prefix.borrow(), i));
        }
    }

    pub fn fill_with_field_elements(
        &self,
        pb: &RcCell<protoboard<FieldT, PB>>,
        vals: &Vec<FieldT>,
    ) {
        assert!(self.contents.len() == vals.len());
        for i in 0..vals.len() {
            *pb.borrow_mut().val_ref(&self.contents[i]) = vals[i].clone();
        }
    }

    pub fn fill_with_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>, bits: &bit_vector) {
        assert!(self.contents.len() == bits.len());
        for i in 0..bits.len() {
            *pb.borrow_mut().val_ref(&self.contents[i]) = if bits[i] {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_field_element(&self, pb: &RcCell<protoboard<FieldT, PB>>, r: &FieldT) {
        let rint = r.as_bigint::<4>(); //{ FieldT::num_limbs as usize }>
        for i in 0..self.contents.len() {
            *pb.borrow_mut().val_ref(&self.contents[i]) = if rint.test_bit(i) {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_ulong(&self, pb: &RcCell<protoboard<FieldT, PB>>, i: u64) {
        self.fill_with_bits_of_field_element(&pb, &FieldT::from_int(i, true));
    }

    pub fn get_vals(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> Vec<FieldT> {
        let mut result = Vec::with_capacity(self.contents.len());
        for i in 0..self.contents.len() {
            result[i] = pb.borrow().val(&self.contents[i]);
        }
        return result;
    }

    pub fn get_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> bit_vector {
        let mut result = bit_vector::new();
        for i in 0..self.contents.len() {
            let v = pb.borrow().val(&self.contents[i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result.push(v == FieldT::one());
        }
        return result;
    }

    pub fn get_field_element_from_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> FieldT {
        let mut result = FieldT::zero();

        for i in 0..self.contents.len() {
            /* push in the new bit */
            let v = pb.borrow().val(&self.contents[self.contents.len() - 1 - i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result += result.clone() + v.clone();
        }

        return result;
    }
}

impl pb_linear_combination {
    pub fn new<FieldT: FieldTConfig>() -> linear_combination<FieldT, pb_variable, Self> {
        let t = Self {
            is_variable: false,
            index: 0,
        };
        linear_combination { terms: vec![], t }
    }

    pub fn new_with_var<FieldT: FieldTConfig>(
        var: variable<FieldT, pb_variable>,
    ) -> linear_combination<FieldT, pb_variable, Self> {
        let t = Self {
            is_variable: true,
            index: var.index,
        };
        linear_combination {
            terms: vec![linear_term::<FieldT, pb_variable>::from(var)],
            t,
        }
    }
}
impl SubLinearCombinationConfig for pb_linear_combination {}

impl<FieldT: FieldTConfig> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    pub fn assign<PB: PBConfig>(
        &mut self,
        pb: &RcCell<protoboard<FieldT, PB>>,
        lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    ) {
        assert!(!self.t.is_variable);
        self.t.index = pb.borrow_mut().allocate_lc_index();
        self.terms = lc.terms.clone();
    }

    pub fn evaluate_pb<PB: PBConfig>(&self, pb: &RcCell<protoboard<FieldT, PB>>) {
        if self.t.is_variable {
            return; // do nothing
        }

        let mut sum = FieldT::zero();
        for term in &self.terms {
            sum += term.coeff.clone() * pb.borrow().val(&term.index);
        }

        *pb.borrow_mut().lc_val_ref(self) = sum;
    }

    pub fn is_constant(&self) -> bool {
        if self.t.is_variable {
            return (self.t.index == 0);
        }
        for term in &self.terms {
            if term.index.index != 0 {
                return false;
            }
        }

        true
    }

    pub fn constant_term(&self) -> FieldT {
        if self.t.is_variable {
            return if self.t.index == 0 {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
        let mut result = FieldT::zero();
        for term in &self.terms {
            if term.index.index == 0 {
                result += term.coeff.clone();
            }
        }
        return result;
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> IntoIterator for pb_linear_combination_array<FieldT, PB> {
    type Item = linear_combination<FieldT, pb_variable, pb_linear_combination>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> Index<usize> for pb_linear_combination_array<FieldT, PB> {
    type Output = linear_combination<FieldT, pb_variable, pb_linear_combination>;

    fn index(&self, index: usize) -> &Self::Output {
        self.contents.get(index).unwrap()
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> IndexMut<usize>
    for pb_linear_combination_array<FieldT, PB>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.contents.get_mut(index).unwrap()
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> pb_linear_combination_array<FieldT, PB> {
    pub fn new(
        contents: Vec<linear_combination<FieldT, pb_variable, pb_linear_combination>>,
    ) -> Self {
        Self {
            contents,
            _pb: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.contents.len()
    }
    pub fn iter(
        &self,
    ) -> std::slice::Iter<linear_combination<FieldT, pb_variable, pb_linear_combination>> {
        self.contents.iter()
    }
    pub fn evaluate(&self, pb: &RcCell<protoboard<FieldT, PB>>) {
        for i in 0..self.contents.len() {
            self.contents[i].evaluate_pb(pb);
        }
    }

    pub fn fill_with_field_elements(
        &self,
        pb: &RcCell<protoboard<FieldT, PB>>,
        vals: &Vec<FieldT>,
    ) {
        assert!(self.contents.len() == vals.len());
        for i in 0..vals.len() {
            *pb.borrow_mut().lc_val_ref(&self.contents[i]) = vals[i].clone();
        }
    }

    pub fn fill_with_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>, bits: &bit_vector) {
        assert!(self.contents.len() == bits.len());
        for i in 0..bits.len() {
            *pb.borrow_mut().lc_val_ref(&self.contents[i]) = if bits[i] {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_field_element(&self, pb: &RcCell<protoboard<FieldT, PB>>, r: &FieldT) {
        let rint = r.as_bigint::<4>(); //{ FieldT::num_limbs as usize }
        for i in 0..self.contents.len() {
            *pb.borrow_mut().lc_val_ref(&self.contents[i]) = if rint.test_bit(i) {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_ulong(&self, pb: &RcCell<protoboard<FieldT, PB>>, i: usize) {
        self.fill_with_bits_of_field_element(&pb, &FieldT::from(i));
    }

    pub fn get_vals(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> Vec<FieldT> {
        let mut result = Vec::with_capacity(self.contents.len());
        for i in 0..self.contents.len() {
            result[i] = pb.borrow().lc_val(&self.contents[i]);
        }
        return result;
    }

    pub fn get_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> bit_vector {
        let mut result = bit_vector::new();
        for i in 0..self.contents.len() {
            let v = pb.borrow().lc_val(&self.contents[i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result.push(v == FieldT::one());
        }
        return result;
    }

    pub fn get_field_element_from_bits(&self, pb: &RcCell<protoboard<FieldT, PB>>) -> FieldT {
        let mut result = FieldT::zero();

        for i in 0..self.contents.len() {
            /* push in the new bit */
            let v = pb
                .borrow()
                .lc_val(&self.contents[self.contents.len() - 1 - i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result += result.clone() + v.clone();
        }

        return result;
    }
}

pub fn pb_sum<FieldT: FieldTConfig, PB: PBConfig, SV: SubVariableConfig>(
    v: &pb_linear_combination_array<FieldT, PB>,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    let mut result =
        linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(FieldT::from(0));
    for term in &v.contents {
        result = term.clone() + result;
    }

    return result;
}

pub fn pb_packing_sum<FieldT: FieldTConfig, PB: PBConfig>(
    v: &pb_linear_combination_array<FieldT, PB>,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    let mut twoi = FieldT::one(); // will hold 2^i entering each iteration
    let mut all_terms = vec![]; //Vec<linear_term<FieldT> > 
    for lc in &v.contents {
        for term in &lc.terms {
            all_terms.push(term.clone() * twoi.clone());
        }
        twoi += twoi.clone();
    }

    return linear_combination::<FieldT, pb_variable, pb_linear_combination>::new(all_terms);
}

pub fn pb_coeff_sum<FieldT: FieldTConfig, PB: PBConfig>(
    v: &pb_linear_combination_array<FieldT, PB>,
    coeffs: &Vec<FieldT>,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    assert!(v.contents.len() == coeffs.len());
    let mut all_terms = vec![]; //Vec<linear_term<FieldT> >

    let mut coeff_it = coeffs.iter();
    for lc in &v.contents {
        for term in &lc.terms {
            all_terms.push(term.clone() * (coeff_it.next().unwrap().clone()));
        }
    }

    return linear_combination::<FieldT, pb_variable, pb_linear_combination>::new(all_terms);
}

//#endif // PB_VARIABLE_TCC
