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

use ffec::common::utils;

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use crate::relations::ram_computations::memory::memory_interface;
use crate::relations::ram_computations::rams::ram_params;



enum tinyram_opcode {
    tinyram_opcode_AND    = 0b00000,
    tinyram_opcode_OR     = 0b00001,
    tinyram_opcode_XOR    = 0b00010,
    tinyram_opcode_NOT    = 0b00011,
    tinyram_opcode_ADD    = 0b00100,
    tinyram_opcode_SUB    = 0b00101,
    tinyram_opcode_MULL   = 0b00110,
    tinyram_opcode_UMULH  = 0b00111,
    tinyram_opcode_SMULH  = 0b01000,
    tinyram_opcode_UDIV   = 0b01001,
    tinyram_opcode_UMOD   = 0b01010,
    tinyram_opcode_SHL    = 0b01011,
    tinyram_opcode_SHR    = 0b01100,

    tinyram_opcode_CMPE   = 0b01101,
    tinyram_opcode_CMPA   = 0b01110,
    tinyram_opcode_CMPAE  = 0b01111,
    tinyram_opcode_CMPG   = 0b10000,
    tinyram_opcode_CMPGE  = 0b10001,

    tinyram_opcode_MOV    = 0b10010,
    tinyram_opcode_CMOV   = 0b10011,

    tinyram_opcode_JMP    = 0b10100,
    tinyram_opcode_CJMP   = 0b10101,
    tinyram_opcode_CNJMP  = 0b10110,

    tinyram_opcode_10111  = 0b10111,
    tinyram_opcode_11000  = 0b11000,
    tinyram_opcode_11001  = 0b11001,

    tinyram_opcode_STOREB = 0b11010,
    tinyram_opcode_LOADB  = 0b11011,
    tinyram_opcode_STOREW = 0b11100,
    tinyram_opcode_LOADW  = 0b11101,
    tinyram_opcode_READ   = 0b11110,
    tinyram_opcode_ANSWER = 0b11111
}

enum tinyram_opcode_args {
    tinyram_opcode_args_des_arg1_arg2 = 1,
    tinyram_opcode_args_des_arg2 = 2,
    tinyram_opcode_args_arg1_arg2 = 3,
    tinyram_opcode_args_arg2 = 4,
    tinyram_opcode_args_none = 5,
    tinyram_opcode_args_arg2_des = 6
}

/**
 * Instructions that may change a register or the flag.
 * All other instructions leave all registers and the flag intact.
 */
const tinyram_opcodes_register=[i32;23] = [
    tinyram_opcode_AND,
    tinyram_opcode_OR,
    tinyram_opcode_XOR,
    tinyram_opcode_NOT,
    tinyram_opcode_ADD,
    tinyram_opcode_SUB,
    tinyram_opcode_MULL,
    tinyram_opcode_UMULH,
    tinyram_opcode_SMULH,
    tinyram_opcode_UDIV,
    tinyram_opcode_UMOD,
    tinyram_opcode_SHL,
    tinyram_opcode_SHR,

    tinyram_opcode_CMPE,
    tinyram_opcode_CMPA,
    tinyram_opcode_CMPAE,
    tinyram_opcode_CMPG,
    tinyram_opcode_CMPGE,

    tinyram_opcode_MOV,
    tinyram_opcode_CMOV,

    tinyram_opcode_LOADB,
    tinyram_opcode_LOADW,
    tinyram_opcode_READ
];

/**
 * Instructions that modify the program counter.
 * All other instructions either advance it (+1) or stall (see below).
 */
const  tinyram_opcodes_control_flow:[i32;3] = [
    tinyram_opcode_JMP,
    tinyram_opcode_CJMP,
    tinyram_opcode_CNJMP
];

/**
 * Instructions that make the program counter stall;
 * these are "answer" plus all the undefined opcodes.
 */
const  tinyram_opcodes_stall:[i32;4] = [
    tinyram_opcode_10111,
    tinyram_opcode_11000,
    tinyram_opcode_11001,

    tinyram_opcode_ANSWER
];

type reg_count_t=usize ; // type for the number of registers
type reg_width_t=usize ; // type for the width of a register

// extern std::map<tinyram_opcode, std::string> tinyram_opcode_names;

// extern std::map<std::string, tinyram_opcode> opcode_values;

// extern std::map<tinyram_opcode, tinyram_opcode_args> opcode_args;

// void ensure_tinyram_opcode_value_map();

// pub struct tinyram_program;
type tinyram_input_tape=std::vector<usize> ;
type tinyram_input_tape_iterator= tinyram_input_tape::const_iterator ;

pub struct tinyram_architecture_params {
// 
     w:reg_width_t, /* width of a register */
     k:reg_count_t, /* number of registers */
}
//     tinyram_architecture_params() {};
//     tinyram_architecture_params(const reg_width_t w, const reg_count_t k) : w(w), k(k) { assert!(w == 1u64 << log2(w)); };

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

//     void print() const;
// };

/* order everywhere is reversed (i.e. MSB comes first),
   corresponding to the order in memory */

pub struct tinyram_instruction {
// 
    opcode:tinyram_opcode,
    arg2_is_imm:bool,
    desidx:usize,
    arg1idx:usize,
    arg2idx_or_imm:usize,
}
//     tinyram_instruction(opcode:&tinyram_opcode,
//                         const bool arg2_is_imm,
//                         desidx:&usize,
//                         arg1idx:&usize,
//                         arg2idx_or_imm:&usize);

//     usize as_dword(ap:&tinyram_architecture_params) const;
// };

// tinyram_instruction random_tinyram_instruction(ap:&tinyram_architecture_params);

// std::vector<tinyram_instruction> generate_tinyram_prelude(ap:&tinyram_architecture_params);
// extern tinyram_instruction tinyram_default_instruction;

pub struct tinyram_program {
// 
instructions:    std::vector<tinyram_instruction>,
}
impl tinyram_program {
    pub fn  size(&self) ->usize { return instructions.len(); }
    // void add_instruction(instr:&tinyram_instruction);
}

// tinyram_program load_preprocessed_program(ap:&tinyram_architecture_params,
//                                           std::istream &preprocessed);

// memory_store_trace tinyram_boot_trace_from_program_and_input(ap:&tinyram_architecture_params,
//                                                              const usize boot_trace_size_bound,
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



// tinyram_instruction tinyram_default_instruction = tinyram_instruction(tinyram_opcode_ANSWER, true, 0, 0, 1);

const tinyram_opcode_names:[(i32,&str);40] =
[
    ( tinyram_opcode_AND,    "and" ),
    ( tinyram_opcode_OR,     "or" ),
    ( tinyram_opcode_XOR,    "xor" ),
    ( tinyram_opcode_NOT,    "not" ),
    ( tinyram_opcode_ADD,    "add" ),
    ( tinyram_opcode_SUB,    "sub" ),
    ( tinyram_opcode_MULL,   "mull" ),
    ( tinyram_opcode_UMULH,  "umulh" ),
    ( tinyram_opcode_SMULH,  "smulh" ),
    ( tinyram_opcode_UDIV,   "udiv" ),
    ( tinyram_opcode_UMOD,   "umod" ),
    ( tinyram_opcode_SHL,    "shl" ),
    ( tinyram_opcode_SHR,    "shr" ),

    ( tinyram_opcode_CMPE,   "cmpe" ),
    ( tinyram_opcode_CMPA,   "cmpa" ),
    ( tinyram_opcode_CMPAE,  "cmpae" ),
    ( tinyram_opcode_CMPG,   "cmpg" ),
    ( tinyram_opcode_CMPGE,  "cmpge" ),

    ( tinyram_opcode_MOV,    "mov" ),
    ( tinyram_opcode_CMOV,   "cmov" ),
    ( tinyram_opcode_JMP,    "jmp" ),

    ( tinyram_opcode_CJMP,   "cjmp" ),
    ( tinyram_opcode_CNJMP,  "cnjmp" ),

    ( tinyram_opcode_10111,  "opcode_10111" ),
    ( tinyram_opcode_11000,  "opcode_11000" ),
    ( tinyram_opcode_11001,  "opcode_11001" ),
    ( tinyram_opcode_STOREB, "store.b" ),
    ( tinyram_opcode_LOADB,  "load.b" ),

    ( tinyram_opcode_STOREW, "store.w" ),
    ( tinyram_opcode_LOADW,  "load.w" ),
    ( tinyram_opcode_READ,   "read" ),
    ( tinyram_opcode_ANSWER, "answer" )
];
//std::map<tinyram_opcode, tinyram_opcode_args> 
const opcode_args:[(i32,i32);35] =
[
    ( tinyram_opcode_AND,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_OR,      tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_XOR,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_NOT,     tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_ADD,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_SUB,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_MULL,    tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_UMULH,   tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_SMULH,   tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_UDIV,    tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_UMOD,    tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_SHL,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_SHR,     tinyram_opcode_args_des_arg1_arg2 ),
    ( tinyram_opcode_CMPE,    tinyram_opcode_args_arg1_arg2 ),
    ( tinyram_opcode_CMPA,    tinyram_opcode_args_arg1_arg2 ),
    ( tinyram_opcode_CMPAE,   tinyram_opcode_args_arg1_arg2 ),
    ( tinyram_opcode_CMPG,    tinyram_opcode_args_arg1_arg2 ),
    ( tinyram_opcode_CMPGE,   tinyram_opcode_args_arg1_arg2 ),
    ( tinyram_opcode_MOV,     tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_CMOV,    tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_JMP,     tinyram_opcode_args_arg2 ),
    ( tinyram_opcode_CJMP,    tinyram_opcode_args_arg2 ),
    ( tinyram_opcode_CNJMP,   tinyram_opcode_args_arg2 ),
    ( tinyram_opcode_10111,   tinyram_opcode_args_none ),
    ( tinyram_opcode_11000,   tinyram_opcode_args_none ),
    ( tinyram_opcode_11001,   tinyram_opcode_args_none ),
    ( tinyram_opcode_STOREB,  tinyram_opcode_args_arg2_des ),
    ( tinyram_opcode_LOADB,   tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_STOREW,  tinyram_opcode_args_arg2_des ),
    ( tinyram_opcode_LOADW,   tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_READ,    tinyram_opcode_args_des_arg2 ),
    ( tinyram_opcode_ANSWER,  tinyram_opcode_args_arg2 )
];

// std::map<std::string, tinyram_opcode> opcode_values;

pub fn  ensure_tinyram_opcode_value_map()
{
    if opcode_values.empty()
    {
        for it in &tinyram_opcode_names
        {
            opcode_values[it.1] = it.0;
        }
    }
}

pub fn generate_tinyram_prelude(ap:&tinyram_architecture_params)->std::vector<tinyram_instruction> 
{
    let mut  result=vec![];
    let  increment = log2(ap.w)/8;
    let mem_start = 1u64<<(ap.w-1);
    result.push(tinyram_instruction(tinyram_opcode_STOREW,  true, 0, 0, 0));         // 0: store.w 0, r0
    result.push(tinyram_instruction(tinyram_opcode_MOV,     true, 0, 0, mem_start)); // 1: mov r0, 2^{W-1}
    result.push(tinyram_instruction(tinyram_opcode_READ,    true, 1, 0, 0));         // 2: read r1, 0
    result.push(tinyram_instruction(tinyram_opcode_CJMP,    true, 0, 0, 7));         // 3: cjmp 7
    result.push(tinyram_instruction(tinyram_opcode_ADD,     true, 0, 0, increment)); // 4: add r0, r0, INCREMENT
    result.push(tinyram_instruction(tinyram_opcode_STOREW, false, 1, 0, 0));         // 5: store.w r0, r1
    result.push(tinyram_instruction(tinyram_opcode_JMP,     true, 0, 0, 2));         // 6: jmp 2
    result.push(tinyram_instruction(tinyram_opcode_STOREW,  true, 0, 0, mem_start)); // 7: store.w 2^{W-1}, r0
    return result;
}
impl tinyram_architecture_params{
pub fn address_size(&self)->usize
{
    return dwaddr_len();
}

pub fn value_size(&self)->usize
{
    return 2*w;
}

pub fn cpu_state_size(&self)->usize
{
    return k * w + 2; /* + flag + tape1_exhausted */
}

pub fn initial_pc_addr(&self)->usize
{
    /* the initial PC address is memory units for the RAM reduction */
    let initial_pc_addr = generate_tinyram_prelude(*this).len();
    return initial_pc_addr;
}

pub fn initial_cpu_state(&self)->bit_vector
{
    let  result=vec![false;self.cpu_state_size()];
    return result;
}

pub fn initial_memory_contents(program:&tinyram_program,
                                                                     primary_input:&tinyram_input_tape) ->memory_contents
{
    // remember that memory consists of 1u64<<dwaddr_len() double words (!)
     let mut m=memory_contents::new();

    for i in 0..program.instructions.len()
    {
        m[i] = program.instructions[i].as_dword(*this);
    }

    let  input_addr = 1u64 << (dwaddr_len() - 1);
    let  latest_double_word = (1u64<<(w-1)) + primary_input.len(); // the first word will contain 2^{w-1} + input_size (the location where the last input word was stored)

    for i in 0..primary_input.len()/2 + 1
    {
        if 2*i < primary_input.len()
        {
            latest_double_word += (primary_input[2*i] << w);
        }

        m[input_addr + i] = latest_double_word;

        if 2*i + 1 < primary_input.len()
        {
            latest_double_word = primary_input[2*i+1];
        }
    }

    return m;
}

pub fn opcode_width(&self)->usize
{
    return log2(tinyram_opcode_ANSWER as usize); /* assumption: answer is the last */
}

pub fn reg_arg_width(&self)->usize
{
    return log2(k);
}

pub fn instruction_padding_width(&self)->usize
{
    return 2 * w - (opcode_width() + 1 + 2 * reg_arg_width() + reg_arg_or_imm_width());
}

pub fn reg_arg_or_imm_width(&self)->usize
{
    return std::cmp::max(w, reg_arg_width());
}

pub fn dwaddr_len(&self)->usize
{
    return w-(log2(w)-2);
}

pub fn subaddr_len(&self)->usize
{
    return log2(w)-2;
}

pub fn bytes_in_word(&self)->usize
{
    return w/8;
}

pub fn instr_size(&self)->usize
{
    return 2*w;
}
}

// bool tinyram_architecture_params::operator==(other:&tinyram_architecture_params) const
// {
//     return (self.w == other.w &&
//             self.k == other.k);
// }

// std::ostream& operator<<(std::ostream &out, ap:&tinyram_architecture_params)
// {
//     out << ap.w << "\n";
//     out << ap.k << "\n";
//     return out;
// }

// std::istream& operator>>(std::istream &in, tinyram_architecture_params &ap)
// {
//     in >> ap.w;
//     consume_newline(in);
//     in >> ap.k;
//     consume_newline(in);
//     return in;
// }

impl tinyram_instruction{
pub fn new(opcode:&tinyram_opcode,
                                         arg2_is_imm:bool,
                                         desidx:&usize,
                                         arg1idx:&usize,
                                         arg2idx_or_imm:&usize) ->Self
   
{
 Self{opcode,
    arg2_is_imm,
    desidx,
    arg1idx,
    arg2idx_or_imm}
}

pub fn as_dword(ap:&tinyram_architecture_params) ->usize
{
    let  result = opcode as usize;
    result = (result << 1) | ( if arg2_is_imm  {1} else {0});
    result = (result << log2(ap.k)) | desidx;
    result = (result << log2(ap.k)) | arg1idx;
    result = (result << (2*ap.w - ap.opcode_width() - 1 - 2 * log2(ap.k))) | arg2idx_or_imm;

    return result;
}

pub fn print() 
{
    print!("* Number of registers (k): {}\n", k);
    print!("* Word size (w): {}\n", w);
}
}

 pub fn random_tinyram_instruction(ap:&tinyram_architecture_params)->tinyram_instruction
{
     use rand::Rng;
    let mut rng = rand::thread_rng();
    let  opcode = (rng::r#gen::<u64>() % (1u64<<ap.opcode_width()));//(tinyram_opcode)
    let arg2_is_imm = rng::r#gen::<u64>() & 1;
    let desidx = rng::r#gen::<u64>() % (1u64<<ap.reg_arg_width());
    let arg1idx = rng::r#gen::<u64>() % (1u64<<ap.reg_arg_width());
    let arg2idx_or_imm = rng::r#gen::<u64>() % (1u64<<ap.reg_arg_or_imm_width());
    return tinyram_instruction::new(opcode, arg2_is_imm, desidx, arg1idx, arg2idx_or_imm);
}
impl tinyram_program{
pub fn add_instruction(instr:&tinyram_instruction)
{
    instructions.push(instr);
}
}

 pub fn load_preprocessed_program(ap:&tinyram_architecture_params,
                                          preprocessed:&String)->tinyram_program
{
    ensure_tinyram_opcode_value_map();

     let program=tinyram_program::new();

    enter_block("Loading program");
    let (mut instr, mut line)=(String::new(),String::new());
    let mut it=preprocessed.split_ascii_whitespace();
    while let Some(instr)=it.next()
    {
        print_indent();
        let ( immflag, des, a1);
        let ( mut  a2);
        if it.is_some()
        {
            let immflag=it.next().unwrap();
            let des=it.next().unwrap();
            let a1=it.next().unwrap();
            let a2=it.next().unwrap();
            // preprocessed >> immflag >> des >> a1 >> a2;
            a2 = ((1u64<<ap.w)+(a2 % (1u64<<ap.w))) % (1u64<<ap.w);
            program.add_instruction(tinyram_instruction(opcode_values[instr], immflag, des, a1, a2));
        }
    }
    leave_block("Loading program");

    return program;
}

 pub fn tinyram_boot_trace_from_program_and_input(ap:&tinyram_architecture_params,
                                                             boot_trace_size_bound:usize,
                                                             program:&tinyram_program,
                                                             primary_input:&tinyram_input_tape)->memory_store_trace
{
    // TODO: document the reverse order here

     let mut result=memory_store_trace::new();

    let  boot_pos = boot_trace_size_bound-1;
    for i in 0..program.instructions.len()
    {
        result.set_trace_entry(boot_pos, std::make_pair(i, program.instructions[i].as_dword(ap)));
boot_pos-=1;
    }

    letprimary_input_base_addr = (1u64 << (ap.dwaddr_len()-1));

    for  j in (0.. primary_input.len()).step_by(2)
    {
        let  memory_dword = primary_input[j] + (( if j+1 < primary_input.len() { primary_input[j+1]} else {0}) << ap.w);
        result.set_trace_entry(boot_pos, std::make_pair(primary_input_base_addr + j, memory_dword));
        boot_pos-=1;
    }

    return result;
}

 pub fn load_tape(tape:&String)->tinyram_input_tape
{
    enter_block("Loading tape");
    let mut  result=tinyram_input_tape::new();

    print_indent();
    print!("Tape contents:");
    let mut tape=tape.split_ascii_whitespace();
    while let Some(cell)=tape.next()
    {
        print!("\t{}", cell);
        result.push(cell);
    }
    print!("\n");

    leave_block("Loading tape");
    return result;
}


