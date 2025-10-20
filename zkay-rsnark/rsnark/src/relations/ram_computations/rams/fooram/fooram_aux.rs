/** @file
 *****************************************************************************

 Declaration of auxiliary functions for FOORAM.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_AUX_HPP_
// #define FOORAM_AUX_HPP_

// use  <iostream>
// use  <vector>

use ffec::common::utils;

use crate::relations::ram_computations::memory::memory_interface;



type fooram_program=std::vector<usize> ;
type fooram_input_tape=std::vector<usize> ;
type fooram_input_tape_iterator = std::vector<usize>::const_iterator ;

pub struct  fooram_architecture_params {
// public:
     w:usize,

    // fooram_architecture_params(const usize w=16);

    // usize num_addresses() const;
    // usize address_size() const;
    // usize value_size() const;
    // usize cpu_state_size() const;
    // usize initial_pc_addr() const;

    // memory_contents initial_memory_contents(program:&fooram_program,
    //                                         primary_input:&fooram_input_tape) const;

    // ffec::bit_vector initial_cpu_state() const;
    // void print() const;
    // bool operator==(other:&fooram_architecture_params) const;

    // friend std::ostream& operator<<(std::ostream &out, ap:&fooram_architecture_params);
    // friend std::istream& operator>>(std::istream &in, fooram_architecture_params &ap);
}



//#endif // FOORAM_AUX_HPP_
/** @file
 *****************************************************************************

 Implementation of auxiliary functions for fooram.

 See fooram_aux.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use ffec::common::serialization;

// use crate::relations::ram_computations::rams::fooram::fooram_aux;


impl fooram_architecture_params{
pub fn new( w:usize)->Self
{
    Self{w}
}

pub fn num_addresses(&self)->usize
{
    return 1usize<<w;
}

pub fn address_size(&self)->usize
{
    return w;
}

pub fn value_size(&self)->usize
{
    return w;
}

pub fn cpu_state_size(&self)->usize
{
    return w;
}

pub fn initial_pc_addr(&self)->usize
{
    return 0;
}

pub fn initial_memory_contents(program:&fooram_program,
                                                                    primary_input:&fooram_input_tape) ->memory_contents
{
    let  m=memory_contents::new();
    /* fooram memory contents do not depend on program/input. */
    // ffec::UNUSED(program, primary_input);
    return m;
}

pub fn initial_cpu_state(&self)->bit_vector
{
    let  state=vec![false;w];
    // state.resize(w, false);
    return state;
}

pub fn print(&self)
{
    print!("w = {}\n", w);
}
}

// bool fooram_architecture_params::operator==(other:&fooram_architecture_params) const
// {
//     return (self.w == other.w);
// }

// std::ostream& operator<<(std::ostream &out, ap:&fooram_architecture_params)
// {
//     out << ap.w << "\n";
//     return out;
// }

// std::istream& operator>>(std::istream &in, fooram_architecture_params &ap)
// {
//     in >> ap.w;
//     ffec::consume_newline(in);
//     return in;
// }



