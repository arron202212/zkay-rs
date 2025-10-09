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

use libsnark/relations/ram_computations/rams/ram_params;



template<typename ramT>
struct ram_example {
    ram_architecture_params<ramT> ap;
    size_t boot_trace_size_bound;
    size_t time_bound;
    ram_boot_trace<ramT> boot_trace;
    ram_input_tape<ramT> auxiliary_input;
};

/**
 * For now: only specialized to TinyRAM
 */
template<typename ramT>
ram_example<ramT> gen_ram_example_simple(const ram_architecture_params<ramT> &ap, const size_t boot_trace_size_bound, const size_t time_bound, const bool satisfiable=true);

/**
 * For now: only specialized to TinyRAM
 */
template<typename ramT>
ram_example<ramT> gen_ram_example_complex(const ram_architecture_params<ramT> &ap, const size_t boot_trace_size_bound, const size_t time_bound, const bool satisfiable=true);



use libsnark/relations/ram_computations/rams/examples/ram_examples;

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

use libsnark/relations/ram_computations/rams/tinyram/tinyram_aux;



template<typename ramT>
ram_example<ramT> gen_ram_example_simple(const ram_architecture_params<ramT> &ap, const size_t boot_trace_size_bound, const size_t time_bound, const bool satisfiable)
{
    ffec::enter_block("Call to gen_ram_example_simple");

    const size_t program_size = boot_trace_size_bound / 2;
    const size_t input_size = boot_trace_size_bound - program_size;

    ram_example<ramT> result;

    result.ap = ap;
    result.boot_trace_size_bound = boot_trace_size_bound;
    result.time_bound = time_bound;

    tinyram_program prelude; prelude.instructions = generate_tinyram_prelude(ap);

    size_t boot_pos = 0;
    for (size_t i = 0; i < prelude.instructions.size(); ++i)
    {
        result.boot_trace.set_trace_entry(boot_pos++, std::make_pair(i, prelude.instructions[i].as_dword(ap)));
    }

    result.boot_trace[boot_pos] = std::make_pair(boot_pos++, tinyram_instruction(tinyram_opcode_ANSWER, true,      0,       0,       satisfiable ? 0 : 1).as_dword(ap)); /* answer 0/1 depending on satisfiability */

    while (boot_pos < program_size)
    {
        result.boot_trace.set_trace_entry(boot_pos++, random_tinyram_instruction(ap).as_dword(ap));
    }

    for (size_t i = 0; i < input_size; ++i)
    {
        result.boot_trace.set_trace_entry(boot_pos++, std::make_pair((1ul<<(ap.dwaddr_len()-1)) + i, std::rand() % (1ul<<(2*ap.w))));
    }

    assert!(boot_pos == boot_trace_size_bound);

    ffec::leave_block("Call to gen_ram_example_simple");
    return result;
}

template<typename ramT>
ram_example<ramT> gen_ram_example_complex(const ram_architecture_params<ramT> &ap, const size_t boot_trace_size_bound, const size_t time_bound, const bool satisfiable)
{
    ffec::enter_block("Call to gen_ram_example_complex");

    const size_t program_size = boot_trace_size_bound / 2;
    const size_t input_size = boot_trace_size_bound - program_size;

    assert!(2*ap.w/8*program_size < 1ul<<(ap.w-1));
    assert!(ap.w/8*input_size < 1ul<<(ap.w-1));

    ram_example<ramT> result;

    result.ap = ap;
    result.boot_trace_size_bound = boot_trace_size_bound;
    result.time_bound = time_bound;

    tinyram_program prelude; prelude.instructions = generate_tinyram_prelude(ap);

    size_t boot_pos = 0;
    for (size_t i = 0; i < prelude.instructions.size(); ++i)
    {
        result.boot_trace.set_trace_entry(boot_pos++, std::make_pair(i, prelude.instructions[i].as_dword(ap)));
    }

    const size_t prelude_len = prelude.instructions.size();
    const size_t instr_addr = (prelude_len+4)*(2*ap.w/8);
    const size_t input_addr = (1ul<<(ap.w-1)) + (ap.w/8); // byte address of the first input word

    result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, tinyram_instruction(tinyram_opcode_LOADB,  true,      1,       0, instr_addr).as_dword(ap)));
    boot_pos+=1;
    result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, tinyram_instruction(tinyram_opcode_LOADW,  true,      2,       0, input_addr).as_dword(ap)));
    boot_pos+=1;
    result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, tinyram_instruction(tinyram_opcode_SUB,    false,     1,       1, 2).as_dword(ap)));
    boot_pos+=1;
    result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, tinyram_instruction(tinyram_opcode_STOREB, true,      1,       0, instr_addr).as_dword(ap)));
    boot_pos+=1;
    result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, tinyram_instruction(tinyram_opcode_ANSWER, true,      0,       0, 1).as_dword(ap)));
    boot_pos+=1;

    while (boot_pos < program_size)
    {
        result.boot_trace.set_trace_entry(boot_pos, std::make_pair(boot_pos, random_tinyram_instruction(ap).as_dword(ap)));
        boot_pos+=1;
    }

    result.boot_trace.set_trace_entry(boot_pos++, std::make_pair(1ul<<(ap.dwaddr_len()-1), satisfiable ? 1ul<<ap.w : 0));

    for (size_t i = 1; i < input_size; ++i)
    {
        result.boot_trace.set_trace_entry(boot_pos++, std::make_pair((1ul<<(ap.dwaddr_len()-1)) + i + 1, std::rand() % (1ul<<(2*ap.w))));
    }

    ffec::leave_block("Call to gen_ram_example_complex");
    return result;
}



//#endif // RAM_EXAMPLES_TCC_
