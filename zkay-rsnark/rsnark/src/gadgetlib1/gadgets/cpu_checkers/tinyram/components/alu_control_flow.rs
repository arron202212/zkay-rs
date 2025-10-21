/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM ALU control-flow gadgets.

 These gadget check the correct execution of control-flow TinyRAM instructions.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_CONTROL_FLOW_HPP_
// #define ALU_CONTROL_FLOW_HPP_

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget;



/* control flow gadgets */
// 
pub struct  ALU_control_flow_gadget<FieldT>  {
// public:: public tinyram_standard_gadget<FieldT>
    pc:word_variable_gadget<FieldT>,
    argval2:word_variable_gadget<FieldT>,
    flag:pb_variable<FieldT>,
    result:pb_variable<FieldT>,
}
impl ALU_control_flow_gadget<FieldT>  {
    pub fn new(pb:tinyram_protoboard<FieldT>,
                            pc:word_variable_gadget<FieldT>,
                            argval2:word_variable_gadget<FieldT>,
                            flag:pb_variable<FieldT>,
                            result:pb_variable<FieldT>,
                            annotation_prefix:std::string) ->Self
         {
// tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
        Self{pc(pc),
        argval2(argval2),
        flag(flag),
        result(result)}
}
}

// 
pub struct ALU_jmp_gadget {
}
impl ALU_jmp_gadget {
    pub fn new(pb:tinyram_protoboard<FieldT>,
                   pc:word_variable_gadget<FieldT>,
                   argval2:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   annotation_prefix:std::string) ->Self
        {
// ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix) 
    Self{}
}

   
}

// 
// pub fn test_ALU_jmp_gadget();

// 
pub struct ALU_cjmp_gadget {
}
impl ALU_cjmp_gadget {
    pub fn new(pb:tinyram_protoboard<FieldT>,
                    pc:word_variable_gadget<FieldT>,
                    argval2:word_variable_gadget<FieldT>,
                    flag:pb_variable<FieldT>,
                    result:pb_variable<FieldT>,
                    annotation_prefix:std::string) ->Self
         {
// ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix)
    Self{}
}

   
}

// 
// pub fn test_ALU_cjmp_gadget();

// 
pub struct ALU_cnjmp_gadget {
}
// public:
impl ALU_cnjmp_gadget {
    pub fn new(pb:tinyram_protoboard<FieldT>,
                     pc:word_variable_gadget<FieldT>,
                     argval2:word_variable_gadget<FieldT>,
                     flag:pb_variable<FieldT>,
                     result:pb_variable<FieldT>,
                     annotation_prefix:std::string) ->Self
      {
//   ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix) 
    Self{}
}

   
}

// 
// pub fn test_ALU_cnjmp_gadget();



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_control_flow;

//#endif // ALU_CONTROL_FLOW_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM ALU control-flow gadgets.

 See alu_control_flow.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_CONTROL_FLOW_TCC_
// #define ALU_CONTROL_FLOW_TCC_

use ffec::common::profiling;



/* jmp */
impl ALU_jmp_gadget<FieldT>{

pub fn generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>(
            { ONE },
            { self.argval2.packed },
            { self.result }),
        format!("{} jmp_result",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(self.result) = self.pb.val(self.argval2.packed);
}
}

pub fn test_ALU_jmp_gadget()
{
    ffec::print_time("starting jmp test");

    let mut ap=tinyram_architecture_params ::new(16, 16);
    let mut  P=tinyram_program::new(); 
    P.instructions = generate_tinyram_prelude(ap);
    let mut  pb=tinyram_protoboard::<FieldT>::new(ap, P.size(), 0, 10);

    let mut pc=word_variable_gadget::<FieldT>::new(pb, "pc"), argval2(pb, "argval2");
   let (mut  flag,mut  result)=( pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

     let mut jmp=ALU_jmp_gadget::<FieldT>::new(pb, pc, argval2, flag, result, "jmp");
    jmp.generate_r1cs_constraints();

    pb.val(argval2.packed) = FieldT(123);
    argval2.generate_r1cs_witness_from_packed();

    jmp.generate_r1cs_witness();

    assert!(pb.val(result) == FieldT(123));
    assert!(pb.is_satisfied());
    ffec::print_time("positive jmp test successful");

    pb.val(result) = FieldT(1);
    assert!(!pb.is_satisfied());
    ffec::print_time("negative jmp test successful");
}

/* cjmp */
impl ALU_cjmp_gadget<FieldT>{

pub fn generate_r1cs_constraints()
{
    /*
      flag1 * argval2 + (1-flag1) * (pc1 + 1) = cjmp_result
      flag1 * (argval2 - pc1 - 1) = cjmp_result - pc1 - 1

      Note that instruction fetch semantics require program counter to
      be aligned to the double word by rounding down, and pc_addr in
      the outer reduction is expressed as a double word address. To
      achieve this we just discard the first ap.subaddr_len() bits of
      the byte address of the PC.
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>(
            self.flag,
            pb_packing_sum::<FieldT>(pb_variable_array::<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end())) - self.pc.packed - 1,
            self.result - self.pc.packed - 1),
        format!("{} cjmp_result",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  FieldT(self.pb.val(self.argval2.packed).as_ulong() >> self.pb.ap.subaddr_len()) :
                                  self.pb.val(self.pc.packed) + FieldT::one());
}
}

pub fn test_ALU_cjmp_gadget()
{
    // TODO: update
    ffec::print_time("starting cjmp test");

    let mut ap=tinyram_architecture_params::new(16, 16);
    let mut P=tinyram_program::new(); P.instructions = generate_tinyram_prelude(ap);
     let mut pb=tinyram_protoboard::<FieldT>::new(ap, P.size(), 0, 10);

    let mut  pc=word_variable_gadget::<FieldT>::new(pb, "pc");
    let mut argval2=word_variable_gadget::<FieldT>::new(pb, "argval2");
    let (mut  flag,mut  result)=( pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

    let mut  cjmp=ALU_cjmp_gadget::<FieldT>::new(pb, pc, argval2, flag, result, "cjmp");
    cjmp.generate_r1cs_constraints();

    pb.val(argval2.packed) = FieldT(123);
    argval2.generate_r1cs_witness_from_packed();
    pb.val(pc.packed) = FieldT(456);
    pc.generate_r1cs_witness_from_packed();

    pb.val(flag) = FieldT(1);
    cjmp.generate_r1cs_witness();

    assert!(pb.val(result) == FieldT(123));
    assert!(pb.is_satisfied());
    ffec::print_time("positive cjmp test successful");

    pb.val(flag) = FieldT(0);
    assert!(!pb.is_satisfied());
    ffec::print_time("negative cjmp test successful");

    pb.val(flag) = FieldT(0);
    cjmp.generate_r1cs_witness();

    assert!(pb.val(result) == FieldT(456+2*ap.w/8));
    assert!(pb.is_satisfied());
    ffec::print_time("positive cjmp test successful");

    pb.val(flag) = FieldT(1);
    assert!(!pb.is_satisfied());
    ffec::print_time("negative cjmp test successful");
}

/* cnjmp */
impl ALU_cnjmp_gadget<FieldT>{

pub fn generate_r1cs_constraints()
{
    /*
      flag1 * (pc1 + inc) + (1-flag1) * argval2 = cnjmp_result
      flag1 * (pc1 + inc - argval2) = cnjmp_result - argval2

      Note that instruction fetch semantics require program counter to
      be aligned to the double word by rounding down, and pc_addr in
      the outer reduction is expressed as a double word address. To
      achieve this we just discard the first ap.subaddr_len() bits of
      the byte address of the PC.
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>(
            self.flag,
            self.pc.packed + 1 - pb_packing_sum::<FieldT>(pb_variable_array::<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end())),
            self.result - pb_packing_sum::<FieldT>(pb_variable_array::<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end()))),
        format!("{} cnjmp_result",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  self.pb.val(self.pc.packed) + FieldT::one() :
                                  FieldT(self.pb.val(self.argval2.packed).as_ulong() >> self.pb.ap.subaddr_len()));
}
}

pub fn test_ALU_cnjmp_gadget()
{
    // TODO: update
    ffec::print_time("starting cnjmp test");

    let mut  ap=tinyram_architecture_params::new(16, 16);
    let mut  P=tinyram_program::new();
     P.instructions = generate_tinyram_prelude(ap);
   let mut  pb= tinyram_protoboard::<FieldT>::new(ap, P.size(), 0, 10);

    let mut  pc=word_variable_gadget::<FieldT>::new(pb, "pc");
    let mut  argval2=word_variable_gadget::<FieldT>::new(pb, "argval2");
    let (mut  flag,mut  result)=( pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

    let mut cnjmp= ALU_cnjmp_gadget::<FieldT>::new(pb, pc, argval2, flag, result, "cjmp");
    cnjmp.generate_r1cs_constraints();

    pb.val(argval2.packed) = FieldT(123);
    argval2.generate_r1cs_witness_from_packed();
    pb.val(pc.packed) = FieldT(456);
    pc.generate_r1cs_witness_from_packed();

    pb.val(flag) = FieldT(0);
    cnjmp.generate_r1cs_witness();

    assert!(pb.val(result) == FieldT(123));
    assert!(pb.is_satisfied());
    ffec::print_time("positive cnjmp test successful");

    pb.val(flag) = FieldT(1);
    assert!(!pb.is_satisfied());
    ffec::print_time("negative cnjmp test successful");

    pb.val(flag) = FieldT(1);
    cnjmp.generate_r1cs_witness();

    assert!(pb.val(result) == FieldT(456 + (2*pb.ap.w/8)));
    assert!(pb.is_satisfied());
    ffec::print_time("positive cnjmp test successful");

    pb.val(flag) = FieldT(0);
    assert!(!pb.is_satisfied());
    ffec::print_time("negative cnjmp test successful");
}



//#endif // ALU_CONTROL_FLOW_TCC_
