// use crate::algebra::curves::alt_bn128::alt_bn128_init;
// use crate::algebra::curves::curve_utils;
use crate::algebra::curves::alt_bn128::alt_bn128_fields::{
    alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fr,
};
use crate::algebra::curves::alt_bn128::alt_bn128_init::{
    alt_bn128_coeff_b, alt_bn128_twist_mul_by_b_c0, alt_bn128_twist_mul_by_b_c1,
};
use crate::algebra::curves::alt_bn128::curves::Bn254;
use crate::algebra::curves::pairing::Pairing;
use ffec::field_utils::bigint::GMP_NUMB_BITS;
use ffec::field_utils::bigint::bigint;
use ffec::field_utils::field_utils::batch_invert;
use ffec::{One, Zero};
// pub type alt_bn128_G1 = <Bn254 as Pairing>::G1;

// pub struct alt_bn128_G1;
// std::ostream& operator<<(std::ostream &, const alt_bn128_G1&);
// std::istream& operator>>(std::istream &, alt_bn128_G1&);

pub struct alt_bn128_G1 {
    // #ifdef PROFILE_OP_COUNTS
    // static i64 add_cnt;
    // static i64 dbl_cnt;
    //#endif
    // static Vec<std::usize> wnaf_window_table;
    // static Vec<std::usize> fixed_base_exp_window_table;
    // static alt_bn128_G1 G1_zero;
    // static alt_bn128_G1 G1_one;
    // static bool initialized;

    // type base_field=alt_bn128_Fq;
    // type scalar_field=alt_bn128_Fr;

    // // Cofactor
    // static let h_bitcount= 1;
    // static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
    // static bigint<h_limbs> h;
    X: alt_bn128_Fq,
    Y: alt_bn128_Fq,
    Z: alt_bn128_Fq,
    // // using Jacobian coordinates
    // alt_bn128_G1();
    // alt_bn128_G1(X:alt_bn128_Fq&, Y:alt_bn128_Fq&, Z:&alt_bn128_Fq)->SelfX,Y,Z {};

    // pub fn  print();
    // pub fn  print_coordinates();

    // pub fn  to_affine_coordinates();
    // pub fn  to_special();
    // bool is_special();

    // bool is_zero();

    // bool operator==(other:&alt_bn128_G1);
    // bool operator!=(other:&alt_bn128_G1);

    // alt_bn128_G1 operator+(other:&alt_bn128_G1);
    // alt_bn128_G1 operator-();
    // alt_bn128_G1 operator-(other:&alt_bn128_G1);

    // alt_bn128_G1 add(other:&alt_bn128_G1);
    // alt_bn128_G1 mixed_add(other:&alt_bn128_G1);
    // alt_bn128_G1 dbl();
    // alt_bn128_G1 mul_by_cofactor();

    // bool is_well_formed();

    // static alt_bn128_G1 zero();
    // static alt_bn128_G1 one();
    // static alt_bn128_G1 random_element();

    // static std::usize size_in_bits() { return base_field::ceil_size_in_bits() + 1; }
    // static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
    // static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

    // // friend std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G1);
    // // friend std::istream& operator>>(std::istream &in, alt_bn128_G1 &g);

    // static pub fn  batch_to_special_all_non_zeros(Vec<alt_bn128_G1> &vec);
}

// alt_bn128_G1 operator*(lhs:&bigint<m>, rhs:&alt_bn128_G1)
// {
//     return scalar_mul<alt_bn128_G1, m>(rhs, lhs);
// }

// alt_bn128_G1 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&alt_bn128_G1)
// {
//     return scalar_mul<alt_bn128_G1, m>(rhs, lhs.as_bigint());
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<alt_bn128_G1>);
// std::istream& operator>>(std::istream& in, Vec<alt_bn128_G1> &v);

// using std::usize;

// #ifdef PROFILE_OP_COUNTS
// i64 alt_bn128_G1::add_cnt = 0;
// i64 alt_bn128_G1::dbl_cnt = 0;
//#endif
pub trait alt_bn128_G1Config: Send + Sync + Sized + 'static {
    const wnaf_window_table: &'static [usize];
    const fixed_base_exp_window_table: &'static [usize];
}
// Vec<usize> alt_bn128_G1::wnaf_window_table;
// Vec<usize> alt_bn128_G1::fixed_base_exp_window_table;
// alt_bn128_G1 alt_bn128_G1::G1_zero = {};
// alt_bn128_G1 alt_bn128_G1::G1_one = {};
// bool alt_bn128_G1::initialized = false;
// bigint<alt_bn128_G1::h_limbs> alt_bn128_G1::h;
pub type base_field = alt_bn128_Fq;
pub type scalar_field = alt_bn128_Fr;

impl alt_bn128_G1 {
    const h_bitcount: usize = 1;
    const h_limbs: usize = (Self::h_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
    pub fn size_in_bits() -> usize {
        return base_field::ceil_size_in_bits() + 1;
    }
    pub fn field_char() -> bigint<{ base_field::num_limbs }> {
        return base_field::field_char();
    }
    pub fn order() -> bigint<{ scalar_field::num_limbs }> {
        return scalar_field::field_char();
    }
    fn G1_zero() -> Self {
        Self::new(
            alt_bn128_Fq::zero(),
            alt_bn128_Fq::one(),
            alt_bn128_Fq::zero(),
        )
    }
    fn G1_one() -> Self {
        Self::new(
            alt_bn128_Fq::zero(),
            alt_bn128_Fq::one(),
            alt_bn128_Fq::zero(),
        )
    }
    pub fn new(X: alt_bn128_Fq, Y: alt_bn128_Fq, Z: alt_bn128_Fq) -> Self {
        Self { X, Y, Z }
    }
    pub fn print(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            let copy = self.clone(); //alt_bn128_G1
            copy.to_affine_coordinates();
            print!(
                "({:N$} , {:N$})\n",
                copy.X.as_bigint().0.0,
                copy.Y.as_bigint().0.0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn print_coordinates(&self) {
        if self.is_zero() {
            print!("O\n");
        } else {
            print!(
                "({:N$} : {:N$} : {:N$})\n",
                self.X.as_bigint().0.0,
                self.Y.as_bigint().0.0,
                self.Z.as_bigint().0.0,
                N = alt_bn128_Fq::num_limbs
            );
        }
    }

    pub fn to_affine_coordinates(&mut self) {
        if self.is_zero() {
            self.X = alt_bn128_Fq::zero();
            self.Y = alt_bn128_Fq::one();
            self.Z = alt_bn128_Fq::zero();
        } else {
            let Z_inv = self.Z.inverse();
            let Z2_inv = Z_inv.squared();
            let Z3_inv = Z2_inv * Z_inv;
            self.X = self.X * Z2_inv;
            self.Y = self.Y * Z3_inv;
            self.Z = alt_bn128_Fq::one();
        }
    }

    pub fn to_special(&self) {
        self.to_affine_coordinates();
    }

    pub fn is_special(&self) -> bool {
        return (self.is_zero() || self.Z == alt_bn128_Fq::one());
    }

    pub fn is_zero(&self) -> bool {
        return (self.Z.is_zero());
    }

    pub fn add(&self, other: &alt_bn128_G1) -> Self {
        return self.clone() + other;
    }

    pub fn mixed_add(&self, other: &alt_bn128_G1) -> Self {
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
        let I = HH + HH; // I = 4*HH
        I = I + I;
        let J = H * I; // J = H*I
        let r = S2 - (self.Y); // r = 2*(S2-Y1)
        r = r + r;
        let V = (self.X) * I; // V = X1*I
        let X3 = r.squared() - J - V - V; // X3 = r^2-J-2*V
        let Y3 = (self.Y) * J; // Y3 = r*(V-X3)-2*Y1*J
        Y3 = r * (V - X3) - Y3 - Y3;
        let Z3 = ((self.Z) + H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

        alt_bn128_G1(X3, Y3, Z3)
    }

    pub fn dbl(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // self.dbl_cnt++;
        //#endif
        // handle point at infinity
        if self.is_zero() {
            return self.clone();
        }

        // no need to handle points of order 2,4
        // (they cannot exist in a prime-order subgroup)

        // using Jacobian coordinates according to
        // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l

        let A = (self.X).squared(); // A = X1^2
        let B = (self.Y).squared(); // B = Y1^2
        let C = B.squared(); // C = B^2
        let D = (self.X + B).squared() - A - C;
        D = D + D; // D = 2 * ((X1 + B)^2 - A - C)
        let E = A + A + A; // E = 3 * A
        let F = E.squared(); // F = E^2
        let X3 = F - (D + D); // X3 = F - 2 D
        let eightC = C + C;
        eightC = eightC + eightC;
        eightC = eightC + eightC;
        let Y3 = E * (D - X3) - eightC; // Y3 = E * (D - X3) - 8 * C
        let Y1Z1 = (self.Y) * (self.Z);
        let Z3 = Y1Z1 + Y1Z1; // Z3 = 2 * Y1 * Z1

        alt_bn128_G1(X3, Y3, Z3)
    }

    pub fn mul_by_cofactor(&self) -> Self {
        // Cofactor = 1
        self.clone()
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

        return (Y2 == X3 + alt_bn128_coeff_b * Z6);
    }

    fn zero() -> Self {
        Self::G1_zero()
    }

    fn one() -> Self {
        Self::G1_zero()
    }

    fn random_element() -> Self {
        (scalar_field::random_element().as_bigint()) * G1_one()
    }

    pub fn batch_to_special_all_non_zeros(vec: &Vec<alt_bn128_G1>) {
        let mut Z_vec = Vec::with_capacity(vec.len());

        for el in &vec {
            Z_vec.push(el.Z);
        }
        batch_invert::<alt_bn128_Fq>(Z_vec);

        let one = alt_bn128_Fq::one();

        for i in 0..vec.len() {
            let Z2 = Z_vec[i].squared();
            let Z3 = Z_vec[i] * Z2;

            vec[i].X = vec[i].X * Z2;
            vec[i].Y = vec[i].Y * Z3;
            vec[i].Z = one;
        }
    }
}

// bool alt_bn128_G1::operator==(other:&alt_bn128_G1)
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

//     alt_bn128_Fq Z1_squared = (self.Z).squared();
//     alt_bn128_Fq Z2_squared = (other.Z).squared();

//     if (self.X * Z2_squared) != (other.X * Z1_squared)
//     {
//         return false;
//     }

//     alt_bn128_Fq Z1_cubed = (self.Z) * Z1_squared;
//     alt_bn128_Fq Z2_cubed = (other.Z) * Z2_squared;

//     return !((self.Y * Z2_cubed) != (other.Y * Z1_cubed));
// }

// bool alt_bn128_G1::operator!=(other:&alt_bn128_G1)
// {
//     return !(operator==(other));
// }

// alt_bn128_G1 alt_bn128_G1::operator+(other:&alt_bn128_G1)
// {
//     // handle special cases having to do with O
//     if self.is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return self.clone();
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

//     alt_bn128_Fq Z1Z1 = (self.Z).squared();
//     alt_bn128_Fq Z2Z2 = (other.Z).squared();

//     alt_bn128_Fq U1 = self.X * Z2Z2;
//     alt_bn128_Fq U2 = other.X * Z1Z1;

//     alt_bn128_Fq Z1_cubed = (self.Z) * Z1Z1;
//     alt_bn128_Fq Z2_cubed = (other.Z) * Z2Z2;

//     alt_bn128_Fq S1 = (self.Y) * Z2_cubed;      // S1 = Y1 * Z2 * Z2Z2
//     alt_bn128_Fq S2 = (other.Y) * Z1_cubed;      // S2 = Y2 * Z1 * Z1Z1

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
//     alt_bn128_Fq H = U2 - U1;                            // H = U2-U1
//     alt_bn128_Fq S2_minus_S1 = S2-S1;
//     alt_bn128_Fq I = (H+H).squared();                    // I = (2 * H)^2
//     alt_bn128_Fq J = H * I;                              // J = H * I
//     alt_bn128_Fq r = S2_minus_S1 + S2_minus_S1;          // r = 2 * (S2-S1)
//     alt_bn128_Fq V = U1 * I;                             // V = U1 * I
//     alt_bn128_Fq X3 = r.squared() - J - (V+V);           // X3 = r^2 - J - 2 * V
//     alt_bn128_Fq S1_J = S1 * J;
//     alt_bn128_Fq Y3 = r * (V-X3) - (S1_J+S1_J);          // Y3 = r * (V-X3)-2 S1 J
//     alt_bn128_Fq Z3 = ((self.Z+other.Z).squared()-Z1Z1-Z2Z2) * H; // Z3 = ((Z1+Z2)^2-Z1Z1-Z2Z2) * H

//     return alt_bn128_G1(X3, Y3, Z3);
// }

// alt_bn128_G1 alt_bn128_G1::operator-()
// {
//     return alt_bn128_G1(self.X, -(self.Y), self.Z);
// }

// alt_bn128_G1 alt_bn128_G1::operator-(other:&alt_bn128_G1)
// {
//     return self.clone() + (-other);
// }

// std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G1)
// {
//     alt_bn128_G1 copy(g);
//     copy.to_affine_coordinates();

//     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // #ifdef NO_PT_COMPRESSION
//     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// #else
//     /* storing LSB of Y */
//     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.as_bigint().0.0[0] & 1);
// //#endif

//     return out;
// }

// std::istream& operator>>(std::istream &in, alt_bn128_G1 &g)
// {
//     char is_zero;
//     alt_bn128_Fq tX, tY;

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
//         alt_bn128_Fq tX2 = tX.squared();
//         alt_bn128_Fq tY2 = tX2*tX + alt_bn128_coeff_b;
//         tY = tY2.sqrt();

//         if (tY.as_bigint().0.0[0] & 1) != Y_lsb
//         {
//             tY = -tY;
//         }
//     }
// //#endif
//     // using Jacobian coordinates
//     if is_zero == 0
//     {
//         g.X = tX;
//         g.Y = tY;
//         g.Z = alt_bn128_Fq::one();
//     }
//     else
//     {
//         g = alt_bn128_G1::zero();
//     }

//     return in;
// }

// std::ostream& operator<<(std::ostream& out, v:&Vec<alt_bn128_G1>)
// {
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// std::istream& operator>>(std::istream& in, Vec<alt_bn128_G1> &v)
// {
//     v.clear();

//     usize s;
//     in >> s;
//     consume_newline(in);

//     v.reserve(s);

//     for i in 0..s
//     {
//         alt_bn128_G1 g;
//         in >> g;
//         consume_OUTPUT_NEWLINE(in);
//         v.push(g);
//     }

//     return in;
// }
