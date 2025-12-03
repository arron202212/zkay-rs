/** @file
*****************************************************************************

Declaration of auxiliary functions for TinyRAM.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TINYRAM_AUX_HPP_
// #define TINYRAM_AUX_HPP_

// use  <cassert>
// use  <iostream>
// use  <map>
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, InstructionConfig, ProgramConfig,
};

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use crate::relations::ram_computations::memory::memory_interface::memory_contents;
use crate::relations::ram_computations::memory::memory_store_trace::memory_store_trace;
use crate::relations::ram_computations::rams::ram_params;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::collections::BTreeMap;
use strum::Display;
#[derive(Display, Hash, PartialEq, Eq, Debug, Default, Clone, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum tinyram_opcode {
    #[default]
    tinyram_opcode_AND = 0b00000,
    tinyram_opcode_OR = 0b00001,
    tinyram_opcode_XOR = 0b00010,
    tinyram_opcode_NOT = 0b00011,
    tinyram_opcode_ADD = 0b00100,
    tinyram_opcode_SUB = 0b00101,
    tinyram_opcode_MULL = 0b00110,
    tinyram_opcode_UMULH = 0b00111,
    tinyram_opcode_SMULH = 0b01000,
    tinyram_opcode_UDIV = 0b01001,
    tinyram_opcode_UMOD = 0b01010,
    tinyram_opcode_SHL = 0b01011,
    tinyram_opcode_SHR = 0b01100,

    tinyram_opcode_CMPE = 0b01101,
    tinyram_opcode_CMPA = 0b01110,
    tinyram_opcode_CMPAE = 0b01111,
    tinyram_opcode_CMPG = 0b10000,
    tinyram_opcode_CMPGE = 0b10001,

    tinyram_opcode_MOV = 0b10010,
    tinyram_opcode_CMOV = 0b10011,

    tinyram_opcode_JMP = 0b10100,
    tinyram_opcode_CJMP = 0b10101,
    tinyram_opcode_CNJMP = 0b10110,

    tinyram_opcode_10111 = 0b10111,
    tinyram_opcode_11000 = 0b11000,
    tinyram_opcode_11001 = 0b11001,

    tinyram_opcode_STOREB = 0b11010,
    tinyram_opcode_LOADB = 0b11011,
    tinyram_opcode_STOREW = 0b11100,
    tinyram_opcode_LOADW = 0b11101,
    tinyram_opcode_READ = 0b11110,
    tinyram_opcode_ANSWER = 0b11111,
}
impl tinyram_opcode {
    pub fn usize(self) -> usize {
        self as _
    }
}
pub enum tinyram_opcode_args {
    tinyram_opcode_args_des_arg1_arg2 = 1,
    tinyram_opcode_args_des_arg2 = 2,
    tinyram_opcode_args_arg1_arg2 = 3,
    tinyram_opcode_args_arg2 = 4,
    tinyram_opcode_args_none = 5,
    tinyram_opcode_args_arg2_des = 6,
}

/**
 * Instructions that may change a register or the flag.
 * All other instructions leave all registers and the flag intact.
 */
pub const tinyram_opcodes_register: [tinyram_opcode; 23] = [
    tinyram_opcode::tinyram_opcode_AND,
    tinyram_opcode::tinyram_opcode_OR,
    tinyram_opcode::tinyram_opcode_XOR,
    tinyram_opcode::tinyram_opcode_NOT,
    tinyram_opcode::tinyram_opcode_ADD,
    tinyram_opcode::tinyram_opcode_SUB,
    tinyram_opcode::tinyram_opcode_MULL,
    tinyram_opcode::tinyram_opcode_UMULH,
    tinyram_opcode::tinyram_opcode_SMULH,
    tinyram_opcode::tinyram_opcode_UDIV,
    tinyram_opcode::tinyram_opcode_UMOD,
    tinyram_opcode::tinyram_opcode_SHL,
    tinyram_opcode::tinyram_opcode_SHR,
    tinyram_opcode::tinyram_opcode_CMPE,
    tinyram_opcode::tinyram_opcode_CMPA,
    tinyram_opcode::tinyram_opcode_CMPAE,
    tinyram_opcode::tinyram_opcode_CMPG,
    tinyram_opcode::tinyram_opcode_CMPGE,
    tinyram_opcode::tinyram_opcode_MOV,
    tinyram_opcode::tinyram_opcode_CMOV,
    tinyram_opcode::tinyram_opcode_LOADB,
    tinyram_opcode::tinyram_opcode_LOADW,
    tinyram_opcode::tinyram_opcode_READ,
];

/**
 * Instructions that modify the program counter.
 * All other instructions either advance it (+1) or stall (see below).
 */
pub const tinyram_opcodes_control_flow: [tinyram_opcode; 3] = [
    tinyram_opcode::tinyram_opcode_JMP,
    tinyram_opcode::tinyram_opcode_CJMP,
    tinyram_opcode::tinyram_opcode_CNJMP,
];

/**
 * Instructions that make the program counter stall;
 * these are "answer" plus all the undefined opcodes.
 */
pub const tinyram_opcodes_stall: [tinyram_opcode; 4] = [
    tinyram_opcode::tinyram_opcode_10111,
    tinyram_opcode::tinyram_opcode_11000,
    tinyram_opcode::tinyram_opcode_11001,
    tinyram_opcode::tinyram_opcode_ANSWER,
];

pub type reg_count_t = usize; // type for the number of registers
pub type reg_width_t = usize; // type for the width of a register
pub fn opcode_values() -> BTreeMap<String, tinyram_opcode> {
    BTreeMap::new()
}
// extern BTreeMap<tinyram_opcode, String> tinyram_opcode_names;

// extern BTreeMap<String, tinyram_opcode> opcode_values;

// extern BTreeMap<tinyram_opcode, tinyram_opcode_args> opcode_args;

// pub fn  ensure_tinyram_opcode_value_map();

// pub struct tinyram_program;
pub type tinyram_input_tape = Vec<usize>;
// pub type tinyram_input_tape_iterator =  <Vec<usize> as Example>::into_iterator;
#[derive(Default, Clone)]
pub struct tinyram_architecture_params {
    //
    pub w: reg_width_t, /* width of a register */
    pub k: reg_count_t, /* number of registers */
}
//     tinyram_architecture_params() {};
impl tinyram_architecture_params {
    pub fn new(w: reg_width_t, k: reg_count_t) -> Self {
        assert!(w == 1usize << log2(w));
        Self { w, k }
    }
}
//     usize address_size() const;
//     usize value_size() const;
//     usize cpu_state_size() const;
//     usize initial_pc_addr() const;

//     bit_vector initial_cpu_state() const;
//     memory_contents initial_memory_contents(program:&tinyram_program,
//                                             primary_input:&tinyram_input_tape) const;

//     usize opcode_width() const;
//     usize reg_arg_width() const;
//     usize instruction_padding_width() const;
//     usize reg_arg_or_imm_width() const;

//     usize dwaddr_len() const;
//     usize subaddr_len() const;

//     usize bytes_in_word() const;

//     usize instr_size() const;

//     bool operator==(other:&tinyram_architecture_params) const;

//     friend std::ostream& operator<<(std::ostream &out, ap:&tinyram_architecture_params);
//     friend std::istream& operator>>(std::istream &in, tinyram_architecture_params &ap);

//     pub fn  print() const;
// };

/* order everywhere is reversed (i.e. MSB comes first),
corresponding to the order in memory */

pub struct tinyram_instruction {
    //
    pub opcode: tinyram_opcode,
    pub arg2_is_imm: bool,
    pub desidx: usize,
    pub arg1idx: usize,
    pub arg2idx_or_imm: usize,
}
//     tinyram_instruction::new(opcode:&tinyram_opcode,
//                         arg2_is_imm:bool,
//                         desidx:&usize,
//                         arg1idx:&usize,
//                         arg2idx_or_imm:&usize);

//     usize as_dword(ap:&tinyram_architecture_params) const;
// };

// tinyram_instruction random_tinyram_instruction(ap:&tinyram_architecture_params);

// Vec<tinyram_instruction> generate_tinyram_prelude(ap:&tinyram_architecture_params);
// extern tinyram_instruction tinyram_default_instruction;
#[derive(Default)]
pub struct tinyram_program {
    pub instructions: Vec<tinyram_instruction>,
}
impl tinyram_program {
    pub fn size(&self) -> usize {
        return self.instructions.len();
    }
    // pub fn  add_instruction(instr:&tinyram_instruction);
}

// tinyram_program load_preprocessed_program(ap:&tinyram_architecture_params,
//                                           std::istream &preprocessed);

// memory_store_trace tinyram_boot_trace_from_program_and_input(ap:&tinyram_architecture_params,
//                                                              boot_trace_size_bound:usize,
//                                                              program:&tinyram_program,
//                                                              primary_input:&tinyram_input_tape);

// tinyram_input_tape load_tape(std::istream &tape);

//#endif // TINYRAM_AUX_HPP_
/** @file
*****************************************************************************

Implementation of auxiliary functions for TinyRAM.

See tinyram_aux.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// use  <cassert>
// use  <fstream>
// use  <string>
use ffec::common::profiling;
// use ffec::common::utils;

// use crate::relations::ram_computations::rams::tinyram::tinyram_aux;

// tinyram_instruction tinyram_default_instruction = tinyram_instruction::new(tinyram_opcode_ANSWER, true, 0, 0, 1);

pub const tinyram_opcode_names: [(tinyram_opcode, &str); 32] = [
    (tinyram_opcode::tinyram_opcode_AND, "and"),
    (tinyram_opcode::tinyram_opcode_OR, "or"),
    (tinyram_opcode::tinyram_opcode_XOR, "xor"),
    (tinyram_opcode::tinyram_opcode_NOT, "not"),
    (tinyram_opcode::tinyram_opcode_ADD, "add"),
    (tinyram_opcode::tinyram_opcode_SUB, "sub"),
    (tinyram_opcode::tinyram_opcode_MULL, "mull"),
    (tinyram_opcode::tinyram_opcode_UMULH, "umulh"),
    (tinyram_opcode::tinyram_opcode_SMULH, "smulh"),
    (tinyram_opcode::tinyram_opcode_UDIV, "udiv"),
    (tinyram_opcode::tinyram_opcode_UMOD, "umod"),
    (tinyram_opcode::tinyram_opcode_SHL, "shl"),
    (tinyram_opcode::tinyram_opcode_SHR, "shr"),
    (tinyram_opcode::tinyram_opcode_CMPE, "cmpe"),
    (tinyram_opcode::tinyram_opcode_CMPA, "cmpa"),
    (tinyram_opcode::tinyram_opcode_CMPAE, "cmpae"),
    (tinyram_opcode::tinyram_opcode_CMPG, "cmpg"),
    (tinyram_opcode::tinyram_opcode_CMPGE, "cmpge"),
    (tinyram_opcode::tinyram_opcode_MOV, "mov"),
    (tinyram_opcode::tinyram_opcode_CMOV, "cmov"),
    (tinyram_opcode::tinyram_opcode_JMP, "jmp"),
    (tinyram_opcode::tinyram_opcode_CJMP, "cjmp"),
    (tinyram_opcode::tinyram_opcode_CNJMP, "cnjmp"),
    (tinyram_opcode::tinyram_opcode_10111, "opcode_10111"),
    (tinyram_opcode::tinyram_opcode_11000, "opcode_11000"),
    (tinyram_opcode::tinyram_opcode_11001, "opcode_11001"),
    (tinyram_opcode::tinyram_opcode_STOREB, "store.b"),
    (tinyram_opcode::tinyram_opcode_LOADB, "load.b"),
    (tinyram_opcode::tinyram_opcode_STOREW, "store.w"),
    (tinyram_opcode::tinyram_opcode_LOADW, "load.w"),
    (tinyram_opcode::tinyram_opcode_READ, "read"),
    (tinyram_opcode::tinyram_opcode_ANSWER, "answer"),
];
//BTreeMap<tinyram_opcode, tinyram_opcode_args>
const opcode_args: [(tinyram_opcode, tinyram_opcode_args); 32] = [
    (
        tinyram_opcode::tinyram_opcode_AND,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_OR,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_XOR,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_NOT,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_ADD,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_SUB,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_MULL,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_UMULH,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_SMULH,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_UDIV,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_UMOD,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_SHL,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_SHR,
        tinyram_opcode_args::tinyram_opcode_args_des_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMPE,
        tinyram_opcode_args::tinyram_opcode_args_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMPA,
        tinyram_opcode_args::tinyram_opcode_args_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMPAE,
        tinyram_opcode_args::tinyram_opcode_args_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMPG,
        tinyram_opcode_args::tinyram_opcode_args_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMPGE,
        tinyram_opcode_args::tinyram_opcode_args_arg1_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_MOV,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CMOV,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_JMP,
        tinyram_opcode_args::tinyram_opcode_args_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CJMP,
        tinyram_opcode_args::tinyram_opcode_args_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_CNJMP,
        tinyram_opcode_args::tinyram_opcode_args_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_10111,
        tinyram_opcode_args::tinyram_opcode_args_none,
    ),
    (
        tinyram_opcode::tinyram_opcode_11000,
        tinyram_opcode_args::tinyram_opcode_args_none,
    ),
    (
        tinyram_opcode::tinyram_opcode_11001,
        tinyram_opcode_args::tinyram_opcode_args_none,
    ),
    (
        tinyram_opcode::tinyram_opcode_STOREB,
        tinyram_opcode_args::tinyram_opcode_args_arg2_des,
    ),
    (
        tinyram_opcode::tinyram_opcode_LOADB,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_STOREW,
        tinyram_opcode_args::tinyram_opcode_args_arg2_des,
    ),
    (
        tinyram_opcode::tinyram_opcode_LOADW,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_READ,
        tinyram_opcode_args::tinyram_opcode_args_des_arg2,
    ),
    (
        tinyram_opcode::tinyram_opcode_ANSWER,
        tinyram_opcode_args::tinyram_opcode_args_arg2,
    ),
];

//

use std::sync::OnceLock;
static opcode_values_s: OnceLock<BTreeMap<String, tinyram_opcode>> = OnceLock::new();

pub fn ensure_tinyram_opcode_value_map() -> &'static BTreeMap<String, tinyram_opcode> {
    opcode_values_s.get_or_init(|| {
        tinyram_opcode_names
            .iter()
            .map(|(k, v)| (v.to_string(), k.clone()))
            .collect::<BTreeMap<String, tinyram_opcode>>()
    })
}

pub fn generate_tinyram_prelude<APT: ArchitectureParamsTypeConfig>(
    ap: &APT,
) -> Vec<tinyram_instruction> {
    let mut result = vec![];
    let increment = log2(ap.w()) / 8;
    let mem_start = 1usize << (ap.w() - 1);
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_STOREW,
        true,
        0,
        0,
        0,
    )); // 0: store.w 0, r0
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_MOV,
        true,
        0,
        0,
        mem_start,
    )); // 1: mov r0, 2^{W-1}
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_READ,
        true,
        1,
        0,
        0,
    )); // 2: read r1, 0
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_CJMP,
        true,
        0,
        0,
        7,
    )); // 3: cjmp 7
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_ADD,
        true,
        0,
        0,
        increment,
    )); // 4: add r0, r0, INCREMENT
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_STOREW,
        false,
        1,
        0,
        0,
    )); // 5: store.w r0, r1
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_JMP,
        true,
        0,
        0,
        2,
    )); // 6: jmp 2
    result.push(tinyram_instruction::new(
        tinyram_opcode::tinyram_opcode_STOREW,
        true,
        0,
        0,
        mem_start,
    )); // 7: store.w 2^{W-1}, r0
    return result;
}
impl ArchitectureParamsTypeConfig for tinyram_architecture_params {
    fn address_size(&self) -> usize {
        return self.dwaddr_len();
    }

    fn value_size(&self) -> usize {
        return 2 * self.w;
    }

    fn cpu_state_size(&self) -> usize {
        return self.k * self.w + 2; /* + flag + tape1_exhausted */
    }

    fn initial_pc_addr(&self) -> usize {
        /* the initial PC address is memory units for the RAM reduction */
        let initial_pc_addr = generate_tinyram_prelude(self).len();
        return initial_pc_addr;
    }

    fn initial_cpu_state(&self) -> bit_vector {
        let result = vec![false; self.cpu_state_size()];
        return result;
    }

    fn initial_memory_contents<IC: InstructionConfig, PC: ProgramConfig<IC>>(
        &self,
        program: &PC,
        primary_input: &tinyram_input_tape,
    ) -> memory_contents {
        // remember that memory consists of 1usize<<dwaddr_len() double words (!)
        let mut m = memory_contents::new();

        for i in 0..program.instructions().len() {
            m.insert(i, program.instructions()[i].as_dword(self));
        }

        let input_addr = 1usize << (self.dwaddr_len() - 1);
        let mut latest_double_word = (1usize << (self.w - 1)) + primary_input.len(); // the first word will contain 2^{w-1} + input_size (the location where the last input word was stored)

        for i in 0..primary_input.len() / 2 + 1 {
            if 2 * i < primary_input.len() {
                latest_double_word += (primary_input[2 * i] << self.w);
            }

            m.insert(input_addr + i, latest_double_word);

            if 2 * i + 1 < primary_input.len() {
                latest_double_word = primary_input[2 * i + 1];
            }
        }

        return m;
    }

    fn opcode_width(&self) -> usize {
        return log2(tinyram_opcode::tinyram_opcode_ANSWER as usize); /* assumption: answer is the last */
    }

    fn reg_arg_width(&self) -> usize {
        return log2(self.k);
    }

    fn instruction_padding_width(&self) -> usize {
        return 2 * self.w
            - (self.opcode_width() + 1 + 2 * self.reg_arg_width() + self.reg_arg_or_imm_width());
    }

    fn reg_arg_or_imm_width(&self) -> usize {
        return std::cmp::max(self.w, self.reg_arg_width());
    }

    fn dwaddr_len(&self) -> usize {
        return self.w - (log2(self.w) - 2);
    }

    fn subaddr_len(&self) -> usize {
        return log2(self.w) - 2;
    }

    fn bytes_in_word(&self) -> usize {
        return self.w / 8;
    }

    fn instr_size(&self) -> usize {
        return 2 * self.w;
    }
    fn print(&self) {
        print!("* Number of registers (k): {}\n", self.k);
        print!("* Word size (w): {}\n", self.w);
    }
}

// bool tinyram_architecture_params::operator==(other:&tinyram_architecture_params) const
// {
//     return (self.w == other.w &&
//             self.k == other.k);
// }

// std::ostream& operator<<(std::ostream &out, ap:&tinyram_architecture_params)
// {
//     out << ap.w() << "\n";
//     out << ap.k() << "\n";
//     return out;
// }

// std::istream& operator>>(std::istream &in, tinyram_architecture_params &ap)
// {
//     in >> ap.w();
//     consume_newline(in);
//     in >> ap.k();
//     consume_newline(in);
//     return in;
// }

impl tinyram_instruction {
    pub fn new(
        opcode: tinyram_opcode,
        arg2_is_imm: bool,
        desidx: usize,
        arg1idx: usize,
        arg2idx_or_imm: usize,
    ) -> Self {
        Self {
            opcode,
            arg2_is_imm,
            desidx,
            arg1idx,
            arg2idx_or_imm,
        }
    }

    pub fn as_dword<APT: ArchitectureParamsTypeConfig>(&self, ap: &APT) -> usize {
        let mut result = self.opcode.clone() as usize;
        result = (result << 1) | (if self.arg2_is_imm { 1 } else { 0 });
        result = (result << log2(ap.k())) | self.desidx;
        result = (result << log2(ap.k())) | self.arg1idx;
        result = (result << (2 * ap.w() - ap.opcode_width() - 1 - 2 * log2(ap.k())))
            | self.arg2idx_or_imm;

        return result;
    }
}

pub fn random_tinyram_instruction<APT: ArchitectureParamsTypeConfig>(
    ap: &APT,
) -> tinyram_instruction {
    // use rand::Rng;
    // let mut rng = rand::thread_rng();
    let opcode = (rand::random::<usize>() % (1usize << ap.opcode_width())); //(tinyram_opcode)
    let arg2_is_imm = (rand::random::<usize>() & 1) != 0;
    let desidx = rand::random::<usize>() % (1usize << ap.reg_arg_width());
    let arg1idx = rand::random::<usize>() % (1usize << ap.reg_arg_width());
    let arg2idx_or_imm = rand::random::<usize>() % (1usize << ap.reg_arg_or_imm_width());
    return tinyram_instruction::new(
        tinyram_opcode::from(opcode as u8),
        arg2_is_imm,
        desidx,
        arg1idx,
        arg2idx_or_imm,
    );
}
impl tinyram_program {
    pub fn add_instruction(&mut self, instr: tinyram_instruction) {
        self.instructions.push(instr);
    }
}

pub fn load_preprocessed_program(
    ap: &tinyram_architecture_params,
    preprocessed: &String,
) -> tinyram_program {
    let opcode_values = ensure_tinyram_opcode_value_map();

    let mut program = tinyram_program::default();

    enter_block("Loading program", false);
    let (mut instr, mut line) = (String::new(), String::new());
    let mut it = preprocessed.split_ascii_whitespace();
    while let Some(instr) = it.next() {
        print_indent();
        let immflag = it.next().unwrap().parse::<bool>().unwrap();
        let des = it.next().unwrap().parse::<usize>().unwrap();
        let a1 = it.next().unwrap().parse::<usize>().unwrap();
        let mut a2 = it.next().unwrap().parse::<usize>().unwrap();
        // preprocessed >> immflag >> des >> a1 >> a2;
        a2 = ((1usize << ap.w()) + (a2 % (1usize << ap.w()))) % (1usize << ap.w());
        program.add_instruction(tinyram_instruction::new(
            opcode_values[instr].clone(),
            immflag,
            des,
            a1,
            a2,
        ));
    }
    leave_block("Loading program", false);

    return program;
}

pub fn tinyram_boot_trace_from_program_and_input(
    ap: &tinyram_architecture_params,
    boot_trace_size_bound: usize,
    program: &tinyram_program,
    primary_input: &tinyram_input_tape,
) -> memory_store_trace {
    // TODO: document the reverse order here

    let mut result = memory_store_trace::new();

    let mut boot_pos = boot_trace_size_bound - 1;
    for i in 0..program.instructions.len() {
        result.set_trace_entry(boot_pos, (i, program.instructions[i].as_dword(ap)));
        boot_pos -= 1;
    }

    let primary_input_base_addr = (1usize << (ap.dwaddr_len() - 1));

    for j in (0..primary_input.len()).step_by(2) {
        let memory_dword = primary_input[j]
            + ((if j + 1 < primary_input.len() {
                primary_input[j + 1]
            } else {
                0
            }) << ap.w());
        result.set_trace_entry(boot_pos, (primary_input_base_addr + j, memory_dword));
        boot_pos -= 1;
    }

    return result;
}

pub fn load_tape(tape: &String) -> tinyram_input_tape {
    enter_block("Loading tape", false);
    let mut result = tinyram_input_tape::new();

    print_indent();
    print!("Tape contents:");
    let mut tape = tape.split_ascii_whitespace();
    while let Some(cell) = tape.next() {
        print!("\t{}", cell);
        result.push(cell.parse::<usize>().unwrap());
    }
    print!("\n");

    leave_block("Loading tape", false);
    return result;
}
