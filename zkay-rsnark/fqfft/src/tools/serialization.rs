/** @file
*****************************************************************************

Declaration of serialization routines and constants.

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
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

//namespace libfqfft {

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
cfg_if::cfg_if! {
 if #[cfg(feature="BINARY_OUTPUT")]
{const OUTPUT_NEWLINE:&str= "";
const OUTPUT_SEPARATOR:&str= "";}
else
{const OUTPUT_NEWLINE:&str= "\n";
const OUTPUT_SEPARATOR:&str= " ";
}
}

// inline pub fn  consume_newline(std::istream &in);
// inline pub fn  consume_OUTPUT_NEWLINE(std::istream &in);
// inline pub fn  consume_OUTPUT_SEPARATOR(std::istream &in);

// inline pub fn  output_bool(std::ostream &out, b:bool);
// inline pub fn  input_bool(std::istream &in, bool &b);

// inline pub fn  output_bool_vector(std::ostream &out, v:&Vec<bool>);
// inline pub fn  input_bool_vector(std::istream &in, Vec<bool> &v);

//
// T reserialize(obj:&T);

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<T>);

//
// std::istream& operator>>(std::ostream& out, Vec<T> &v);

//
// std::ostream& operator<<(std::ostream& out, m:&BTreeMap<T1, T2>);

//
// std::istream& operator>>(std::istream& in, BTreeMap<T1, T2> &m);

//
// std::ostream& operator<<(std::ostream& out, s:&BTreeSet<T>);

//
// std::istream& operator>>(std::istream& in, BTreeSet<T> &s);

// //} // libfqfft

// // #include "common/serialization.tcc"
// use crate::tools::serialization.tcc;

//#endif // SERIALIZATION_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of serialization routines.

//  See serialization.hpp .

//  *****************************************************************************
//  * @author     This file is part of libfqfft, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
//#ifndef SERIALIZATION_TCC_
// #define SERIALIZATION_TCC_

//#include <cassert>
//#include <sstream>

//namespace libfqfft {

// inline pub fn  consume_newline(std::istream &in)
// {
//     char c;
//     in.read(&c, 1);
// }

// inline pub fn  consume_OUTPUT_NEWLINE(std::istream &in)
// {
// // #ifdef BINARY_OUTPUT
//     // nothing to consume
// #else
//     char c;
//     in.read(&c, 1);
// //#endif
// }

// inline pub fn  consume_OUTPUT_SEPARATOR(std::istream &in)
// {
// // #ifdef BINARY_OUTPUT
//     // nothing to consume
// #else
//     char c;
//     in.read(&c, 1);
// //#endif
// }

// inline pub fn  output_bool(std::ostream &out, b:bool)
// {
//     out << if b {1} else{0} << "\n";
// }

// inline pub fn  input_bool(std::istream &in, bool &b)
// {
//     usize tmp;
//     in >> tmp;
//     consume_newline(in);
//     assert!(tmp == 0 || tmp == 1);

//     b = if tmp == 1 {true} else{false};
// }

// inline pub fn  output_bool_vector(std::ostream &out, v:&Vec<bool>)
// {
//     out << v.len() << "\n";
//     for b in &v
//     {
//         output_bool(out, b);
//     }
// }

// inline pub fn  input_bool_vector(std::istream &in, Vec<bool> &v)
// {
//     usize size;
//     in >> size;
//     consume_newline(in);
//     v.resize(size);
//     for i in 0..size
//     {
//         bool b;
//         input_bool(in, b);
//         v[i] = b;
//     }
// }

//
// T reserialize(obj:&T)
// {
//     std::stringstream ss;
//     ss << obj;
//     T tmp;
//     ss >> tmp;
//     assert!(obj == tmp);
//     return tmp;
// }

//
// std::ostream& operator<<(std::ostream& out, v:&Vec<T>)
// {
//     assert!(!std::is_same<T, bool>::value, "this does not work for Vec<bool>");
//     out << v.len() << "\n";
//     for t in &v
//     {
//         out << t << OUTPUT_NEWLINE;
//     }

//     return out;
// }

//
// std::istream& operator>>(std::istream& in, Vec<T> &v)
// {
//     assert!(!std::is_same<T, bool>::value, "this does not work for Vec<bool>");
//     usize size;
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

//
// std::ostream& operator<<(std::ostream& out, m:&BTreeMap<T1, T2>)
// {
//     out << m.len() << "\n";

//     for it in &m
//     {
//         out << it.first << "\n";
//         out << it.second << "\n";
//     }

//     return out;
// }

//
// std::istream& operator>>(std::istream& in, BTreeMap<T1, T2> &m)
// {
//     m.clear();
//     usize size;
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

//
// std::ostream& operator<<(std::ostream& out, s:&BTreeSet<T>)
// {
//     out << s.len() << "\n";

//     for el in &s
//     {
//         out << el << "\n";
//     }

//     return out;
// }

//
// std::istream& operator>>(std::istream& in, BTreeSet<T> &s)
// {
//     s.clear();
//     usize size;
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

// }

//#endif // SERIALIZATION_TCC_
