#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
/** @file
 *****************************************************************************
 Declaration of serialization routines and constants.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef SERIALIZATION_HPP_
// #define SERIALIZATION_HPP_

//#include <istream>
//#include <map>
//#include <ostream>
//#include <set>
//#include <vector>

// namespace libff {

/*
 * @todo
 * The serialization is fragile. Shoud be rewritten using a standard, portable-format
 * library like boost::serialize.
 *
 * However, for now the following conventions are used within the code.
 *
 * All algebraic objects support either binary or decimal output using
 * the standard C++ stream operators (operator<<, operator>>).
 *
 * The binary mode is activated by defining a BINARY_OUTPUT
 * preprocessor macro (e.g. g++ -DBINARY_OUTPUT ...).
 *
 * Binary output assumes that the stream is to be binary read at its
 * current position so any white space should be consumed beforehand.
 *
 * Consecutive algebraic objects are separated by OUTPUT_NEWLINE and
 * within themselves (e.g. X and Y coordinates for field elements) with
 * OUTPUT_SEPARATOR (as defined below).
 *
 * Therefore to dump two integers, two Fp elements and another integer
 * one would:
 *
 * out << 3 << "\n";
 * out << 4 << "\n";
 * out << FieldT(56) << OUTPUT_NEWLINE;
 * out << FieldT(78) << OUTPUT_NEWLINE;
 * out << 9 << "\n";
 *
 * Then reading back it its reader's responsibility (!) to consume "\n"
 * after 4, but Fp::operator<< will correctly consume OUTPUT_NEWLINE.
 *
 * The reader should also consume "\n" after 9, so that another field
 * element can be properly chained. This is especially important for
 * binary output.
 *
 * The binary serialization of algebraic objects is currently *not*
 * portable between machines of different word sizes.
 */

// #ifdef BINARY_OUTPUT
// #define OUTPUT_NEWLINE ""
// #define OUTPUT_SEPARATOR ""
// #else
// #define OUTPUT_NEWLINE "\n"
// #define OUTPUT_SEPARATOR " "
//#endif

// inline void consume_newline(ins:&impl BufRead);
// inline void consume_OUTPUT_NEWLINE(ins:&impl BufRead);
// inline void consume_OUTPUT_SEPARATOR(ins:&impl BufRead);

// inline void output_bool(out:& BufWriter, b:bool);
// inline void input_bool(ins:&impl BufRead, bool &b);

// inline void output_bool_vector(out:& BufWriter, const std::vector<bool> &v);
// inline void input_bool_vector(ins:&impl BufRead, std::vector<bool> &v);

// template<typename T>
// T reserialize(const T &obj);

// template<typename T>
// std::ostream& operator<<(std::ostream& out, const std::vector<T> &v);

// template<typename T>
// std::istream& operator>>(std::ostream& out, std::vector<T> &v);

// template<typename T1, typename T2>
// std::ostream& operator<<(std::ostream& out, const std::map<T1, T2> &m);

// template<typename T1, typename T2>
// std::istream& operator>>(std::istream& in, std::map<T1, T2> &m);

// template<typename T>
// std::ostream& operator<<(std::ostream& out, const std::set<T> &s);

// template<typename T>
// std::istream& operator>>(std::istream& in, std::set<T> &s);

// } // namespace libff

// use libff/common/serialization.tcc;

//#endif // SERIALIZATION_HPP_


/** @file
 *****************************************************************************
 Implementation of serialization routines.

 See serialization.hpp .
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef SERIALIZATION_TCC_
// #define SERIALIZATION_TCC_

//#include <cassert>
//#include <sstream>

use crate::common::utils;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
// namespace libff {

// using std::size_t;

#[inline]
pub fn  consume_newline(ins:&impl BufRead)
{
    // let mut c;
    // ins.read(&c, 1);
}

#[inline]
pub fn  consume_OUTPUT_NEWLINE(ins:&impl BufRead)
{
// #ifdef BINARY_OUTPUT
    // nothing to consume
    // UNUSED(in);
// #else
    // let mut c;
    // in.read(&c, 1);
//#endif
}

#[inline]
pub fn  consume_OUTPUT_SEPARATOR(ins:&impl BufRead)
{
// #ifdef BINARY_OUTPUT
    // nothing to consume
//     UNUSED(in);
// #else
    // char c;
    // ins.read(&c, 1);
//#endif
}

#[inline]
pub fn  output_bool<W: ?Sized + Write> (out:& BufWriter<W>, b:bool)
{
    write!(out,"{}\n", b as u8);
    // out << (b ? 1 : 0) << "\n";
}

#[inline]
pub fn input_bool(ins:&impl BufRead, b:bool)
{
    // size_t tmp;
    // in >> tmp;
    // consume_newline(in);
    // assert!(tmp == 0 || tmp == 1);

    // b = (tmp == 1 ? true : false);
}

#[inline]
pub fn  output_bool_vector<W: ?Sized + Write> (out:& BufWriter<W>, v:&Vec<bool>)
{
    // out << v.size() << "\n";
    // for (b:bool : v)
    // {
    //     output_bool(out, b);
    // }
}

#[inline]
pub fn  input_bool_vector(ins:&impl BufRead, v:&Vec<bool>)
{
    // size_t size;
    // in >> size;
    // consume_newline(in);
    // v.resize(size);
    // for i in 0..size
    // {
    //     bool b;
    //     input_bool(in, b);
    //     v[i] = b;
    // }
}

pub fn  reserialize<T>(obj:&T)->T
{
    // std::stringstream ss;
    // ss << obj;
    // T tmp;
    // ss >> tmp;
    // return tmp;

}



// }

//#endif // SERIALIZATION_TCC_


// template<typename T>
// std::ostream& operator<<(std::ostream& out, const std::vector<T> &v)
// {
//     assert!(!std::is_same<T, bool>::value, "this does not work for std::vector<bool>");
//     out << v.size() << "\n";
//     for (const T& t : v)
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// template<typename T>
// std::istream& operator>>(std::istream& in, std::vector<T> &v)
// {
//     assert!(!std::is_same<T, bool>::value, "this does not work for std::vector<bool>");
//     size_t size;
//     in >> size;
//     consume_newline(in);

//     v.resize(0);
//     for i in 0..size
//     {
//         T elt;
//         in >> elt;
//         consume_OUTPUT_NEWLINE(in);
//         v.push_back(elt);
//     }

//     return in;
// }

// template<typename T1, typename T2>
// std::ostream& operator<<(std::ostream& out, const std::map<T1, T2> &m)
// {
//     out << m.size() << "\n";

//     for (auto &it : m)
//     {
//         out << it.first << "\n";
//         out << it.second << "\n";
//     }

//     return out;
// }

// template<typename T1, typename T2>
// std::istream& operator>>(std::istream& in, std::map<T1, T2> &m)
// {
//     m.clear();
//     size_t size;
//     in >> size;
//     consume_newline(in);

//     for i in 0..size
//     {
//         T1 k;
//         T2 v;
//         in >> k;
//         consume_newline(in);
//         in >> v;
//         consume_newline(in);
//         m[k] = v;
//     }

//     return in;
// }

// template<typename T>
// std::ostream& operator<<(std::ostream& out, const std::set<T> &s)
// {
//     out << s.size() << "\n";

//     for (auto &el : s)
//     {
//         out << el << "\n";
//     }

//     return out;
// }

// template<typename T>
// std::istream& operator>>(std::istream& in, std::set<T> &s)
// {
//     s.clear();
//     size_t size;
//     in >> size;
//     consume_newline(in);

//     for i in 0..size
//     {
//         T el;
//         in >> el;
//         consume_newline(in);
//         s.insert(el);
//     }

//     return in;
// }
