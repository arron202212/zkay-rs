#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// Declaration of interfaces for evaluation domains.

// Roughly, given a desired size m for the domain, the constructor selects
// a choice of domain S with size ~m that has been selected so to optimize
// - computations of Lagrange polynomials, and
// - FFT/iFFT computations.
// An evaluation domain also provides other other functions, e.g., accessing
// individual elements in S or evaluating its vanishing polynomial.

// The descriptions below make use of the definition of a *Lagrange polynomial*,
// which we recall. Given a field F, a subset S=(a_i)_i of F, and an index idx
// in {0,...,|S-1|}, the idx-th Lagrange polynomial (wrt to subset S) is defined to be
// \f[   L_{idx,S}(z)->Self= prod_{k \neq idx} (z - a_k) / prod_{k \neq idx} (a_{idx} - a_k)   \f]
// Note that, by construction:
// \f[   \forall j \neq idx: L_{idx,S}(a_{idx}) = 1  \text{ and }  L_{idx,S}(a_j) = 0   \f]

use crate::evaluation_domain::domains::arithmetic_sequence_domain::arithmetic_sequence_domains;
use crate::evaluation_domain::domains::basic_radix2_domain::basic_radix2_domains;
use crate::evaluation_domain::domains::extended_radix2_domain::extended_radix2_domains;
use crate::evaluation_domain::domains::geometric_sequence_domain::geometric_sequence_domains;
use crate::evaluation_domain::domains::step_radix2_domain::step_radix2_domains;
use enum_dispatch::enum_dispatch;
use ffec::FieldTConfig;

#[derive(Default, Clone)]
pub struct evaluation_domain<T: Default + Clone> {
    pub m: usize,
    pub t: T,
}
impl<T: Default + Clone> evaluation_domain<T> {
    pub fn new(m: usize, t: T) -> Self {
        Self { m, t }
    }
}

#[enum_dispatch(EvaluationDomainConfig<FieldT>)]
#[derive(Clone)]
pub enum EvaluationDomainType<FieldT: FieldTConfig> {
    ArithmeticSequence(arithmetic_sequence_domains<FieldT>),
    BasicRadix2(basic_radix2_domains<FieldT>),
    GeometricSequence(geometric_sequence_domains<FieldT>),
    ExtendedRadix2(extended_radix2_domains<FieldT>),
    StepRadix2(step_radix2_domains<FieldT>),
}

impl<FieldT: FieldTConfig> Default for EvaluationDomainType<FieldT> {
    fn default() -> Self {
        Self::ArithmeticSequence(arithmetic_sequence_domains::<FieldT>::default())
    }
}
/**
 * An evaluation domain.
 */

#[enum_dispatch]
pub trait EvaluationDomainConfig<FieldT> {
    fn m(&self) -> usize {
        0
    }
    // const M: usize = 0;

    /**
     * Construct an evaluation domain S of size m, if possible.
     *
     * (See the function get_evaluation_domain below.)
     */
    // evaluation_domain(m:usize)->Self m(m) {};

    /**
     * Get the idx-th element in S.
     */
    fn get_domain_element(&mut self, idx: usize) -> FieldT;

    /**
     * Compute the FFT, over the domain S, of the vector a.
     */
    fn FFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()>;

    /**
     * Compute the inverse FFT, over the domain S, of the vector a.
     */
    fn iFFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()>;

    /**
     * Compute the FFT, over the domain g*S, of the vector a.
     */
    fn cosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()>;

    /**
     * Compute the inverse FFT, over the domain g*S, of the vector a.
     */
    fn icosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()>;

    /**
     * Evaluate all Lagrange polynomials.
     *
     * The inputs are:
     * - an integer m
     * - an element t
     * The output is a vector (b_{0},...,b_{m-1})
     * where b_{i} is the evaluation of L_{i,S}(z) at z = t.
     */
    fn evaluate_all_lagrange_polynomials(&mut self, t: &FieldT) -> Vec<FieldT>;

    /**
     * Evaluate the vanishing polynomial of S at the field element t.
     */
    fn compute_vanishing_polynomial(&mut self, t: &FieldT) -> FieldT;

    /**
     * Add the coefficients of the vanishing polynomial of S to the coefficients of the polynomial H.
     */
    fn add_poly_Z(&mut self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()>;

    /**
     * Multiply by the evaluation, on a coset of S, of the inverse of the vanishing polynomial of S.
     */
    fn divide_by_Z_on_coset(&self, P: &mut Vec<FieldT>);
}
