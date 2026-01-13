// Declaration of public-parameter selector for RAMs.

use crate::relations::ram_computations::memory::memory_interface::memory_contents;
use crate::relations::ram_computations::memory::memory_store_trace::memory_store_trace;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    tinyram_input_tape, tinyram_program,
};
use ffec::common::utils;
use ffec::common::utils::bit_vector;
pub trait InstructionConfig {
    fn as_dword<APT: ArchitectureParamsTypeConfig>(&self, ap: &APT) -> usize;
}
pub trait ProgramConfig<IC: InstructionConfig> {
    fn instructions(&self) -> &Vec<IC>;
}
/*
  When declaring a new ramT one should do a make it a pub struct that declares typedefs for:

  base_field_type
  ram_cpu_checker_type
  architecture_params_type

  For ram_to_r1cs reduction currently the following are also necessary:
  protoboard_type (e.g. tinyram_protoboard<FieldT>)
  gadget_base_type (e.g. tinyram_gadget<FieldT>)
  cpu_state_variable_type (must have pb_variable_array<FieldT> all_vars)

  The ramT pub struct must also have a static usize variable
  timestamp_length, which specifies the zk-SNARK reduction timestamp
  length.
*/
pub trait ArchitectureParamsTypeConfig: Default + Clone {
    fn w(&self) -> usize {
        0
    }
    fn k(&self) -> usize {
        0
    }
    fn num_addresses(&self) -> usize {
        0
    }
    fn address_size(&self) -> usize {
        0
    }

    fn value_size(&self) -> usize {
        0
    }

    fn cpu_state_size(&self) -> usize {
        0
    }

    fn initial_pc_addr(&self) -> usize {
        0
    }

    fn initial_cpu_state(&self) -> bit_vector {
        let result = vec![];
        return result;
    }

    fn initial_memory_contents<IC: InstructionConfig, PC: ProgramConfig<IC>>(
        &self,
        program: &PC,
        primary_input: &tinyram_input_tape,
    ) -> memory_contents {
        // remember that memory consists of 1u64<<dwaddr_len() double words (!)
        let mut m = memory_contents::new();

        return m;
    }

    fn opcode_width(&self) -> usize {
        0
    }

    fn reg_arg_width(&self) -> usize {
        0
    }

    fn instruction_padding_width(&self) -> usize {
        0
    }

    fn reg_arg_or_imm_width(&self) -> usize {
        0
    }

    fn dwaddr_len(&self) -> usize {
        0
    }

    fn subaddr_len(&self) -> usize {
        0
    }

    fn bytes_in_word(&self) -> usize {
        0
    }

    fn instr_size(&self) -> usize {
        0
    }
    fn print(&self) {}
}
pub trait ram_params_type: Default {
    const timestamp_length: usize;
    type base_field_type;
    type protoboard_type;
    type gadget_base_type;
    type cpu_checker_type;
    type architecture_params_type: ArchitectureParamsTypeConfig;
}
pub type ram_base_field<ramT> = <ramT as ram_params_type>::base_field_type;

pub type ram_cpu_state = bit_vector;

pub type ram_boot_trace = memory_store_trace;

pub type ram_protoboard<ramT> = <ramT as ram_params_type>::protoboard_type;

pub type ram_gadget_base<ramT> = <ramT as ram_params_type>::gadget_base_type;

pub type ram_cpu_checker<ramT> = <ramT as ram_params_type>::cpu_checker_type;

pub type ram_architecture_params<ramT> = <ramT as ram_params_type>::architecture_params_type;

pub type ram_input_tape = Vec<usize>;

/*
  One should also make the following methods for ram_architecture_params

  (We are not yet making a ram_architecture_params base class, as it
  would require base pub struct for ram_program

  TODO: make this base class)

  usize address_size();
  usize value_size();
  usize cpu_state_size();
  usize initial_pc_addr();
  bit_vector initial_cpu_state();
*/
