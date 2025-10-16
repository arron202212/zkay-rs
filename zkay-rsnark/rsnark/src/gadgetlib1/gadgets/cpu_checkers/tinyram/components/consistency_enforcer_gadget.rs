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

use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/tinyram_protoboard;



template<typename FieldT>
class consistency_enforcer_gadget : public tinyram_standard_gadget<FieldT>  {
private:
    pb_variable<FieldT> is_register_instruction;
    pb_variable<FieldT> is_control_flow_instruction;
    pb_variable<FieldT> is_stall_instruction;

    pb_variable<FieldT> packed_desidx;
    std::shared_ptr<packing_gadget<FieldT> > pack_desidx;

    pb_variable<FieldT> computed_result;
    pb_variable<FieldT> computed_flag;
    std::shared_ptr<inner_product_gadget<FieldT> > compute_computed_result;
    std::shared_ptr<inner_product_gadget<FieldT> > compute_computed_flag;

    pb_variable<FieldT> pc_from_cf_or_zero;

    std::shared_ptr<loose_multiplexing_gadget<FieldT> > demux_packed_outgoing_desval;
public:
    pb_variable_array<FieldT> opcode_indicators;
    pb_variable_array<FieldT> instruction_results;
    pb_variable_array<FieldT> instruction_flags;
    pb_variable_array<FieldT> desidx;
    pb_variable<FieldT> packed_incoming_pc;
    pb_variable_array<FieldT> packed_incoming_registers;
    pb_variable<FieldT> packed_incoming_desval;
    pb_variable<FieldT> incoming_flag;
    pb_variable<FieldT> packed_outgoing_pc;
    pb_variable_array<FieldT> packed_outgoing_registers;
    pb_variable<FieldT> outgoing_flag;
    pb_variable<FieldT> packed_outgoing_desval;

    consistency_enforcer_gadget(tinyram_protoboard<FieldT> &pb,
                                const pb_variable_array<FieldT> &opcode_indicators,
                                const pb_variable_array<FieldT> &instruction_results,
                                const pb_variable_array<FieldT> &instruction_flags,
                                const pb_variable_array<FieldT> &desidx,
                                const pb_variable<FieldT> &packed_incoming_pc,
                                const pb_variable_array<FieldT> &packed_incoming_registers,
                                const pb_variable<FieldT> &packed_incoming_desval,
                                const pb_variable<FieldT> &incoming_flag,
                                const pb_variable<FieldT> &packed_outgoing_pc,
                                const pb_variable_array<FieldT> &packed_outgoing_registers,
                                const pb_variable<FieldT> &outgoing_flag,
                                const std::string &annotation_prefix="");

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/consistency_enforcer_gadget;

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



template<typename FieldT>
consistency_enforcer_gadget<FieldT>::consistency_enforcer_gadget(tinyram_protoboard<FieldT> &pb,
                                                                 const pb_variable_array<FieldT> &opcode_indicators,
                                                                 const pb_variable_array<FieldT> &instruction_results,
                                                                 const pb_variable_array<FieldT> &instruction_flags,
                                                                 const pb_variable_array<FieldT> &desidx,
                                                                 const pb_variable<FieldT> &packed_incoming_pc,
                                                                 const pb_variable_array<FieldT> &packed_incoming_registers,
                                                                 const pb_variable<FieldT> &packed_incoming_desval,
                                                                 const pb_variable<FieldT> &incoming_flag,
                                                                 const pb_variable<FieldT> &packed_outgoing_pc,
                                                                 const pb_variable_array<FieldT> &packed_outgoing_registers,
                                                                 const pb_variable<FieldT> &outgoing_flag,
                                                                 const std::string &annotation_prefix) :
    tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
    opcode_indicators(opcode_indicators),
    instruction_results(instruction_results),
    instruction_flags(instruction_flags),
    desidx(desidx),
    packed_incoming_pc(packed_incoming_pc),
    packed_incoming_registers(packed_incoming_registers),
    packed_incoming_desval(packed_incoming_desval),
    incoming_flag(incoming_flag),
    packed_outgoing_pc(packed_outgoing_pc),
    packed_outgoing_registers(packed_outgoing_registers),
    outgoing_flag(outgoing_flag)
{
    assert!(desidx.size() == pb.ap.reg_arg_width());

    packed_outgoing_desval.allocate(pb, FMT(self.annotation_prefix, " packed_outgoing_desval"));
    is_register_instruction.allocate(pb, FMT(self.annotation_prefix, " is_register_instruction"));
    is_control_flow_instruction.allocate(pb, FMT(self.annotation_prefix, " is_control_flow_instruction"));
    is_stall_instruction.allocate(pb, FMT(self.annotation_prefix, " is_stall_instruction"));

    packed_desidx.allocate(pb, FMT(self.annotation_prefix, " packed_desidx"));
    pack_desidx.reset(new packing_gadget<FieldT>(pb, desidx, packed_desidx, FMT(self.annotation_prefix, "pack_desidx")));

    computed_result.allocate(pb,  FMT(self.annotation_prefix, " computed_result"));
    computed_flag.allocate(pb, FMT(self.annotation_prefix, " computed_flag"));

    compute_computed_result.reset(
        new inner_product_gadget<FieldT>(pb, opcode_indicators, instruction_results, computed_result,
                                         FMT(self.annotation_prefix, " compute_computed_result")));
    compute_computed_flag.reset(
        new inner_product_gadget<FieldT>(pb, opcode_indicators, instruction_flags, computed_flag,
                                         FMT(self.annotation_prefix, " compute_computed_flag")));

    pc_from_cf_or_zero.allocate(pb, FMT(self.annotation_prefix, " pc_from_cf_or_zero"));

    demux_packed_outgoing_desval.reset(
        new loose_multiplexing_gadget<FieldT>(pb, packed_outgoing_registers, packed_desidx, packed_outgoing_desval, ONE,
                                              FMT(self.annotation_prefix, " demux_packed_outgoing_desval")));

}

template<typename FieldT>
void consistency_enforcer_gadget<FieldT>::generate_r1cs_constraints()
{
    /* pack destination index */
    pack_desidx->generate_r1cs_constraints(false);

    /* demux result register */
    demux_packed_outgoing_desval->generate_r1cs_constraints();

    /* is_register_instruction */
    linear_combination<FieldT> reg_a, reg_b, reg_c;
    reg_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_register)
    {
        reg_b.add_term(opcode_indicators[tinyram_opcodes_register[i]], 1);
    }
    reg_c.add_term(is_register_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(reg_a, reg_b, reg_c), FMT(self.annotation_prefix, " is_register_instruction"));

    /* is_control_flow_instruction */
    linear_combination<FieldT> cf_a, cf_b, cf_c;
    cf_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_control_flow)
    {
        cf_b.add_term(opcode_indicators[tinyram_opcodes_control_flow[i]], 1);
    }
    cf_c.add_term(is_control_flow_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(cf_a, cf_b, cf_c), FMT(self.annotation_prefix, " is_control_flow_instruction"));

    /* is_stall_instruction */
    linear_combination<FieldT> stall_a, stall_b, stall_c;
    stall_a.add_term(ONE, 1);
    for i in 0..ARRAY_SIZE(tinyram_opcodes_stall)
    {
        stall_b.add_term(opcode_indicators[tinyram_opcodes_stall[i]], 1);
    }
    stall_c.add_term(is_stall_instruction, 1);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(stall_a, stall_b, stall_c), FMT(self.annotation_prefix, " is_stall_instruction"));

    /* compute actual result/actual flag */
    compute_computed_result->generate_r1cs_constraints();
    compute_computed_flag->generate_r1cs_constraints();

    /*
      compute new PC address (in double words, not bytes!):

      PC' = computed_result * is_control_flow_instruction + PC * is_stall_instruction + (PC+1) * (1-is_control_flow_instruction - is_stall_instruction)
      PC' - pc_from_cf_or_zero - (1-is_control_flow_instruction - is_stall_instruction) = PC * (1 - is_control_flow_instruction)
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            computed_result,
            is_control_flow_instruction,
            pc_from_cf_or_zero),
        FMT(self.annotation_prefix, " pc_from_cf_or_zero"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            packed_incoming_pc,
            1 - is_control_flow_instruction,
            packed_outgoing_pc - pc_from_cf_or_zero - (1 - is_control_flow_instruction - is_stall_instruction)),
        FMT(self.annotation_prefix, " packed_outgoing_pc"));

    /*
      enforce new flag:

      flag' = computed_flag * is_register_instruction + flag * (1-is_register_instruction)
      flag' - flag = (computed_flag - flag) * is_register_instruction
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { computed_flag, incoming_flag * (-1) },
            { is_register_instruction },
            { outgoing_flag, incoming_flag * (-1) }),
        FMT(self.annotation_prefix, " outgoing_flag"));

    /*
      force carryover of unchanged registers

      (1-indicator) * (new-old) = 0

      In order to save constraints we "borrow" indicator variables
      from loose multiplexing gadget.
    */
    for i in 0..self.pb.ap.k
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { ONE, demux_packed_outgoing_desval->alpha[i] * (-1) },
                { packed_outgoing_registers[i], packed_incoming_registers[i] * (-1) },
                { ONE * 0 }),
            FMT(self.annotation_prefix, " register_carryover_{}", i));
    }

    /*
      enforce correct destination register value:

      next_desval = computed_result * is_register_instruction + packed_incoming_desval * (1-is_register_instruction)
      next_desval - packed_incoming_desval = (computed_result - packed_incoming_desval) * is_register_instruction
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { computed_result, packed_incoming_desval * (-1) },
            { is_register_instruction },
            { packed_outgoing_desval, packed_incoming_desval * (-1) }),
        FMT(self.annotation_prefix, " packed_outgoing_desval"));
}

template<typename FieldT>
void consistency_enforcer_gadget<FieldT>::generate_r1cs_witness()
{
    /* pack destination index */
    pack_desidx->generate_r1cs_witness_from_bits();

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
    compute_computed_result->generate_r1cs_witness();
    compute_computed_flag->generate_r1cs_witness();

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
    FieldT changed_register_contents =
        self.pb.val(computed_result) * self.pb.val(is_register_instruction) +
        self.pb.val(packed_incoming_desval) * (FieldT::one() - self.pb.val(is_register_instruction));

    for i in 0..self.pb.ap.k
    {
        self.pb.val(packed_outgoing_registers[i]) =
            (self.pb.val(packed_desidx).as_ulong() == i) ?
            changed_register_contents :
            self.pb.val(packed_incoming_registers[i]);
    }

    /* demux result register (it is important to do witness generation
       here after all registers have been set to the correct
       values!) */
    demux_packed_outgoing_desval->generate_r1cs_witness();
}

#if 0
template<typename FieldT>
void test_arithmetic_consistency_enforcer_gadget()
{
    ffec::print_time("starting arithmetic_consistency_enforcer test");

    tinyram_architecture_params ap(16, 16);
    tinyram_protoboard<FieldT> pb(ap);

    pb_variable_array<FieldT> opcode_indicators, instruction_results, instruction_flags;
    opcode_indicators.allocate(pb, 1ul<<ap.opcode_width(), "opcode_indicators");
    instruction_results.allocate(pb, 1ul<<ap.opcode_width(), "instruction_results");
    instruction_flags.allocate(pb, 1ul<<ap.opcode_width(), "instruction_flags");

    dual_variable_gadget<FieldT> desidx(pb, ap.reg_arg_width(), "desidx");

    pb_variable<FieldT>  incoming_pc;
    incoming_pc.allocate(pb, "incoming_pc");

    pb_variable_array<FieldT> packed_incoming_registers;
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

    pb_variable<FieldT>  incoming_load_flag;
    incoming_load_flag.allocate(pb, "incoming_load_flag");

    pb_variable<FieldT>  outgoing_pc, outgoing_flag;
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");

    pb_variable_array<FieldT> packed_outgoing_registers;
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    arithmetic_consistency_enforcer_gadget g(pb, opcode_indicators, instruction_results, instruction_flags,
                                             desidx.bits, incoming_pc, packed_incoming_registers,
                                             incoming_load_flag, outgoing_pc, packed_outgoing_registers, outgoing_flag, "g");
    g.generate_r1cs_constraints();

    for i in 0..1ul<<ap.opcode_width()
    {
        self.pb.val(instruction_results[i]) = FieldT(std::rand());
        self.pb.val(instruction_flags[i]) = FieldT(std::rand() % 2);
    }

    self.pb.val(incoming_pc) = FieldT(12345);
    self.pb.val(incoming_load_flag) = FieldT::zero();

    for i in 0..ap.k
    {
        self.pb.val(packed_incoming_registers[i]) = FieldT(1000+i);
    }

    for t in 0..1ul<<ap.opcode_width()
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
                   self.pb.val(i == j ?
                                instruction_results[tinyram_opcode_AND] :
                                packed_incoming_registers[j]));
        }

        assert!(self.pb.val(outgoing_flag) == self.pb.val(instruction_flags[tinyram_opcode_AND]));
        assert!(pb.is_satisfied());
    }

    print!("arithmetic test successful\n");
    for t in 0..1ul<<ap.opcode_width()
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

    for t in 0..1ul<<ap.opcode_width()
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

template<typename FieldT>
void test_control_flow_consistency_enforcer_gadget()
{
    ffec::print_time("starting control_flow_consistency_enforcer test");

    tinyram_architecture_params ap(16, 16);
    tinyram_protoboard<FieldT> pb(ap);

    pb_variable_array<FieldT> opcode_indicators, instruction_results;
    opcode_indicators.allocate(pb, 1ul<<ap.opcode_width(), "opcode_indicators");
    instruction_results.allocate(pb, 1ul<<ap.opcode_width(), "instruction_results");

    pb_variable<FieldT>  incoming_pc, incoming_flag;
    incoming_pc.allocate(pb, "incoming_pc");
    incoming_flag.allocate(pb, "incoming_flag");

    pb_variable_array<FieldT> packed_incoming_registers;
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

    pb_variable<FieldT>  outgoing_pc, outgoing_flag;
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");

    pb_variable_array<FieldT> packed_outgoing_registers;
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    control_flow_consistency_enforcer_gadget g(pb, opcode_indicators, instruction_results,
                                               incoming_pc, packed_incoming_registers, incoming_flag,
                                               outgoing_pc, packed_outgoing_registers, outgoing_flag, "g");
    g.generate_r1cs_constraints();

    for i in 0..1ul<<ap.opcode_width()
    {
        self.pb.val(instruction_results[i]) = FieldT(std::rand());
    }

    self.pb.val(incoming_pc) = FieldT(12345);

    for i in 0..ap.k
    {
        self.pb.val(packed_incoming_registers[i]) = FieldT(1000+i);
    }

    for t in 0..1ul<<ap.opcode_width()
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

template<typename FieldT>
void test_special_consistency_enforcer_gadget()
{
    ffec::print_time("starting special_consistency_enforcer_gadget test");

    tinyram_architecture_params ap(16, 16);
    tinyram_protoboard<FieldT> pb(ap);

    pb_variable_array<FieldT> opcode_indicators;
    opcode_indicators.allocate(pb, 1ul<<ap.opcode_width(), "opcode_indicators");

    pb_variable<FieldT>  incoming_pc, incoming_flag, incoming_load_flag;
    incoming_pc.allocate(pb, "incoming_pc");
    incoming_flag.allocate(pb, "incoming_flag");
    incoming_load_flag.allocate(pb, "incoming_load_flag");

    pb_variable_array<FieldT> packed_incoming_registers;
    packed_incoming_registers.allocate(pb, ap.k, "packed_incoming_registers");

    pb_variable<FieldT>  outgoing_pc, outgoing_flag, outgoing_load_flag;
    outgoing_pc.allocate(pb, "outgoing_pc");
    outgoing_flag.allocate(pb, "outgoing_flag");
    outgoing_load_flag.allocate(pb, "outgoing_load_flag");

    pb_variable_array<FieldT> packed_outgoing_registers;
    packed_outgoing_registers.allocate(pb, ap.k, "packed_outgoing_registers");

    special_consistency_enforcer_gadget g(pb, opcode_indicators,
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

    for t in 0..1ul<<ap.opcode_width()
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

    for t in 0..1ul<<ap.opcode_width()
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

    for t in 0..1ul<<ap.opcode_width()
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

    for t in 0..1ul<<ap.opcode_width()
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

    for t in 0..1ul<<ap.opcode_width()
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



//#endif // CONSISTENCY_ENFORCER_GADGET_TCC_
