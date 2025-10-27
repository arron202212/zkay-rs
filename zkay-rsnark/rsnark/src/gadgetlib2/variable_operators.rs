/** @file
 *****************************************************************************
 Holds all of the arithmetic operators for the classes declared in variable.hpp .

 This take clutter out of variable.hpp while leaving the * operators in a header file,
 thus allowing them to be inlined, for optimization purposes.
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLEOPERATORS_HPP_
// #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLEOPERATORS_HPP_

use crate::gadgetlib2::variable;

// namespace gadgetlib2 {

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    lots o' operators                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/***********************************/
/***         operator+           ***/
/***********************************/

// // Polynomial
// inline Polynomial        operator+(first:Polynomial&,        second:&Polynomial)        {auto retval = first; return retval += second;}

// // Monomial
// inline Polynomial        operator+(first:Monomial&,          second:&Polynomial)        {return Polynomial(first) + second;}
// inline Polynomial        operator+(first:Monomial&,          second:&Monomial)          {return Polynomial(first) + Polynomial(second);}

// // LinearCombination
// inline Polynomial        operator+(first:LinearCombination&, second:&Polynomial)        {return Polynomial(first) + second;}
// inline Polynomial        operator+(first:LinearCombination&, second:&Monomial)          {return Polynomial(first) + second;}
// inline LinearCombination operator+(first:LinearCombination&, second:&LinearCombination) {auto retval = first; return retval += second;}

// // LinearTerm
// inline Polynomial        operator+(first:LinearTerm&,        second:&Polynomial)        {return LinearCombination(first) + second;}
// inline Polynomial        operator+(first:LinearTerm&,        second:&Monomial)          {return LinearCombination(first) + second;}
// inline LinearCombination operator+(first:LinearTerm&,        second:&LinearCombination) {return LinearCombination(first) + second;}
// inline LinearCombination operator+(first:LinearTerm&,        second:&LinearTerm)        {return LinearCombination(first) + LinearCombination(second);}

// // Variable
// inline Polynomial        operator+(first:Variable&,          second:&Polynomial)        {return LinearTerm(first) + second;}
// inline Polynomial        operator+(first:Variable&,          second:&Monomial)          {return LinearTerm(first) + second;}
// inline LinearCombination operator+(first:Variable&,          second:&LinearCombination) {return LinearTerm(first) + second;}
// inline LinearCombination operator+(first:Variable&,          second:&LinearTerm)        {return LinearTerm(first) + second;}
// inline LinearCombination operator+(first:Variable&,          second:&Variable)          {return LinearTerm(first) + LinearTerm(second);}

// // FElem
// inline Polynomial        operator+(first:FElem&,             second:&Polynomial)        {return LinearCombination(first) + second;}
// inline Polynomial        operator+(first:FElem&,             second:&Monomial)          {return LinearCombination(first) + second;}
// inline LinearCombination operator+(first:FElem&,             second:&LinearCombination) {return LinearCombination(first) + second;}
// inline LinearCombination operator+(first:FElem&,             second:&LinearTerm)        {return LinearCombination(first) + LinearCombination(second);}
// inline LinearCombination operator+(first:FElem&,             second:&Variable)          {return LinearCombination(first) + LinearCombination(second);}
// inline FElem             operator+(first:FElem&,             second:&FElem)             {auto retval = first; return retval += second;}

// // int
// inline FElem             operator+(first:int,                second:&FElem)             {return FElem(first) + second;}
// inline LinearCombination operator+(first:int,                second:&Variable)          {return FElem(first) + second;}
// inline LinearCombination operator+(first:int,                second:&LinearTerm)        {return FElem(first) + second;}
// inline LinearCombination operator+(first:int,                second:&LinearCombination) {return FElem(first) + second;}
// inline Polynomial        operator+(first:int,                second:&Monomial)          {return FElem(first) + second;}
// inline Polynomial        operator+(first:int,                second:&Polynomial)        {return FElem(first) + second;}

// // symetrical operators
// inline Polynomial        operator+(first:Polynomial&,        second:&Monomial)          {return second + first;}
// inline Polynomial        operator+(first:Monomial&,          second:&LinearCombination) {return second + first;}
// inline Polynomial        operator+(first:Polynomial&,        second:&LinearCombination) {return second + first;}
// inline LinearCombination operator+(first:LinearCombination&, second:&LinearTerm)        {return second + first;}
// inline Polynomial        operator+(first:Monomial&,          second:&LinearTerm)        {return second + first;}
// inline Polynomial        operator+(first:Polynomial&,        second:&LinearTerm)        {return second + first;}
// inline LinearCombination operator+(first:LinearTerm&,        second:&Variable)          {return second + first;}
// inline LinearCombination operator+(first:LinearCombination&, second:&Variable)          {return second + first;}
// inline Polynomial        operator+(first:Monomial&,          second:&Variable)          {return second + first;}
// inline Polynomial        operator+(first:Polynomial&,        second:&Variable)          {return second + first;}
// inline LinearCombination operator+(first:Variable&,          second:&FElem)             {return second + first;}
// inline LinearCombination operator+(first:LinearTerm&,        second:&FElem)             {return second + first;}
// inline LinearCombination operator+(first:LinearCombination&, second:&FElem)             {return second + first;}
// inline Polynomial        operator+(first:Monomial&,          second:&FElem)             {return second + first;}
// inline Polynomial        operator+(first:Polynomial&,        second:&FElem)             {return second + first;}
// inline FElem             operator+(first:FElem&,             first:int second)                {return second +,}
// inline LinearCombination operator+(first:Variable&,          first:int second)                {return second +,}
// inline LinearCombination operator+(first:LinearTerm&,        first:int second)                {return second +,}
// inline LinearCombination operator+(first:LinearCombination&, first:int second)                {return second +,}
// inline Polynomial        operator+(first:Monomial&,          first:int second)                {return second +,}
// inline Polynomial        operator+(first:Polynomial&,        first:int second)                {return second +,}

// /***********************************/
// /***           operator-         ***/
// /***********************************/
// inline LinearTerm        operator-(src:&Variable) {return LinearTerm(src, -1);}

// inline Polynomial        operator-(first:Polynomial&,        second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&Monomial)          {return first + (-second);}
// inline Polynomial        operator-(first:LinearCombination&, second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:LinearCombination&, second:&Monomial)          {return first + (-second);}
// inline LinearCombination operator-(first:LinearCombination&, second:&LinearCombination) {return first + (-second);}
// inline Polynomial        operator-(first:LinearTerm&,        second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:LinearTerm&,        second:&Monomial)          {return first + (-second);}
// inline LinearCombination operator-(first:LinearTerm&,        second:&LinearCombination) {return first + (-second);}
// inline LinearCombination operator-(first:LinearTerm&,        second:&LinearTerm)        {return first + (-second);}
// inline Polynomial        operator-(first:Variable&,          second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:Variable&,          second:&Monomial)          {return first + (-second);}
// inline LinearCombination operator-(first:Variable&,          second:&LinearCombination) {return first + (-second);}
// inline LinearCombination operator-(first:Variable&,          second:&LinearTerm)        {return first + (-second);}
// inline LinearCombination operator-(first:Variable&,          second:&Variable)          {return first + (-second);}
// inline Polynomial        operator-(first:FElem&,             second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:FElem&,             second:&Monomial)          {return first + (-second);}
// inline LinearCombination operator-(first:FElem&,             second:&LinearCombination) {return first + (-second);}
// inline LinearCombination operator-(first:FElem&,             second:&LinearTerm)        {return first + (-second);}
// inline LinearCombination operator-(first:FElem&,             second:&Variable)          {return first + (-second);}
// inline FElem             operator-(first:FElem&,             second:&FElem)             {return first + (-second);}
// inline FElem             operator-(first:int,                second:&FElem)             {return first + (-second);}
// inline LinearCombination operator-(first:int,                second:&Variable)          {return first + (-second);}
// inline LinearCombination operator-(first:int,                second:&LinearTerm)        {return first + (-second);}
// inline LinearCombination operator-(first:int,                second:&LinearCombination) {return first + (-second);}
// inline Polynomial        operator-(first:int,                second:&Monomial)          {return first + (-second);}
// inline Polynomial        operator-(first:int,                second:&Polynomial)        {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        second:&Monomial)          {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&LinearCombination) {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        second:&LinearCombination) {return first + (-second);}
// inline LinearCombination operator-(first:LinearCombination&, second:&LinearTerm)        {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&LinearTerm)        {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        second:&LinearTerm)        {return first + (-second);}
// inline LinearCombination operator-(first:LinearTerm&,        second:&Variable)          {return first + (-second);}
// inline LinearCombination operator-(first:LinearCombination&, second:&Variable)          {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&Variable)          {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        second:&Variable)          {return first + (-second);}
// inline LinearCombination operator-(first:Variable&,          second:&FElem)             {return first + (-second);}
// inline LinearCombination operator-(first:LinearTerm&,        second:&FElem)             {return first + (-second);}
// inline LinearCombination operator-(first:LinearCombination&, second:&FElem)             {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          second:&FElem)             {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        second:&FElem)             {return first + (-second);}
// inline FElem             operator-(first:FElem&,             const int second)                {return first + (-second);}
// inline LinearCombination operator-(first:Variable&,          const int second)                {return first + (-second);}
// inline LinearCombination operator-(first:LinearTerm&,        const int second)                {return first + (-second);}
// inline LinearCombination operator-(first:LinearCombination&, const int second)                {return first + (-second);}
// inline Polynomial        operator-(first:Monomial&,          const int second)                {return first + (-second);}
// inline Polynomial        operator-(first:Polynomial&,        const int second)                {return first + (-second);}

// /***********************************/
// /***         operator*           ***/
// /***********************************/

// // Polynomial
// inline Polynomial        operator*(first:Polynomial&,        second:&Polynomial)        {auto retval = first; return retval *= second;}

// // Monomial
// inline Polynomial        operator*(first:Monomial&,          second:&Polynomial)        {return Polynomial(first) * second;}
// inline Monomial          operator*(first:Monomial&,          second:&Monomial)          {auto retval = first; return retval *= second;}

// // LinearCombination
// inline Polynomial        operator*(first:LinearCombination&, second:&Polynomial)        {return Polynomial(first) * second;}
// inline Polynomial        operator*(first:LinearCombination&, second:&Monomial)          {return first * Polynomial(second);}
// inline Polynomial        operator*(first:LinearCombination&, second:&LinearCombination) {return first * Polynomial(second);}

// // LinearTerm
// inline Polynomial        operator*(first:LinearTerm&,        second:&Polynomial)        {return LinearCombination(first) * second;}
// inline Monomial          operator*(first:LinearTerm&,        second:&Monomial)          {return Monomial(first) * second;}
// inline Polynomial        operator*(first:LinearTerm&,        second:&LinearCombination) {return LinearCombination(first) * second;}
// inline Monomial          operator*(first:LinearTerm&,        second:&LinearTerm)        {return Monomial(first) * Monomial(second);}

// // Variable
// inline Polynomial        operator*(first:Variable&,          second:&Polynomial)        {return LinearTerm(first) * second;}
// inline Monomial          operator*(first:Variable&,          second:&Monomial)          {return Monomial(first) * second;}
// inline Polynomial        operator*(first:Variable&,          second:&LinearCombination) {return LinearTerm(first) * second;}
// inline Monomial          operator*(first:Variable&,          second:&LinearTerm)        {return LinearTerm(first) * second;}
// inline Monomial          operator*(first:Variable&,          second:&Variable)          {return LinearTerm(first) * LinearTerm(second);}

// // FElem
// inline Polynomial        operator*(first:FElem&,             second:&Polynomial)        {return LinearCombination(first) * second;}
// inline Monomial          operator*(first:FElem&,             second:&Monomial)          {return Monomial(first) * second;}
// inline LinearCombination operator*(first:FElem&,             second:&LinearCombination) {auto retval = second; return retval *= first;}
// inline LinearTerm        operator*(first:FElem&,             second:&LinearTerm)        {auto retval = second; return retval *= first;}
// inline LinearTerm        operator*(first:FElem&,             second:&Variable)          {return LinearTerm(second) *= first;}
// inline FElem             operator*(first:FElem&,             second:&FElem)             {auto retval = first; return retval *= second;}

// // int
// inline FElem             operator*(first:int,                second:&FElem)             {return FElem(first) * second;}
// inline LinearTerm        operator*(first:int,                second:&Variable)          {return FElem(first) * second;}
// inline LinearTerm        operator*(first:int,                second:&LinearTerm)        {return FElem(first) * second;}
// inline LinearCombination operator*(first:int,                second:&LinearCombination) {return FElem(first) * second;}
// inline Monomial          operator*(first:int,                second:&Monomial)          {return FElem(first) * second;}
// inline Polynomial        operator*(first:int,                second:&Polynomial)        {return FElem(first) * second;}

// // symetrical operators
// inline Polynomial        operator*(first:Polynomial&,        second:&Monomial)          {return second * first;}
// inline Polynomial        operator*(first:Monomial&,          second:&LinearCombination) {return second * first;}
// inline Polynomial        operator*(first:Polynomial&,        second:&LinearCombination) {return second * first;}
// inline Polynomial        operator*(first:LinearCombination&, second:&LinearTerm)        {return second * first;}
// inline Monomial          operator*(first:Monomial&,          second:&LinearTerm)        {return second * first;}
// inline Polynomial        operator*(first:Polynomial&,        second:&LinearTerm)        {return second * first;}
// inline Monomial          operator*(first:LinearTerm&,        second:&Variable)          {return second * first;}
// inline Polynomial        operator*(first:LinearCombination&, second:&Variable)          {return second * first;}
// inline Monomial          operator*(first:Monomial&,          second:&Variable)          {return second * first;}
// inline Polynomial        operator*(first:Polynomial&,        second:&Variable)          {return second * first;}
// inline LinearTerm        operator*(first:Variable&,          second:&FElem)             {return second * first;}
// inline LinearTerm        operator*(first:LinearTerm&,        second:&FElem)             {return second * first;}
// inline LinearCombination operator*(first:LinearCombination&, second:&FElem)             {return second * first;}
// inline Monomial          operator*(first:Monomial&,          second:&FElem)             {return second * first;}
// inline Polynomial        operator*(first:Polynomial&,        second:&FElem)             {return second * first;}
// inline FElem             operator*(first:FElem&,             first:int second)                {return second *,}
// inline LinearTerm        operator*(first:Variable&,          first:int second)                {return second *,}
// inline LinearTerm        operator*(first:LinearTerm&,        first:int second)                {return second *,}
// inline LinearCombination operator*(first:LinearCombination&, first:int second)                {return second *,}
// inline Monomial          operator*(first:Monomial&,          first:int second)                {return second *,}
// inline Polynomial        operator*(first:Polynomial&,        first:int second)                {return second *,}


// /***********************************/
// /***      END OF OPERATORS       ***/
// /***********************************/

// } // namespace gadgetlib2

//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLEOPERATORS_HPP_
