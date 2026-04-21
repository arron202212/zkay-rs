#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// Declaration of serialization routines and constants.
use std::io::{self, Read};

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

cfg_if::cfg_if! {
 if #[cfg(feature="BINARY_OUTPUT")]
{
pub const OUTPUT_NEWLINE:&str= "";
pub const OUTPUT_SEPARATOR:&str= "";}
else
{
pub const OUTPUT_NEWLINE:&str= "\n";
pub const OUTPUT_SEPARATOR:&str= " ";
}
}

use crate::common::utils;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

// using std::usize;

#[inline]
pub fn consume_newline<R: Read>(ins: &mut R) -> io::Result<()> {
    // let mut c;
    // ins.read(&c, 1);
    Ok(())
}

#[inline]
pub fn consume_output_newline<R: Read>(ins: &mut R) -> io::Result<()> {
    // #ifdef BINARY_OUTPUT
    // nothing to consume
    // UNUSED(in);
    // #else
    // let mut c;
    // in.read(&c, 1);
    Ok(())
}

#[inline]
pub fn consume_output_separator<R: Read>(ins: &mut R) -> io::Result<()> {
    // #ifdef BINARY_OUTPUT
    // nothing to consume
    //     UNUSED(in);
    // #else
    // char c;
    // ins.read(&c, 1);
    Ok(())
}
#[inline]
pub fn read_line_as_usize<R: Read>(ins: &mut R) -> io::Result<usize> {
    // #ifdef BINARY_OUTPUT
    // nothing to consume
    //     UNUSED(in);
    // #else
    // char c;
    // ins.read(&c, 1);
    Ok(0)
}
#[inline]
pub fn output_bool<W: ?Sized + Write>(out: &mut BufWriter<W>, b: bool) {
    write!(out, "{}\n", b as u8);
    // out << (if b { 1} else {0}) << "\n";
}

#[inline]
pub fn input_bool(ins: &impl BufRead, b: bool) {
    // usize tmp;
    // in >> tmp;
    // consume_newline(in);
    // assert!(tmp == 0 || tmp == 1);

    // b = (tmp == 1) ;
}

#[inline]
pub fn output_bool_vector<W: ?Sized + Write>(out: &BufWriter<W>, v: &Vec<bool>) {
    // out << v.len() << "\n";
    // for bool in &v
    // {
    //     output_bool(out, b);
    // }
}

#[inline]
pub fn input_bool_vector(ins: &impl BufRead, v: &Vec<bool>) {
    // usize size;
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

pub fn reserialize<T: Clone>(obj: &T) -> T {
    // std::stringstream ss;
    // ss << obj;
    // T tmp;
    // ss >> tmp;
    // return tmp;
    obj.clone()
}
