// Declaration of arithmetic in the finite field F[p], for prime p of fixed length.
use crate::{
    FieldTConfig, PpConfig,
    algebra::{
        field_utils::{
            BigInteger,
            algorithms::{
                FPMConfig, FieldTForPowersConfig, PowerConfig, Powers, tonelli_shanks_sqrt,
            },
            bigint::{BigIntegerT, GMP_NUMB_BITS, bigint},
            field_utils, fp_aux, {BigInt, algorithms},
        },
        fields::{
            field::{AdditiveGroup, Field},
            fpn_field::PrimeField,
            sqrt::SqrtPrecomputation,
        },
    },
    common::utils::bit_vector,
    fp_aux::{add_assign_portable, mul_reduce_portable, sub_assign_portable},
};
use cfg_if::cfg_if;
use educe::Educe;
use num_bigint::BigUint;
use num_integer::{ExtendedGcd, Integer};
use num_traits::{Num, One, Signed, Zero};
use std::{
    borrow::Borrow,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
//  use crate::algebra::field_utils::bigint::bigint;

//  * Arithmetic in the finite field F[p], for prime p of fixed length.
//  *
//  * This pub struct implements Fp-arithmetic, for a large prime p, using a fixed number
//  * of words. It is optimized for tight memory consumption, so the modulus p is
//  * passed as a template parameter, to avoid per-element overheads.
//  *
//  * The implementation is mostly a wrapper around GMP's MPN (constant-size integers).
//  * But for the integer sizes of interest for libff (3 to 5 limbs of 64 bits each),
//  * we implement performance-critical routines, like addition and multiplication,
//  * using hand-optimzied assembly code.

pub trait Fp_modelConfig<const N: usize>:
    Send + Sync + 'static + Sized + Default + Clone + Copy + Eq + Debug
{
    // const num_limbs: usize = 4;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 254;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 42; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator^((modulus-1)/2^s)
    const inv: u64 = 0xc2e1f593efffffff; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
    fn from_bigint(r: bigint<N>) -> Option<Self> {
        None //P::from_bigint(r)
    }
}

#[derive(Educe)]
#[educe(Default, Clone, Debug, Hash, Copy, PartialOrd, Ord, Eq)] // PartialEq,
pub struct Fp_model<const N: usize, T: Fp_modelConfig<N>> {
    pub mont_repr: bigint<N>,
    pub t: PhantomData<T>,
}

impl<const N: usize, T: Fp_modelConfig<N>> Fp_modelConfig<N> for Fp_model<N, T> {
    // const num_limbs: usize = T::num_limbs;
    const modulus: bigint<N> = bigint::<N>::one();
    const num_bits: usize = 1;
    const euler: bigint<N> = bigint::<N>::one(); // (modulus-1)/2
    const s: usize = 1; // modulus = 2^s * t + 1
    const t: bigint<N> = bigint::<N>::one(); // with t odd
    const t_minus_1_over_2: bigint<N> = bigint::<N>::one(); // (t-1)/2
    const nqr: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // a quadratic nonresidue
    const nqr_to_t: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // nqr^t
    const multiplicative_generator: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator of Fp^*
    const root_of_unity: Fp_model<N, Self> = Fp_model::<N, Self>::const_default(); // generator^((modulus-1)/2^s)
    const inv: u64 = 1; // modulus^(-1) mod W, where W = 2^(word size)
    const Rsquared: bigint<N> = bigint::<N>::one(); // R^2, where R = W^k, where k = ??
    const Rcubed: bigint<N> = bigint::<N>::one(); // R^3
}

// impl<const N: usize, T: Fp_modelConfig<N>> Borrow<Self> for Fp_model<N, T> {
//     fn borrow(&self)->Self{
//         *self
//     }
// }

impl<const N: usize, T: Fp_modelConfig<N>> FieldTConfig for Fp_model<N, T> {
    fn as_ulong(&self) -> usize {
        self.as_bigint().as_ulong() as usize
    }

    fn geometric_generator() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 2;
        res.mul_reduce(&T::Rsquared);
        res
    }

    fn arithmetic_generator() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        res
    }

    fn squared(&self) -> Self {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {
               let out= squared_n3(&self.mont_repr.0.0,&T::modulus.0,T::inv);
                self.mont_repr.0.0.copy_from_slice(out);
            }else{

        let mut r: Self = self.clone();
        r *= &r.clone();
        r
            }
        }
    }

    fn inverse(&self) -> Self {
        let mut r = self.clone();
        r.invert()
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> PpConfig for Fp_model<N, T> {
    type BigIntT = bigint<N>;
    const num_limbs: usize = N;
    fn size_in_bits() -> usize {
        T::num_bits
    }

    fn dbl(&self) -> Self {
        self.clone()
    }
    fn random_element() -> Self {
        
        let mut r_data = [0u64; N];
        let mut rng = rand::thread_rng();
        loop {
            
            rng.fill(&mut r_data[..]);

            
            
            let unused_bits = T::modulus[N - 1].leading_zeros();
            if unused_bits > 0 {
                
                let mask = u64::MAX >> unused_bits;
                r_data[N - 1] &= mask;
            }

            
            
            if r_data < T::modulus.0.0 {
                break;
            }
        }

        
        Fp_model::new(bigint::<N>(BigInt::<N>(r_data)))
    }

    fn print(&self) {
        let mut tmp = Self::zero();
        tmp.mont_repr.0.0[0] = 1;
        tmp.mul_reduce(&self.mont_repr);

        tmp.mont_repr.print();
    }

    fn num_bits() -> usize {
        T::num_bits
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> AsMut<[u64]> for Fp_model<N, T> {
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.mont_repr.0.0
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<usize> for Fp_model<N, T> {
    fn from(b: usize) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<u32> for Fp_model<N, T> {
    fn from(b: u32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i32> for Fp_model<N, T> {
    fn from(b: i32) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<i64> for Fp_model<N, T> {
    fn from(b: i64) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b as u64),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> From<u64> for Fp_model<N, T> {
    fn from(b: u64) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new(b),
            t: PhantomData,
        }
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> From<&str> for Fp_model<N, T> {
    fn from(b: &str) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::new_with_str(b).expect(b),
            t: PhantomData,
        }
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> From<BigUint> for Fp_model<N, T> {
    fn from(b: BigUint) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>(b.try_into().unwrap()),
            t: PhantomData,
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> FPMConfig for Fp_model<N, T> {}
impl<const N: usize, T: Fp_modelConfig<N>> FieldTForPowersConfig<N> for Fp_model<N, T> {
    type FPM = Self;
    const num_limbs: usize = N;
    const s: usize = T::s; // modulus = 2^s * t + 1
    const t: bigint<N> = T::t; // with t odd
    const t_minus_1_over_2: bigint<N> = T::t_minus_1_over_2; // (t-1)/2
    const nqr: Self = T::nqr; // a quadratic nonresidue
    const nqr_to_t: Self = T::nqr_to_t; // nqr^t
    fn squared_(&self) -> Self {
        self.squared()
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    pub fn ceil_size_in_bits() -> usize {
        T::num_bits
    }
    pub fn floor_size_in_bits() -> usize {
        T::num_bits - 1
    }

    pub fn extension_degree() -> usize {
        1
    }
    pub fn field_char() -> bigint<N> {
        T::modulus
    }
    pub fn modulus_is_valid() -> bool {
        T::modulus.0.0[N - 1] != 0
    } // mpn inverse assumes that highest limb is non-zero

    pub const fn const_default() -> Fp_model<N, T> {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        }
    }
    pub const fn const_new(b: BigInt<N>) -> Self {
        Fp_model::<N, T> {
            mont_repr: bigint::<N>(b),
            t: PhantomData,
        }
    }

    pub fn mul_reduce(&mut self, other: &bigint<N>) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

               let data= match N {
                    3 => unsafe { mul_reduce_n3(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    4 => unsafe { mul_reduce_n4(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    5 => unsafe { mul_reduce_n5(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv) },
                    _ => {return }
                };
                self.mont_repr.0.0.copy_from_slice(&data[N..N*2]);
            }else{
               let data= mul_reduce_portable::<N>(&self.mont_repr.0.0,&other.0.0, &T::modulus.0.0,T::inv);
                self.mont_repr.0.0.copy_from_slice(&data);
            }
        }
    }

    pub fn new(b: bigint<N>) -> Self {
        let mut _self = Self {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        };
        _self.mont_repr.0.0.copy_from_slice(&T::Rsquared.0.0[..N]);
        _self.mul_reduce(&b);
        _self
    }

    pub fn new_with_i64(x: i64, is_unsigned: bool) -> Self {
        // assert!(std::numeric_limits<mp_limb_t>::max() >= std::numeric_limits<long>::max() as u64, "long won't fit in mp_limb_t");
        let mut _self = Self {
            mont_repr: bigint::<N>::one(),
            t: PhantomData,
        };
        if is_unsigned || x >= 0 {
            _self.mont_repr.0.0[0] = x as u64;
        } else {
            
            
            

            let sub_val = (-(x as i64)) as u64; 
            let mut borrow = 0u8;

            
            let (res, b) = _self.mont_repr.0.0[0].overflowing_sub(sub_val);
            _self.mont_repr.0.0[0] = res;
            borrow = b as u8;

            
            for i in 1..N {
                if borrow == 0 {
                    break;
                }
                let (res, b) = _self.mont_repr.0.0[i].overflowing_sub(borrow as u64);
                _self.mont_repr.0.0[i] = res;
                borrow = b as u8;
            }

            
            debug_assert_eq!(borrow, 0, "Borrow must be zero in prime field subtraction");
        }

        _self.mul_reduce(&T::Rsquared);
        _self
    }
    pub fn set_ulong(&mut self, x: u64) {
        self.mont_repr.clear();
        self.mont_repr.0.0[0] = x;
        self.mul_reduce(&T::Rsquared);
    }

    pub const fn clear(&mut self) {
        self.mont_repr.clear();
    }

    pub fn as_bigint(&self) -> bigint<N> {
        let mut one = bigint::<N>::one();
        let mut res = self.clone();
        res.mul_reduce(&one);

        res.mont_repr
    }

    pub fn is_zero(&self) -> bool {
        self.mont_repr.is_zero() // zero maps to zero
    }
    fn randomize(&mut self) {
        *self = Self::random_element();
    }

    pub const fn zero() -> Self {
        let mut res = Self::const_default();
        res.mont_repr.clear();
        res
    }

    pub fn one() -> Self {
        let mut res = Self::default();
        res.mont_repr.0.0[0] = 1;
        res.mul_reduce(&T::Rsquared);
        res
    }

    pub fn square(&mut self) -> &Self {
        *self = self.squared();
        self
    }

    pub fn invert(&mut self) -> Self {
        
        debug_assert!(!self.is_zero());

        
        let mut v = num_bigint::BigInt::from(T::modulus);
        let mut u = num_bigint::BigInt::from(self.mont_repr);

        
        
        let ExtendedGcd::<num_bigint::BigInt> {
            gcd: g,
            x: s,
            y: _t,
        } = u.extended_gcd(&v);

        
        debug_assert!(g.is_one(), "Inverse does not exist");

        
        let mut res = if s.is_negative() {
            
            let mut tmp = num_bigint::BigInt::from(T::modulus);
            tmp.clone().sub(s.abs()); //sub_noborrow
            tmp
        } else {
            
            s % num_bigint::BigInt::from(T::modulus)
        };
        let mut res: bigint<N> = BigUint::try_from(res).unwrap().try_into().unwrap();
        
        
        res.mul_assign(&T::Rcubed);

        self.mont_repr = res;
        self.clone()
    }

    pub fn Frobenius_map(&self, _power: usize) -> Self {
        self.clone()
    }

    pub fn sqrt(&self) -> Option<Self> {
        tonelli_shanks_sqrt(self)
    }

    pub fn to_words(&self) -> Vec<u64> {
        // TODO: implement for other bit architectures
        assert!(
            GMP_NUMB_BITS == 64,
            "Only 64-bit architectures are currently supported"
        );
        
        
        // #[cfg(not(target_pointer_width = "64"))]
        // compile_error!("Only 64-bit architectures are currently supported");

        
        
        let repr = self.bigint_repr();

        
        
        let words: Vec<u64> = repr.as_ref().to_vec();

        words
    }

    pub fn from_words(&mut self, words: &[u64]) -> bool {
        // // TODO: implement for other bit architectures
        // assert!(
        //     GMP_NUMB_BITS == 64,
        //     "Only 64-bit architectures are currently supported"
        // );

        // let start_bit = words.len() * 64; //- FieldT::ceil_size_in_bits();
        // assert!(start_bit >= 0); // Check the vector is big enough.
        // let start_word = start_bit / 64;
        // let bit_offset = start_bit % 64;

        // // Assumes mont_repr.0.0 is just the right size to fit ceil_size_in_bits().
        // // std::copy(words.begin() + start_word, words.end(),self.mont_repr.0.0);
        // self.mont_repr.0.0.clone_from_slice(&words[start_word..]);
        // // Zero out the left-most bit_offset bits.
        // self.mont_repr.0.0[N - 1] =
        //     ((self.mont_repr.0.0[N - 1] as u64) << bit_offset) >> bit_offset; //mp_limb_t

        // // return self.mont_repr < modulus;
        // false
        
        #[cfg(not(target_pointer_width = "64"))]
        compile_error!("Only 64-bit architectures are currently supported");

        
        
        let ceil_size_in_bits = Self::ceil_size_in_bits() as i64;
        let start_bit = (words.len() as i64 * 64) - ceil_size_in_bits;

        
        assert!(start_bit >= 0, "The vector is not big enough");

        let start_word = (start_bit / 64) as usize;
        let bit_offset = (start_bit % 64) as u32;

        
        
        
        let mut data = [0u64; N];
        let copy_len = words.len() - start_word;
        data[..copy_len].copy_from_slice(&words[start_word..]);

        
        
        if bit_offset > 0 {
            data[N - 1] = (data[N - 1] << bit_offset) >> bit_offset;
        }

        
        
        
        let mut result = Fp_model::<N, T>::new(bigint::<N>(BigInt::<N>(data)));
        // if !cfg!(feature = "montgomery_output") {
        //     result.mul_assign(&T::Rsquared); 
        // }

        
        self.mont_repr < T::modulus 
    }

    pub fn bigint_repr(&self) -> bigint<N> {
        
        
        cfg_if! {
        if  #[cfg(feature = "montgomery_output")]
         {
             
             return self.mont_repr;
         }
         else

         {
             
             return self.as_bigint();
         }
          }
    }
}

use rand::Rng;
use rand::distributions::{Distribution, Standard};

impl<const N: usize, T: Fp_modelConfig<N>> Distribution<Fp_model<N, T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fp_model<N, T> {
        let mut r_data = [0u64; N];
        let modulus = &T::modulus.0.0; 

        loop {
            
            for limb in r_data.iter_mut() {
                *limb = rng.next_u64();
            }

            
            
            let unused_bits = modulus[N - 1].leading_zeros();
            if unused_bits > 0 {
                let mask = u64::MAX >> unused_bits;
                r_data[N - 1] &= mask;
            }

            
            
            if &r_data < modulus {
                break;
            }
        }

        Fp_model::new(bigint::<N>(BigInt::<N>(r_data)))
    }
}
// let mut rng = rand::thread_rng();


// let r: Fp384 = rng.sample(Standard);


// let r: Fp384 = rng.gen();

impl<const N: usize, T: Fp_modelConfig<N>> PartialEq for Fp_model<N, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mont_repr == other.mont_repr
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> AddAssign<O> for Fp_model<N, T> {
    fn add_assign(&mut self, other: O) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

                match N {
                    3 => unsafe { add_assign_n3(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    4 => unsafe { add_assign_n4(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    5 => unsafe { add_assign_n5(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    _ => {return }
                };
            }else{
               add_assign_portable(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0);
            }
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> SubAssign<O> for Fp_model<N, T> {
    fn sub_assign(&mut self, other: O) {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", feature = "asm"))]
            {

                match N {
                    3 => unsafe { sub_assign_n3(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    4 => unsafe { sub_assign_n4(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    5 => unsafe { sub_assign_n5(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0) },
                    _ => {return }
                };
            }else{
               sub_assign_portable(&mut self.mont_repr.0.0,&other.borrow().mont_repr.0.0, &T::modulus.0.0);
            }
        }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> MulAssign<O> for Fp_model<N, T> {
    fn mul_assign(&mut self, rhs: O) {
        let rhs = rhs.borrow();
        self.mul_reduce(&rhs.mont_repr);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXorAssign<u64> for Fp_model<N, T> {
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXorAssign<bigint<N>> for Fp_model<N, T> {
    fn bitxor_assign(&mut self, rhs: bigint<N>) {
        *self = Powers::power::<Fp_model<N, T>>(self, rhs);
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Add<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>, O: Borrow<Self>> Add<O> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += *other.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub<i32> for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Sub for Fp_model<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<bigint<N>> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<BigUint> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: BigUint) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Mul<i32> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Mul<i64> for Fp_model<N, T> {
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>, OT: Fp_modelConfig<N>> Mul<Fp_model<N, OT>>
    for Fp_model<N, T>
{
    type Output = Fp_model<N, T>;

    fn mul(self, rhs: Fp_model<N, OT>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
// impl<const N: usize, T: Fp_modelConfig<N>> Mul for Fp_model<N, T> {
//     type Output = Fp_model<N, T>;

//     fn mul(self, rhs: Fp_model<N, T> ) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<u64> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u64) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> BitXor<usize> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> BitXor<bigint<N>> for Fp_model<N, T> {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        r ^= rhs;
        r
    }
}


#[inline]
pub fn sub_n(res: &mut [u64], a: &[u64], b: &[u64]) -> u64 {
    let mut borrow = 0u64;

    
    for i in 0..a.len() {
        // t = a[i] - b[i] - borrow
        let (v1, b1) = a[i].overflowing_sub(b[i]);
        let (v2, b2) = v1.overflowing_sub(borrow);

        res[i] = v2;
        borrow = (b1 | b2) as u64; 
    }

    borrow
}
macro_rules! sub_n {
    ($res:expr, $a:expr, $b:expr, $n:expr) => {{
        let mut borrow = 0u64;
        let mut i = 0;

        
        while i < $n {
            let (v1, b1) = $a[i].overflowing_sub($b[i]);
            let (v2, b2) = v1.overflowing_sub(borrow);
            $res[i] = v2;
            
            borrow = (b1 | b2) as u64;
            i += 1;
        }
        borrow
    }};
}
impl<const N: usize, T: Fp_modelConfig<N>> Neg for Fp_model<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        
        if self.is_zero() {
            return self;
        }
        let mut res_data = [0u64; N];
        let a = T::modulus; 
        let b = self.mont_repr;

        
        sub_n!(res_data, a, b, N);

        
        Self::const_new(BigInt::<N>(res_data))
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> One for Fp_model<N, T> {
    fn one() -> Self {
        Self::one()
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Zero for Fp_model<N, T> {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

use std::fmt;
impl<const N: usize, T: Fp_modelConfig<N>> fmt::Display for Fp_model<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bigint_repr(),)
    }
}


impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        
        let mut mont_repr = bigint::read(&mut reader)?;

        
        let mut p = Self::new(mont_repr);

        
        // #[cfg(not(feature = "montgomery_output"))]
        // {
        
        //     p.mul_assign(&T::Rsquared);
        // }

        Ok(p)
    }
}
use std::io::{self, Read, Write};

impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    
    pub fn read_from_circuit<R: Read>(mut reader: R) -> io::Result<Self> {
        
        let mut repr = [0u64; N];
        for limb in repr.iter_mut() {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            *limb = u64::from_le_bytes(buf); 
        }

        let mut p = Self::const_new(BigInt::<N>(repr));

        
        
        // #[cfg(not(feature = "montgomery_output"))]
        // {
        //     p.mul_assign(T::Rsquared);
        // }

        
        // if p.is_valid() {
        Ok(p)
        // } else {
        //     Err(io::Error::new(
        //         io::ErrorKind::InvalidData,
        //         "Out of field range",
        //     ))
        // }
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> Fp_model<N, T> {
    
    pub fn write_to_circuit<W: Write>(&self, mut writer: W) -> io::Result<()> {
        
        let repr = if cfg!(feature = "montgomery_output") {
            self.mont_repr
        } else {
            self.as_bigint() 
        };

        
        for limb in repr.0.0.iter() {
            writer.write_all(&limb.to_le_bytes())?;
        }
        Ok(())
    }
}

impl<const N: usize, T: Fp_modelConfig<N>> FromStr for Fp_model<N, T> {
    type Err = ();

    /// Interpret a string of numbers as a (congruent) prime field element.
    /// Does not accept unnecessary leading zeroes or a blank string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use num_bigint::{BigInt, BigUint};
        // use num_traits::Signed;

        // let modulus = BigInt::from(P::MODULUS);
        // let mut a = BigInt::from_str(s).map_err(|_| ())? % &modulus;
        // if a.is_negative() {
        //     a += modulus
        // }
        // BigUint::try_from(a)
        //     .map_err(|_| ())
        //     .and_then(TryFrom::try_from)
        //     .ok()
        //     .and_then(Self::from_bigint)
        //     .ok_or(())
        Ok(Self::default())
    }
}

// impl {
// fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
//         let num_modulus_bytes = ((Self::num_bits + 7) / 8) as usize;
//         let num_bytes_to_directly_convert = bytes.len().min(num_modulus_bytes - 1 );
//         // Copy the leading little-endian bytes directly into a field element.
//         // The number of bytes directly converted must be less than the
//         // number of bytes needed to represent the modulus, as we must begin
//         // modular reduction once the data is of the same number of bytes as the
//         // modulus.
//         let (bytes, bytes_to_directly_convert) =
//             bytes.split_at(bytes.len() - num_bytes_to_directly_convert);
//         // Guaranteed to not be None, as the input is less than the modulus size.
//         let mut res = Self::from_random_bytes(bytes_to_directly_convert).unwrap();

//         // Update the result, byte by byte.
//         // We go through existing field arithmetic, which handles the reduction.
//         // TODO: If we need higher speeds, parse more bytes at once, or implement
//         // modular multiplication by a u64
//         let window_size = Self::from(256u64);
//         for byte in bytes.iter().rev() {
//             res *= window_size;
//             res += Self::from(*byte);
//         }
//         res
//     }
//   fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
//         Self::from_random_bytes_with_flags::<EmptyFlags>(bytes).map(|f| f.0)
//     }

//     #[inline]
//     fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)> {
//         if F::BIT_SIZE > 8 {
//             None
//         } else {
//             let shave_bits = Self::num_bits_to_shave();
//             let mut result_bytes = const_helpers::SerBuffer::<N>::zeroed();
//             // Copy the input into a temporary buffer.
//             result_bytes.copy_from_u8_slice(bytes);
//             // This mask retains everything in the last limb
//             // that is below `P::MODULUS_BIT_SIZE`.
//             let last_limb_mask =
//                 (u64::MAX.checked_shr(shave_bits as u32).unwrap_or(0)).to_le_bytes();
//             let mut last_bytes_mask = [0u8; 9];
//             last_bytes_mask[..8].copy_from_slice(&last_limb_mask);

//             // Length of the buffer containing the field element and the flag.
//             let output_byte_size = buffer_byte_size(Self::MODULUS_BIT_SIZE as usize + F::BIT_SIZE);
//             // Location of the flag is the last byte of the serialized
//             // form of the field element.
//             let flag_location = output_byte_size - 1;

//             // At which byte is the flag located in the last limb?
//             let flag_location_in_last_limb = flag_location.saturating_sub(8 * (N - 1));

//             // Take all but the last 9 bytes.
//             let last_bytes = result_bytes.last_n_plus_1_bytes_mut();

//             // The mask only has the last `F::BIT_SIZE` bits set
//             let flags_mask = u8::MAX.checked_shl(8 - (F::BIT_SIZE as u32)).unwrap_or(0);

//             // Mask away the remaining bytes, and try to reconstruct the
//             // flag
//             let mut flags: u8 = 0;
//             for (i, (b, m)) in last_bytes.zip(&last_bytes_mask).enumerate() {
//                 if i == flag_location_in_last_limb {
//                     flags = *b & flags_mask
//                 }
//                 *b &= m;
//             }
//             Self::deserialize_compressed(&result_bytes.as_slice()[..(N * 8)])
//                 .ok()
//                 .and_then(|f| F::from_u8(flags).map(|flag| (f, flag)))
//         }
//     }
// }
// impl<P: FpConfig<N>, const N: usize> CanonicalDeserializeWithFlags for Fp<P, N> {
//     fn deserialize_with_flags<R: ark_std::io::Read, F: Flags>(
//         reader: R,
//     ) -> Result<(Self, F), SerializationError> {
//         // All reasonable `Flags` should be less than 8 bits in size
//         // (256 values are enough for anyone!)
//         if F::BIT_SIZE > 8 {
//             return Err(SerializationError::NotEnoughSpace);
//         }
//         // Calculate the number of bytes required to represent a field element
//         // serialized with `flags`.
//         let output_byte_size = Self::zero().serialized_size_with_flags::<F>();

//         let mut masked_bytes = const_helpers::SerBuffer::zeroed();
//         masked_bytes.read_exact_up_to(reader, output_byte_size)?;
//         let flags = F::from_u8_remove_flags(&mut masked_bytes[output_byte_size - 1])
//             .ok_or(SerializationError::UnexpectedFlags)?;

//         let self_integer = masked_bytes.to_bigint();
//         Self::from_bigint(self_integer)
//             .map(|v| (v, flags))
//             .ok_or(SerializationError::InvalidData)
//     }
// }

// impl<P: FpConfig<N>, const N: usize> CanonicalDeserializeWithFlags for Fp<P, N>
// { #[inline]
//     fn from_bigint(r: BigInt<N>) -> Option<Self> {
//         P::from_bigint(r)
//     }

//     fn into_bigint(self) -> BigInt<N> {
//         P::into_bigint(self)
//     }
// }

/// A trait that specifies the configuration of a prime field.
/// Also specifies how to perform arithmetic on field elements.
pub trait FpConfig<const N: usize>: Send + Sync + 'static + Sized {
    /// The modulus of the field.
    const MODULUS: BigInt<N>;

    /// A multiplicative generator of the field.
    /// `Self::GENERATOR` is an element having multiplicative order
    /// `Self::modulus - 1`.
    const GENERATOR: Fp<Self, N>;

    /// Additive identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e + f = f`.
    const ZERO: Fp<Self, N>;

    /// Multiplicative identity of the field, i.e. the element `e`
    /// such that, for all elements `f` of the field, `e * f = f`.
    const ONE: Fp<Self, N>;

    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    const TWO_ADICITY: u32;

    /// 2^s root of unity computed by GENERATOR^t
    const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N>;

    /// An integer `b` such that there exists a multiplicative subgroup
    /// of size `b^k` for some integer `k`.
    const SMALL_SUBGROUP_BASE: Option<u32> = None;

    /// The integer `k` such that there exists a multiplicative subgroup
    /// of size `Self::SMALL_SUBGROUP_BASE^k`.
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32> = None;

    /// GENERATOR^((modulus-1) / (2^s *
    /// SMALL_SUBGROUP_BASE^SMALL_SUBGROUP_BASE_ADICITY)) Used for mixed-radix
    /// FFT.
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<Fp<Self, N>> = None;

    /// Precomputed material for use when computing square roots.
    /// Currently uses the generic Tonelli-Shanks,
    /// which works for every modulus.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp<Self, N>>>;

    /// Set a += b.
    fn add_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a -= b.
    fn sub_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Set a = a + a.
    fn double_in_place(a: &mut Fp<Self, N>);

    /// Set a = -a;
    fn neg_in_place(a: &mut Fp<Self, N>);

    /// Set a *= b.
    fn mul_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Compute the inner product `<a, b>`.
    fn sum_of_products<const T: usize>(a: &[Fp<Self, N>; T], b: &[Fp<Self, N>; T]) -> Fp<Self, N>;

    /// Set a *= a.
    fn square_in_place(a: &mut Fp<Self, N>);

    /// Compute a^{-1} if `a` is not zero.
    fn inverse(a: &Fp<Self, N>) -> Option<Fp<Self, N>>;

    /// Construct a field element from an integer in the range
    /// `0..(Self::modulus - 1)`. Returns `None` if the integer is outside
    /// this range.
    fn from_bigint(other: BigInt<N>) -> Option<Fp<Self, N>>;

    /// Convert a field element to an integer in the range `0..(Self::modulus -
    /// 1)`.
    fn into_bigint(other: Fp<Self, N>) -> BigInt<N>;
}
/// Represents an element of the prime field F_p, where `p == P::modulus`.
/// This type can represent elements in any field of size at most N * 64 bits.
#[derive(Educe)]
#[educe(Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Fp<P: FpConfig<N>, const N: usize>(
    /// Contains the element in Montgomery form for efficient multiplication.
    /// To convert an element to a [`BigInt`](struct@BigInt), use `into_bigint` or `into`.
    #[doc(hidden)]
    pub BigInt<N>,
    #[doc(hidden)] pub PhantomData<P>,
);

pub type Fp64<P> = Fp<P, 1>;
pub type Fp128<P> = Fp<P, 2>;
pub type Fp192<P> = Fp<P, 3>;
pub type Fp256<P> = Fp<P, 4>;
pub type Fp320<P> = Fp<P, 5>;
pub type Fp384<P> = Fp<P, 6>;
pub type Fp448<P> = Fp<P, 7>;
pub type Fp512<P> = Fp<P, 8>;
pub type Fp576<P> = Fp<P, 9>;
pub type Fp640<P> = Fp<P, 10>;
pub type Fp704<P> = Fp<P, 11>;
pub type Fp768<P> = Fp<P, 12>;
pub type Fp832<P> = Fp<P, 13>;
