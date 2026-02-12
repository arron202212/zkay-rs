//  Holds all of the arithmetic operators for the classes declared in variable.hpp .

//  This take clutter out of variable.hpp while leaving the * operators in a header file,
//  thus allowing them to be inlined, for optimization purposes.

use super::variable::{FElem, LinearCombination, LinearTerm, Monomial, Polynomial, Variable};
use std::ops::{Add, Mul, Neg, Sub};

/***         operator+           ***/

// // Polynomial
// inline Polynomial        operator+(first:Polynomial&,        second:&Polynomial)        {auto retval = first; return retval += second;}
impl Add<&Self> for Polynomial {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        let mut retval = self;
        retval += rhs;
        retval
    }
}
// // Monomial
// inline Polynomial        operator+(first:Monomial&,          second:&Polynomial)        {return Polynomial(first) + second;}
impl Add<&Polynomial> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Polynomial) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}
// inline Polynomial        operator+(first:Monomial&,          second:&Monomial)          {return Polynomial(first) + Polynomial(second);}
impl Add<&Self> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Polynomial::from(self) + &Polynomial::from(rhs.clone())
    }
}
// // LinearCombination
// inline Polynomial        operator+(first:LinearCombination&, second:&Polynomial)        {return Polynomial(first) + second;}
impl Add<&Polynomial> for LinearCombination {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Polynomial) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}
// inline Polynomial        operator+(first:LinearCombination&, second:&Monomial)          {return Polynomial(first) + second;}
impl Add<&Monomial> for LinearCombination {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Monomial) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:LinearCombination&, second:&LinearCombination) {auto retval = first; return retval += second;}
impl Add<&Self> for LinearCombination {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        let mut retval = self;
        retval += rhs;
        retval
    }
}

// // LinearTerm
// inline Polynomial        operator+(first:LinearTerm&,        second:&Polynomial)        {return LinearCombination(first) + second;}
impl Add<&Polynomial> for LinearTerm {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Polynomial) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline Polynomial        operator+(first:LinearTerm&,        second:&Monomial)          {return LinearCombination(first) + second;}
impl Add<&Monomial> for LinearTerm {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Monomial) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:LinearTerm&,        second:&LinearCombination) {return LinearCombination(first) + second;}
impl Add<&LinearCombination> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearCombination) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:LinearTerm&,        second:&LinearTerm)        {return LinearCombination(first) + LinearCombination(second);}
impl Add<&LinearTerm> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        LinearCombination::from(self) + &LinearCombination::from(rhs.clone())
    }
}

// // Variable
// inline Polynomial        operator+(first:Variable&,          second:&Polynomial)        {return LinearTerm(first) + second;}
impl Add<&Polynomial> for Variable {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Polynomial) -> Self::Output {
        LinearTerm::from(self) + rhs
    }
}
// inline Polynomial        operator+(first:Variable&,          second:&Monomial)          {return LinearTerm(first) + second;}
impl Add<&Monomial> for Variable {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Monomial) -> Self::Output {
        LinearTerm::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:Variable&,          second:&LinearCombination) {return LinearTerm(first) + second;}
impl Add<&LinearCombination> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearCombination) -> Self::Output {
        LinearTerm::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:Variable&,          second:&LinearTerm)        {return LinearTerm(first) + second;}
impl Add<&LinearTerm> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        LinearTerm::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:Variable&,          second:&Variable)          {return LinearTerm(first) + LinearTerm(second);}
impl Add<&Variable> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        LinearTerm::from(self) + &LinearTerm::from(rhs.clone())
    }
}

// // FElem
// inline Polynomial        operator+(first:FElem&,             second:&Polynomial)        {return LinearCombination(first) + second;}
impl Add<&Polynomial> for FElem {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Polynomial) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline Polynomial        operator+(first:FElem&,             second:&Monomial)          {return LinearCombination(first) + second;}
impl Add<&Monomial> for FElem {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Monomial) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:FElem&,             second:&LinearCombination) {return LinearCombination(first) + second;}
impl Add<&LinearCombination> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearCombination) -> Self::Output {
        LinearCombination::from(self) + rhs
    }
}
// inline LinearCombination operator+(first:FElem&,             second:&LinearTerm)        {return LinearCombination(first) + LinearCombination(second);}
impl Add<&LinearTerm> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        LinearCombination::from(self) + &LinearCombination::from(rhs.clone())
    }
}
// inline LinearCombination operator+(first:FElem&,             second:&Variable)          {return LinearCombination(first) + LinearCombination(second);}
impl Add<&Variable> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        LinearCombination::from(self) + &LinearCombination::from(rhs.clone())
    }
}
// inline FElem             operator+(first:FElem&,             second:&FElem)             {auto retval = first; return retval += second;}
impl Add<&FElem> for FElem {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        let mut retval = self;
        retval += rhs;
        retval
    }
}

// // int
// inline FElem             operator+(first:int,                second:&FElem)             {return FElem(first) + second;}

// inline LinearCombination operator+(first:int,                second:&Variable)          {return FElem(first) + second;}

// inline LinearCombination operator+(first:int,                second:&LinearTerm)        {return FElem(first) + second;}

// inline LinearCombination operator+(first:int,                second:&LinearCombination) {return FElem(first) + second;}

// inline Polynomial        operator+(first:int,                second:&Monomial)          {return FElem(first) + second;}

// inline Polynomial        operator+(first:int,                second:&Polynomial)        {return FElem(first) + second;}

// // symetrical operators
// inline Polynomial        operator+(first:Polynomial&,        second:&Monomial)          {return second + first;}
impl Add<&Monomial> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Monomial) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Monomial&,          second:&LinearCombination) {return second + first;}
impl Add<&LinearCombination> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &LinearCombination) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Polynomial&,        second:&LinearCombination) {return second + first;}
impl Add<&LinearCombination> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &LinearCombination) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline LinearCombination operator+(first:LinearCombination&, second:&LinearTerm)        {return second + first;}
impl Add<&LinearTerm> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Monomial&,          second:&LinearTerm)        {return second + first;}
impl Add<&LinearTerm> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Polynomial&,        second:&LinearTerm)        {return second + first;}
impl Add<&LinearTerm> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline LinearCombination operator+(first:LinearTerm&,        second:&Variable)          {return second + first;}
impl Add<&Variable> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline LinearCombination operator+(first:LinearCombination&, second:&Variable)          {return second + first;}
impl Add<&Variable> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Monomial&,          second:&Variable)          {return second + first;}
impl Add<&Variable> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Polynomial&,        second:&Variable)          {return second + first;}
impl Add<&Variable> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &Variable) -> Self::Output {
        (rhs.clone() + &self).into()
    }
}
// inline LinearCombination operator+(first:Variable&,          second:&FElem)             {return second + first;}
impl Add<&FElem> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline LinearCombination operator+(first:LinearTerm&,        second:&FElem)             {return second + first;}
impl Add<&FElem> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline LinearCombination operator+(first:LinearCombination&, second:&FElem)             {return second + first;}
impl Add<&FElem> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Monomial&,          second:&FElem)             {return second + first;}
impl Add<&FElem> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline Polynomial        operator+(first:Polynomial&,        second:&FElem)             {return second + first;}
impl Add<&FElem> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: &FElem) -> Self::Output {
        rhs.clone() + &self
    }
}
// inline FElem             operator+(first:FElem&,             first:int second)                {return second +,}
impl Add<i32> for FElem {
    type Output = FElem;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}
// inline LinearCombination operator+(first:Variable&,          first:int second)                {return second +,}
impl Add<i32> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}
// inline LinearCombination operator+(first:LinearTerm&,        first:int second)                {return second +,}
impl Add<i32> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}
// inline LinearCombination operator+(first:LinearCombination&, first:int second)                {return second +,}
impl Add<i32> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}
// inline Polynomial        operator+(first:Monomial&,          first:int second)                {return second +,}
impl Add<i32> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}
// inline Polynomial        operator+(first:Polynomial&,        first:int second)                {return second +,}
impl Add<i32> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        FElem::from(rhs) + &self
    }
}

//
// /***           operator-         ***/
//
// inline LinearTerm        operator-(src:&Variable) {return LinearTerm(src, -1);}
impl Neg for Variable {
    type Output = LinearTerm;
    #[inline]
    fn neg(self) -> Self::Output {
        LinearTerm::new2(self, -1)
    }
}
// inline Polynomial        operator-(first:Polynomial&,        second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Monomial&,          second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Monomial&,          second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:LinearCombination&, second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for LinearCombination {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:LinearCombination&, second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for LinearCombination {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearCombination&, second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:LinearTerm&,        second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for LinearTerm {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:LinearTerm&,        second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for LinearTerm {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearTerm&,        second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearTerm&,        second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Variable&,          second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for Variable {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Variable&,          second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for Variable {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:Variable&,          second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:Variable&,          second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:Variable&,          second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:FElem&,             second:&Polynomial)        {return first + (-second);}
impl Sub<&Polynomial> for FElem {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Polynomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:FElem&,             second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for FElem {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:FElem&,             second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:FElem&,             second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:FElem&,             second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for FElem {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline FElem             operator-(first:FElem&,             second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for FElem {
    type Output = FElem;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}

// inline FElem             operator-(first:int,                second:&FElem)             {return first + (-second);}

// inline LinearCombination operator-(first:int,                second:&Variable)          {return first + (-second);}

// inline LinearCombination operator-(first:int,                second:&LinearTerm)        {return first + (-second);}

// inline LinearCombination operator-(first:int,                second:&LinearCombination) {return first + (-second);}

// inline Polynomial        operator-(first:int,                second:&Monomial)          {return first + (-second);}

// inline Polynomial        operator-(first:int,                second:&Polynomial)        {return first + (-second);}

// inline Polynomial        operator-(first:Polynomial&,        second:&Monomial)          {return first + (-second);}
impl Sub<&Monomial> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Monomial) -> Self::Output {
        self + &(-rhs.clone())
    }
}

// inline Polynomial        operator-(first:Monomial&,          second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Polynomial&,        second:&LinearCombination) {return first + (-second);}
impl Sub<&LinearCombination> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &LinearCombination) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearCombination&, second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Monomial&,          second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Polynomial&,        second:&LinearTerm)        {return first + (-second);}
impl Sub<&LinearTerm> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &LinearTerm) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearTerm&,        second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearCombination&, second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Monomial&,          second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Polynomial&,        second:&Variable)          {return first + (-second);}
impl Sub<&Variable> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &Variable) -> Self::Output {
        self + &(-rhs.clone())
    }
}

// inline LinearCombination operator-(first:Variable&,          second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearTerm&,        second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline LinearCombination operator-(first:LinearCombination&, second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Monomial&,          second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline Polynomial        operator-(first:Polynomial&,        second:&FElem)             {return first + (-second);}
impl Sub<&FElem> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: &FElem) -> Self::Output {
        self + &(-rhs.clone())
    }
}
// inline FElem             operator-(first:FElem&,             const int second)                {return first + (-second);}
impl Sub<i32> for FElem {
    type Output = FElem;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}
// inline LinearCombination operator-(first:Variable&,          const int second)                {return first + (-second);}
impl Sub<i32> for Variable {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}
// inline LinearCombination operator-(first:LinearTerm&,        const int second)                {return first + (-second);}
impl Sub<i32> for LinearTerm {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}
// inline LinearCombination operator-(first:LinearCombination&, const int second)                {return first + (-second);}
impl Sub<i32> for LinearCombination {
    type Output = LinearCombination;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}
// inline Polynomial        operator-(first:Monomial&,          const int second)                {return first + (-second);}
impl Sub<i32> for Monomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}
// inline Polynomial        operator-(first:Polynomial&,        const int second)                {return first + (-second);}
impl Sub<i32> for Polynomial {
    type Output = Polynomial;
    #[inline]
    fn sub(self, rhs: i32) -> Self::Output {
        (self + (-rhs)).into()
    }
}

//
// /***         operator*           ***/
//
// // Polynomial
// inline Polynomial        operator*(first:Polynomial&,        second:&Polynomial)        {auto retval = first; return retval *= second;}
impl Mul<&Polynomial> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        let mut retval = self;
        retval *= rhs;
        retval
    }
}
// // Monomial
// inline Polynomial        operator*(first:Monomial&,          second:&Polynomial)        {return Polynomial(first) * second;}
impl Mul<&Polynomial> for Monomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}
// inline Monomial          operator*(first:Monomial&,          second:&Monomial)          {auto retval = first; return retval *= second;}
impl Mul<&Monomial> for Monomial {
    type Output = Self;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        let mut retval = self;
        retval *= rhs;
        retval
    }
}
// // LinearCombination
// inline Polynomial        operator*(first:LinearCombination&, second:&Polynomial)        {return Polynomial(first) * second;}
impl Mul<&Polynomial> for LinearCombination {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}
// inline Polynomial        operator*(first:LinearCombination&, second:&Monomial)          {return first * Polynomial(second);}
impl Mul<&Monomial> for LinearCombination {
    type Output = Polynomial;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        self * &Polynomial::from(rhs.clone())
    }
}
// inline Polynomial        operator*(first:LinearCombination&, second:&LinearCombination) {return first * Polynomial(second);}
impl Mul<&LinearCombination> for LinearCombination {
    type Output = Polynomial;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        self * &Polynomial::from(rhs.clone())
    }
}

// // LinearTerm
// inline Polynomial        operator*(first:LinearTerm&,        second:&Polynomial)        {return LinearCombination(first) * second;}
impl Mul<&Polynomial> for LinearTerm {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        LinearCombination::from(self) * rhs
    }
}
// inline Monomial          operator*(first:LinearTerm&,        second:&Monomial)          {return Monomial(first) * second;}
impl Mul<&Monomial> for LinearTerm {
    type Output = Monomial;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        Monomial::from(self) * rhs
    }
}
// inline Polynomial        operator*(first:LinearTerm&,        second:&LinearCombination) {return LinearCombination(first) * second;}
impl Mul<&LinearCombination> for LinearTerm {
    type Output = Polynomial;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        LinearCombination::from(self) * rhs
    }
}
// inline Monomial          operator*(first:LinearTerm&,        second:&LinearTerm)        {return Monomial(first) * Monomial(second);}
impl Mul<&LinearTerm> for LinearTerm {
    type Output = Monomial;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        Monomial::from(self) * &Monomial::from(rhs.clone())
    }
}

// // Variable
// inline Polynomial        operator*(first:Variable&,          second:&Polynomial)        {return LinearTerm(first) * second;}
impl Mul<&Polynomial> for Variable {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        LinearTerm::from(self) * rhs
    }
}
// inline Monomial          operator*(first:Variable&,          second:&Monomial)          {return Monomial(first) * second;}
impl Mul<&Monomial> for Variable {
    type Output = Monomial;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        Monomial::from(self) * rhs
    }
}
// inline Polynomial        operator*(first:Variable&,          second:&LinearCombination) {return LinearTerm(first) * second;}
impl Mul<&LinearCombination> for Variable {
    type Output = Polynomial;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        LinearTerm::from(self) * rhs
    }
}
// inline Monomial          operator*(first:Variable&,          second:&LinearTerm)        {return LinearTerm(first) * second;}
impl Mul<&LinearTerm> for Variable {
    type Output = Monomial;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        LinearTerm::from(self) * rhs
    }
}
// inline Monomial          operator*(first:Variable&,          second:&Variable)          {return LinearTerm(first) * LinearTerm(second);}
impl Mul<&Variable> for Variable {
    type Output = Monomial;

    fn mul(self, rhs: &Variable) -> Self::Output {
        LinearTerm::from(self) * &LinearTerm::from(rhs.clone())
    }
}

// // FElem
// inline Polynomial        operator*(first:FElem&,             second:&Polynomial)        {return LinearCombination(first) * second;}
impl Mul<&Polynomial> for FElem {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Self::Output {
        LinearCombination::from(self) * rhs
    }
}
// inline Monomial          operator*(first:FElem&,             second:&Monomial)          {return Monomial(first) * second;}
impl Mul<&Monomial> for FElem {
    type Output = Monomial;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        Monomial::from(self) * rhs
    }
}
// inline LinearCombination operator*(first:FElem&,             second:&LinearCombination) {auto retval = second; return retval *= first;}
impl Mul<&LinearCombination> for FElem {
    type Output = LinearCombination;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        let mut retval = rhs.clone();
        retval *= &self;
        retval
    }
}
// inline LinearTerm        operator*(first:FElem&,             second:&LinearTerm)        {auto retval = second; return retval *= first;}
impl Mul<&LinearTerm> for FElem {
    type Output = LinearTerm;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        let mut retval = rhs.clone();
        retval *= &self;
        retval
    }
}
// inline LinearTerm        operator*(first:FElem&,             second:&Variable)          {return LinearTerm(second) *= first;}
impl Mul<&Variable> for FElem {
    type Output = LinearTerm;

    fn mul(self, rhs: &Variable) -> Self::Output {
        LinearTerm::from(rhs.clone()) * &self
    }
}
// inline FElem             operator*(first:FElem&,             second:&FElem)             {auto retval = first; return retval *= second;}
impl Mul<&FElem> for FElem {
    type Output = Self;

    fn mul(self, rhs: &FElem) -> Self::Output {
        let mut retval = self;
        retval *= rhs;
        retval
    }
}

// // int
// inline FElem             operator*(first:int,                second:&FElem)             {return FElem(first) * second;}

// inline LinearTerm        operator*(first:int,                second:&Variable)          {return FElem(first) * second;}

// inline LinearTerm        operator*(first:int,                second:&LinearTerm)        {return FElem(first) * second;}

// inline LinearCombination operator*(first:int,                second:&LinearCombination) {return FElem(first) * second;}

// inline Monomial          operator*(first:int,                second:&Monomial)          {return FElem(first) * second;}

// inline Polynomial        operator*(first:int,                second:&Polynomial)        {return FElem(first) * second;}

// // symetrical operators
// inline Polynomial        operator*(first:Polynomial&,        second:&Monomial)          {return second * first;}
impl Mul<&Monomial> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &Monomial) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Polynomial        operator*(first:Monomial&,          second:&LinearCombination) {return second * first;}
impl Mul<&LinearCombination> for Monomial {
    type Output = Polynomial;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        let rhs: LinearCombination = rhs.clone();
        rhs * &self
    }
}
// inline Polynomial        operator*(first:Polynomial&,        second:&LinearCombination) {return second * first;}
impl Mul<&LinearCombination> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &LinearCombination) -> Self::Output {
        let rhs: LinearCombination = rhs.clone();
        rhs * &self
    }
}
// inline Polynomial        operator*(first:LinearCombination&, second:&LinearTerm)        {return second * first;}
impl Mul<&LinearTerm> for LinearCombination {
    type Output = Polynomial;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Monomial          operator*(first:Monomial&,          second:&LinearTerm)        {return second * first;}
impl Mul<&LinearTerm> for Monomial {
    type Output = Self;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Polynomial        operator*(first:Polynomial&,        second:&LinearTerm)        {return second * first;}
impl Mul<&LinearTerm> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &LinearTerm) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Monomial          operator*(first:LinearTerm&,        second:&Variable)          {return second * first;}
impl Mul<&Variable> for LinearTerm {
    type Output = Monomial;

    fn mul(self, rhs: &Variable) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Polynomial        operator*(first:LinearCombination&, second:&Variable)          {return second * first;}
impl Mul<&Variable> for LinearCombination {
    type Output = Polynomial;

    fn mul(self, rhs: &Variable) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Monomial          operator*(first:Monomial&,          second:&Variable)          {return second * first;}
impl Mul<&Variable> for Monomial {
    type Output = Self;

    fn mul(self, rhs: &Variable) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Polynomial        operator*(first:Polynomial&,        second:&Variable)          {return second * first;}
impl Mul<&Variable> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &Variable) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline LinearTerm        operator*(first:Variable&,          second:&FElem)             {return second * first;}
impl Mul<&FElem> for Variable {
    type Output = LinearTerm;

    fn mul(self, rhs: &FElem) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline LinearTerm        operator*(first:LinearTerm&,        second:&FElem)             {return second * first;}
impl Mul<&FElem> for LinearTerm {
    type Output = Self;

    fn mul(self, rhs: &FElem) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline LinearCombination operator*(first:LinearCombination&, second:&FElem)             {return second * first;}
impl Mul<&FElem> for LinearCombination {
    type Output = Self;

    fn mul(self, rhs: &FElem) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Monomial          operator*(first:Monomial&,          second:&FElem)             {return second * first;}
impl Mul<&FElem> for Monomial {
    type Output = Self;

    fn mul(self, rhs: &FElem) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline Polynomial        operator*(first:Polynomial&,        second:&FElem)             {return second * first;}
impl Mul<&FElem> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: &FElem) -> Self::Output {
        rhs.clone() * &self
    }
}
// inline FElem             operator*(first:FElem&,             first:int second)                {return second *,}
impl Mul<i32> for FElem {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}
// inline LinearTerm        operator*(first:Variable&,          first:int second)                {return second *,}
impl Mul<i32> for Variable {
    type Output = LinearTerm;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}
// inline LinearTerm        operator*(first:LinearTerm&,        first:int second)                {return second *,}
impl Mul<i32> for LinearTerm {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}
// inline LinearCombination operator*(first:LinearCombination&, first:int second)                {return second *,}
impl Mul<i32> for LinearCombination {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}
// inline Monomial          operator*(first:Monomial&,          first:int second)                {return second *,}
impl Mul<i32> for Monomial {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}
// inline Polynomial        operator*(first:Polynomial&,        first:int second)                {return second *,}
impl Mul<i32> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        FElem::from(rhs.clone()) * &self
    }
}

//
// /***      END OF OPERATORS       ***/
//
