// use crate::algebra::curves::curve_utils;
use crate::algebra::curves::alt_bn128::alt_bn128_fields::{
    alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fr,
};
use crate::FpmConfig;
use crate::algebra::curves::alt_bn128::alt_bn128_init::{
    alt_bn128_coeff_b, alt_bn128_twist_coeff_b, alt_bn128_twist_mul_by_b_c0,
    alt_bn128_twist_mul_by_b_c1, alt_bn128_twist_mul_by_q_X, alt_bn128_twist_mul_by_q_Y,
};
use crate::algebra::curves::alt_bn128::curves::Bn254;
use crate::algebra::curves::pairing::Pairing;
use ffec::Fp_model;
use ffec::Fp_modelConfig;
use ffec::PpConfig;
use ffec::field_utils::bigint::GMP_NUMB_BITS;
use ffec::field_utils::bigint::bigint;
use ffec::field_utils::field_utils::batch_invert;

use ffec::{One, Zero};
use num_bigint::BigUint;
use std::borrow::Borrow;
use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// impl One for alt_bn128_G2 {
// fn one() -> Self { Self::G1_zero() }
// }
// impl BigInteger for alt_bn128_G2 {}
impl From<BigUint> for alt_bn128_G2 {
    fn from(_: BigUint) -> Self {
        Default::default()
    }
}

// impl AsRef<[u64]> for bigint<1> {
//     fn as_ref(&self) -> &[u64] {
//         &self.0
//     }
// }

impl PpConfig for alt_bn128_G2 {
    type TT = bigint<1>;
    // type Fr=Self;
}

impl Add<i32> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Add<O> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl Sub for alt_bn128_G2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<const N: usize> Mul<bigint<N>> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: bigint<N>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Mul<i32> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += *other.borrow();
        r
    }
}

impl<O: Borrow<Self>> Mul<O> for alt_bn128_G2 {
    type Output = alt_bn128_G2;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}

impl Neg for alt_bn128_G2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

use std::fmt;
impl fmt::Display for alt_bn128_G2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::one())
    }
}

impl One for alt_bn128_G2 {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for alt_bn128_G2 {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

// pub type alt_bn128_G2 = <Bn254 as Pairing>::G2;
#[derive(Clone, Default, PartialEq)]
pub struct alt_bn128_G2 {
    // #ifdef PROFILE_OP_COUNTS
    // static i64 add_cnt;
    // static i64 dbl_cnt;
    //#endif
    // static Vec<std::usize> wnaf_window_table;
    // static Vec<std::usize> fixed_base_exp_window_table;
    // static alt_bn128_G2 G2_zero;
    // static alt_bn128_G2 G2_one;
    // static bool initialized;

    // // Cofactor
    // static let h_bitcount= 256;
    // static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
    // static bigint<h_limbs> h;
    X: alt_bn128_Fq2,
    Y: alt_bn128_Fq2,
    Z: alt_bn128_Fq2,
    // using Jacobian coordinates
    // alt_bn128_G2();
    //

    // static alt_bn128_Fq2 mul_by_b(elt:&alt_bn128_Fq2);

    // pub fn  print();
    // pub fn  print_coordinates();

    // pub fn  to_affine_coordinates();
    // pub fn  to_special();
    // bool is_special();

    // bool is_zero();

    // bool operator==(other:&alt_bn128_G2);
    // bool operator!=(other:&alt_bn128_G2);

    // alt_bn128_G2 operator+(other:&alt_bn128_G2);
    // alt_bn128_G2 operator-();
    // alt_bn128_G2 operator-(other:&alt_bn128_G2);

    // alt_bn128_G2 add(other:&alt_bn128_G2);
    // alt_bn128_G2 mixed_add(other:&alt_bn128_G2);
    // alt_bn128_G2 dbl();
    // alt_bn128_G2 mul_by_q();
    // alt_bn128_G2 mul_by_cofactor();

    // bool is_well_formed();

    // static alt_bn128_G2 zero();
    // static alt_bn128_G2 one();
    // static alt_bn128_G2 random_element();

    // static std::usize size_in_bits() { return twist_field::ceil_size_in_bits() + 1; }
    // static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
    // static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

    // friend std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G2);
    // friend std::istream& operator>>(std::istream &in, alt_bn128_G2 &g);

    // static pub fn  batch_to_special_all_non_zeros(Vec<alt_bn128_G2> &vec);
}

// alt_bn128_G2 operator*(lhs:&bigint<m>, rhs:&alt_bn128_G2)
// {
//     return scalar_mul<alt_bn128_G2, m>(rhs, lhs);
// }

// alt_bn128_G2 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&alt_bn128_G2)
// {
//     return scalar_mul<alt_bn128_G2, m>(rhs, lhs.as_bigint());
// }

// using std::usize;

// // #ifdef PROFILE_OP_COUNTS
// i64 alt_bn128_G2::add_cnt = 0;
// i64 alt_bn128_G2::dbl_cnt = 0;
// //#endif
pub trait alt_bn128_G2Config: Send + Sync + Sized + 'static {
    const wnaf_window_table: &'static [usize];
    const fixed_base_exp_window_table: &'static [usize];
}
// Vec<usize> alt_bn128_G2::wnaf_window_table;
// Vec<usize> alt_bn128_G2::fixed_base_exp_window_table;
// alt_bn128_G2 alt_bn128_G2::G2_zero = {};
// alt_bn128_G2 alt_bn128_G2::G2_one = {};
// bool alt_bn128_G2::initialized = false;
// bigint<alt_bn128_G2::h_limbs> alt_bn128_G2::h;
type base_field = alt_bn128_Fq;
type twist_field = alt_bn128_Fq2;
type scalar_field = alt_bn128_Fr;

impl alt_bn128_G2 {
    const h_bitcount: usize = 256;
    const h_limbs: usize = (Self::h_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
    pub fn size_in_bits() -> usize {
        twist_field::ceil_size_in_bits() + 1
    }
    pub fn field_char() -> Vec<u64> {
        base_field::field_char().as_ref().to_vec()
    }
    pub fn order() -> Vec<u64> {
        scalar_field::field_char().as_ref().to_vec()
    }

    pub fn h<const N: usize>() -> bigint<N> {
        bigint::<N>::default()
    }
    pub fn new(X: alt_bn128_Fq2, Y: alt_bn128_Fq2, Z: alt_bn128_Fq2) -> Self {
        Self { X, Y, Z }
    }
    fn G2_zero() -> Self {
        Self::new(
            alt_bn128_Fq2::zero(),
            alt_bn128_Fq2::one(),
            alt_bn128_Fq2::zero(),
        )
    }
    fn G2_one() -> Self {
        Self::new(
            alt_bn128_Fq2::zero(),
            alt_bn128_Fq2::one(),
            alt_bn128_Fq2::zero(),
        )
    }
    pub fn initialize() {
        // if initialized
        // {
        //     self.X = G2_zero.X;
        //     self.Y = G2_zero.Y;
        //     self.Z = G2_zero.Z;
        // }
    }

    pub fn mul_by_b(elt: &alt_bn128_Fq2) -> alt_bn128_Fq2 {
        alt_bn128_Fq2::new(
            alt_bn128_twist_mul_by_b_c0 * elt.c0,
            alt_bn128_twist_mul_by_b_c1 * elt.c1,
        )
    }

    pub fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let mut copy = self.clone();
            copy.to_affine_coordinates();
            print!(
                "({:N$}*z + {:N$} , {:N$}*z + {:N$})\n",
                copy.X.c1.as_bigint().0,
                copy.X.c0.as_bigint().0,
                copy.Y.c1.as_bigint().0,
                copy.Y.c0.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$}*z + {:N$} : {:N$}*z + {:N$} : {:N$}*z + {:N$})\n",
                self.X.c1.as_bigint().0,
                self.X.c0.as_bigint().0,
                self.Y.c1.as_bigint().0,
                self.Y.c0.as_bigint().0,
                self.Z.c1.as_bigint().0,
                self.Z.c0.as_bigint().0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = alt_bn128_Fq2::zero();
            self.Y = alt_bn128_Fq2::one();
            self.Z = alt_bn128_Fq2::zero();
        } else {
            let Z_inv = self.Z.inverse();
            let Z2_inv = Z_inv.squared();
            let Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = alt_bn128_Fq2::one();
        }
    }

    pub fn to_special(&mut self) {
        self.to_affine_coordinates();
    }

    pub fn is_special(&self) -> bool {
        self.is_zero() || self.Z == alt_bn128_Fq2::one()
    }

    pub fn is_zero(&self) -> bool {
        self.Z.is_zero()
    }

    pub fn add(&self, other: &alt_bn128_G2) -> Self {
        self.clone() + other
    }

    pub fn mixed_add(&self, other: &alt_bn128_G2) -> Self {
        // #ifdef DEBUG
        assert!(other.is_special());
        //#endif

        // handle special cases having to do with O
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-madd-2007-bl
        // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
        // iff
        // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
        // iff
        // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3
        // we know that Z2 = 1

        let Z1Z1 = (self.Z).squared();

        let U1 = self.X;
        let U2 = other.X * Z1Z1;

        let Z1_cubed = (self.Z) * Z1Z1;

        let S1 = (self.Y); // S1 = Y1 * Z2 * Z2Z2
        let S2 = (other.Y) * Z1_cubed; // S2 = Y2 * Z1 * Z1Z1

        // check for doubling case
        if U1 == U2 && S1 == S2 {
            // dbl case; nothing of above can be reused
            return self.dbl();
        }

        // #ifdef PROFILE_OP_COUNTS
        // self.add_cnt++;
        //#endif

        let H = U2 - (self.X); // H = U2-X1
        let HH = H.squared(); // HH = H^2
        let mut I = HH + HH; // I = 4*HH
        I = I + I;
        let J = H * I; // J = H*I
        let mut r = S2 - (self.Y); // r = 2*(S2-Y1)
        r = r + r;
        let V = (self.X) * I; // V = X1*I
        let X3 = r.squared() - J - V - V; // X3 = r^2-J-2*V
        let mut Y3 = (self.Y) * J; // Y3 = r*(V-X3)-2*Y1*J
        Y3 = r * (V - X3) - Y3 - Y3;
        let Z3 = ((self.Z) + H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

        Self::new(X3, Y3, Z3)
    }

    pub fn dbl(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt++;
        //#endif
        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // NOTE: does not handle O and pts of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l

        let A = (self.X).squared(); // A = X1^2
        let B = (self.Y).squared(); // B = Y1^2
        let C = B.squared(); // C = B^2
        let mut D = (self.X + B).squared() - A - C;
        D = D + D; // D = 2 * ((X1 + B)^2 - A - C)
        let E = A + A + A; // E = 3 * A
        let F = E.squared(); // F = E^2
        let X3 = F - (D + D); // X3 = F - 2 D
        let mut eightC = C + C;
        eightC = eightC + eightC;
        eightC = eightC + eightC;
        let Y3 = E * (D - X3) - eightC; // Y3 = E * (D - X3) - 8 * C
        let Y1Z1 = (self.Y) * (self.Z);
        let Z3 = Y1Z1 + Y1Z1; // Z3 = 2 * Y1 * Z1

        Self::new(X3, Y3, Z3)
    }

    pub fn mul_by_q(&self) -> Self {
        Self::new(
            alt_bn128_twist_mul_by_q_X * (self.X).Frobenius_map(1),
            alt_bn128_twist_mul_by_q_Y * (self.Y).Frobenius_map(1),
            (self.Z).Frobenius_map(1),
        )
    }

    pub fn mul_by_cofactor(&self) -> Self {
        self.clone() * Self::h::<4>()
    }

    pub fn is_well_formed(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        /*
            y^2 = x^3 + b

            We are using Jacobian coordinates, so equation we need to check is actually

            (y/z^3)^2 = (x/z^2)^3 + b
            y^2 / z^6 = x^3 / z^6 + b
            y^2 = x^3 + b z^6
        */
        let X2 = self.X.squared();
        let Y2 = self.Y.squared();
        let Z2 = self.Z.squared();

        let X3 = self.X * X2;
        let Z3 = self.Z * Z2;
        let Z6 = Z3.squared();

        (Y2 == X3 + alt_bn128_twist_coeff_b * Z6)
    }

    pub fn zero() -> Self {
        Self::G2_zero()
    }

    pub fn one() -> Self {
        Self::G2_one()
    }

    pub fn random_element() -> Self {
        Self::G2_one() * (alt_bn128_Fr::random_element().as_bigint())
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<alt_bn128_G2>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in vec.iter() {
            Z_vec.push(el.Z);
        }
        batch_invert::<alt_bn128_Fq2>(&mut Z_vec);

        let one = alt_bn128_Fq2::one();

        for i in 0..vec.len() {
            let Z2 = Z_vec[i].squared();
            let Z3 = Z_vec[i] * Z2;

            vec[i].X = vec[i].X * Z2;
            vec[i].Y = vec[i].Y * Z3;
            vec[i].Z = one;
        }
    }
}
impl FpmConfig for alt_bn128_G2{
    type Fr=alt_bn128_Fq;
}
// bool alt_bn128_G2::operator==(other:&alt_bn128_G2)
// {
//     if self.is_zero()
//     {
//         return other.is_zero();
//     }

//     if other.is_zero()
//     {
//         return false;
//     }

//     /* now neither is O */
//     // using Jacobian coordinates so:
//     // (X1:Y1:Z1) = (X2:Y2:Z2)
//     // iff
//     // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
//     // iff
//     // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

//     alt_bn128_Fq2 Z1_squared = (self.Z).squared();
//     alt_bn128_Fq2 Z2_squared = (other.Z).squared();

//     if (self.X * Z2_squared) != (other.X * Z1_squared)
//     {
//         return false;
//     }

//     alt_bn128_Fq2 Z1_cubed = (self.Z) * Z1_squared;
//     alt_bn128_Fq2 Z2_cubed = (other.Z) * Z2_squared;

//     return !((self.Y * Z2_cubed) != (other.Y * Z1_cubed));
// }

// bool alt_bn128_G2::operator!=(other:&alt_bn128_G2)
// {
//     return !(operator==(other));
// }

// alt_bn128_G2 alt_bn128_G2::operator+(other:&alt_bn128_G2)
// {
//     // handle special cases having to do with O
//     if self.is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return *this;
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // using Jacobian coordinates according to
//     // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
//     // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
//     // iff
//     // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
//     // iff
//     // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

//     alt_bn128_Fq2 Z1Z1 = (self.Z).squared();
//     alt_bn128_Fq2 Z2Z2 = (other.Z).squared();

//     alt_bn128_Fq2 U1 = self.X * Z2Z2;
//     alt_bn128_Fq2 U2 = other.X * Z1Z1;

//     alt_bn128_Fq2 Z1_cubed = (self.Z) * Z1Z1;
//     alt_bn128_Fq2 Z2_cubed = (other.Z) * Z2Z2;

//     alt_bn128_Fq2 S1 = (self.Y) * Z2_cubed;      // S1 = Y1 * Z2 * Z2Z2
//     alt_bn128_Fq2 S2 = (other.Y) * Z1_cubed;      // S2 = Y2 * Z1 * Z1Z1

//     // check for doubling case
//     if U1 == U2 && S1 == S2
//     {
//         // dbl case; nothing of above can be reused
//         return self.dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     self.add_cnt++;
// //#endif

//     // rest of add case
//     alt_bn128_Fq2 H = U2 - U1;                            // H = U2-U1
//     alt_bn128_Fq2 S2_minus_S1 = S2-S1;
//     alt_bn128_Fq2 I = (H+H).squared();                    // I = (2 * H)^2
//     alt_bn128_Fq2 J = H * I;                              // J = H * I
//     alt_bn128_Fq2 r = S2_minus_S1 + S2_minus_S1;          // r = 2 * (S2-S1)
//     alt_bn128_Fq2 V = U1 * I;                             // V = U1 * I
//     alt_bn128_Fq2 X3 = r.squared() - J - (V+V);           // X3 = r^2 - J - 2 * V
//     alt_bn128_Fq2 S1_J = S1 * J;
//     alt_bn128_Fq2 Y3 = r * (V-X3) - (S1_J+S1_J);          // Y3 = r * (V-X3)-2 S1 J
//     alt_bn128_Fq2 Z3 = ((self.Z+other.Z).squared()-Z1Z1-Z2Z2) * H; // Z3 = ((Z1+Z2)^2-Z1Z1-Z2Z2) * H

//     return alt_bn128_G2(X3, Y3, Z3);
// }

// alt_bn128_G2 alt_bn128_G2::operator-()
// {
//     return alt_bn128_G2(self.X, -(self.Y), self.Z);
// }

// alt_bn128_G2 alt_bn128_G2::operator-(other:&alt_bn128_G2)
// {
//     return self.clone() + (-other);
// }

// std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G2)
// {
//     alt_bn128_G2 copy(g);
//     copy.to_affine_coordinates();
//     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // #ifdef NO_PT_COMPRESSION
//     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// #else
//     /* storing LSB of Y */
//     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.c0.as_bigint().0.0[0] & 1);
// //#endif

//     return out;
// }

// std::istream& operator>>(std::istream &in, alt_bn128_G2 &g)
// {
//     char is_zero;
//     alt_bn128_Fq2 tX, tY;

// // #ifdef NO_PT_COMPRESSION
//     in >> is_zero >> tX >> tY;
//     is_zero -= '0';
// #else
//     in.read((char*)&is_zero, 1); // this reads is_zero;
//     is_zero -= '0';
//     consume_OUTPUT_SEPARATOR(in);

//     unsigned char Y_lsb;
//     in >> tX;
//     consume_OUTPUT_SEPARATOR(in);
//     in.read((char*)&Y_lsb, 1);
//     Y_lsb -= '0';

//     // y = +/- sqrt(x^3 + b)
//     if is_zero == 0
//     {
//         alt_bn128_Fq2 tX2 = tX.squared();
//         alt_bn128_Fq2 tY2 = tX2 * tX + alt_bn128_twist_coeff_b;
//         tY = tY2.sqrt();

//         if (tY.c0.as_bigint().0.0[0] & 1) != Y_lsb
//         {
//             tY = -tY;
//         }
//     }
// //#endif
//     // using projective coordinates
//     if is_zero == 0
//     {
//         g.X = tX;
//         g.Y = tY;
//         g.Z = alt_bn128_Fq2::one();
//     }
//     else
//     {
//         g = alt_bn128_G2::zero();
//     }

//     return in;
// }
