/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM ALU gadget.

 The gadget checks the correct execution of a given TinyRAM instruction.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_GADGET_HPP_
// #define ALU_GADGET_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_arithmetic;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_control_flow;



// 
pub struct ALU_gadget{
// : public tinyram_standard_gadget<FieldT> 
components:    Vec<RcCell<tinyram_standard_gadget<FieldT> > >,


// 
opcode_indicators:    pb_variable_array<FieldT>,
pc:    word_variable_gadget<FieldT>,
desval:    word_variable_gadget<FieldT>,
arg1val:    word_variable_gadget<FieldT>,
arg2val:    word_variable_gadget<FieldT>,
flag:    pb_variable<FieldT>,
instruction_results:    pb_variable_array<FieldT>,
instruction_flags:    pb_variable_array<FieldT>,
}
impl ALU_gadget{
    pub fn new(
pb:tinyram_protoboard<FieldT>,
                       opcode_indicators:pb_variable_array<FieldT>,
                       pc:word_variable_gadget<FieldT>,
                       desval:word_variable_gadget<FieldT>,
                       arg1val:word_variable_gadget<FieldT>,
                       arg2val:word_variable_gadget<FieldT>,
                       flag:pb_variable<FieldT>,
                       instruction_results:pb_variable_array<FieldT>,
                       instruction_flags:pb_variable_array<FieldT>,
                       annotation_prefix:String) ->Self
        
    {
        components.resize(1u64<<pb.ap.opcode_width());

        /* arithmetic */
        components[tinyram_opcode_AND].reset(
            ALU_and_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_AND],
                                       instruction_flags[tinyram_opcode_AND],
                                       format!("{} AND",self.annotation_prefix)));

        components[tinyram_opcode_OR].reset(
            ALU_or_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                      instruction_results[tinyram_opcode_OR],
                                      instruction_flags[tinyram_opcode_OR],
                                      format!("{} OR",self.annotation_prefix)));

        components[tinyram_opcode_XOR].reset(
            ALU_xor_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_XOR],
                                       instruction_flags[tinyram_opcode_XOR],
                                       format!("{} XOR",self.annotation_prefix)));

        components[tinyram_opcode_NOT].reset(
            ALU_not_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_NOT],
                                       instruction_flags[tinyram_opcode_NOT],
                                       format!("{} NOT",self.annotation_prefix)));

        components[tinyram_opcode_ADD].reset(
            ALU_add_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_ADD],
                                       instruction_flags[tinyram_opcode_ADD],
                                       format!("{} ADD",self.annotation_prefix)));

        components[tinyram_opcode_SUB].reset(
            ALU_sub_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_SUB],
                                       instruction_flags[tinyram_opcode_SUB],
                                       format!("{} SUB",self.annotation_prefix)));

        components[tinyram_opcode_MOV].reset(
            ALU_mov_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_MOV],
                                       instruction_flags[tinyram_opcode_MOV],
                                       format!("{} MOV",self.annotation_prefix)));

        components[tinyram_opcode_CMOV].reset(
            ALU_cmov_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                        instruction_results[tinyram_opcode_CMOV],
                                        instruction_flags[tinyram_opcode_CMOV],
                                        format!("{} CMOV",self.annotation_prefix)));

        components[tinyram_opcode_CMPA].reset(
            ALU_cmp_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                       instruction_results[tinyram_opcode_CMPE],
                                       instruction_flags[tinyram_opcode_CMPE],
                                       instruction_results[tinyram_opcode_CMPA],
                                       instruction_flags[tinyram_opcode_CMPA],
                                       instruction_results[tinyram_opcode_CMPAE],
                                       instruction_flags[tinyram_opcode_CMPAE],
                                       format!("{} CMP_unsigned",self.annotation_prefix)));

        components[tinyram_opcode_CMPG].reset(
            ALU_cmps_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                        instruction_results[tinyram_opcode_CMPG],
                                        instruction_flags[tinyram_opcode_CMPG],
                                        instruction_results[tinyram_opcode_CMPGE],
                                        instruction_flags[tinyram_opcode_CMPGE],
                                        format!("{} CMP_signed",self.annotation_prefix)));

        components[tinyram_opcode_UMULH].reset(
            ALU_umul_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                        instruction_results[tinyram_opcode_MULL],
                                        instruction_flags[tinyram_opcode_MULL],
                                        instruction_results[tinyram_opcode_UMULH],
                                        instruction_flags[tinyram_opcode_UMULH],
                                        format!("{} MUL_unsigned",self.annotation_prefix)));

        components[tinyram_opcode_SMULH].reset(
            ALU_smul_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                        instruction_results[tinyram_opcode_SMULH],
                                        instruction_flags[tinyram_opcode_SMULH],
                                        format!("{} MUL_signed",self.annotation_prefix)));


        components[tinyram_opcode_UDIV].reset(
            ALU_divmod_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                          instruction_results[tinyram_opcode_UDIV],
                                          instruction_flags[tinyram_opcode_UDIV],
                                          instruction_results[tinyram_opcode_UMOD],
                                          instruction_flags[tinyram_opcode_UMOD],
                                          format!("{} DIV",self.annotation_prefix)));

        components[tinyram_opcode_SHR].reset(
            ALU_shr_shl_gadget::<FieldT>::new(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                           instruction_results[tinyram_opcode_SHR],
                                           instruction_flags[tinyram_opcode_SHR],
                                           instruction_results[tinyram_opcode_SHL],
                                           instruction_flags[tinyram_opcode_SHL],
                                           format!("{} SHR_SHL",self.annotation_prefix)));

        /* control flow */
        components[tinyram_opcode_JMP].reset(
            ALU_jmp_gadget::<FieldT>::new(pb, pc, arg2val, flag,
                                       instruction_results[tinyram_opcode_JMP],
                                       format!("{} JMP",self.annotation_prefix)));

        components[tinyram_opcode_CJMP].reset(
            ALU_cjmp_gadget::<FieldT>::new(pb, pc, arg2val, flag,
                                        instruction_results[tinyram_opcode_CJMP],
                                        format!("{} CJMP",self.annotation_prefix)));

        components[tinyram_opcode_CNJMP].reset(
            ALU_cnjmp_gadget::<FieldT>::new(pb, pc, arg2val, flag,
                                         instruction_results[tinyram_opcode_CNJMP],
                                         format!("{} CNJMP",self.annotation_prefix)));

    // tinyram_standard_gadget<FieldT>(&pb, annotation_prefix),
        Self{opcode_indicators,
        pc,
        desval,
        arg1val,
        arg2val,
        flag,
        instruction_results,
        instruction_flags}
    }

    // pub fn  generate_r1cs_constraints();

    // pub fn  generate_r1cs_witness();

}



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_gadget;

//#endif // ALU_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM ALU gadget.

 See alu.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_GADGET_TCC_
// #define ALU_GADGET_TCC_


impl ALU_gadget<FieldT>{

pub fn generate_r1cs_constraints()
{
    for i in 0..1u64<<self.pb.ap.opcode_width()
    {
        if components[i]
        {
            components[i].generate_r1cs_constraints();
        }
    }
}


pub fn generate_r1cs_witness()
{
    for i in 0..1u64<<self.pb.ap.opcode_width()
    {
        if components[i]
        {
            components[i].generate_r1cs_witness();
        }
    }
}

}

//#endif // ALU_GADGET_TCC_
