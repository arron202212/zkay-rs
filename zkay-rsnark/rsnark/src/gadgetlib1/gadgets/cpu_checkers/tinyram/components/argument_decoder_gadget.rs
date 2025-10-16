/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM argument decoder gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ARGUMENT_DECODER_GADGET_HPP_
// #define ARGUMENT_DECODER_GADGET_HPP_

use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/tinyram_protoboard;



template<typename FieldT>
class argument_decoder_gadget : public tinyram_standard_gadget<FieldT> {
private:
    pb_variable<FieldT> packed_desidx;
    pb_variable<FieldT> packed_arg1idx;
    pb_variable<FieldT> packed_arg2idx;

    std::shared_ptr<packing_gadget<FieldT> > pack_desidx;
    std::shared_ptr<packing_gadget<FieldT> > pack_arg1idx;
    std::shared_ptr<packing_gadget<FieldT> > pack_arg2idx;

    pb_variable<FieldT> arg2_demux_result;
    pb_variable<FieldT> arg2_demux_success;

    std::shared_ptr<loose_multiplexing_gadget<FieldT> > demux_des;
    std::shared_ptr<loose_multiplexing_gadget<FieldT> > demux_arg1;
    std::shared_ptr<loose_multiplexing_gadget<FieldT> > demux_arg2;
public:
    pb_variable<FieldT> arg2_is_imm;
    pb_variable_array<FieldT> desidx;
    pb_variable_array<FieldT> arg1idx;
    pb_variable_array<FieldT> arg2idx;
    pb_variable_array<FieldT> packed_registers;
    pb_variable<FieldT> packed_desval;
    pb_variable<FieldT> packed_arg1val;
    pb_variable<FieldT> packed_arg2val;

    argument_decoder_gadget(tinyram_protoboard<FieldT> &pb,
                            const pb_variable<FieldT> &arg2_is_imm,
                            const pb_variable_array<FieldT> &desidx,
                            const pb_variable_array<FieldT> &arg1idx,
                            const pb_variable_array<FieldT> &arg2idx,
                            const pb_variable_array<FieldT> &packed_registers,
                            const pb_variable<FieldT> &packed_desval,
                            const pb_variable<FieldT> &packed_arg1val,
                            const pb_variable<FieldT> &packed_arg2val,
                            const std::string &annotation_prefix="");

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_argument_decoder_gadget();



use crate::gadgetlib1::gadgets/cpu_checkers/tinyram/components/argument_decoder_gadget;

//#endif // ARGUMENT_DECODER_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM argument decoder gadget.

 See argument_decoder_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ARGUMENT_DECODER_GADGET_TCC_
// #define ARGUMENT_DECODER_GADGET_TCC_



template<typename FieldT>
argument_decoder_gadget<FieldT>::argument_decoder_gadget(tinyram_protoboard<FieldT> &pb,
                                                         const pb_variable<FieldT> &arg2_is_imm,
                                                         const pb_variable_array<FieldT> &desidx,
                                                         const pb_variable_array<FieldT> &arg1idx,
                                                         const pb_variable_array<FieldT> &arg2idx,
                                                         const pb_variable_array<FieldT> &packed_registers,
                                                         const pb_variable<FieldT> &packed_desval,
                                                         const pb_variable<FieldT> &packed_arg1val,
                                                         const pb_variable<FieldT> &packed_arg2val,
                                                         const std::string &annotation_prefix) :
    tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
    arg2_is_imm(arg2_is_imm),
    desidx(desidx),
    arg1idx(arg1idx),
    arg2idx(arg2idx),
    packed_registers(packed_registers),
    packed_desval(packed_desval),
    packed_arg1val(packed_arg1val),
    packed_arg2val(packed_arg2val)
{
    assert!(desidx.size() == pb.ap.reg_arg_width());
    assert!(arg1idx.size() == pb.ap.reg_arg_width());
    assert!(arg2idx.size() == pb.ap.reg_arg_or_imm_width());

    /* decode accordingly */
    packed_desidx.allocate(pb, FMT(self.annotation_prefix, " packed_desidx"));
    packed_arg1idx.allocate(pb, FMT(self.annotation_prefix, " packed_arg1idx"));
    packed_arg2idx.allocate(pb, FMT(self.annotation_prefix, " packed_arg2idx"));

    pack_desidx.reset(new packing_gadget<FieldT>(pb, desidx, packed_desidx, FMT(self.annotation_prefix, "pack_desidx")));
    pack_arg1idx.reset(new packing_gadget<FieldT>(pb, arg1idx, packed_arg1idx, FMT(self.annotation_prefix, "pack_arg1idx")));
    pack_arg2idx.reset(new packing_gadget<FieldT>(pb, arg2idx, packed_arg2idx, FMT(self.annotation_prefix, "pack_arg2idx")));

    arg2_demux_result.allocate(pb, FMT(self.annotation_prefix, " arg2_demux_result"));
    arg2_demux_success.allocate(pb, FMT(self.annotation_prefix, " arg2_demux_success"));

    demux_des.reset(
        new loose_multiplexing_gadget<FieldT>(pb, packed_registers, packed_desidx, packed_desval, ONE,
                                              FMT(self.annotation_prefix, " demux_des")));
    demux_arg1.reset(
        new loose_multiplexing_gadget<FieldT>(pb, packed_registers, packed_arg1idx, packed_arg1val, ONE,
                                              FMT(self.annotation_prefix, " demux_arg1")));
    demux_arg2.reset(
        new loose_multiplexing_gadget<FieldT>(pb, packed_registers, packed_arg2idx, arg2_demux_result, arg2_demux_success,
                                              FMT(self.annotation_prefix, " demux_arg2")));
}

template<typename FieldT>
void argument_decoder_gadget<FieldT>::generate_r1cs_constraints()
{
    /* pack */
    pack_desidx->generate_r1cs_constraints(true);
    pack_arg1idx->generate_r1cs_constraints(true);
    pack_arg2idx->generate_r1cs_constraints(true);

    /* demux */
    demux_des->generate_r1cs_constraints();
    demux_arg1->generate_r1cs_constraints();
    demux_arg2->generate_r1cs_constraints();

    /* enforce correct handling of arg2val */

    /* it is false that arg2 is reg and demux failed:
       (1 - arg2_is_imm) * (1 - arg2_demux_success) = 0 */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>({ ONE, arg2_is_imm * (-1) },
            { ONE, arg2_demux_success * (-1) },
            { ONE * 0 }),
        FMT(self.annotation_prefix, " ensure_correc_demux"));

    /*
      arg2val = arg2_is_imm * packed_arg2idx +
      (1 - arg2_is_imm) * arg2_demux_result

      arg2val - arg2_demux_result = arg2_is_imm * (packed_arg2idx - arg2_demux_result)
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>({ arg2_is_imm },
            { packed_arg2idx, arg2_demux_result * (-1) },
            { packed_arg2val, arg2_demux_result * (-1) }),
        FMT(self.annotation_prefix, " compute_arg2val"));
}

template<typename FieldT>
void argument_decoder_gadget<FieldT>::generate_r1cs_witness()
{
    /* pack */
    pack_desidx->generate_r1cs_witness_from_bits();
    pack_arg1idx->generate_r1cs_witness_from_bits();
    pack_arg2idx->generate_r1cs_witness_from_bits();

    /* demux */
    demux_des->generate_r1cs_witness();
    demux_arg1->generate_r1cs_witness();
    demux_arg2->generate_r1cs_witness();

    /* handle arg2val */
    self.pb.val(packed_arg2val) =
        (self.pb.val(arg2_is_imm) == FieldT::one() ?
         self.pb.val(packed_arg2idx) : self.pb.val(arg2_demux_result));
}

template<typename FieldT>
void test_argument_decoder_gadget()
{
    ffec::print_time("starting argument_decoder_gadget test");

    tinyram_architecture_params ap(16, 16);
    tinyram_program P; P.instructions = generate_tinyram_prelude(ap);
    tinyram_protoboard<FieldT> pb(ap, P.size(), 0, 10);

    pb_variable_array<FieldT> packed_registers;
    packed_registers.allocate(pb, ap.k, "packed_registers");

    pb_variable<FieldT>  arg2_is_imm;
    arg2_is_imm.allocate(pb, "arg_is_imm");

    dual_variable_gadget<FieldT> desidx(pb, ap.reg_arg_width(), "desidx");
    dual_variable_gadget<FieldT> arg1idx(pb, ap.reg_arg_width(), "arg1idx");
    dual_variable_gadget<FieldT> arg2idx(pb, ap.reg_arg_or_imm_width(), "arg2idx");

    pb_variable<FieldT>  packed_desval, packed_arg1val, packed_arg2val;
    packed_desval.allocate(pb, "packed_desval");
    packed_arg1val.allocate(pb, "packed_arg1val");
    packed_arg2val.allocate(pb, "packed_arg2val");

    argument_decoder_gadget<FieldT> g(pb, packed_registers, arg2_is_imm,
                                      desidx.bits, arg1idx.bits, arg2idx.bits,
                                      packed_desval, packed_arg1val, packed_arg2val, "g");

    g.generate_r1cs_constraints();
    for i in 0..ap.k
    {
        pb.val(packed_registers[i]) = FieldT(1000+i);
    }

    pb.val(desidx.packed) = FieldT(2);
    pb.val(arg1idx.packed) = FieldT(5);
    pb.val(arg2idx.packed) = FieldT(7);
    pb.val(arg2_is_imm) = FieldT::zero();

    desidx.generate_r1cs_witness_from_packed();
    arg1idx.generate_r1cs_witness_from_packed();
    arg2idx.generate_r1cs_witness_from_packed();

    g.generate_r1cs_witness();

    assert!(pb.val(packed_desval) == FieldT(1002));
    assert!(pb.val(packed_arg1val) == FieldT(1005));
    assert!(pb.val(packed_arg2val) == FieldT(1007));
    assert!(pb.is_satisfied());
    print!("positive test (get reg) successful\n");

    pb.val(arg2_is_imm) = FieldT::one();
    g.generate_r1cs_witness();

    assert!(pb.val(packed_desval) == FieldT(1002));
    assert!(pb.val(packed_arg1val) == FieldT(1005));
    assert!(pb.val(packed_arg2val) == FieldT(7));
    assert!(pb.is_satisfied());
    print!("positive test (get imm) successful\n");

    ffec::print_time("argument_decoder_gadget tests successful");
}



//#endif // ARGUMENT_DECODER_GADGET_TCC_
