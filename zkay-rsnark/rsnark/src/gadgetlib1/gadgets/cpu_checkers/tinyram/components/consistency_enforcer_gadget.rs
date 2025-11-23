/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM consistency enforcer gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CONSISTENCY_ENFORCER_GADGET_HPP_
// #define CONSISTENCY_ENFORCER_GADGET_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;



// 
pub struct consistency_enforcer_gadget {
// : public tinyram_standard_gadget<FieldT>  
is_register_instruction:    pb_variable<FieldT>,
is_control_flow_instruction:    pb_variable<FieldT>,
is_stall_instruction:    pb_variable<FieldT>,

packed_desidx:    pb_variable<FieldT>,
pack_desidx:    RcCell<packing_gadget<FieldT> >,

computed_result:    pb_variable<FieldT>,
computed_flag:    pb_variable<FieldT>,
compute_computed_result:    RcCell<inner_product_gadget<FieldT> >,
compute_computed_flag:    RcCell<inner_product_gadget<FieldT> >,

pc_from_cf_or_zero:    pb_variable<FieldT>,

demux_packed_outgoing_desval:    RcCell<loose_multiplexing_gadget<FieldT> >,
// 
opcode_indicators:    pb_variable_array<FieldT>,
instruction_results:    pb_variable_array<FieldT>,
instruction_flags:    pb_variable_array<FieldT>,
desidx:    pb_variable_array<FieldT>,
packed_incoming_pc:    pb_variable<FieldT>,
packed_incoming_registers:    pb_variable_array<FieldT>,
packed_incoming_desval:    pb_variable<FieldT>,
incoming_flag:    pb_variable<FieldT>,
packed_outgoing_pc:    pb_variable<FieldT>,
packed_outgoing_registers:    pb_variable_array<FieldT>,
outgoing_flag:    pb_variable<FieldT>,
packed_outgoing_desval:    pb_variable<FieldT>,
}

//     consistency_enforcer_gadget(
// tinyram_protoboard<FieldT> &pb,
//                                 opcode_indicators:pb_variable_array<FieldT>,
//                                 instruction_results:pb_variable_array<FieldT>,
//                                 instruction_flags:pb_variable_array<FieldT>,
//                                 desidx:pb_variable_array<FieldT>,
//                                 packed_incoming_pc:pb_variable<FieldT>,
//                                 packed_incoming_registers:pb_variable_array<FieldT>,
//                                 packed_incoming_desval:pb_variable<FieldT>,
//                                 incoming_flag:pb_variable<FieldT>,
//                                 packed_outgoing_pc:pb_variable<FieldT>,
//                                 packed_outgoing_registers:pb_variable_array<FieldT>,
//                                 outgoing_flag:pb_variable<FieldT>,
//                                 annotation_prefix:String="");

//     pub fn  generate_r1cs_constraints();
//     pub fn  generate_r1cs_witness();
// };



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::consistency_enforcer_gadget;

//#endif // CONSISTENCY_ENFORCER_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM consistency enforcer gadget.

 See consistency_enforcer_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CONSISTENCY_ENFORCER_GADGET_TCC_
// #define CONSISTENCY_ENFORCER_GADGET_TCC_

impl consistency_enforcer_gadget<FieldT>{

// 
pub fn new(
pb:tinyram_protoboard<FieldT>,
                                                                 opcode_indicators:pb_variable_array<FieldT>,
                                                                 instruction_results:pb_variable_array<FieldT>,
                                                                 instruction_flags:pb_variable_array<FieldT>,
                                                                 desidx:pb_variable_array<FieldT>,
                                                                 packed_incoming_pc:pb_variable<FieldT>,
                                                                 packed_incoming_registers:pb_variable_array<FieldT>,
                                                                 packed_incoming_desval:pb_variable<FieldT>,
                                                                 incoming_flag:pb_variable<FieldT>,
                                                                 packed_outgoing_pc:pb_variable<FieldT>,
                                                                 packed_outgoing_registers:pb_variable_array<FieldT>,
                                                                 outgoing_flag:pb_variable<FieldT>,
                                                                 annotation_prefix:String) ->Self
   
{
    assert!(desidx.len() == pb.ap.reg_arg_width());

    packed_outgoing_desval.allocate(pb, format!("{} packed_outgoing_desval",self.annotation_prefix));
    is_register_instruction.allocate(pb, format!("{} is_register_instruction",self.annotation_prefix));
    is_control_flow_instruction.allocate(pb, format!("{} is_control_flow_instruction",self.annotation_prefix));
    is_stall_instruction.allocate(pb, format!("{} is_stall_instruction",self.annotation_prefix));

    packed_desidx.allocate(pb, format!("{} packed_desidx",self.annotation_prefix));
    pack_desidx.reset(packing_gadget::<FieldT>::new(pb, desidx, packed_desidx, format!("{}pack_desidx",self.annotation_prefix)));

    computed_result.allocate(pb,  format!("{} computed_result",self.annotation_prefix));
    computed_flag.allocate(pb, format!("{} computed_flag",self.annotation_prefix));

    compute_computed_result.reset(
        inner_product_gadget::<FieldT>::new(pb, opcode_indicators, instruction_results, computed_result,
                                         format!("{} compute_computed_result",self.annotation_prefix)));
    compute_computed_flag.reset(
        inner_product_gadget::<FieldT>::new(pb, opcode_indicators, instruction_flags, computed_flag,
                                         format!("{} compute_computed_flag",self.annotation_prefix)));

    pc_from_cf_or_zero.allocate(pb, format!("{} pc_from_cf_or_zero",self.annotation_prefix));

    demux_packed_outgoing_desval.reset(
        loose_multiplexing_gadget::<FieldT>::new(pb, packed_outgoing_registers, packed_desidx, packed_outgoing_desval, ONE,
                                              format!("{} demux_packed_outgoing_desval",self.annotation_prefix)));

    //  tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
    Self{opcode_indicators,
    instruction_results,
    instruction_flags,
    desidx,
    packed_incoming_pc,
    packed_incoming_registers,
    packed_incoming_desval,
    incoming_flag,
    packed_outgoing_pc,
    packed_outgoing_registers,
    outgoing_flag}

}


pub fn generate_r1cs_constraints()
{
    /* pack destination index */
    pack_desidx.generate_r1cs_constraints(false);

    /* demux result register */
    demux_packed_outgoing_desval.generate_r1cs_constraints();

    /* is_register_instruction */
    let (mut reg_a,mut reg_b,mut reg_c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    reg_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_register)
    {
        reg_b.add_term(opcode_indicators[tinyram_opcodes_register[i]], 1);
    }
    reg_c.add_term(is_register_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>::new(reg_a, reg_b, reg_c), format!("{} is_register_instruction",self.annotation_prefix));

    /* is_control_flow_instruction */
    let (mut cf_a,mut cf_b,mut cf_c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    cf_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_control_flow)
    {
        cf_b.add_term(opcode_indicators[tinyram_opcodes_control_flow[i]], 1);
    }
    cf_c.add_term(is_control_flow_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>::new(cf_a, cf_b, cf_c), format!("{} is_control_flow_instruction",self.annotation_prefix));

    /* is_stall_instruction */
    let (mut stall_a,mut stall_b,mut stall_c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    stall_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_stall)
    {
        stall_b.add_term(opcode_indicators[tinyram_opcodes_stall[i]], 1);
    }
    stall_c.add_term(is_stall_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>::new(stall_a, stall_b, stall_c), format!("{} is_stall_instruction",self.annotation_prefix));

    /* compute actual result/actual flag */
    compute_computed_result.generate_r1cs_constraints();
    compute_computed_flag.generate_r1cs_constraints();

    /*
      compute new PC address (in double words, not bytes!):

      PC' = computed_result * is_control_flow_instruction + PC * is_stall_instruction + (PC+1) * (1-is_control_flow_instruction - is_stall_instruction)
      PC' - pc_from_cf_or_zero - (1-is_control_flow_instruction - is_stall_instruction) = PC * (1 - is_control_flow_instruction)
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>::new(
            computed_result,
            is_control_flow_instruction,
            pc_from_cf_or_zero),
        format!("{} pc_from_cf_or_zero",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>::new(
            packed_incoming_pc,
            1 - is_control_flow_instruction,
            packed_outgoing_pc - pc_from_cf_or_zero - (1 - is_control_flow_instruction - is_stall_instruction)),
        format!("{} packed_outgoing_pc",self.annotation_prefix));

    /*
      enforce new flag:

      flag' = computed_flag * is_register_instruction + flag * (1-is_register_instruction)
      flag' - flag = (computed_flag - flag) * is_register_instruction
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>::new(
            vec![ computed_flag, incoming_flag * (-1) ],
            vec![ is_register_instruction ],
            vec![ outgoing_flag, incoming_flag * (-1) ]),
        format!("{} outgoing_flag",self.annotation_prefix));

    /*
      force carryover of unchanged registers

      (1-indicator) * (new-old) = 0

      In order to save constraints we "borrow" indicator variables
      from loose multiplexing gadget.
    */
    for i in 0..self.pb.ap.k
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint::<FieldT>::new(
                vec![ ONE, demux_packed_outgoing_desval.alpha[i] * (-1) ],
                vec![ packed_outgoing_registers[i], packed_incoming_registers[i] * (-1) ],
                vec![ ONE * 0 ]),
            format!("{} register_carryover_{}",self.annotation_prefix, i));
    }

    /*
      enforce correct destination register value:

      next_desval = computed_result * is_register_instruction + packed_incoming_desval * (1-is_register_instruction)
      next_desval - packed_incoming_desval = (computed_result - packed_incoming_desval) * is_register_instruction
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>::new(
            vec![ computed_result, packed_incoming_desval * (-1) ],
            vec![ is_register_instruction ],
            vec![ packed_outgoing_desval, packed_incoming_desval * (-1) ]),
        format!("{} packed_outgoing_desval",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    /* pack destination index */
    pack_desidx.generate_r1cs_witness_from_bits();

    /* is_register_instruction */
    self.pb.val(is_register_instruction) = FieldT::zero();

    for i in 0..ARRAY_SIZE(tinyram_opcodes_register)
    {
        self.pb.val(is_register_instruction) += self.pb.val(opcode_indicators[tinyram_opcodes_register[i]]);
    }

    /* is_control_flow_instruction */
    self.pb.val(is_control_flow_instruction) = FieldT::zero();

    for i in 0..ARRAY_SIZE(tinyram_opcodes_control_flow)
    {
        self.pb.val(is_control_flow_instruction) += self.pb.val(opcode_indicators[tinyram_opcodes_control_flow[i]]);
    }

    /* is_stall_instruction */
    self.pb.val(is_stall_instruction) = FieldT::zero();

    for i in 0..ARRAY_SIZE(tinyram_opcodes_stall)
    {
        self.pb.val(is_stall_instruction) += self.pb.val(opcode_indicators[tinyram_opcodes_stall[i]]);
    }

    /* compute actual result/actual flag */
    compute_computed_result.generate_r1cs_witness();
    compute_computed_flag.generate_r1cs_witness();

    /*
      compute new PC address (in double words, not bytes!):

      PC' = computed_result * is_control_flow_instruction + PC * is_stall_instruction + (PC+1) * (1-is_control_flow_instruction - is_stall_instruction)
      PC' - pc_from_cf_or_zero - (1-is_control_flow_instruction - is_stall_instruction) = PC * (1 - is_control_flow_instruction)
    */
    self.pb.val(pc_from_cf_or_zero) = self.pb.val(computed_result) * self.pb.val(is_control_flow_instruction);
    self.pb.val(packed_outgoing_pc) =
        self.pb.val(pc_from_cf_or_zero) +
        self.pb.val(packed_incoming_pc) * self.pb.val(is_stall_instruction) +
        (self.pb.val(packed_incoming_pc) + FieldT::one()) * (FieldT::one() - self.pb.val(is_control_flow_instruction) - self.pb.val(is_stall_instruction));

    /*
      enforce new flag:

      flag' = computed_flag * is_register_instruction + flag * (1-is_register_instruction)
      flag' - flag = (computed_flag - flag) * is_register_instruction
    */
    self.pb.val(outgoing_flag) =
        self.pb.val(computed_flag) * self.pb.val(is_register_instruction) +
        self.pb.val(incoming_flag) * (FieldT::one() - self.pb.val(is_register_instruction));

    /*
      update registers (changed and unchanged)

      next_desval = computed_result * is_register_instruction + packed_incoming_desval * (1-is_register_instruction)
    */
    let  changed_register_contents =
        self.pb.val(computed_result) * self.pb.val(is_register_instruction) +
        self.pb.val(packed_incoming_desval) * (FieldT::one() - self.pb.val(is_register_instruction));

    for i in 0..self.pb.ap.k
    {
        self.pb.val(packed_outgoing_registers[i]) =
            if (self.pb.val(packed_desidx).as_ulong() == i) 
            {changed_register_contents} else
            {self.pb.val(packed_incoming_registers[i])};
    }

    /* demux result register (it is important to do witness generation
       here after all registers have been set to the correct
       values!) */
    demux_packed_outgoing_desval.generate_r1cs_witness();
}

// #if 0

pub fn  test_arithmetic_consistency_enforcer_gadget()
{
    ffec::print_time("starting arithmetic_consistency_enforcer test");

    let mut  ap=tinyram_architecture_params::new(16, 16);
     let mut pb=tinyram_protoboard::<FieldT>::new(ap);

    let ( opcode_indicators, instruction_results, instruction_flags)=(pb_variable_array::<FieldT>::new(),pb_variable_array::<FieldT>::new(),pb_variable_array::<FieldT>::new());
    opcode_indicators.allocate(pb, 1u64<<ap.opcode_width(), "opcode_indicators");
    instruction_results.allocate(pb, 1u64<<ap.opcode_width(), "instruction_results");
    instruction_flags.allocate(pb, 1u64<<ap.opcode_width(), "instruction_flags");

    let mut desidx=dual_variable_gadget::<FieldT> ::new(pb, ap.reg_arg_width(), "desidx");

    let mut   incoming_pc=pb_variable::<FieldT>::new();
    incoming_pc.allocate(pb, "incoming_pc");

    let mut  packed_incoming_registers=pb_variable_array::<FieldT>::new();
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

     let mut incoming_load_flag=pb_variable::<FieldT> ::new();
    incoming_load_flag.allocate(pb, "incoming_load_flag");

    let  (mut outgoing_pc, mut outgoing_flag)=(pb_variable::<FieldT> ::new(),pb_variable::<FieldT> ::new());
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");

    let mut  packed_outgoing_registers=pb_variable_array::<FieldT>::new();
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    let mut  g=arithmetic_consistency_enforcer_gadget::new(pb, opcode_indicators, instruction_results, instruction_flags,
                                             desidx.bits, incoming_pc, packed_incoming_registers,
                                             incoming_load_flag, outgoing_pc, packed_outgoing_registers, outgoing_flag, "g");
    g.generate_r1cs_constraints();

    for i in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(instruction_results[i]) = FieldT(rand::random());
        self.pb.val(instruction_flags[i]) = FieldT(rand::random() % 2);
    }

    self.pb.val(incoming_pc) = FieldT(12345);
    self.pb.val(incoming_load_flag) = FieldT::zero();

    for i in 0..ap.k
    {
        self.pb.val(packed_incoming_registers[i]) = FieldT(1000+i);
    }

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }

    self.pb.val(opcode_indicators[tinyram_opcode_AND]) = FieldT::one();

    for i in 0..ap.k
    {
        self.pb.val(desidx.packed) = FieldT(i);
        desidx.generate_r1cs_witness_from_packed();

        g.generate_r1cs_witness();

        assert!(self.pb.val(outgoing_pc) == FieldT(12346));

        for j in 0..ap.k
        {
            assert!(self.pb.val(packed_outgoing_registers[j]) ==
                   self.pb.val( if i == j 
                                {instruction_results[tinyram_opcode_AND] }else
                                {packed_incoming_registers[j]}));
        }

        assert!(self.pb.val(outgoing_flag) == self.pb.val(instruction_flags[tinyram_opcode_AND]));
        assert!(pb.is_satisfied());
    }

    print!("arithmetic test successful\n");
    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_LOAD]) = FieldT::one();
    self.pb.val(incoming_load_flag) = FieldT::one();

    g.generate_r1cs_witness();

    self.pb.val(outgoing_pc) == FieldT(12345);
    assert!(pb.is_satisfied());

    self.pb.val(incoming_load_flag) = FieldT::zero();
    print!("test that firstload doesn't increment PC successful\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }

    self.pb.val(opcode_indicators[tinyram_opcode_JMP]) = FieldT::one();

    for i in 0..ap.k
    {
        self.pb.val(desidx.packed) = FieldT(i);
        desidx.generate_r1cs_witness_from_packed();

        g.generate_r1cs_witness();

        for j in 0..ap.k
        {
            assert!(self.pb.val(packed_outgoing_registers[j]) == self.pb.val(packed_incoming_registers[j]));
        }

        assert!(pb.is_satisfied());
    }

    print!("non-arithmetic test successful\n");

    ffec::print_time("arithmetic_consistency_enforcer tests successful");
}


pub fn  test_control_flow_consistency_enforcer_gadget()
{
    ffec::print_time("starting control_flow_consistency_enforcer test");

    let mut ap =tinyram_architecture_params::new(16, 16);
    let mut pb=tinyram_protoboard::<FieldT>::new();(ap);

    let (mut opcode_indicators,mut  instruction_results)=( pb_variable_array::<FieldT>::new(),pb_variable_array::<FieldT>::new());
    opcode_indicators.allocate(pb, 1u64<<ap.opcode_width(), "opcode_indicators");
    instruction_results.allocate(pb, 1u64<<ap.opcode_width(), "instruction_results");

     let  (mut incoming_pc, mut incoming_flag)=(pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    incoming_pc.allocate(pb, "incoming_pc");
    incoming_flag.allocate(pb, "incoming_flag");

    let mut  packed_incoming_registers=pb_variable_array::<FieldT>::new();
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

    let  (mut outgoing_pc,mut  outgoing_flag)=(pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");

   let mut  packed_outgoing_registers=pb_variable_array::<FieldT>::new();
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    let mut g=control_flow_consistency_enforcer_gadget::new(pb, opcode_indicators, instruction_results,
                                               incoming_pc, packed_incoming_registers, incoming_flag,
                                               outgoing_pc, packed_outgoing_registers, outgoing_flag, "g");
    g.generate_r1cs_constraints();

    for i in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(instruction_results[i]) = FieldT(rand::random());
    }

    self.pb.val(incoming_pc) = FieldT(12345);

    for i in 0..ap.k
    {
        self.pb.val(packed_incoming_registers[i]) = FieldT(1000+i);
    }

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_JMP]) = FieldT::one();

    for flag in 0..=1
    {
        self.pb.val(incoming_flag) = FieldT(flag);

        g.generate_r1cs_witness();

        assert!(self.pb.val(outgoing_pc) == self.pb.val(instruction_results[tinyram_opcode_JMP]));
        assert!(self.pb.val(outgoing_flag) == self.pb.val(incoming_flag));

        for j in 0..ap.k
        {
            assert!(self.pb.val(packed_outgoing_registers[j]) == self.pb.val(packed_incoming_registers[j]));
        }
        assert!(pb.is_satisfied());
    }

    ffec::print_time("control_flow_consistency_enforcer tests successful");
}


pub fn  test_special_consistency_enforcer_gadget()
{
    ffec::print_time("starting special_consistency_enforcer_gadget test");

    let mut ap =tinyram_architecture_params::new(16, 16);
    let mut pb=tinyram_protoboard::<FieldT>::new();(ap);

    let mut opcode_indicators=pb_variable_array::<FieldT>::new();
    opcode_indicators.allocate(pb, 1u64<<ap.opcode_width(), "opcode_indicators");

    let  (mut incoming_pc, mut incoming_flag,mut  incoming_load_flag)=(pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    incoming_pc.allocate(pb, "incoming_pc");
    incoming_flag.allocate(pb, "incoming_flag");
    incoming_load_flag.allocate(pb, "incoming_load_flag");

    let mut packed_incoming_registers=pb_variable_array::<FieldT>::new();
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

    let   (mut outgoing_pc, mut outgoing_flag,mut  outgoing_load_flag)=(pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");
    outgoing_load_flag.allocate(pb, "outgoing_load_flag");

    let mut packed_outgoing_registers=pb_variable_array::<FieldT>::new();
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    let mut  g=special_consistency_enforcer_gadget::new(pb, opcode_indicators,
                                          incoming_pc, packed_incoming_registers, incoming_flag, incoming_load_flag,
                                          outgoing_pc, packed_outgoing_registers, outgoing_flag, outgoing_load_flag, "g");
    g.generate_r1cs_constraints();

    self.pb.val(incoming_pc) = FieldT(12345);
    for i in 0..ap.k
    {
        self.pb.val(packed_incoming_registers[i]) = FieldT(1000+i);
    }
    self.pb.val(incoming_flag) = FieldT::zero();
    self.pb.val(incoming_load_flag) = FieldT::zero();

    /* test that accept stalls */
    print!("test that ACCEPT stalls\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_ACCEPT]) = FieldT::one();

    g.generate_r1cs_witness();

    assert!(self.pb.val(outgoing_flag) == self.pb.val(incoming_flag));
    for j in 0..ap.k
    {
        assert!(self.pb.val(packed_outgoing_registers[j]) == self.pb.val(packed_incoming_registers[j]));
    }

    assert!(self.pb.val(outgoing_pc) == self.pb.val(incoming_pc));
    assert!(pb.is_satisfied());

    print!("test that ACCEPT preserves registers\n");
    self.pb.val(packed_outgoing_registers[0]) = FieldT::zero();
    assert!(!pb.is_satisfied());

    /* test that other special instructions (e.g. STORE) don't and also preserve registers */
    print!("test that others (e.g. STORE) don't stall\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_STORE]) = FieldT::one();

    g.generate_r1cs_witness();

    assert!(self.pb.val(outgoing_flag) == self.pb.val(incoming_flag));
    for j in 0..ap.k
    {
        assert!(self.pb.val(packed_outgoing_registers[j]) == self.pb.val(packed_incoming_registers[j]));
    }

    assert!(self.pb.val(outgoing_pc) == self.pb.val(incoming_pc) + FieldT::one());
    assert!(pb.is_satisfied());

    print!("test that STORE preserves registers\n");
    self.pb.val(packed_outgoing_registers[0]) = FieldT::zero();
    assert!(!pb.is_satisfied());

    print!("test that STORE can't have load_flag\n");
    g.generate_r1cs_witness();
    self.pb.val(incoming_load_flag) = FieldT::one();

    assert!(!pb.is_satisfied());

    /* test that load can modify outgoing register and sets load_flag */
    print!("test that LOAD sets load_flag\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_LOAD]) = FieldT::one();
    self.pb.val(incoming_load_flag) = FieldT::zero();

    g.generate_r1cs_witness();

    assert!(self.pb.val(outgoing_load_flag) == FieldT::one());
    assert!(pb.is_satisfied());

    print!("test that LOAD can modify registers\n");
    self.pb.val(packed_outgoing_registers[0]) = FieldT::zero();
    assert!(pb.is_satisfied());

    /* test that postload clears load_flag */
    print!("test that postload clears load_flag\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_LOAD]) = FieldT::one();
    self.pb.val(incoming_load_flag) = FieldT::one();

    g.generate_r1cs_witness();

    assert!(self.pb.val(outgoing_load_flag) == FieldT::zero());
    assert!(pb.is_satisfied());

    /* test non-special instructions */
    print!("test non-special instructions\n");

    for t in 0..1u64<<ap.opcode_width()
    {
        self.pb.val(opcode_indicators[t]) = FieldT::zero();
    }
    self.pb.val(opcode_indicators[tinyram_opcode_JMP]) = FieldT::one();
    self.pb.val(incoming_load_flag) = FieldT::zero();
    g.generate_r1cs_witness();

    assert!(pb.is_satisfied());

    print!("test that non-special can't have load_flag\n");
    g.generate_r1cs_witness();
    self.pb.val(incoming_load_flag) = FieldT::one();

    assert!(!pb.is_satisfied());

    ffec::print_time("special_consistency_enforcer_gadget tests successful");
}
//#endif

}

//#endif // CONSISTENCY_ENFORCER_GADGET_TCC_
