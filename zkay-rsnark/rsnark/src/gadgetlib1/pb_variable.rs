use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::FieldTConfig;
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, linear_term, variable,
};
use ffec::common::utils;
use ffec::common::utils::bit_vector;

pub type lc_index_t = usize;

#[derive(Clone, Default, PartialEq)]
pub struct pb_variable;
// <FieldT> {
// public variable<FieldT>
// pb_variable(let index= 0)->Self variable<FieldT>(index) {};

// pub fn  allocate(pb:&protoboard<FieldT>, annotation:&String="");
// }
// pub trait ContentsConfig {
//     type contents;
// }
// impl ContentsConfig for pb_variable_array<FieldT> {
//     type contents = Vec<pb_variable<FieldT>>;
// }
impl SubVariableConfig for pb_variable {}
// : private Vec<pb_variable<FieldT> >
pub struct pb_variable_array<FieldT: FieldTConfig> {
    pub contents: Vec<variable<FieldT, pb_variable>>,
    // pb_variable_array()->Self contents() {};
    // pb_variable_array(usize count, value:&pb_variable<FieldT>)->Self contents(count, value) {};
    // pb_variable_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_variable_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};
    // pub fn  allocate(pb:&protoboard<FieldT>, n:usize, annotation_prefix:&String="");

    // pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

/* index 0 corresponds to the constant term (used in legacy code) */
// #define ONE pb_variable<FieldT>(0)

#[derive(Clone, Default, PartialEq)]
pub struct pb_linear_combination {
    // : public linear_combination<FieldT>
    pub is_variable: bool,
    pub index: lc_index_t,
    // pb_linear_combination();
    // pb_linear_combination(var:&pb_variable<FieldT>);

    // pub fn  assign(pb:&protoboard<FieldT>, lc:&linear_combination<FieldT>);
    // pub fn  evaluate(pb:&protoboard<FieldT>) ;

    // bool is_constant() ;
    // FieldT constant_term() ;
}
// impl ContentsConfig for pb_linear_combination_array<FieldT> {
//     type contents = Vec<pb_linear_combination<FieldT>>;
// }
//
pub struct pb_linear_combination_array<FieldT: FieldTConfig> {
    //: private
    pub contents: Vec<linear_combination<FieldT, pb_variable, pb_linear_combination>>,
    // pb_linear_combination_array()->Self contents() {};
    // pb_linear_combination_array(arr:&arr:&pb_variable_array<FieldT>) { for v in self.push(pb_linear_combination<FieldT>(v)); };
    // pb_linear_combination_array(usize count)->Self contents(count) {};
    // pb_linear_combination_array(usize count, value:&pb_linear_combination<FieldT>)->Self contents(count, value) {};
    // pb_linear_combination_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_linear_combination_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};

    // pub fn  evaluate(pb:&protoboard<FieldT>) ;

    // pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>,  i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:protoboard<FieldT>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

// linear_combination<FieldT> pb_sum(v:&pb_linear_combination_array<FieldT>);

// linear_combination<FieldT> pb_packing_sum(v:&pb_linear_combination_array<FieldT>);

// linear_combination<FieldT> pb_coeff_sum(v:&pb_linear_combination_array<FieldT>, coeffs:&Vec<FieldT>);

// use ffec::common::utils;

impl<FieldT: FieldTConfig> variable<FieldT, pb_variable> {
    pub fn allocate(&mut self, pb: &mut protoboard<FieldT>, annotation: String) {
        self.index = pb.allocate_var_index(annotation);
    }
}

impl<FieldT: FieldTConfig> pb_variable_array<FieldT> {
    /* allocates pb_variable<FieldT> array in MSB->LSB order */

    pub fn allocate(&mut self, pb: &mut protoboard<FieldT>, n: usize, annotation_prefix: &String) {
        // #ifdef DEBUG
        assert!(annotation_prefix != "");
        //#endif
        self.contents
            .resize(n, variable::<FieldT, pb_variable>::default());

        for i in 0..n {
            self.contents[i].allocate(pb, format!("{annotation_prefix}_{}", i));
        }
    }

    pub fn fill_with_field_elements(&self, pb: &mut protoboard<FieldT>, vals: &Vec<FieldT>) {
        assert!(self.contents.len() == vals.len());
        for i in 0..vals.len() {
            *pb.val_ref(&self.contents[i]) = vals[i].clone();
        }
    }

    pub fn fill_with_bits(&self, pb: &mut protoboard<FieldT>, bits: &bit_vector) {
        assert!(self.contents.len() == bits.len());
        for i in 0..bits.len() {
            *pb.val_ref(&self.contents[i]) = if bits[i] {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_field_element(&self, pb: &mut protoboard<FieldT>, r: &FieldT)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        let rint = r.as_bigint::<{ FieldT::num_limbs as usize }>();
        for i in 0..self.contents.len() {
            *pb.val_ref(&self.contents[i]) = if rint.test_bit(i) {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_ulong(&self, pb: &mut protoboard<FieldT>, i: u64)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.fill_with_bits_of_field_element(pb, &FieldT::from_int(i, true));
    }

    pub fn get_vals(&self, pb: &protoboard<FieldT>) -> Vec<FieldT> {
        let mut result = Vec::with_capacity(self.contents.len());
        for i in 0..self.contents.len() {
            result[i] = pb.val(&self.contents[i]);
        }
        return result;
    }

    pub fn get_bits(&self, pb: &protoboard<FieldT>) -> bit_vector {
        let mut result = bit_vector::new();
        for i in 0..self.contents.len() {
            let v = pb.val(&self.contents[i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result.push(v == FieldT::one());
        }
        return result;
    }

    pub fn get_field_element_from_bits(&self, pb: &protoboard<FieldT>) -> FieldT {
        let mut result = FieldT::zero();

        for i in 0..self.contents.len() {
            /* push in the new bit */
            let v = pb.val(&self.contents[self.contents.len() - 1 - i]);
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

    pub fn new2<FieldT: FieldTConfig>(
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
    pub fn assign(
        &mut self,
        pb: &mut protoboard<FieldT>,
        lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    ) {
        assert!(!self.t.is_variable);
        self.t.index = pb.allocate_lc_index();
        self.terms = lc.terms.clone();
    }

    pub fn evaluatep(&self, pb: &mut protoboard<FieldT>) {
        if self.t.is_variable {
            return; // do nothing
        }

        let mut sum = FieldT::zero();
        for term in &self.terms {
            sum += term.coeff.clone() * pb.val(&term.index);
        }

        *pb.lc_val_ref(self) = sum;
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

impl<FieldT: FieldTConfig> pb_linear_combination_array<FieldT> {
    pub fn evaluate(&self, pb: &mut protoboard<FieldT>) {
        for i in 0..self.contents.len() {
            self.contents[i].evaluatep(pb);
        }
    }

    pub fn fill_with_field_elements(&self, pb: &mut protoboard<FieldT>, vals: &Vec<FieldT>) {
        assert!(self.contents.len() == vals.len());
        for i in 0..vals.len() {
            *pb.lc_val_ref(&self.contents[i]) = vals[i].clone();
        }
    }

    pub fn fill_with_bits(&self, pb: &mut protoboard<FieldT>, bits: &bit_vector) {
        assert!(self.contents.len() == bits.len());
        for i in 0..bits.len() {
            *pb.lc_val_ref(&self.contents[i]) = if bits[i] {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_field_element(&self, pb: &mut protoboard<FieldT>, r: &FieldT)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        let rint = r.as_bigint::<{ FieldT::num_limbs as usize }>();
        for i in 0..self.contents.len() {
            *pb.lc_val_ref(&self.contents[i]) = if rint.test_bit(i) {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }
    }

    pub fn fill_with_bits_of_ulong(&self, pb: &mut protoboard<FieldT>, i: usize)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.fill_with_bits_of_field_element(pb, &FieldT::from(i));
    }

    pub fn get_vals(&self, pb: &protoboard<FieldT>) -> Vec<FieldT> {
        let mut result = Vec::with_capacity(self.contents.len());
        for i in 0..self.contents.len() {
            result[i] = pb.lc_val(&self.contents[i]);
        }
        return result;
    }

    pub fn get_bits(&self, pb: &protoboard<FieldT>) -> bit_vector {
        let mut result = bit_vector::new();
        for i in 0..self.contents.len() {
            let v = pb.lc_val(&self.contents[i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result.push(v == FieldT::one());
        }
        return result;
    }

    pub fn get_field_element_from_bits(&self, pb: &protoboard<FieldT>) -> FieldT {
        let mut result = FieldT::zero();

        for i in 0..self.contents.len() {
            /* push in the new bit */
            let v = pb.lc_val(&self.contents[self.contents.len() - 1 - i]);
            assert!(v == FieldT::zero() || v == FieldT::one());
            result += result.clone() + v.clone();
        }

        return result;
    }
}

pub fn pb_sum<FieldT: FieldTConfig, SV: SubVariableConfig>(
    v: &pb_linear_combination_array<FieldT>,
) -> linear_combination<FieldT, pb_variable, pb_linear_combination> {
    let mut result = linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(0);
    for term in &v.contents {
        result = term.clone() + result;
    }

    return result;
}

pub fn pb_packing_sum<FieldT: FieldTConfig>(
    v: &pb_linear_combination_array<FieldT>,
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

pub fn pb_coeff_sum<FieldT: FieldTConfig>(
    v: &pb_linear_combination_array<FieldT>,
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
