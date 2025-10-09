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

use  <iostream>
use  <vector>

use ffec::common::utils;

use libsnark/relations/ram_computations/memory/memory_interface;



type std::vector<size_t> fooram_program;
type std::vector<size_t> fooram_input_tape;
type typename std::vector<size_t>::const_iterator fooram_input_tape_iterator;

class fooram_architecture_params {
public:
    size_t w;
    fooram_architecture_params(const size_t w=16);

    size_t num_addresses() const;
    size_t address_size() const;
    size_t value_size() const;
    size_t cpu_state_size() const;
    size_t initial_pc_addr() const;

    memory_contents initial_memory_contents(const fooram_program &program,
                                            const fooram_input_tape &primary_input) const;

    ffec::bit_vector initial_cpu_state() const;
    void print() const;
    bool operator==(const fooram_architecture_params &other) const;

    friend std::ostream& operator<<(std::ostream &out, const fooram_architecture_params &ap);
    friend std::istream& operator>>(std::istream &in, fooram_architecture_params &ap);
};



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

use libsnark/relations/ram_computations/rams/fooram/fooram_aux;



fooram_architecture_params::fooram_architecture_params(const size_t w) : w(w)
{
}

size_t fooram_architecture_params::num_addresses() const
{
    return 1ul<<w;
}

size_t fooram_architecture_params::address_size() const
{
    return w;
}

size_t fooram_architecture_params::value_size() const
{
    return w;
}

size_t fooram_architecture_params::cpu_state_size() const
{
    return w;
}

size_t fooram_architecture_params::initial_pc_addr() const
{
    return 0;
}

memory_contents fooram_architecture_params::initial_memory_contents(const fooram_program &program,
                                                                    const fooram_input_tape &primary_input) const
{
    memory_contents m;
    /* fooram memory contents do not depend on program/input. */
    ffec::UNUSED(program, primary_input);
    return m;
}

ffec::bit_vector fooram_architecture_params::initial_cpu_state() const
{
    ffec::bit_vector state;
    state.resize(w, false);
    return state;
}

void fooram_architecture_params::print() const
{
    print!("w = {}\n", w);
}

bool fooram_architecture_params::operator==(const fooram_architecture_params &other) const
{
    return (self.w == other.w);
}

std::ostream& operator<<(std::ostream &out, const fooram_architecture_params &ap)
{
    out << ap.w << "\n";
    return out;
}

std::istream& operator>>(std::istream &in, fooram_architecture_params &ap)
{
    in >> ap.w;
    ffec::consume_newline(in);
    return in;
}



