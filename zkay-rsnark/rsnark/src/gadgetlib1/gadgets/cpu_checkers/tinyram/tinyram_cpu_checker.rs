/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM CPU checker gadget.

 The gadget checks the correct operation for the CPU of the TinyRAM architecture.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_CPU_CHECKER_HPP_
// #define TINYRAM_CPU_CHECKER_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::argument_decoder_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::consistency_enforcer_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::memory_masking_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget;



// 
pub struct tinyram_cpu_checker<FieldT> {
// : public tinyram_standard_gadget<FieldT> 
opcode:    pb_variable_array<FieldT>,
arg2_is_imm:    pb_variable<FieldT>,
desidx:    pb_variable_array<FieldT>,
arg1idx:    pb_variable_array<FieldT>,
arg2idx:    pb_variable_array<FieldT>,

prev_registers:    Vec<word_variable_gadget<FieldT> >,
next_registers:    Vec<word_variable_gadget<FieldT> >,
prev_flag:    pb_variable<FieldT>,
next_flag:    pb_variable<FieldT>,
prev_tape1_exhausted:    pb_variable<FieldT>,
next_tape1_exhausted:    pb_variable<FieldT>,

prev_pc_addr_as_word_variable:    RcCell<word_variable_gadget<FieldT> >,
desval:    RcCell<word_variable_gadget<FieldT> >,
arg1val:    RcCell<word_variable_gadget<FieldT> >,
arg2val:    RcCell<word_variable_gadget<FieldT> >,

decode_arguments:    RcCell<argument_decoder_gadget<FieldT> >,
opcode_indicators:    pb_variable_array<FieldT>,
ALU:    RcCell<ALU_gadget<FieldT> >,

ls_prev_val_as_doubleword_variable:    RcCell<doubleword_variable_gadget<FieldT> >,
ls_next_val_as_doubleword_variable:    RcCell<doubleword_variable_gadget<FieldT> >,
memory_subaddress:    RcCell<dual_variable_gadget<FieldT> >,
memory_subcontents:    pb_variable<FieldT>,
memory_access_is_word:    pb_linear_combination<FieldT>,
memory_access_is_byte:    pb_linear_combination<FieldT>,
check_memory:    RcCell<memory_masking_gadget<FieldT> >,

next_pc_addr_as_word_variable:    RcCell<word_variable_gadget<FieldT> >,
consistency_enforcer:    RcCell<consistency_enforcer_gadget<FieldT> >,

instruction_results:    pb_variable_array<FieldT>,
instruction_flags:    pb_variable_array<FieldT>,

read_not1:    pb_variable<FieldT>,
// 
prev_pc_addr:    pb_variable_array<FieldT>,
prev_pc_val:    pb_variable_array<FieldT>,
prev_state:    pb_variable_array<FieldT>,
ls_addr:    pb_variable_array<FieldT>,
ls_prev_val:    pb_variable_array<FieldT>,
ls_next_val:    pb_variable_array<FieldT>,
next_state:    pb_variable_array<FieldT>,
next_pc_addr:    pb_variable_array<FieldT>,
next_has_accepted:    pb_variable<FieldT>,

    // tinyram_cpu_checker(tinyram_protoboard<FieldT> &pb,
    //                     pb_variable_array<FieldT> &prev_pc_addr,
    //                     pb_variable_array<FieldT> &prev_pc_val,
    //                     pb_variable_array<FieldT> &prev_state,
    //                     pb_variable_array<FieldT> &ls_addr,
    //                     pb_variable_array<FieldT> &ls_prev_val,
    //                     pb_variable_array<FieldT> &ls_next_val,
    //                     pb_variable_array<FieldT> &next_state,
    //                     pb_variable_array<FieldT> &next_pc_addr,
    //                     pb_variable<FieldT> &next_has_accepted,
    //                     annotation_prefix:&String);

    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness() { assert!(0); }
    // pub fn  generate_r1cs_witness_address();
    // pub fn  generate_r1cs_witness_other(tinyram_input_tape_iterator &aux_it,
    //                                  aux_end:&tinyram_input_tape_iterator);
    // pub fn  dump() const;
}



// use crate::gadgetlib1::gadgets::cpu_checkers/tinyram/tinyram_cpu_checker;

//#endif // TINYRAM_CPU_CHECKER_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM CPU checker gadget.

 See tinyram_cpu_checker.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_CPU_CHECKER_TCC_
// #define TINYRAM_CPU_CHECKER_TCC_

use ffec::algebra::field_utils::field_utils;


impl tinyram_cpu_checker<FieldT>{

pub fn new(
pb:tinyram_protoboard<FieldT>,
prev_pc_addr:                                                 pb_variable_array<FieldT>,
prev_pc_val:                                                 pb_variable_array<FieldT>,
prev_state:                                                 pb_variable_array<FieldT>,
ls_addr:                                                 pb_variable_array<FieldT>,
ls_prev_val:                                                 pb_variable_array<FieldT>,
ls_next_val:                                                 pb_variable_array<FieldT>,
next_state:                                                 pb_variable_array<FieldT>,
next_pc_addr:                                                 pb_variable_array<FieldT>,
next_has_accepted:                                                 pb_variable<FieldT>,
annotation_prefix:                                                  String
) ->Self

{
    /* parse previous PC value as an instruction (note that we start
       parsing from LSB of the instruction doubleword and go to the
       MSB) */
    let  pc_val_it = prev_pc_val.begin();

    arg2idx = pb_variable_array::<FieldT>(pc_val_it, pc_val_it + pb.ap.reg_arg_or_imm_width()); std::advance(pc_val_it, pb.ap.reg_arg_or_imm_width());
    std::advance(pc_val_it, pb.ap.instruction_padding_width());
    arg1idx = pb_variable_array::<FieldT>(pc_val_it, pc_val_it + pb.ap.reg_arg_width()); std::advance(pc_val_it, pb.ap.reg_arg_width());
    desidx = pb_variable_array::<FieldT>(pc_val_it, pc_val_it + pb.ap.reg_arg_width()); std::advance(pc_val_it, pb.ap.reg_arg_width());
    arg2_is_imm = *pc_val_it; std::advance(pc_val_it, 1);
    opcode = pb_variable_array::<FieldT>(pc_val_it, pc_val_it + pb.ap.opcode_width()); std::advance(pc_val_it, pb.ap.opcode_width());

    assert!(pc_val_it == prev_pc_val.end());

    /* parse state as registers + flags */
   let ( packed_prev_registers, packed_next_registers)=( pb_variable_array::<FieldT>::new(), pb_variable_array::<FieldT>::new());
    for i in 0..pb.ap.k
    {
        prev_registers.push(word_variable_gadget::<FieldT>(&pb, pb_variable_array::<FieldT>(prev_state.begin() + i * pb.ap.w, prev_state.begin() + (i + 1) * pb.ap.w), format!("{annotation_prefix} prev_registers_{}", i)));
        next_registers.push(word_variable_gadget::<FieldT>(&pb, pb_variable_array::<FieldT>(next_state.begin() + i * pb.ap.w, next_state.begin() + (i + 1) * pb.ap.w), format!("{annotation_prefix} next_registers_{}", i)));

        packed_prev_registers.push(prev_registers[i].packed);
        packed_next_registers.push(next_registers[i].packed);
    }
prev_state.next_back();
    prev_flag = *(prev_state.next_back().unwrap());
    next_state.next_back();
    next_flag = *(next_state.next_back().unwrap());
    prev_tape1_exhausted = *(prev_state.last().unwrap());
    next_tape1_exhausted = *(next_state.lat().unwrap());

    /* decode arguments */
    prev_pc_addr_as_word_variable.reset(word_variable_gadget::<FieldT>::new(pb, prev_pc_addr, format!("{annotation_prefix} prev_pc_addr_as_word_variable")));
    desval.reset(word_variable_gadget::<FieldT>::new(pb, format!("{annotation_prefix} desval")));
    arg1val.reset(word_variable_gadget::<FieldT>::new(pb, format!("{annotation_prefix} arg1val")));
    arg2val.reset(word_variable_gadget::<FieldT>::new(pb, format!("{annotation_prefix} arg2val")));

    decode_arguments.reset(argument_decoder_gadget::<FieldT>::new(pb, arg2_is_imm, desidx, arg1idx, arg2idx, packed_prev_registers,
                                                               desval.packed, arg1val.packed, arg2val.packed,
                                                               format!("{annotation_prefix} decode_arguments")));

    /* create indicator variables for opcodes */
    opcode_indicators.allocate(&pb, 1u64<<pb.ap.opcode_width(), format!("{annotation_prefix} opcode_indicators"));

    /* perform the ALU operations */
    instruction_results.allocate(&pb, 1u64<<pb.ap.opcode_width(), format!("{annotation_prefix} instruction_results"));
    instruction_flags.allocate(&pb, 1u64<<pb.ap.opcode_width(), format!("{annotation_prefix} instruction_flags"));

    ALU.reset(ALU_gadget::<FieldT>::new(pb, opcode_indicators, *prev_pc_addr_as_word_variable, *desval, *arg1val, *arg2val, prev_flag, instruction_results, instruction_flags,
                                     format!("{annotation_prefix} ALU")));

    /* check correctness of memory operations */
    ls_prev_val_as_doubleword_variable.reset(doubleword_variable_gadget::<FieldT>::new(pb, ls_prev_val, format!("{annotation_prefix} ls_prev_val_as_doubleword_variable")))
;
    ls_next_val_as_doubleword_variable.reset(doubleword_variable_gadget::<FieldT>::new(pb, ls_next_val, format!("{annotation_prefix} ls_next_val_as_doubleword_variable")));
    memory_subaddress.reset(dual_variable_gadget::<FieldT>::new(pb, pb_variable_array::<FieldT>(arg2val.bits.begin(), arg2val.bits.begin() + pb.ap.subaddr_len()),
                                                             format!("{annotation_prefix} memory_subaddress")));

    memory_subcontents.allocate(&pb, format!("{annotation_prefix} memory_subcontents"));
    memory_access_is_word.assign(&pb, 1 - (opcode_indicators[tinyram_opcode_LOADB] + opcode_indicators[tinyram_opcode_STOREB]));
    memory_access_is_byte.assign(&pb, opcode_indicators[tinyram_opcode_LOADB] + opcode_indicators[tinyram_opcode_STOREB]);

    check_memory.reset( memory_masking_gadget::<FieldT>::new(pb,
                                                         *ls_prev_val_as_doubleword_variable,
                                                         *memory_subaddress,
                                                         memory_subcontents,
                                                         memory_access_is_word,
                                                         memory_access_is_byte,
                                                         *ls_next_val_as_doubleword_variable,
                                                         format!("{annotation_prefix} check_memory")));

    /* handle reads */
    read_not1.allocate(&pb, format!("{annotation_prefix} read_not1"));

    /* check consistency of the states according to the ALU results */
    next_pc_addr_as_word_variable.reset(word_variable_gadget::<FieldT>::new(pb, next_pc_addr, format!("{annotation_prefix} next_pc_addr_as_word_variable")));

    consistency_enforcer.reset(consistency_enforcer_gadget::<FieldT>::new(pb, opcode_indicators, instruction_results, instruction_flags,
                                                                       desidx, prev_pc_addr_as_word_variable.packed,
                                                                       packed_prev_registers,
                                                                       desval.packed,
                                                                       prev_flag,
                                                                       next_pc_addr_as_word_variable.packed,
                                                                       packed_next_registers,
                                                                       next_flag,
                                                                       format!("{annotation_prefix} consistency_enforcer")));
    // tinyram_standard_gadget<FieldT>(&pb, annotation_prefix), 
   Self{ prev_pc_addr, prev_pc_val,
    prev_state, ls_addr, ls_prev_val, ls_next_val,
    next_state, next_pc_addr, next_has_accepted}
}


pub fn generate_r1cs_constraints()
{
    decode_arguments.generate_r1cs_constraints();

    /* generate indicator variables for opcode */
    for i in 0..1u64<<self.pb.ap.opcode_width()
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[i], pb_packing_sum::<FieldT>(opcode) - i, 0),
                                     format!("{} opcode_indicators_{}",self.annotation_prefix, i));
    }
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, pb_sum::<FieldT>(opcode_indicators), 1),
                                 format!("{} opcode_indicators_sum_to_1",self.annotation_prefix));

    /* consistency checks for repacked variables */
    for i in 0..self.pb.ap.k
    {
        prev_registers[i].generate_r1cs_constraints(true);
        next_registers[i].generate_r1cs_constraints(true);
    }
    prev_pc_addr_as_word_variable.generate_r1cs_constraints(true);
    next_pc_addr_as_word_variable.generate_r1cs_constraints(true);
    ls_prev_val_as_doubleword_variable.generate_r1cs_constraints(true);
    ls_next_val_as_doubleword_variable.generate_r1cs_constraints(true);

    /* main consistency checks */
    decode_arguments.generate_r1cs_constraints();
    ALU.generate_r1cs_constraints();
    consistency_enforcer.generate_r1cs_constraints();

    /* check correct access to memory */
    ls_prev_val_as_doubleword_variable.generate_r1cs_constraints(false);
    ls_next_val_as_doubleword_variable.generate_r1cs_constraints(false);
    memory_subaddress.generate_r1cs_constraints(false);
    check_memory.generate_r1cs_constraints();

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1,
                                                         pb_packing_sum::<FieldT>(
                                                             pb_variable_array::<FieldT>(arg2val.bits.begin() + self.pb.ap.subaddr_len(),
                                                                                       arg2val.bits.end())),
                                                         pb_packing_sum::<FieldT>(ls_addr)),
                                 format!("{} ls_addr_is_arg2val_minus_subaddress",self.annotation_prefix));

    /* We require that if opcode is one of load.{b,w}, then
       subcontents is appropriately stored in instruction_results. If
       opcode is store.b we only take the necessary portion of arg1val
       (i.e. last byte), and take entire arg1val for store.w.

       Note that ls_addr is *always* going to be arg2val. If the
       instruction is a non-memory instruction, we will treat it as a
       load from that memory location. */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[tinyram_opcode_LOADB],
                                                         memory_subcontents - instruction_results[tinyram_opcode_LOADB],
                                                         0),
                                 format!("{} handle_loadb",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[tinyram_opcode_LOADW],
                                                         memory_subcontents - instruction_results[tinyram_opcode_LOADW],
                                                         0),
                                 format!("{} handle_loadw",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[tinyram_opcode_STOREB],
                                                         memory_subcontents - pb_packing_sum::<FieldT>(
                                                             pb_variable_array::<FieldT>(desval.bits.begin(),
                                                                                       desval.bits.begin() + 8)),
                                                         0),
                                 format!("{} handle_storeb",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[tinyram_opcode_STOREW],
                                                         memory_subcontents - desval.packed,
                                                         0),
                                 format!("{} handle_storew",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1 - (opcode_indicators[tinyram_opcode_STOREB] + opcode_indicators[tinyram_opcode_STOREW]),
                                                         ls_prev_val_as_doubleword_variable.packed - ls_next_val_as_doubleword_variable.packed,
                                                         0),
                                 format!("{} non_store_instructions_dont_change_memory",self.annotation_prefix));

    /* specify that accepting state implies opcode = answer && arg2val == 0 */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(next_has_accepted,
                                                         1 - opcode_indicators[tinyram_opcode_ANSWER],
                                                         0),
                                 format!("{} accepting_requires_answer",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(next_has_accepted,
                                                         arg2val.packed,
                                                         0),
                                 format!("{} accepting_requires_arg2val_equal_zero",self.annotation_prefix));

    /*
       handle tapes:

       we require that:
       prev_tape1_exhausted implies next_tape1_exhausted,
       prev_tape1_exhausted implies flag to be set
       reads other than from tape 1 imply flag to be set
       flag implies result to be 0
    */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(prev_tape1_exhausted,
                                                         1 - next_tape1_exhausted,
                                                         0),
                                 format!("{} prev_tape1_exhausted_implies_next_tape1_exhausted",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(prev_tape1_exhausted,
                                                         1 - instruction_flags[tinyram_opcode_READ],
                                                         0),
                                 format!("{} prev_tape1_exhausted_implies_flag",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(opcode_indicators[tinyram_opcode_READ],
                                                         1 - arg2val.packed,
                                                         read_not1),
                                 format!("{} read_not1",self.annotation_prefix)); /* will be nonzero for read X for X != 1 */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(read_not1,
                                                         1 - instruction_flags[tinyram_opcode_READ],
                                                         0),
                                 format!("{} other_reads_imply_flag",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(instruction_flags[tinyram_opcode_READ],
                                                         instruction_results[tinyram_opcode_READ],
                                                         0),
                                 format!("{} read_flag_implies_result_0",self.annotation_prefix));
}


pub fn generate_r1cs_witness_address()
{
    /* decode instruction and arguments */
    prev_pc_addr_as_word_variable.generate_r1cs_witness_from_bits();
    for i in 0..self.pb.ap.k
    {
        prev_registers[i].generate_r1cs_witness_from_bits();
    }

    decode_arguments.generate_r1cs_witness();

    desval.generate_r1cs_witness_from_packed();
    arg1val.generate_r1cs_witness_from_packed();
    arg2val.generate_r1cs_witness_from_packed();

    /* clear out ls_addr and fill with everything of arg2val except the subaddress */
    ls_addr.fill_with_bits_of_field_element(self.pb, self.pb.val(arg2val.packed).as_ulong() >> self.pb.ap.subaddr_len());
}


pub fn generate_r1cs_witness_other(
aux_it:&tinyram_input_tape_iterator,
                                                              aux_end:& tinyram_input_tape_iterator)
{
    /* now ls_prev_val is filled with memory contents at ls_addr. we
       now ensure consistency with its doubleword representation */
    ls_prev_val_as_doubleword_variable.generate_r1cs_witness_from_bits();

    /* fill in the opcode indicators */
    let  opcode_val = opcode.get_field_element_from_bits(self.pb).as_ulong();
    for i in 0..1u64<<self.pb.ap.opcode_width()
    {
        self.pb.val(opcode_indicators[i]) = if i == opcode_val {FieldT::one()} else{FieldT::zero()};
    }

    /* execute the ALU */
    ALU.generate_r1cs_witness();

    /* fill memory_subaddress */
    memory_subaddress.bits.fill_with_bits(self.pb, pb_variable_array::<FieldT>(arg2val.bits.begin(),
                                                                               arg2val.bits.begin()  + self.pb.ap.subaddr_len()).get_bits(self.pb));
    memory_subaddress.generate_r1cs_witness_from_bits();

    /* we distinguish four cases for memory handling:
       a) load.b
       b) store.b
       c) store.w
       d) load.w or any non-memory instruction */
    let prev_doubleword = self.pb.val(ls_prev_val_as_doubleword_variable.packed).as_ulong();
    let subaddress = self.pb.val(memory_subaddress.packed).as_ulong();

    if self.pb.val(opcode_indicators[tinyram_opcode_LOADB]) == FieldT::one()
    {
        let loaded_byte = (prev_doubleword >> (8 * subaddress)) & 0xFF;
        self.pb.val(instruction_results[tinyram_opcode_LOADB]) = FieldT(loaded_byte);
        self.pb.val(memory_subcontents) = FieldT(loaded_byte);
    }
    else if self.pb.val(opcode_indicators[tinyram_opcode_STOREB]) == FieldT::one()
    {
        let stored_byte = (self.pb.val(desval.packed).as_ulong()) & 0xFF;
        self.pb.val(memory_subcontents) = FieldT(stored_byte);
    }
    else if self.pb.val(opcode_indicators[tinyram_opcode_STOREW]) == FieldT::one()
    {
        let stored_word = (self.pb.val(desval.packed).as_ulong());
        self.pb.val(memory_subcontents) = FieldT(stored_word);
    }
    else
    {
        let  access_is_word0 = (self.pb.val(*memory_subaddress.bits.rbegin()) == FieldT::zero());
        let loaded_word =  (prev_doubleword >> ( if access_is_word0 {0} else{self.pb.ap.w})) & ((1u64 << self.pb.ap.w) - 1);
        self.pb.val(instruction_results[tinyram_opcode_LOADW]) = FieldT(loaded_word); /* does not hurt even for non-memory instructions */
        self.pb.val(memory_subcontents) = FieldT(loaded_word);
    }

    memory_access_is_word.evaluate(self.pb);
    memory_access_is_byte.evaluate(self.pb);

    check_memory.generate_r1cs_witness();

    /* handle reads */
    if self.pb.val(prev_tape1_exhausted) == FieldT::one()
    {
        /* if tape was exhausted before, it will always be
           exhausted. we also need to only handle reads from tape 1,
           so we can safely set flag here */
        self.pb.val(next_tape1_exhausted) = FieldT::one();
        self.pb.val(instruction_flags[tinyram_opcode_READ]) = FieldT::one();
    }

    self.pb.val(read_not1) = self.pb.val(opcode_indicators[tinyram_opcode_READ]) * (FieldT::one() - self.pb.val(arg2val.packed));
    if self.pb.val(read_not1) != FieldT::one()
    {
        /* reading from tape other than 0 raises the flag */
        self.pb.val(instruction_flags[tinyram_opcode_READ]) = FieldT::one();
    }
    else
    {
        /* otherwise perform the actual read */
        if aux_it != aux_end
        {
            self.pb.val(instruction_results[tinyram_opcode_READ]) = FieldT(*aux_it);
            aux_it+=1;
            if aux_it == aux_end
            {
                /* tape has ended! */
                self.pb.val(next_tape1_exhausted) = FieldT::one();
            }
        }
        else
        {
            /* handled above, so nothing to do here */
        }
    }

    /* flag implies result zero */
    if self.pb.val(instruction_flags[tinyram_opcode_READ]) == FieldT::one()
    {
        self.pb.val(instruction_results[tinyram_opcode_READ]) = FieldT::zero();
    }

    /* execute consistency enforcer */
    consistency_enforcer.generate_r1cs_witness();
    next_pc_addr_as_word_variable.generate_r1cs_witness_from_packed();

    for i in 0..self.pb.ap.k
    {
        next_registers[i].generate_r1cs_witness_from_packed();
    }

    /* finally set has_accepted to 1 if both the opcode is ANSWER and arg2val is 0 */
    self.pb.val(next_has_accepted) = if (self.pb.val(opcode_indicators[tinyram_opcode_ANSWER]) == FieldT::one() &&
                                       self.pb.val(arg2val.packed) == FieldT::zero())  {FieldT::one() }else {FieldT::zero()};
}


pub fn dump() 
{
    print!("   pc = {}, flag = {}\n",
           self.pb.val(prev_pc_addr_as_word_variable.packed).as_ulong(),
           self.pb.val(prev_flag).as_ulong());
    print!("   ");

    for j in 0..self.pb.ap.k
    {
        print!("r{} = {:2} ", j, self.pb.val(prev_registers[j].packed).as_ulong());
    }
    print!("\n");

    let opcode_val = opcode.get_field_element_from_bits(self.pb).as_ulong();
    print!("   {} r{}, r{}, {}{}\n",
           tinyram_opcode_names[(opcode_val)],
           desidx.get_field_element_from_bits(self.pb).as_ulong(),
           arg1idx.get_field_element_from_bits(self.pb).as_ulong(),
           if self.pb.val(arg2_is_imm) == FieldT::one() {""} else{"r"},
           arg2idx.get_field_element_from_bits(self.pb).as_ulong());
}
}


//#endif // TINYRAM_CPU_CHECKER_TCC_
