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

#[derive(Default, Clone)]
pub struct fooram_architecture_params {
    pub w: usize,
   
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
        //fooram memory contents do not depend on program/input.
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
