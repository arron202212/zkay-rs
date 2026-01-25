use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
/** @file
*****************************************************************************

Declaration of interfaces for a RAM example, as well as functions to sample
RAM examples with prescribed parameters (according to some distribution).

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RAM_EXAMPLES_HPP_
// #define RAM_EXAMPLES_HPP_
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_boot_trace, ram_input_tape, ram_params_type,
};
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    generate_tinyram_prelude, random_tinyram_instruction, tinyram_opcode, tinyram_program,
};
use ffec::common::profiling::{enter_block, leave_block};
#[derive(Default)]
pub struct ram_example<RamT: ram_params_type> {
    ap: ram_architecture_params<RamT>,
    boot_trace_size_bound: usize,
    time_bound: usize,
    boot_trace: ram_boot_trace,
    auxiliary_input: ram_input_tape,
}

// /**
//  * For now: only specialized to TinyRAM
//  */
//
// ram_example<RamT> gen_ram_example_simple(ap:&ram_architecture_params<RamT>, boot_trace_size_bound:usize, time_bound:usize, satisfiable:bool=true);

// /**
//  * For now: only specialized to TinyRAM
//  */
//
// ram_example<RamT> gen_ram_example_complex(ap:&ram_architecture_params<RamT>, boot_trace_size_bound:usize, time_bound:usize, satisfiable:bool=true);

// use crate::relations::ram_computations::rams::examples::ram_examples;

//#endif // RAM_EXAMPLES_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a RAM example, as well as functions to sample
RAM examples with prescribed parameters (according to some distribution).

See ram_examples.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RAM_EXAMPLES_TCC_
// #define RAM_EXAMPLES_TCC_
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_instruction;

pub fn gen_ram_example_simple<RamT: ram_params_type>(
    ap: ram_architecture_params<RamT>,
    boot_trace_size_bound: usize,
    time_bound: usize,
    satisfiable: bool,
) -> ram_example<RamT> {
    enter_block("Call to gen_ram_example_simple", false);

    let program_size = boot_trace_size_bound / 2;
    let input_size = boot_trace_size_bound - program_size;

    let mut result = ram_example::<RamT>::default();

    result.ap = ap.clone();
    result.boot_trace_size_bound = boot_trace_size_bound;
    result.time_bound = time_bound;

    let mut prelude = tinyram_program::default();
    prelude.instructions = generate_tinyram_prelude(&ap);

    let mut boot_pos = 0;
    for i in 0..prelude.instructions.len() {
        result
            .boot_trace
            .set_trace_entry(boot_pos, (i, prelude.instructions[i].as_dword(&ap)));
        boot_pos += 1;
    }

    result.boot_trace[boot_pos] = (
        boot_pos,
        tinyram_instruction::new(
            tinyram_opcode::tinyram_opcode_ANSWER,
            true,
            0,
            0,
            if satisfiable { 0 } else { 1 },
        )
        .as_dword(&ap),
    ); /* answer 0/1 depending on satisfiability */
    boot_pos += 1;
    while (boot_pos < program_size) {
        result.boot_trace.set_trace_entry(
            boot_pos,
            (boot_pos, random_tinyram_instruction(&ap).as_dword(&ap)),
        );
        boot_pos += 1;
    }

    for i in 0..input_size {
        result.boot_trace.set_trace_entry(
            boot_pos,
            (
                (1usize << (ap.dwaddr_len() - 1)) + i,
                rand::random::<usize>() % (1usize << (2 * ap.w())),
            ),
        );
        boot_pos += 1;
    }

    assert!(boot_pos == boot_trace_size_bound);

    leave_block("Call to gen_ram_example_simple", false);
    return result;
}

pub fn gen_ram_example_complex<RamT: ram_params_type>(
    ap: ram_architecture_params<RamT>,
    boot_trace_size_bound: usize,
    time_bound: usize,
    satisfiable: bool,
) -> ram_example<RamT> {
    enter_block("Call to gen_ram_example_complex", false);

    let program_size = boot_trace_size_bound / 2;
    let input_size = boot_trace_size_bound - program_size;

    assert!(2 * ap.w() / 8 * program_size < 1usize << (ap.w() - 1));
    assert!(ap.w() / 8 * input_size < 1usize << (ap.w() - 1));

    let mut result = ram_example::<RamT>::default();

    result.ap = ap.clone();
    result.boot_trace_size_bound = boot_trace_size_bound;
    result.time_bound = time_bound;

    let mut prelude = tinyram_program::default();
    prelude.instructions = generate_tinyram_prelude(&ap);

    let mut boot_pos = 0;
    for i in 0..prelude.instructions.len() {
        result
            .boot_trace
            .set_trace_entry(boot_pos, (i, prelude.instructions[i].as_dword(&ap)));
        boot_pos += 1;
    }

    let prelude_len = prelude.instructions.len();
    let instr_addr = (prelude_len + 4) * (2 * ap.w() / 8);
    let input_addr = (1usize << (ap.w() - 1)) + (ap.w() / 8); // byte address of the first input word

    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            boot_pos,
            tinyram_instruction::new(tinyram_opcode::tinyram_opcode_LOADB, true, 1, 0, instr_addr)
                .as_dword(&ap),
        ),
    );
    boot_pos += 1;
    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            boot_pos,
            tinyram_instruction::new(tinyram_opcode::tinyram_opcode_LOADW, true, 2, 0, input_addr)
                .as_dword(&ap),
        ),
    );
    boot_pos += 1;
    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            boot_pos,
            tinyram_instruction::new(tinyram_opcode::tinyram_opcode_SUB, false, 1, 1, 2)
                .as_dword(&ap),
        ),
    );
    boot_pos += 1;
    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            boot_pos,
            tinyram_instruction::new(
                tinyram_opcode::tinyram_opcode_STOREB,
                true,
                1,
                0,
                instr_addr,
            )
            .as_dword(&ap),
        ),
    );
    boot_pos += 1;
    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            boot_pos,
            tinyram_instruction::new(tinyram_opcode::tinyram_opcode_ANSWER, true, 0, 0, 1)
                .as_dword(&ap),
        ),
    );
    boot_pos += 1;

    while (boot_pos < program_size) {
        result.boot_trace.set_trace_entry(
            boot_pos,
            (boot_pos, random_tinyram_instruction(&ap).as_dword(&ap)),
        );
        boot_pos += 1;
    }

    result.boot_trace.set_trace_entry(
        boot_pos,
        (
            1usize << (ap.dwaddr_len() - 1),
            if satisfiable { 1usize << ap.w() } else { 0 },
        ),
    );
    boot_pos += 1;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for i in 1..input_size {
        result.boot_trace.set_trace_entry(
            boot_pos,
            (
                (1usize << (ap.dwaddr_len() - 1)) + i + 1,
                rand::random::<usize>() % (1usize << (2 * ap.w())),
            ),
        );
        boot_pos += 1;
    }

    leave_block("Call to gen_ram_example_complex", false);
    return result;
}

//#endif // RAM_EXAMPLES_TCC_
