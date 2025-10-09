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

use  <cassert>
use  <iostream>
use  <map>

use ffec::common::utils;

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use libsnark/relations/ram_computations/memory/memory_interface;
use libsnark/relations/ram_computations/rams/ram_params;



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
};

enum tinyram_opcode_args {
    tinyram_opcode_args_des_arg1_arg2 = 1,
    tinyram_opcode_args_des_arg2 = 2,
    tinyram_opcode_args_arg1_arg2 = 3,
    tinyram_opcode_args_arg2 = 4,
    tinyram_opcode_args_none = 5,
    tinyram_opcode_args_arg2_des = 6
};

/**
 * Instructions that may change a register or the flag.
 * All other instructions leave all registers and the flag intact.
 */
const static int tinyram_opcodes_register[] = {
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
};

/**
 * Instructions that modify the program counter.
 * All other instructions either advance it (+1) or stall (see below).
 */
const static int tinyram_opcodes_control_flow[] = {
    tinyram_opcode_JMP,
    tinyram_opcode_CJMP,
    tinyram_opcode_CNJMP
};

/**
 * Instructions that make the program counter stall;
 * these are "answer" plus all the undefined opcodes.
 */
const static int tinyram_opcodes_stall[] = {
    tinyram_opcode_10111,
    tinyram_opcode_11000,
    tinyram_opcode_11001,

    tinyram_opcode_ANSWER
};

type size_t reg_count_t; // type for the number of registers
type size_t reg_width_t; // type for the width of a register

extern std::map<tinyram_opcode, std::string> tinyram_opcode_names;

extern std::map<std::string, tinyram_opcode> opcode_values;

extern std::map<tinyram_opcode, tinyram_opcode_args> opcode_args;

void ensure_tinyram_opcode_value_map();

class tinyram_program;
type std::vector<size_t> tinyram_input_tape;
type typename tinyram_input_tape::const_iterator tinyram_input_tape_iterator;

class tinyram_architecture_params {
public:
    reg_width_t w; /* width of a register */
    reg_count_t k; /* number of registers */

    tinyram_architecture_params() {};
    tinyram_architecture_params(const reg_width_t w, const reg_count_t k) : w(w), k(k) { assert!(w == 1ul << ffec::log2(w)); };

    size_t address_size() const;
    size_t value_size() const;
    size_t cpu_state_size() const;
    size_t initial_pc_addr() const;

    ffec::bit_vector initial_cpu_state() const;
    memory_contents initial_memory_contents(const tinyram_program &program,
                                            const tinyram_input_tape &primary_input) const;

    size_t opcode_width() const;
    size_t reg_arg_width() const;
    size_t instruction_padding_width() const;
    size_t reg_arg_or_imm_width() const;

    size_t dwaddr_len() const;
    size_t subaddr_len() const;

    size_t bytes_in_word() const;

    size_t instr_size() const;

    bool operator==(const tinyram_architecture_params &other) const;

    friend std::ostream& operator<<(std::ostream &out, const tinyram_architecture_params &ap);
    friend std::istream& operator>>(std::istream &in, tinyram_architecture_params &ap);

    void print() const;
};

/* order everywhere is reversed (i.e. MSB comes first),
   corresponding to the order in memory */

class tinyram_instruction {
public:
    tinyram_opcode opcode;
    bool arg2_is_imm;
    size_t desidx;
    size_t arg1idx;
    size_t arg2idx_or_imm;

    tinyram_instruction(const tinyram_opcode &opcode,
                        const bool arg2_is_imm,
                        const size_t &desidx,
                        const size_t &arg1idx,
                        const size_t &arg2idx_or_imm);

    size_t as_dword(const tinyram_architecture_params &ap) const;
};

tinyram_instruction random_tinyram_instruction(const tinyram_architecture_params &ap);

std::vector<tinyram_instruction> generate_tinyram_prelude(const tinyram_architecture_params &ap);
extern tinyram_instruction tinyram_default_instruction;

class tinyram_program {
public:
    std::vector<tinyram_instruction> instructions;
    size_t size() const { return instructions.size(); }
    void add_instruction(const tinyram_instruction &instr);
};

tinyram_program load_preprocessed_program(const tinyram_architecture_params &ap,
                                          std::istream &preprocessed);

memory_store_trace tinyram_boot_trace_from_program_and_input(const tinyram_architecture_params &ap,
                                                             const size_t boot_trace_size_bound,
                                                             const tinyram_program &program,
                                                             const tinyram_input_tape &primary_input);

tinyram_input_tape load_tape(std::istream &tape);



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

use  <cassert>
use  <fstream>
use  <string>

use ffec::common::profiling;
use ffec::common::utils;

use libsnark/relations/ram_computations/rams/tinyram/tinyram_aux;



tinyram_instruction tinyram_default_instruction = tinyram_instruction(tinyram_opcode_ANSWER, true, 0, 0, 1);

std::map<tinyram_opcode, std::string> tinyram_opcode_names =
{
    { tinyram_opcode_AND,    "and" },
    { tinyram_opcode_OR,     "or" },
    { tinyram_opcode_XOR,    "xor" },
    { tinyram_opcode_NOT,    "not" },
    { tinyram_opcode_ADD,    "add" },
    { tinyram_opcode_SUB,    "sub" },
    { tinyram_opcode_MULL,   "mull" },
    { tinyram_opcode_UMULH,  "umulh" },
    { tinyram_opcode_SMULH,  "smulh" },
    { tinyram_opcode_UDIV,   "udiv" },
    { tinyram_opcode_UMOD,   "umod" },
    { tinyram_opcode_SHL,    "shl" },
    { tinyram_opcode_SHR,    "shr" },

    { tinyram_opcode_CMPE,   "cmpe" },
    { tinyram_opcode_CMPA,   "cmpa" },
    { tinyram_opcode_CMPAE,  "cmpae" },
    { tinyram_opcode_CMPG,   "cmpg" },
    { tinyram_opcode_CMPGE,  "cmpge" },

    { tinyram_opcode_MOV,    "mov" },
    { tinyram_opcode_CMOV,   "cmov" },
    { tinyram_opcode_JMP,    "jmp" },

    { tinyram_opcode_CJMP,   "cjmp" },
    { tinyram_opcode_CNJMP,  "cnjmp" },

    { tinyram_opcode_10111,  "opcode_10111" },
    { tinyram_opcode_11000,  "opcode_11000" },
    { tinyram_opcode_11001,  "opcode_11001" },
    { tinyram_opcode_STOREB, "store.b" },
    { tinyram_opcode_LOADB,  "load.b" },

    { tinyram_opcode_STOREW, "store.w" },
    { tinyram_opcode_LOADW,  "load.w" },
    { tinyram_opcode_READ,   "read" },
    { tinyram_opcode_ANSWER, "answer" }
};

std::map<tinyram_opcode, tinyram_opcode_args> opcode_args =
{
    { tinyram_opcode_AND,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_OR,      tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_XOR,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_NOT,     tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_ADD,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_SUB,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_MULL,    tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_UMULH,   tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_SMULH,   tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_UDIV,    tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_UMOD,    tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_SHL,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_SHR,     tinyram_opcode_args_des_arg1_arg2 },
    { tinyram_opcode_CMPE,    tinyram_opcode_args_arg1_arg2 },
    { tinyram_opcode_CMPA,    tinyram_opcode_args_arg1_arg2 },
    { tinyram_opcode_CMPAE,   tinyram_opcode_args_arg1_arg2 },
    { tinyram_opcode_CMPG,    tinyram_opcode_args_arg1_arg2 },
    { tinyram_opcode_CMPGE,   tinyram_opcode_args_arg1_arg2 },
    { tinyram_opcode_MOV,     tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_CMOV,    tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_JMP,     tinyram_opcode_args_arg2 },
    { tinyram_opcode_CJMP,    tinyram_opcode_args_arg2 },
    { tinyram_opcode_CNJMP,   tinyram_opcode_args_arg2 },
    { tinyram_opcode_10111,   tinyram_opcode_args_none },
    { tinyram_opcode_11000,   tinyram_opcode_args_none },
    { tinyram_opcode_11001,   tinyram_opcode_args_none },
    { tinyram_opcode_STOREB,  tinyram_opcode_args_arg2_des },
    { tinyram_opcode_LOADB,   tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_STOREW,  tinyram_opcode_args_arg2_des },
    { tinyram_opcode_LOADW,   tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_READ,    tinyram_opcode_args_des_arg2 },
    { tinyram_opcode_ANSWER,  tinyram_opcode_args_arg2 }
};

std::map<std::string, tinyram_opcode> opcode_values;

void ensure_tinyram_opcode_value_map()
{
    if (opcode_values.empty())
    {
        for (auto it : tinyram_opcode_names)
        {
            opcode_values[it.second] = it.first;
        }
    }
}

std::vector<tinyram_instruction> generate_tinyram_prelude(const tinyram_architecture_params &ap)
{
    std::vector<tinyram_instruction> result;
    const size_t increment = ffec::log2(ap.w)/8;
    const size_t mem_start = 1ul<<(ap.w-1);
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

size_t tinyram_architecture_params::address_size() const
{
    return dwaddr_len();
}

size_t tinyram_architecture_params::value_size() const
{
    return 2*w;
}

size_t tinyram_architecture_params::cpu_state_size() const
{
    return k * w + 2; /* + flag + tape1_exhausted */
}

size_t tinyram_architecture_params::initial_pc_addr() const
{
    /* the initial PC address is memory units for the RAM reduction */
    const size_t initial_pc_addr = generate_tinyram_prelude(*this).size();
    return initial_pc_addr;
}

ffec::bit_vector tinyram_architecture_params::initial_cpu_state() const
{
    ffec::bit_vector result(self.cpu_state_size(), false);
    return result;
}

memory_contents tinyram_architecture_params::initial_memory_contents(const tinyram_program &program,
                                                                     const tinyram_input_tape &primary_input) const
{
    // remember that memory consists of 1ul<<dwaddr_len() double words (!)
    memory_contents m;

    for (size_t i = 0; i < program.instructions.size(); ++i)
    {
        m[i] = program.instructions[i].as_dword(*this);
    }

    const size_t input_addr = 1ul << (dwaddr_len() - 1);
    size_t latest_double_word = (1ull<<(w-1)) + primary_input.size(); // the first word will contain 2^{w-1} + input_size (the location where the last input word was stored)

    for (size_t i = 0; i < primary_input.size()/2 + 1; ++i)
    {
        if (2*i < primary_input.size())
        {
            latest_double_word += (primary_input[2*i] << w);
        }

        m[input_addr + i] = latest_double_word;

        if (2*i + 1 < primary_input.size())
        {
            latest_double_word = primary_input[2*i+1];
        }
    }

    return m;
}

size_t tinyram_architecture_params::opcode_width() const
{
    return ffec::log2(static_cast<size_t>(tinyram_opcode_ANSWER)); /* assumption: answer is the last */
}

size_t tinyram_architecture_params::reg_arg_width() const
{
    return ffec::log2(k);
}

size_t tinyram_architecture_params::instruction_padding_width() const
{
    return 2 * w - (opcode_width() + 1 + 2 * reg_arg_width() + reg_arg_or_imm_width());
}

size_t tinyram_architecture_params::reg_arg_or_imm_width() const
{
    return std::max(w, reg_arg_width());
}

size_t tinyram_architecture_params::dwaddr_len() const
{
    return w-(ffec::log2(w)-2);
}

size_t tinyram_architecture_params::subaddr_len() const
{
    return ffec::log2(w)-2;
}

size_t tinyram_architecture_params::bytes_in_word() const
{
    return w/8;
}

size_t tinyram_architecture_params::instr_size() const
{
    return 2*w;
}

bool tinyram_architecture_params::operator==(const tinyram_architecture_params &other) const
{
    return (self.w == other.w &&
            self.k == other.k);
}

std::ostream& operator<<(std::ostream &out, const tinyram_architecture_params &ap)
{
    out << ap.w << "\n";
    out << ap.k << "\n";
    return out;
}

std::istream& operator>>(std::istream &in, tinyram_architecture_params &ap)
{
    in >> ap.w;
    ffec::consume_newline(in);
    in >> ap.k;
    ffec::consume_newline(in);
    return in;
}

tinyram_instruction::tinyram_instruction(const tinyram_opcode &opcode,
                                         const bool arg2_is_imm,
                                         const size_t &desidx,
                                         const size_t &arg1idx,
                                         const size_t &arg2idx_or_imm) :
    opcode(opcode),
    arg2_is_imm(arg2_is_imm),
    desidx(desidx),
    arg1idx(arg1idx),
    arg2idx_or_imm(arg2idx_or_imm)
{
}

size_t tinyram_instruction::as_dword(const tinyram_architecture_params &ap) const
{
    size_t result = static_cast<size_t>(opcode);
    result = (result << 1) | (arg2_is_imm ? 1 : 0);
    result = (result << ffec::log2(ap.k)) | desidx;
    result = (result << ffec::log2(ap.k)) | arg1idx;
    result = (result << (2*ap.w - ap.opcode_width() - 1 - 2 * ffec::log2(ap.k))) | arg2idx_or_imm;

    return result;
}

void tinyram_architecture_params::print() const
{
    print!("* Number of registers (k): {}\n", k);
    print!("* Word size (w): {}\n", w);
}

tinyram_instruction random_tinyram_instruction(const tinyram_architecture_params &ap)
{
    const tinyram_opcode opcode = (tinyram_opcode)(std::rand() % (1ul<<ap.opcode_width()));
    const bool arg2_is_imm = std::rand() & 1;
    const size_t desidx = std::rand() % (1ul<<ap.reg_arg_width());
    const size_t arg1idx = std::rand() % (1ul<<ap.reg_arg_width());
    const size_t arg2idx_or_imm = std::rand() % (1ul<<ap.reg_arg_or_imm_width());
    return tinyram_instruction(opcode, arg2_is_imm, desidx, arg1idx, arg2idx_or_imm);
}

void tinyram_program::add_instruction(const tinyram_instruction &instr)
{
    instructions.push(instr);
}

tinyram_program load_preprocessed_program(const tinyram_architecture_params &ap,
                                          std::istream &preprocessed)
{
    ensure_tinyram_opcode_value_map();

    tinyram_program program;

    ffec::enter_block("Loading program");
    std::string instr, line;

    while (preprocessed >> instr)
    {
        ffec::print_indent();
        size_t immflag, des, a1;
        long long int a2;
        if (preprocessed.good())
        {
            preprocessed >> immflag >> des >> a1 >> a2;
            a2 = ((1ul<<ap.w)+(a2 % (1ul<<ap.w))) % (1ul<<ap.w);
            program.add_instruction(tinyram_instruction(opcode_values[instr], immflag, des, a1, a2));
        }
    }
    ffec::leave_block("Loading program");

    return program;
}

memory_store_trace tinyram_boot_trace_from_program_and_input(const tinyram_architecture_params &ap,
                                                             const size_t boot_trace_size_bound,
                                                             const tinyram_program &program,
                                                             const tinyram_input_tape &primary_input)
{
    // TODO: document the reverse order here

    memory_store_trace result;

    size_t boot_pos = boot_trace_size_bound-1;
    for (size_t i = 0; i < program.instructions.size(); ++i)
    {
        result.set_trace_entry(boot_pos--, std::make_pair(i, program.instructions[i].as_dword(ap)));
    }

    const size_t primary_input_base_addr = (1ul << (ap.dwaddr_len()-1));

    for (size_t j = 0; j < primary_input.size(); j += 2)
    {
        const size_t memory_dword = primary_input[j] + ((j+1 < primary_input.size() ? primary_input[j+1] : 0) << ap.w);
        result.set_trace_entry(boot_pos--, std::make_pair(primary_input_base_addr + j, memory_dword));
    }

    return result;
}

tinyram_input_tape load_tape(std::istream &tape)
{
    ffec::enter_block("Loading tape");
    tinyram_input_tape result;

    ffec::print_indent();
    print!("Tape contents:");
    size_t cell;
    while (tape >> cell)
    {
        print!("\t{}", cell);
        result.push(cell);
    }
    print!("\n");

    ffec::leave_block("Loading tape");
    return result;
}


