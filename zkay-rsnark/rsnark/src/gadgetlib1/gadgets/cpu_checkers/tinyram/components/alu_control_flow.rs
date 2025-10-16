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

use crate::gadgetlib1::gadgets/basic_gadgets;
use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/tinyram_protoboard;
use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/word_variable_gadget;



/* control flow gadgets */
template<typename FieldT>
class ALU_control_flow_gadget : public tinyram_standard_gadget<FieldT> {
public:
    const word_variable_gadget<FieldT> pc;
    const word_variable_gadget<FieldT> argval2;
    const pb_variable<FieldT> flag;
    const pb_variable<FieldT> result;

    ALU_control_flow_gadget(tinyram_protoboard<FieldT> &pb,
                            const word_variable_gadget<FieldT> &pc,
                            const word_variable_gadget<FieldT> &argval2,
                            const pb_variable<FieldT> &flag,
                            const pb_variable<FieldT> &result,
                            const std::string &annotation_prefix="") :
        tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
        pc(pc),
        argval2(argval2),
        flag(flag),
        result(result) {};
};

template<typename FieldT>
class ALU_jmp_gadget : public ALU_control_flow_gadget<FieldT> {
public:
    ALU_jmp_gadget(tinyram_protoboard<FieldT> &pb,
                   const word_variable_gadget<FieldT> &pc,
                   const word_variable_gadget<FieldT> &argval2,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const std::string &annotation_prefix="") :
        ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix) {}

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_jmp_gadget();

template<typename FieldT>
class ALU_cjmp_gadget : public ALU_control_flow_gadget<FieldT> {
public:
    ALU_cjmp_gadget(tinyram_protoboard<FieldT> &pb,
                    const word_variable_gadget<FieldT> &pc,
                    const word_variable_gadget<FieldT> &argval2,
                    const pb_variable<FieldT> &flag,
                    const pb_variable<FieldT> &result,
                    const std::string &annotation_prefix="") :
        ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix) {}

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_cjmp_gadget();

template<typename FieldT>
class ALU_cnjmp_gadget : public ALU_control_flow_gadget<FieldT> {
public:
    ALU_cnjmp_gadget(tinyram_protoboard<FieldT> &pb,
                     const word_variable_gadget<FieldT> &pc,
                     const word_variable_gadget<FieldT> &argval2,
                     const pb_variable<FieldT> &flag,
                     const pb_variable<FieldT> &result,
                     const std::string &annotation_prefix="") :
        ALU_control_flow_gadget<FieldT>(pb, pc, argval2, flag, result, annotation_prefix) {}

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_cnjmp_gadget();



use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/alu_control_flow;

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
template<typename FieldT>
void ALU_jmp_gadget<FieldT>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.argval2.packed },
            { self.result }),
        FMT(self.annotation_prefix, " jmp_result"));
}

template<typename FieldT>
void ALU_jmp_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(self.result) = self.pb.val(self.argval2.packed);
}

template<typename FieldT>
void test_ALU_jmp_gadget()
{
    ffec::print_time("starting jmp test");

    tinyram_architecture_params ap(16, 16);
    tinyram_program P; P.instructions = generate_tinyram_prelude(ap);
    tinyram_protoboard<FieldT> pb(ap, P.size(), 0, 10);

    word_variable_gadget<FieldT> pc(pb, "pc"), argval2(pb, "argval2");
    pb_variable<FieldT> flag, result;

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

    ALU_jmp_gadget<FieldT> jmp(pb, pc, argval2, flag, result, "jmp");
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
template<typename FieldT>
void ALU_cjmp_gadget<FieldT>::generate_r1cs_constraints()
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
        r1cs_constraint<FieldT>(
            self.flag,
            pb_packing_sum<FieldT>(pb_variable_array<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end())) - self.pc.packed - 1,
            self.result - self.pc.packed - 1),
        FMT(self.annotation_prefix, " cjmp_result"));
}

template<typename FieldT>
void ALU_cjmp_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  FieldT(self.pb.val(self.argval2.packed).as_ulong() >> self.pb.ap.subaddr_len()) :
                                  self.pb.val(self.pc.packed) + FieldT::one());
}

template<typename FieldT>
void test_ALU_cjmp_gadget()
{
    // TODO: update
    ffec::print_time("starting cjmp test");

    tinyram_architecture_params ap(16, 16);
    tinyram_program P; P.instructions = generate_tinyram_prelude(ap);
    tinyram_protoboard<FieldT> pb(ap, P.size(), 0, 10);

    word_variable_gadget<FieldT> pc(pb, "pc"), argval2(pb, "argval2");
    pb_variable<FieldT> flag, result;

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

    ALU_cjmp_gadget<FieldT> cjmp(pb, pc, argval2, flag, result, "cjmp");
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
template<typename FieldT>
void ALU_cnjmp_gadget<FieldT>::generate_r1cs_constraints()
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
        r1cs_constraint<FieldT>(
            self.flag,
            self.pc.packed + 1 - pb_packing_sum<FieldT>(pb_variable_array<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end())),
            self.result - pb_packing_sum<FieldT>(pb_variable_array<FieldT>(self.argval2.bits.begin() + self.pb.ap.subaddr_len(), self.argval2.bits.end()))),
        FMT(self.annotation_prefix, " cnjmp_result"));
}

template<typename FieldT>
void ALU_cnjmp_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  self.pb.val(self.pc.packed) + FieldT::one() :
                                  FieldT(self.pb.val(self.argval2.packed).as_ulong() >> self.pb.ap.subaddr_len()));
}

template<typename FieldT>
void test_ALU_cnjmp_gadget()
{
    // TODO: update
    ffec::print_time("starting cnjmp test");

    tinyram_architecture_params ap(16, 16);
    tinyram_program P; P.instructions = generate_tinyram_prelude(ap);
    tinyram_protoboard<FieldT> pb(ap, P.size(), 0, 10);

    word_variable_gadget<FieldT> pc(pb, "pc"), argval2(pb, "argval2");
    pb_variable<FieldT> flag, result;

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(pb, "flag");
    result.allocate(pb, "result");

    ALU_cnjmp_gadget<FieldT> cnjmp(pb, pc, argval2, flag, result, "cjmp");
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
