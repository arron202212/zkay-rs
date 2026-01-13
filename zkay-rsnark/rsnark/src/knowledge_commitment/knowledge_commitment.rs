// Declaration of interfaces for:
// - a knowledge commitment, and
// - a knowledge commitment vector.

use crate::common::data_structures::sparse_vector::sparse_vector;
use ff_curves::PublicParams;
use ffec::algebra::fields::prime_base::fp;
use ffec::common::serialization::{OUTPUT_NEWLINE, OUTPUT_SEPARATOR};
use ffec::field_utils::{BigInteger, bigint::bigint};
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::{FieldTConfig, Fp_model, Fp_modelConfig, One, PpConfig, Zero};
use std::ops::{Add, Mul, Sub};

/********************** Knowledge commitment *********************************/

/**
 * A knowledge commitment is a pair (g,h) where g is in KC::T and h in KC::T2,
 * and KC::T and KC::T2 are groups (written additively).
 *
 * Such pairs form a group by defining:
 * - "zero" = (0,0)
 * - "one" = (1,1)
 * - a * (g,h) + b * (g',h')->Self= ( a * g + b * g', a * h + b * h').
 */

#[derive(Default, Clone)]
pub struct knowledge_commitment<KC: KCConfig> {
    pub g: KC::T,
    pub h: KC::T2,
}
// impl<const N:usize,T1:PpConfig,T2:PpConfig> knowledge_commitment<T1,T2>{
//     // knowledge_commitment<T1,T2>() = default;
//     // knowledge_commitment<T1,T2>(&other:knowledge_commitment<T1,T2>) = default;
//     // knowledge_commitment<T1,T2>(knowledge_commitment<T1,T2> &&other) = default;
//     pub fn new(g:T1, h:T2)->Self{
//         Self{g,h}
//     }

// knowledge_commitment<T1,T2>& operator=(&other:knowledge_commitment<T1,T2>) = default;
// knowledge_commitment<T1,T2>& operator=(knowledge_commitment<T1,T2> &&other) = default;
// knowledge_commitment<T1,T2> operator+(&other:knowledge_commitment<KC>) const;
// knowledge_commitment<T1,T2> mixed_add(&other:knowledge_commitment<KC>) const;
// knowledge_commitment<T1,T2> dbl() const;

//     pub fn to_special(&self){
//     }
//      pub fn is_special() ->bool{
// }

//      pub fn is_zero() ->bool{
//         }
// bool operator==(&other:knowledge_commitment<T1,T2>) const;
// bool operator!=(&other:knowledge_commitment<T1,T2>) const;

// static knowledge_commitment<T1,T2> zero();
// static knowledge_commitment<T1,T2> one();

// pub fn  print() const;

// static usize size_in_bits();

// static pub fn  batch_to_special_all_non_zeros(
//     Vec<knowledge_commitment<T1,T2> > &vec);
// }

//
// knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&ffec::bigint<m>T2>);

//
// knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&ffec::Fp_model<m, modulus_p>T2>);

//
// std::ostream& operator<<(std::ostream& out, &kc:knowledge_commitment<T1,T2>);

//
// std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc);

/******************** Knowledge commitment vector ****************************/

/**
 * A knowledge commitment vector is a sparse vector of knowledge commitments.
 */
//
pub type knowledge_commitment_vector<KC> = sparse_vector<knowledge_commitment<KC>>;

impl<KC: KCConfig> knowledge_commitment<KC> {
    pub fn new(g: KC::T, h: KC::T2) -> Self {
        Self { g, h }
    }

    pub fn zero() -> Self {
        Self::new(KC::T::zero(), KC::T2::zero())
    }

    pub fn one() -> Self {
        Self::new(KC::T::one(), KC::T2::one())
    }

    pub fn mixed_add(&self, other: &knowledge_commitment<KC>) -> Self {
        Self::new(self.g.mixed_add(&other.g), self.h.mixed_add(&other.h))
    }

    pub fn dbl(&self) -> Self {
        Self::new(self.g.dbl(), self.h.dbl())
    }

    pub fn to_special(&self) {
        self.g.to_special();
        self.h.to_special();
    }

    pub fn is_special(&self) -> bool {
        self.g.is_special() && self.h.is_special()
    }

    pub fn is_zero(&self) -> bool {
        (self.g.is_zero() && self.h.is_zero())
    }

    pub fn print(&self) {
        print!("knowledge_commitment.g:\n");
        self.g.print();
        print!("knowledge_commitment.h:\n");
        self.h.print();
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<Self>) {
        // it is guaranteed that every vec[i] is non-zero,
        // but, for any i, *one* of vec[i].g and vec[i].h might still be zero,
        // so we still have to handle zeros separately

        // we separately process g's first, then h's
        // to lower memory consumption
        let mut g_vec = Vec::with_capacity(vec.len());

        for i in 0..vec.len() {
            if !vec[i].g.is_zero() {
                g_vec.push(vec[i].g.clone());
            }
        }

        KC::T::batch_to_special_all_non_zeros(g_vec.clone());
        let mut g_it = g_vec.iter();
        let mut T1_zero_special = KC::T::zero();
        T1_zero_special.to_special();

        for i in 0..vec.len() {
            if !vec[i].g.is_zero() {
                vec[i].g = g_it.next().unwrap().clone();
            } else {
                vec[i].g = T1_zero_special.clone();
            }
        }

        g_vec.clear();

        // exactly the same thing, but for h:
        let mut h_vec = Vec::with_capacity(vec.len());

        for i in 0..vec.len() {
            if !vec[i].h.is_zero() {
                h_vec.push(vec[i].h.clone());
            }
        }

        KC::T2::batch_to_special_all_non_zeros(h_vec.clone());
        let mut h_it = h_vec.iter();
        let mut T2_zero_special = KC::T2::zero();
        T2_zero_special.to_special();

        for i in 0..vec.len() {
            if !vec[i].h.is_zero() {
                vec[i].h = h_it.next().unwrap().clone();
            } else {
                vec[i].h = T2_zero_special.clone();
            }
        }

        h_vec.clear();
    }
}

impl<KC: KCConfig> One for knowledge_commitment<KC> {
    fn one() -> Self {
        Default::default()
    }
}

impl<KC: KCConfig> Zero for knowledge_commitment<KC> {
    fn zero() -> Self {
        Default::default()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

impl<KC: KCConfig> PpConfig for knowledge_commitment<KC> {
    type TT = KC::BigInt;
    // type Fr=Self;
    fn size_in_bits() -> usize {
        KC::T::size_in_bits() + KC::T2::size_in_bits()
    }
}

impl<KC: KCConfig> PartialEq for knowledge_commitment<KC> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g && self.h == other.h
    }
}

use std::fmt;
impl<KC: KCConfig> fmt::Display for knowledge_commitment<KC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_SEPARATOR}{}", self.g, self.h)
    }
}

//
// std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc)
// {
//     in >> kc.g;
//     ffec::consume_OUTPUT_SEPARATOR(in);
//     in >> kc.h;
//     return in;
// }

// bool pub fn operator!=(&other:knowledge_commitment<T1,T2>) const
// {
//     return !((*this) == other);
// }

impl<KC: KCConfig> Add for knowledge_commitment<KC> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.g + other.g, self.h + other.h)
    }
}
impl<KC: KCConfig> Sub for knowledge_commitment<KC> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.g + other.g, self.h + other.h)
    }
}
impl<KC: KCConfig> Mul for knowledge_commitment<KC> {
    type Output = knowledge_commitment<KC>;

    fn mul(self, rhs: Self) -> Self::Output {
        // knowledge_commitment::<KC>::new(self.g * rhs, self.h * rhs)
        self
    }
}
impl<KC: KCConfig, const N: usize> Mul<bigint<N>> for knowledge_commitment<KC> {
    type Output = knowledge_commitment<KC>;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        // knowledge_commitment::<KC>::new(self.g * rhs, self.h * rhs)
        self
    }
}
impl<KC: KCConfig> Mul<&Self> for knowledge_commitment<KC> {
    type Output = knowledge_commitment<KC>;

    fn mul(self, rhs: &Self) -> Self::Output {
        // knowledge_commitment::<KC>::new(self.g * rhs, self.h * rhs)
        self
    }
}

impl<KC: KCConfig, R: AsRef<[u64]>> Mul<R> for &knowledge_commitment<KC> {
    type Output = knowledge_commitment<KC>;

    fn mul(self, rhs: R) -> Self::Output {
        // knowledge_commitment::<KC>::new(self.g * rhs, self.h * rhs)
        self.clone()
    }
}

impl<const N: usize, KC: KCConfig, T: Fp_modelConfig<N>> Mul<&Fp_model<N, T>>
    for knowledge_commitment<KC>
{
    type Output = Self;

    fn mul(self, rhs: &Fp_model<N, T>) -> Self {
        self * rhs.as_bigint()
    }
}
