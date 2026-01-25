// Declaration of auxiliary functions for FOORAM.

use crate::relations::ram_computations::memory::memory_interface::memory_contents;
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, InstructionConfig, ProgramConfig,
};

use ffec::common::serialization;
use ffec::common::utils;
use ffec::common::utils::bit_vector;
pub type fooram_program = Vec<usize>;
pub type fooram_input_tape = Vec<usize>;
// pub type fooram_input_tape_iterator = <Vec<usize> as Example>::into_iterator;

#[derive(Default, Clone)]
pub struct fooram_architecture_params {
    pub w: usize,
    // fooram_architecture_params(w:usize=16);

    // usize num_addresses() const;
    // usize address_size() const;
    // usize value_size() const;
    // usize cpu_state_size() const;
    // usize initial_pc_addr() const;

    // memory_contents initial_memory_contents(program:&fooram_program,
    //                                         primary_input:&fooram_input_tape) const;

    // bit_vector initial_cpu_state() const;
    // pub fn  print() const;
    // bool operator==(other:&fooram_architecture_params) const;

    // friend std::ostream& operator<<(std::ostream &out, ap:&fooram_architecture_params);
    // friend std::istream& operator>>(std::istream &in, fooram_architecture_params &ap);
}

impl fooram_architecture_params {
    pub fn new(w: usize) -> Self {
        Self { w }
    }
}
impl ArchitectureParamsTypeConfig for fooram_architecture_params {
    fn num_addresses(&self) -> usize {
        1usize << self.w
    }

    fn address_size(&self) -> usize {
        self.w
    }

    fn value_size(&self) -> usize {
        self.w
    }

    fn cpu_state_size(&self) -> usize {
        self.w
    }

    fn initial_pc_addr(&self) -> usize {
        0
    }

    fn initial_memory_contents<IC: InstructionConfig, PC: ProgramConfig<IC>>(
        &self,
        program: &PC,
        primary_input: &fooram_input_tape,
    ) -> memory_contents {
        let m = memory_contents::new();
        /* fooram memory contents do not depend on program/input. */
        // //ffec::UNUSED(program, primary_input);
        m
    }

    fn initial_cpu_state(&self) -> bit_vector {
        let state = vec![false; self.w];
        // state.resize(w, false);
        state
    }

    fn print(&self) {
        print!("w = {}\n", self.w);
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
