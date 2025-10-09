/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM ALU arithmetic gadgets.

 These gadget check the correct execution of arithmetic TinyRAM instructions.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_ARITHMETIC_HPP_
// #define ALU_ARITHMETIC_HPP_
use  <memory>

use libsnark/gadgetlib1/gadgets/basic_gadgets;
use libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/tinyram_protoboard;
use libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/word_variable_gadget;



/* arithmetic gadgets */
template<typename FieldT>
class ALU_arithmetic_gadget : public tinyram_standard_gadget<FieldT> {
public:
    const pb_variable_array<FieldT> opcode_indicators;
    const word_variable_gadget<FieldT> desval;
    const word_variable_gadget<FieldT> arg1val;
    const word_variable_gadget<FieldT> arg2val;
    const pb_variable<FieldT> flag;
    const pb_variable<FieldT> result;
    const pb_variable<FieldT> result_flag;

    ALU_arithmetic_gadget(tinyram_protoboard<FieldT> &pb,
                          const pb_variable_array<FieldT> &opcode_indicators,
                          const word_variable_gadget<FieldT> &desval,
                          const word_variable_gadget<FieldT> &arg1val,
                          const word_variable_gadget<FieldT> &arg2val,
                          const pb_variable<FieldT> &flag,
                          const pb_variable<FieldT> &result,
                          const pb_variable<FieldT> &result_flag,
                          const std::string &annotation_prefix="") :
        tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
        opcode_indicators(opcode_indicators),
        desval(desval),
        arg1val(arg1val),
        arg2val(arg2val),
        flag(flag),
        result(result),
        result_flag(result_flag) {}
};

template<typename FieldT>
class ALU_and_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable_array<FieldT> res_word;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;
    std::shared_ptr<disjunction_gadget<FieldT> > not_all_zeros;
    pb_variable<FieldT> not_all_zeros_result;
public:
    ALU_and_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_bit"));
        not_all_zeros_result.allocate(pb, FMT(self.annotation_prefix, " not_all_zeros_result"));

        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
        not_all_zeros.reset(
            new disjunction_gadget<FieldT>(pb, res_word, not_all_zeros_result,
                                           FMT(self.annotation_prefix, "not_all_zeros")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_and_gadget(const size_t w);

template<typename FieldT>
class ALU_or_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable_array<FieldT> res_word;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;
    std::shared_ptr<disjunction_gadget<FieldT> > not_all_zeros;
    pb_variable<FieldT> not_all_zeros_result;
public:
    ALU_or_gadget(tinyram_protoboard<FieldT> &pb,
                  const pb_variable_array<FieldT> &opcode_indicators,
                  const word_variable_gadget<FieldT> &desval,
                  const word_variable_gadget<FieldT> &arg1val,
                  const word_variable_gadget<FieldT> &arg2val,
                  const pb_variable<FieldT> &flag,
                  const pb_variable<FieldT> &result,
                  const pb_variable<FieldT> &result_flag,
                  const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_bit"));
        not_all_zeros_result.allocate(pb, FMT(self.annotation_prefix, " not_all_zeros_result"));

        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
        not_all_zeros.reset(
            new disjunction_gadget<FieldT>(pb, res_word, not_all_zeros_result,
                                           FMT(self.annotation_prefix, "not_all_zeros")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_or_gadget(const size_t w);

template<typename FieldT>
class ALU_xor_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable_array<FieldT> res_word;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;
    std::shared_ptr<disjunction_gadget<FieldT> > not_all_zeros;
    pb_variable<FieldT> not_all_zeros_result;
public:
    ALU_xor_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_bit"));
        not_all_zeros_result.allocate(pb, FMT(self.annotation_prefix, " not_all_zeros_result"));

        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
        not_all_zeros.reset(
            new disjunction_gadget<FieldT>(pb, res_word, not_all_zeros_result,
                                           FMT(self.annotation_prefix, "not_all_zeros")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_xor_gadget(const size_t w);

template<typename FieldT>
class ALU_not_gadget : public ALU_arithmetic_gadget<FieldT> {
/* we do bitwise not, because we need to compute flag */
private:
    pb_variable_array<FieldT> res_word;
    std::shared_ptr<packing_gadget<FieldT> > pack_result;
    std::shared_ptr<disjunction_gadget<FieldT> > not_all_zeros;
    pb_variable<FieldT> not_all_zeros_result;
public:
    ALU_not_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_bit"));
        not_all_zeros_result.allocate(pb, FMT(self.annotation_prefix, " not_all_zeros_result"));

        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
        not_all_zeros.reset(
            new disjunction_gadget<FieldT>(pb, res_word, not_all_zeros_result,
                                           FMT(self.annotation_prefix, "not_all_zeros")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_not_gadget(const size_t w);

template<typename FieldT>
class ALU_add_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable<FieldT> addition_result;
    pb_variable_array<FieldT> res_word;
    pb_variable_array<FieldT> res_word_and_flag;
    std::shared_ptr<packing_gadget<FieldT> > unpack_addition, pack_result;
public:
    ALU_add_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        addition_result.allocate(pb, FMT(self.annotation_prefix, " addition_result"));
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_word"));

        res_word_and_flag = res_word;
        res_word_and_flag.push(result_flag);

        unpack_addition.reset(
            new packing_gadget<FieldT>(pb, res_word_and_flag, addition_result,
                                       FMT(self.annotation_prefix, " unpack_addition")));
        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

void test_ALU_add_gadget(const size_t w);

template<typename FieldT>
class ALU_sub_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable<FieldT> intermediate_result;
    pb_variable<FieldT> negated_flag;
    pb_variable_array<FieldT> res_word;
    pb_variable_array<FieldT> res_word_and_negated_flag;

    std::shared_ptr<packing_gadget<FieldT> > unpack_intermediate, pack_result;
public:
    ALU_sub_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
        intermediate_result.allocate(pb, FMT(self.annotation_prefix, " intermediate_result"));
        negated_flag.allocate(pb, FMT(self.annotation_prefix, " negated_flag"));
        res_word.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " res_word"));

        res_word_and_negated_flag = res_word;
        res_word_and_negated_flag.push(negated_flag);

        unpack_intermediate.reset(
            new packing_gadget<FieldT>(pb, res_word_and_negated_flag, intermediate_result,
                                       FMT(self.annotation_prefix, " unpack_intermediate")));
        pack_result.reset(
            new packing_gadget<FieldT>(pb, res_word, result,
                                       FMT(self.annotation_prefix, " pack_result")));
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

void test_ALU_sub_gadget(const size_t w);

template<typename FieldT>
class ALU_mov_gadget : public ALU_arithmetic_gadget<FieldT> {
public:
    ALU_mov_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &result,
                   const pb_variable<FieldT> &result_flag,
                   const std::string &annotation_prefix="") :
        ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix) {}

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_mov_gadget(const size_t w);

template<typename FieldT>
class ALU_cmov_gadget : public ALU_arithmetic_gadget<FieldT> {
public:
    ALU_cmov_gadget(tinyram_protoboard<FieldT> &pb,
                    const pb_variable_array<FieldT> &opcode_indicators,
                    const word_variable_gadget<FieldT> &desval,
                    const word_variable_gadget<FieldT> &arg1val,
                    const word_variable_gadget<FieldT> &arg2val,
                    const pb_variable<FieldT> &flag,
                    const pb_variable<FieldT> &result,
                    const pb_variable<FieldT> &result_flag,
                    const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    {
    }

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_cmov_gadget(const size_t w);

template<typename FieldT>
class ALU_cmp_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    comparison_gadget<FieldT> comparator;
public:
    const pb_variable<FieldT> cmpe_result;
    const pb_variable<FieldT> cmpe_result_flag;
    const pb_variable<FieldT> cmpa_result;
    const pb_variable<FieldT> cmpa_result_flag;
    const pb_variable<FieldT> cmpae_result;
    const pb_variable<FieldT> cmpae_result_flag;

    ALU_cmp_gadget(tinyram_protoboard<FieldT> &pb,
                   const pb_variable_array<FieldT> &opcode_indicators,
                   const word_variable_gadget<FieldT> &desval,
                   const word_variable_gadget<FieldT> &arg1val,
                   const word_variable_gadget<FieldT> &arg2val,
                   const pb_variable<FieldT> &flag,
                   const pb_variable<FieldT> &cmpe_result,
                   const pb_variable<FieldT> &cmpe_result_flag,
                   const pb_variable<FieldT> &cmpa_result,
                   const pb_variable<FieldT> &cmpa_result_flag,
                   const pb_variable<FieldT> &cmpae_result,
                   const pb_variable<FieldT> &cmpae_result_flag,
                   const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, cmpa_result, cmpa_result_flag, annotation_prefix),
        comparator(pb, pb.ap.w, arg2val.packed, arg1val.packed, cmpa_result_flag, cmpae_result_flag,
                   FMT(self.annotation_prefix, " comparator")),
        cmpe_result(cmpe_result), cmpe_result_flag(cmpe_result_flag),
        cmpa_result(cmpa_result), cmpa_result_flag(cmpa_result_flag),
        cmpae_result(cmpae_result), cmpae_result_flag(cmpae_result_flag) {}

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_cmpe_gadget(const size_t w);

template<typename FieldT>
void test_ALU_cmpa_gadget(const size_t w);

template<typename FieldT>
void test_ALU_cmpae_gadget(const size_t w);

template<typename FieldT>
class ALU_cmps_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable<FieldT> negated_arg1val_sign;
    pb_variable<FieldT> negated_arg2val_sign;
    pb_variable_array<FieldT> modified_arg1;
    pb_variable_array<FieldT> modified_arg2;
    pb_variable<FieldT> packed_modified_arg1;
    pb_variable<FieldT> packed_modified_arg2;
    std::shared_ptr<packing_gadget<FieldT> > pack_modified_arg1;
    std::shared_ptr<packing_gadget<FieldT> > pack_modified_arg2;
    std::shared_ptr<comparison_gadget<FieldT> > comparator;
public:
    const pb_variable<FieldT> cmpg_result;
    const pb_variable<FieldT> cmpg_result_flag;
    const pb_variable<FieldT> cmpge_result;
    const pb_variable<FieldT> cmpge_result_flag;

    ALU_cmps_gadget(tinyram_protoboard<FieldT> &pb,
                    const pb_variable_array<FieldT> &opcode_indicators,
                    const word_variable_gadget<FieldT> &desval,
                    const word_variable_gadget<FieldT> &arg1val,
                    const word_variable_gadget<FieldT> &arg2val,
                    const pb_variable<FieldT> &flag,
                    const pb_variable<FieldT> &cmpg_result,
                    const pb_variable<FieldT> &cmpg_result_flag,
                    const pb_variable<FieldT> &cmpge_result,
                    const pb_variable<FieldT> &cmpge_result_flag,
                    const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, cmpg_result, cmpg_result_flag, annotation_prefix),
        cmpg_result(cmpg_result), cmpg_result_flag(cmpg_result_flag),
        cmpge_result(cmpge_result), cmpge_result_flag(cmpge_result_flag)
    {
        negated_arg1val_sign.allocate(pb, FMT(self.annotation_prefix, " negated_arg1val_sign"));
        negated_arg2val_sign.allocate(pb, FMT(self.annotation_prefix, " negated_arg2val_sign"));

        modified_arg1 = pb_variable_array<FieldT>(arg1val.bits.begin(), --arg1val.bits.end());
        modified_arg1.push(negated_arg1val_sign);

        modified_arg2 = pb_variable_array<FieldT>(arg2val.bits.begin(), --arg2val.bits.end());
        modified_arg2.push(negated_arg2val_sign);

        packed_modified_arg1.allocate(pb, FMT(self.annotation_prefix, " packed_modified_arg1"));
        packed_modified_arg2.allocate(pb, FMT(self.annotation_prefix, " packed_modified_arg2"));

        pack_modified_arg1.reset(new packing_gadget<FieldT>(pb, modified_arg1, packed_modified_arg1,
                                                            FMT(self.annotation_prefix, " pack_modified_arg1")));
        pack_modified_arg2.reset(new packing_gadget<FieldT>(pb, modified_arg2, packed_modified_arg2,
                                                            FMT(self.annotation_prefix, " pack_modified_arg2")));

        comparator.reset(new comparison_gadget<FieldT>(pb, pb.ap.w,
                                                       packed_modified_arg2, packed_modified_arg1,
                                                       cmpg_result_flag, cmpge_result_flag,
                                                       FMT(self.annotation_prefix, " comparator")));
    }
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_cmpg_gadget(const size_t w);

template<typename FieldT>
void test_ALU_cmpge_gadget(const size_t w);

template<typename FieldT>
class ALU_umul_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    dual_variable_gadget<FieldT> mul_result;
    pb_variable_array<FieldT> mull_bits;
    pb_variable_array<FieldT> umulh_bits;
    pb_variable<FieldT> result_flag;
    std::shared_ptr<packing_gadget<FieldT> > pack_mull_result;
    std::shared_ptr<packing_gadget<FieldT> > pack_umulh_result;
    std::shared_ptr<disjunction_gadget<FieldT> > compute_flag;
public:
    const pb_variable<FieldT> mull_result;
    const pb_variable<FieldT> mull_flag;
    const pb_variable<FieldT> umulh_result;
    const pb_variable<FieldT> umulh_flag;

    ALU_umul_gadget(tinyram_protoboard<FieldT> &pb,
                    const pb_variable_array<FieldT> &opcode_indicators,
                    const word_variable_gadget<FieldT> &desval,
                    const word_variable_gadget<FieldT> &arg1val,
                    const word_variable_gadget<FieldT> &arg2val,
                    const pb_variable<FieldT> &flag,
                    const pb_variable<FieldT> &mull_result,
                    const pb_variable<FieldT> &mull_flag,
                    const pb_variable<FieldT> &umulh_result,
                    const pb_variable<FieldT> &umulh_flag,
                    const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, mull_result, mull_flag, annotation_prefix),
        mul_result(pb, 2*pb.ap.w, FMT(self.annotation_prefix, " mul_result")),
        mull_result(mull_result), mull_flag(mull_flag), umulh_result(umulh_result), umulh_flag(umulh_flag)
    {
        mull_bits.insert(mull_bits.end(), mul_result.bits.begin(), mul_result.bits.begin()+pb.ap.w);
        umulh_bits.insert(umulh_bits.end(), mul_result.bits.begin()+pb.ap.w, mul_result.bits.begin()+2*pb.ap.w);

        pack_mull_result.reset(new packing_gadget<FieldT>(pb, mull_bits, mull_result, FMT(self.annotation_prefix, " pack_mull_result")));
        pack_umulh_result.reset(new packing_gadget<FieldT>(pb, umulh_bits, umulh_result, FMT(self.annotation_prefix, " pack_umulh_result")));

        result_flag.allocate(pb, FMT(self.annotation_prefix, " result_flag"));
        compute_flag.reset(new disjunction_gadget<FieldT>(pb, umulh_bits, result_flag, FMT(self.annotation_prefix, " compute_flag")));
    }
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_mull_gadget(const size_t w);

template<typename FieldT>
void test_ALU_umulh_gadget(const size_t w);

template<typename FieldT>
class ALU_smul_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    dual_variable_gadget<FieldT> mul_result;
    pb_variable_array<FieldT> smulh_bits;

    pb_variable<FieldT> top;
    std::shared_ptr<packing_gadget<FieldT> > pack_top;

    pb_variable<FieldT> is_top_empty, is_top_empty_aux;
    pb_variable<FieldT> is_top_full, is_top_full_aux;

    pb_variable<FieldT> result_flag;
    std::shared_ptr<packing_gadget<FieldT> > pack_smulh_result;
public:
    const pb_variable<FieldT> smulh_result;
    const pb_variable<FieldT> smulh_flag;

    ALU_smul_gadget(tinyram_protoboard<FieldT> &pb,
                    const pb_variable_array<FieldT> &opcode_indicators,
                    const word_variable_gadget<FieldT> &desval,
                    const word_variable_gadget<FieldT> &arg1val,
                    const word_variable_gadget<FieldT> &arg2val,
                    const pb_variable<FieldT> &flag,
                    const pb_variable<FieldT> &smulh_result,
                    const pb_variable<FieldT> &smulh_flag,
                    const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, smulh_result, smulh_flag, annotation_prefix),
        mul_result(pb, 2*pb.ap.w+1, FMT(self.annotation_prefix, " mul_result")), /* see witness map for explanation for 2w+1 */
        smulh_result(smulh_result), smulh_flag(smulh_flag)
    {
        smulh_bits.insert(smulh_bits.end(), mul_result.bits.begin()+pb.ap.w, mul_result.bits.begin()+2*pb.ap.w);

        pack_smulh_result.reset(new packing_gadget<FieldT>(pb, smulh_bits, smulh_result, FMT(self.annotation_prefix, " pack_smulh_result")));

        top.allocate(pb, FMT(self.annotation_prefix, " top"));
        pack_top.reset(new packing_gadget<FieldT>(pb, pb_variable_array<FieldT>(mul_result.bits.begin() + pb.ap.w-1, mul_result.bits.begin() + 2*pb.ap.w), top,
                                                  FMT(self.annotation_prefix, " pack_top")));

        is_top_empty.allocate(pb, FMT(self.annotation_prefix, " is_top_empty"));
        is_top_empty_aux.allocate(pb, FMT(self.annotation_prefix, " is_top_empty_aux"));

        is_top_full.allocate(pb, FMT(self.annotation_prefix, " is_top_full"));
        is_top_full_aux.allocate(pb, FMT(self.annotation_prefix, " is_top_full_aux"));

        result_flag.allocate(pb, FMT(self.annotation_prefix, " result_flag"));
    }
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_smulh_gadget(const size_t w);

template<typename FieldT>
class ALU_divmod_gadget : public ALU_arithmetic_gadget<FieldT> {
/*
  <<<<<<< Updated upstream
  B * q + r = A_aux = A * B_nonzero
  q * (1-B_nonzero) = 0
  A<B_gadget<FieldT>(r < B, less=B_nonzero, leq=ONE)
  =======
  B * q + r = A

  r <= B
  >>>>>>> Stashed changes
*/
private:
    pb_variable<FieldT> B_inv;
    pb_variable<FieldT> B_nonzero;
    pb_variable<FieldT> A_aux;
    std::shared_ptr<comparison_gadget<FieldT> > r_less_B;
public:
    const pb_variable<FieldT> udiv_result;
    const pb_variable<FieldT> udiv_flag;
    const pb_variable<FieldT> umod_result;
    const pb_variable<FieldT> umod_flag;

    ALU_divmod_gadget(tinyram_protoboard<FieldT> &pb,
                      const pb_variable_array<FieldT> &opcode_indicators,
                      const word_variable_gadget<FieldT> &desval,
                      const word_variable_gadget<FieldT> &arg1val,
                      const word_variable_gadget<FieldT> &arg2val,
                      const pb_variable<FieldT> &flag,
                      const pb_variable<FieldT> &udiv_result,
                      const pb_variable<FieldT> &udiv_flag,
                      const pb_variable<FieldT> &umod_result,
                      const pb_variable<FieldT> &umod_flag,
                      const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, udiv_result, udiv_flag, annotation_prefix),
        udiv_result(udiv_result), udiv_flag(udiv_flag), umod_result(umod_result), umod_flag(umod_flag)
    {
        B_inv.allocate(pb, FMT(self.annotation_prefix, " B_inv"));
        B_nonzero.allocate(pb, FMT(self.annotation_prefix, " B_nonzer"));
        A_aux.allocate(pb, FMT(self.annotation_prefix, " A_aux"));
        r_less_B.reset(new comparison_gadget<FieldT>(pb, pb.ap.w, umod_result, arg2val.packed,
                                                     B_nonzero, ONE, FMT(self.annotation_prefix, " r_less_B")));
    }
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_udiv_gadget(const size_t w);

template<typename FieldT>
void test_ALU_umod_gadget(const size_t w);

template<typename FieldT>
class ALU_shr_shl_gadget : public ALU_arithmetic_gadget<FieldT> {
private:
    pb_variable<FieldT> reversed_input;
    std::shared_ptr<packing_gadget<FieldT> > pack_reversed_input;

    pb_variable_array<FieldT> barrel_right_internal;
    std::vector<pb_variable_array<FieldT> > shifted_out_bits;

    pb_variable<FieldT> is_oversize_shift;
    std::shared_ptr<disjunction_gadget<FieldT> > check_oversize_shift;
    pb_variable<FieldT> result;

    pb_variable_array<FieldT> result_bits;
    std::shared_ptr<packing_gadget<FieldT> > unpack_result;
    pb_variable<FieldT> reversed_result;
    std::shared_ptr<packing_gadget<FieldT> > pack_reversed_result;
public:
    pb_variable<FieldT> shr_result;
    pb_variable<FieldT> shr_flag;
    pb_variable<FieldT> shl_result;
    pb_variable<FieldT> shl_flag;

    size_t logw;

    ALU_shr_shl_gadget(tinyram_protoboard<FieldT> &pb,
                       const pb_variable_array<FieldT> &opcode_indicators,
                       const word_variable_gadget<FieldT> &desval,
                       const word_variable_gadget<FieldT> &arg1val,
                       const word_variable_gadget<FieldT> &arg2val,
                       const pb_variable<FieldT> &flag,
                       const pb_variable<FieldT> &shr_result,
                       const pb_variable<FieldT> &shr_flag,
                       const pb_variable<FieldT> &shl_result,
                       const pb_variable<FieldT> &shl_flag,
                       const std::string &annotation_prefix="") :
    ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, shr_result, shr_flag, annotation_prefix),
        shr_result(shr_result), shr_flag(shr_flag), shl_result(shl_result), shl_flag(shl_flag)
    {
        logw = ffec::log2(pb.ap.w);

        reversed_input.allocate(pb, FMT(self.annotation_prefix, " reversed_input"));
        pack_reversed_input.reset(
            new packing_gadget<FieldT>(pb, pb_variable_array<FieldT>(arg1val.bits.rbegin(), arg1val.bits.rend()),
                                       reversed_input,
                                       FMT(self.annotation_prefix, " pack_reversed_input")));

        barrel_right_internal.allocate(pb, logw+1, FMT(self.annotation_prefix, " barrel_right_internal"));

        shifted_out_bits.resize(logw);
        for (size_t i = 0; i < logw; ++i)
        {
            shifted_out_bits[i].allocate(pb, 1ul<<i, FMT(self.annotation_prefix, " shifted_out_bits_{}", i));
        }

        is_oversize_shift.allocate(pb, FMT(self.annotation_prefix, " is_oversize_shift"));
        check_oversize_shift.reset(
            new disjunction_gadget<FieldT>(pb,
                                           pb_variable_array<FieldT>(arg2val.bits.begin()+logw, arg2val.bits.end()),
                                           is_oversize_shift,
                                           FMT(self.annotation_prefix, " check_oversize_shift")));
        result.allocate(pb, FMT(self.annotation_prefix, " result"));

        result_bits.allocate(pb, pb.ap.w, FMT(self.annotation_prefix, " result_bits"));
        unpack_result.reset(
            new packing_gadget<FieldT>(pb, result_bits, result, //barrel_right_internal[logw],
                                       FMT(self.annotation_prefix, " unpack_result")));

        reversed_result.allocate(pb, FMT(self.annotation_prefix, " reversed_result"));
        pack_reversed_result.reset(
            new packing_gadget<FieldT>(pb, pb_variable_array<FieldT>(result_bits.rbegin(), result_bits.rend()),
                                       reversed_result,
                                       FMT(self.annotation_prefix, " pack_reversed_result")));
    }
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

template<typename FieldT>
void test_ALU_shr_gadget(const size_t w);

template<typename FieldT>
void test_ALU_shl_gadget(const size_t w);



use libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/alu_arithmetic;

//#endif // ALU_ARITHMETIC_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM ALU arithmetic gadgets.

 See alu_arithmetic.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALU_ARITHMETIC_TCC_
// #define ALU_ARITHMETIC_TCC_

use  <functional>

use ffec::common::profiling;
use ffec::common::utils;



/* the code here is full of template lambda magic, but it is better to
   have limited presence of such code than to have code duplication in
   testing functions, which basically do the same thing: brute force
   the range of inputs which different success predicates */

template<class T, typename FieldT>
using initializer_fn =
    std::function<T*
                  (tinyram_protoboard<FieldT>&,    // pb
                   pb_variable_array<FieldT>&,       // opcode_indicators
                   word_variable_gadget<FieldT>&, // desval
                   word_variable_gadget<FieldT>&, // arg1val
                   word_variable_gadget<FieldT>&, // arg2val
                   pb_variable<FieldT>&,             // flag
                   pb_variable<FieldT>&,             // result
                   pb_variable<FieldT>&              // result_flag
                  )>;

template<class T, typename FieldT>
void brute_force_arithmetic_gadget(const size_t w,
                                   const size_t opcode,
                                   initializer_fn<T, FieldT> initializer,
                                   std::function<size_t(size_t,bool,size_t,size_t)> res_function,
                                   std::function<bool(size_t,bool,size_t,size_t)> flag_function)
/* parameters for res_function and flag_function are both desval, flag, arg1val, arg2val */
{
    print!("testing on all {} bit inputs\n", w);

    tinyram_architecture_params ap(w, 16);
    tinyram_program P; P.instructions = generate_tinyram_prelude(ap);
    tinyram_protoboard<FieldT> pb(ap, P.size(), 0, 10);

    pb_variable_array<FieldT> opcode_indicators;
    opcode_indicators.allocate(pb, 1ul<<ap.opcode_width(), "opcode_indicators");
    for (size_t i = 0; i < 1ul<<ap.opcode_width(); ++i)
    {
        pb.val(opcode_indicators[i]) = (i == opcode ? FieldT::one() : FieldT::zero());
    }

    word_variable_gadget<FieldT> desval(pb, "desval");
    desval.generate_r1cs_constraints(true);
    word_variable_gadget<FieldT> arg1val(pb, "arg1val");
    arg1val.generate_r1cs_constraints(true);
    word_variable_gadget<FieldT> arg2val(pb, "arg2val");
    arg2val.generate_r1cs_constraints(true);
    pb_variable<FieldT> flag; flag.allocate(pb, "flag");
    pb_variable<FieldT> result; result.allocate(pb, "result");
    pb_variable<FieldT> result_flag; result_flag.allocate(pb, "result_flag");

    std::unique_ptr<T> g;
    g.reset(initializer(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag));
    g->generate_r1cs_constraints();

    for (size_t des = 0; des < (1u << w); ++des)
    {
        pb.val(desval.packed) = FieldT(des);
        desval.generate_r1cs_witness_from_packed();

        for (char f = 0; f <= 1; ++f)
        {
            pb.val(flag) = (f ? FieldT::one() : FieldT::zero());

            for (size_t arg1 = 0; arg1 < (1u << w); ++arg1)
            {
                pb.val(arg1val.packed) = FieldT(arg1);
                arg1val.generate_r1cs_witness_from_packed();

                for (size_t arg2 = 0; arg2 < (1u << w); ++arg2)
                {
                    pb.val(arg2val.packed) = FieldT(arg2);
                    arg2val.generate_r1cs_witness_from_packed();

                    size_t res = res_function(des, f, arg1, arg2);
                    bool res_f = flag_function(des, f, arg1, arg2);
// #ifdef DEBUG
                    print!("with the following parameters: flag = %d"
                           ", desval = {} (%d)"
                           ", arg1val = {} (%d)"
                           ", arg2val = {} (%d)"
                           ". expected result: {} (%d), expected flag: %d\n",
                           f,
                           des, ffec::from_twos_complement(des, w),
                           arg1, ffec::from_twos_complement(arg1, w),
                           arg2, ffec::from_twos_complement(arg2, w),
                           res, ffec::from_twos_complement(res, w), res_f);
//#endif
                    g->generate_r1cs_witness();
// #ifdef DEBUG
                    print!("result: ");
                    pb.val(result).print();
                    print!("flag: ");
                    pb.val(result_flag).print();
//#endif
                    assert!(pb.is_satisfied());
                    assert!(pb.val(result) == FieldT(res));
                    assert!(pb.val(result_flag) == (res_f ? FieldT::one() : FieldT::zero()));
                }
            }
        }
    }
}

/* and */
template<typename FieldT>
void ALU_and_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { self.arg1val.bits[i] },
                { self.arg2val.bits[i] },
                { self.res_word[i] }),
            FMT(self.annotation_prefix, " res_word_{}", i));
    }

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
    not_all_zeros->generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        FMT(self.annotation_prefix, " result_flag"));
}

template<typename FieldT>
void ALU_and_gadget<FieldT>::generate_r1cs_witness()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        bool b1 = self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        bool b2 = self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = (b1 && b2 ? FieldT::one() : FieldT::zero());
    }

    pack_result->generate_r1cs_witness_from_bits();
    not_all_zeros->generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(not_all_zeros_result);
}

template<typename FieldT>
void test_ALU_and_gadget(const size_t w)
{
    ffec::print_time("starting and test");
    brute_force_arithmetic_gadget<ALU_and_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_AND,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_and_gadget<FieldT>* {
                                                                      return new ALU_and_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_and_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return x & y; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return (x & y) == 0; });
    ffec::print_time("and tests successful");
}

/* or */
template<typename FieldT>
void ALU_or_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { ONE, self.arg1val.bits[i] * (-1) },
                { ONE, self.arg2val.bits[i] * (-1) },
                { ONE, self.res_word[i] * (-1) }),
            FMT(self.annotation_prefix, " res_word_{}", i));
    }

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
    not_all_zeros->generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        FMT(self.annotation_prefix, " result_flag"));
}

template<typename FieldT>
void ALU_or_gadget<FieldT>::generate_r1cs_witness()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        bool b1 = self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        bool b2 = self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = (b1 || b2 ? FieldT::one() : FieldT::zero());
    }

    pack_result->generate_r1cs_witness_from_bits();
    not_all_zeros->generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}

template<typename FieldT>
void test_ALU_or_gadget(const size_t w)
{
    ffec::print_time("starting or test");
    brute_force_arithmetic_gadget<ALU_or_gadget<FieldT>, FieldT>(w,
                                                                 tinyram_opcode_OR,
                                                                 [] (tinyram_protoboard<FieldT> &pb,
                                                                     pb_variable_array<FieldT> &opcode_indicators,
                                                                     word_variable_gadget<FieldT> &desval,
                                                                     word_variable_gadget<FieldT> &arg1val,
                                                                     word_variable_gadget<FieldT> &arg2val,
                                                                     pb_variable<FieldT> &flag,
                                                                     pb_variable<FieldT> &result,
                                                                     pb_variable<FieldT> &result_flag) ->
                                                                 ALU_or_gadget<FieldT>* {
                                                                     return new ALU_or_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_or_gadget");
                                                                 },
                                                                 [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return x | y; },
                                                                 [w] (size_t des, bool f, size_t x, size_t y) -> bool { return (x | y) == 0; });
    ffec::print_time("or tests successful");
}

/* xor */
template<typename FieldT>
void ALU_xor_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        /* a = b ^ c <=> a = b + c - 2*b*c, (2*b)*c = b+c - a */
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { self.arg1val.bits[i] * 2},
                { self.arg2val.bits[i] },
                { self.arg1val.bits[i], self.arg2val.bits[i], self.res_word[i] * (-1) }),
            FMT(self.annotation_prefix, " res_word_{}", i));
    }

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
    not_all_zeros->generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        FMT(self.annotation_prefix, " result_flag"));
}

template<typename FieldT>
void ALU_xor_gadget<FieldT>::generate_r1cs_witness()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        bool b1 = self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        bool b2 = self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = (b1 ^ b2 ? FieldT::one() : FieldT::zero());
    }

    pack_result->generate_r1cs_witness_from_bits();
    not_all_zeros->generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}

template<typename FieldT>
void test_ALU_xor_gadget(const size_t w)
{
    ffec::print_time("starting xor test");
    brute_force_arithmetic_gadget<ALU_xor_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_XOR,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_xor_gadget<FieldT>* {
                                                                      return new ALU_xor_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_xor_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return x ^ y; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return (x ^ y) == 0; });
    ffec::print_time("xor tests successful");
}

/* not */
template<typename FieldT>
void ALU_not_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { ONE },
                { ONE, self.arg2val.bits[i] * (-1) },
                { self.res_word[i] }),
            FMT(self.annotation_prefix, " res_word_{}", i));
    }

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
    not_all_zeros->generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        FMT(self.annotation_prefix, " result_flag"));
}

template<typename FieldT>
void ALU_not_gadget<FieldT>::generate_r1cs_witness()
{
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        bool b2 = self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = (!b2 ? FieldT::one() : FieldT::zero());
    }

    pack_result->generate_r1cs_witness_from_bits();
    not_all_zeros->generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}

template<typename FieldT>
void test_ALU_not_gadget(const size_t w)
{
    ffec::print_time("starting not test");
    brute_force_arithmetic_gadget<ALU_not_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_NOT,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_not_gadget<FieldT>* {
                                                                      return new ALU_not_gadget<FieldT>(pb, opcode_indicators,desval, arg1val, arg2val, flag, result, result_flag, "ALU_not_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return (1ul<<w)-1-y; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return ((1ul<<w)-1-y) == 0; });
    ffec::print_time("not tests successful");
}

/* add */
template<typename FieldT>
void ALU_add_gadget<FieldT>::generate_r1cs_constraints()
{
    /* addition_result = 1 * (arg1val + arg2val) */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.packed, self.arg2val.packed },
            { self.addition_result }),
        FMT(self.annotation_prefix, " addition_result"));

    /* unpack into bits */
    unpack_addition->generate_r1cs_constraints(true);

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
}

template<typename FieldT>
void ALU_add_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(addition_result) = self.pb.val(self.arg1val.packed) + self.pb.val(self.arg2val.packed);
    unpack_addition->generate_r1cs_witness_from_packed();
    pack_result->generate_r1cs_witness_from_bits();
}

template<typename FieldT>
void test_ALU_add_gadget(const size_t w)
{
    ffec::print_time("starting add test");
    brute_force_arithmetic_gadget<ALU_add_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_ADD,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_add_gadget<FieldT>* {
                                                                      return new ALU_add_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_add_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return (x+y) % (1ul<<w); },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return (x+y) >= (1ul<<w); });
    ffec::print_time("add tests successful");
}

/* sub */
template<typename FieldT>
void ALU_sub_gadget<FieldT>::generate_r1cs_constraints()
{
    /* intermediate_result = 2^w + (arg1val - arg2val) */
    FieldT twoi = FieldT::one();

    linear_combination<FieldT> a, b, c;

    a.add_term(0, 1);
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        twoi = twoi + twoi;
    }
    b.add_term(0, twoi);
    b.add_term(self.arg1val.packed, 1);
    b.add_term(self.arg2val.packed, -1);
    c.add_term(intermediate_result, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), FMT(self.annotation_prefix, " main_constraint"));

    /* unpack into bits */
    unpack_intermediate->generate_r1cs_constraints(true);

    /* generate result */
    pack_result->generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.negated_flag * (-1) },
            { self.result_flag }),
        FMT(self.annotation_prefix, " result_flag"));
}

template<typename FieldT>
void ALU_sub_gadget<FieldT>::generate_r1cs_witness()
{
    FieldT twoi = FieldT::one();
    for (size_t i = 0; i < self.pb.ap.w; ++i)
    {
        twoi = twoi + twoi;
    }

    self.pb.val(intermediate_result) = twoi + self.pb.val(self.arg1val.packed) - self.pb.val(self.arg2val.packed);
    unpack_intermediate->generate_r1cs_witness_from_packed();
    pack_result->generate_r1cs_witness_from_bits();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.negated_flag);
}

template<typename FieldT>
void test_ALU_sub_gadget(const size_t w)
{
    ffec::print_time("starting sub test");
    brute_force_arithmetic_gadget<ALU_sub_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_SUB,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_sub_gadget<FieldT>* {
                                                                      return new ALU_sub_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_sub_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                      const size_t unsigned_result = ((1ul<<w) + x - y) % (1ul<<w);
                                                                      return unsigned_result;
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                      const size_t msb = ((1ul<<w) + x - y) >> w;
                                                                      return (msb == 0);
                                                                  });
    ffec::print_time("sub tests successful");
}

/* mov */
template<typename FieldT>
void ALU_mov_gadget<FieldT>::generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg2val.packed },
            { self.result }),
        FMT(self.annotation_prefix, " mov_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.flag },
            { self.result_flag }),
        FMT(self.annotation_prefix, " mov_result_flag"));
}

template<typename FieldT>
void ALU_mov_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(self.result) = self.pb.val(self.arg2val.packed);
    self.pb.val(self.result_flag) = self.pb.val(self.flag);
}

template<typename FieldT>
void test_ALU_mov_gadget(const size_t w)
{
    ffec::print_time("starting mov test");
    brute_force_arithmetic_gadget<ALU_mov_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_MOV,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_mov_gadget<FieldT>* {
                                                                      return new ALU_mov_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_mov_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return y; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return f; });
    ffec::print_time("mov tests successful");
}

/* cmov */
template<typename FieldT>
void ALU_cmov_gadget<FieldT>::generate_r1cs_constraints()
{
    /*
      flag1 * arg2val + (1-flag1) * desval = result
      flag1 * (arg2val - desval) = result - desval
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.flag },
            { self.arg2val.packed, self.desval.packed * (-1) },
            { self.result, self.desval.packed * (-1) }),
        FMT(self.annotation_prefix, " cmov_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.flag },
            { self.result_flag }),
        FMT(self.annotation_prefix, " cmov_result_flag"));
}

template<typename FieldT>
void ALU_cmov_gadget<FieldT>::generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  self.pb.val(self.arg2val.packed) :
                                  self.pb.val(self.desval.packed));
    self.pb.val(self.result_flag) = self.pb.val(self.flag);
}

template<typename FieldT>
void test_ALU_cmov_gadget(const size_t w)
{
    ffec::print_time("starting cmov test");
    brute_force_arithmetic_gadget<ALU_cmov_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMOV,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_cmov_gadget<FieldT>* {
                                                                       return new ALU_cmov_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_cmov_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return f ? y : des; },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool { return f; });
    ffec::print_time("cmov tests successful");
}

/* unsigned comparison */
template<typename FieldT>
void ALU_cmp_gadget<FieldT>::generate_r1cs_constraints()
{
    comparator.generate_r1cs_constraints();
    /*
      cmpe = cmpae * (1-cmpa)
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { cmpae_result_flag },
            { ONE, cmpa_result_flag * (-1) },
            { cmpe_result_flag }),
        FMT(self.annotation_prefix, " cmpa_result_flag"));

    /* copy over results */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpe_result }),
        FMT(self.annotation_prefix, " cmpe_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpa_result }),
        FMT(self.annotation_prefix, " cmpa_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpae_result }),
        FMT(self.annotation_prefix, " cmpae_result"));
}

template<typename FieldT>
void ALU_cmp_gadget<FieldT>::generate_r1cs_witness()
{
    comparator.generate_r1cs_witness();

    self.pb.val(cmpe_result) = self.pb.val(self.desval.packed);
    self.pb.val(cmpa_result) = self.pb.val(self.desval.packed);
    self.pb.val(cmpae_result) = self.pb.val(self.desval.packed);

    self.pb.val(cmpe_result_flag) = ((self.pb.val(cmpae_result_flag) == FieldT::one()) &&
                                      (self.pb.val(cmpa_result_flag) == FieldT::zero()) ?
                                      FieldT::one() :
                                      FieldT::zero());
}

template<typename FieldT>
void test_ALU_cmpe_gadget(const size_t w)
{
    ffec::print_time("starting cmpe test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPE,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_cmp_gadget<FieldT>* {
                                                                      pb_variable<FieldT> cmpa_result; cmpa_result.allocate(pb, "cmpa_result");
                                                                      pb_variable<FieldT> cmpa_result_flag; cmpa_result_flag.allocate(pb, "cmpa_result_flag");
                                                                      pb_variable<FieldT> cmpae_result; cmpae_result.allocate(pb, "cmpae_result");
                                                                      pb_variable<FieldT> cmpae_result_flag; cmpae_result_flag.allocate(pb, "cmpae_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        result, result_flag,
                                                                                                        cmpa_result, cmpa_result_flag,
                                                                                                        cmpae_result, cmpae_result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return des; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return x == y; });
    ffec::print_time("cmpe tests successful");
}

template<typename FieldT>
void test_ALU_cmpa_gadget(const size_t w)
{
    ffec::print_time("starting cmpa test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPA,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_cmp_gadget<FieldT>* {
                                                                      pb_variable<FieldT> cmpe_result; cmpe_result.allocate(pb, "cmpe_result");
                                                                      pb_variable<FieldT> cmpe_result_flag; cmpe_result_flag.allocate(pb, "cmpe_result_flag");
                                                                      pb_variable<FieldT> cmpae_result; cmpae_result.allocate(pb, "cmpae_result");
                                                                      pb_variable<FieldT> cmpae_result_flag; cmpae_result_flag.allocate(pb, "cmpae_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        cmpe_result, cmpe_result_flag,
                                                                                                        result, result_flag,
                                                                                                        cmpae_result, cmpae_result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return des; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return x > y; });
    ffec::print_time("cmpa tests successful");
}

template<typename FieldT>
void test_ALU_cmpae_gadget(const size_t w)
{
    ffec::print_time("starting cmpae test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPAE,
                                                                  [] (tinyram_protoboard<FieldT> &pb,
                                                                      pb_variable_array<FieldT> &opcode_indicators,
                                                                      word_variable_gadget<FieldT> &desval,
                                                                      word_variable_gadget<FieldT> &arg1val,
                                                                      word_variable_gadget<FieldT> &arg2val,
                                                                      pb_variable<FieldT> &flag,
                                                                      pb_variable<FieldT> &result,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_cmp_gadget<FieldT>* {
                                                                      pb_variable<FieldT> cmpe_result; cmpe_result.allocate(pb, "cmpe_result");
                                                                      pb_variable<FieldT> cmpe_result_flag; cmpe_result_flag.allocate(pb, "cmpe_result_flag");
                                                                      pb_variable<FieldT> cmpa_result; cmpa_result.allocate(pb, "cmpa_result");
                                                                      pb_variable<FieldT> cmpa_result_flag; cmpa_result_flag.allocate(pb, "cmpa_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        cmpe_result, cmpe_result_flag,
                                                                                                        cmpa_result, cmpa_result_flag,
                                                                                                        result, result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return des; },
                                                                  [w] (size_t des, bool f, size_t x, size_t y) -> bool { return x >= y; });
    ffec::print_time("cmpae tests successful");
}

/* signed comparison */
template<typename FieldT>
void ALU_cmps_gadget<FieldT>::generate_r1cs_constraints()
{
    /* negate sign bits */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.arg1val.bits[self.pb.ap.w-1] * (-1) },
            { negated_arg1val_sign }),
        FMT(self.annotation_prefix, " negated_arg1val_sign"));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.arg2val.bits[self.pb.ap.w-1] * (-1) },
            { negated_arg2val_sign }),
        FMT(self.annotation_prefix, " negated_arg2val_sign"));

    /* pack */
    pack_modified_arg1->generate_r1cs_constraints(false);
    pack_modified_arg2->generate_r1cs_constraints(false);

    /* compare */
    comparator->generate_r1cs_constraints();

    /* copy over results */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpg_result }),
        FMT(self.annotation_prefix, " cmpg_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpge_result }),
        FMT(self.annotation_prefix, " cmpge_result"));
}

template<typename FieldT>
void ALU_cmps_gadget<FieldT>::generate_r1cs_witness()
{
    /* negate sign bits */
    self.pb.val(negated_arg1val_sign) = FieldT::one() - self.pb.val(self.arg1val.bits[self.pb.ap.w-1]);
    self.pb.val(negated_arg2val_sign) = FieldT::one() - self.pb.val(self.arg2val.bits[self.pb.ap.w-1]);

    /* pack */
    pack_modified_arg1->generate_r1cs_witness_from_bits();
    pack_modified_arg2->generate_r1cs_witness_from_bits();

    /* produce result */
    comparator->generate_r1cs_witness();

    self.pb.val(cmpg_result) = self.pb.val(self.desval.packed);
    self.pb.val(cmpge_result) = self.pb.val(self.desval.packed);
}

template<typename FieldT>
void test_ALU_cmpg_gadget(const size_t w)
{
    ffec::print_time("starting cmpg test");
    brute_force_arithmetic_gadget<ALU_cmps_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMPG,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_cmps_gadget<FieldT>* {
                                                                       pb_variable<FieldT> cmpge_result; cmpge_result.allocate(pb, "cmpge_result");
                                                                       pb_variable<FieldT> cmpge_result_flag; cmpge_result_flag.allocate(pb, "cmpge_result_flag");
                                                                       return new ALU_cmps_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          cmpge_result, cmpge_result_flag, "ALU_cmps_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return des; },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                       return (ffec::from_twos_complement(x, w) >
                                                                               ffec::from_twos_complement(y, w));
                                                                   });
    ffec::print_time("cmpg tests successful");
}

template<typename FieldT>
void test_ALU_cmpge_gadget(const size_t w)
{
    ffec::print_time("starting cmpge test");
    brute_force_arithmetic_gadget<ALU_cmps_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMPGE,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_cmps_gadget<FieldT>* {
                                                                       pb_variable<FieldT> cmpg_result; cmpg_result.allocate(pb, "cmpg_result");
                                                                       pb_variable<FieldT> cmpg_result_flag; cmpg_result_flag.allocate(pb, "cmpg_result_flag");
                                                                       return new ALU_cmps_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          cmpg_result, cmpg_result_flag,
                                                                                                          result, result_flag, "ALU_cmps_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return des; },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                       return (ffec::from_twos_complement(x, w) >=
                                                                               ffec::from_twos_complement(y, w));
                                                                   });
    ffec::print_time("cmpge tests successful");
}

template<typename FieldT>
void ALU_umul_gadget<FieldT>::generate_r1cs_constraints()
{
    /* do multiplication */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.arg1val.packed },
            { self.arg2val.packed },
            { mul_result.packed }),
        FMT(self.annotation_prefix, " main_constraint"));
    mul_result.generate_r1cs_constraints(true);

    /* pack result */
    pack_mull_result->generate_r1cs_constraints(false);
    pack_umulh_result->generate_r1cs_constraints(false);

    /* compute flag */
    compute_flag->generate_r1cs_constraints();

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.result_flag },
            { mull_flag }),
        FMT(self.annotation_prefix, " mull_flag"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.result_flag },
            { umulh_flag }),
        FMT(self.annotation_prefix, " umulh_flag"));
}

template<typename FieldT>
void ALU_umul_gadget<FieldT>::generate_r1cs_witness()
{
    /* do multiplication */
    self.pb.val(mul_result.packed) = self.pb.val(self.arg1val.packed) * self.pb.val(self.arg2val.packed);
    mul_result.generate_r1cs_witness_from_packed();

    /* pack result */
    pack_mull_result->generate_r1cs_witness_from_bits();
    pack_umulh_result->generate_r1cs_witness_from_bits();

    /* compute flag */
    compute_flag->generate_r1cs_witness();

    self.pb.val(mull_flag) = self.pb.val(self.result_flag);
    self.pb.val(umulh_flag) = self.pb.val(self.result_flag);
}

template<typename FieldT>
void test_ALU_mull_gadget(const size_t w)
{
    ffec::print_time("starting mull test");
    brute_force_arithmetic_gadget<ALU_umul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_MULL,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_umul_gadget<FieldT>* {
                                                                       pb_variable<FieldT> umulh_result; umulh_result.allocate(pb, "umulh_result");
                                                                       pb_variable<FieldT> umulh_flag; umulh_flag.allocate(pb, "umulh_flag");
                                                                       return new ALU_umul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          umulh_result, umulh_flag,
                                                                                                          "ALU_umul_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return (x*y) % (1ul<<w); },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                       return ((x*y) >> w) != 0;
                                                                   });
    ffec::print_time("mull tests successful");
}

template<typename FieldT>
void test_ALU_umulh_gadget(const size_t w)
{
    ffec::print_time("starting umulh test");
    brute_force_arithmetic_gadget<ALU_umul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_UMULH,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_umul_gadget<FieldT>* {
                                                                       pb_variable<FieldT> mull_result; mull_result.allocate(pb, "mull_result");
                                                                       pb_variable<FieldT> mull_flag; mull_flag.allocate(pb, "mull_flag");
                                                                       return new ALU_umul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          mull_result, mull_flag,
                                                                                                          result, result_flag,
                                                                                                          "ALU_umul_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t { return (x*y) >> w; },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                       return ((x*y) >> w) != 0;
                                                                   });
    ffec::print_time("umulh tests successful");
}

template<typename FieldT>
void ALU_smul_gadget<FieldT>::generate_r1cs_constraints()
{
    /* do multiplication */
    /*
      from two's complement: (packed - 2^w * bits[w-1])
      to two's complement: lower order bits of 2^{2w} + result_of_*
    */

    linear_combination<FieldT> a, b, c;
    a.add_term(self.arg1val.packed, 1);
    a.add_term(self.arg1val.bits[self.pb.ap.w-1], -(FieldT(2)^self.pb.ap.w));
    b.add_term(self.arg2val.packed, 1);
    b.add_term(self.arg2val.bits[self.pb.ap.w-1], -(FieldT(2)^self.pb.ap.w));
    c.add_term(mul_result.packed, 1);
    c.add_term(ONE, -(FieldT(2)^(2*self.pb.ap.w)));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), FMT(self.annotation_prefix, " main_constraint"));

    mul_result.generate_r1cs_constraints(true);

    /* pack result */
    pack_smulh_result->generate_r1cs_constraints(false);

    /* compute flag */
    pack_top->generate_r1cs_constraints(false);

    /*
      the gadgets below are FieldT specific:
      I * X = (1-R)
      R * X = 0

      if X = 0 then R = 1
      if X != 0 then R = 0 and I = X^{-1}
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_empty_aux },
            { top },
            { ONE, is_top_empty * (-1) }),
        FMT(self.annotation_prefix, " I*X=1-R (is_top_empty)"));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_empty },
            { top },
            { ONE * 0 }),
        FMT(self.annotation_prefix, " R*X=0 (is_top_full)"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_full_aux },
            { top, ONE * (1l-(1ul<<(self.pb.ap.w+1))) },
            { ONE, is_top_full * (-1) }),
        FMT(self.annotation_prefix, " I*X=1-R (is_top_full)"));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_full },
            { top, ONE * (1l-(1ul<<(self.pb.ap.w+1))) },
            { ONE * 0 }),
        FMT(self.annotation_prefix, " R*X=0 (is_top_full)"));

    /* smulh_flag = 1 - (is_top_full + is_top_empty) */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, is_top_full * (-1), is_top_empty * (-1) },
            { smulh_flag }),
        FMT(self.annotation_prefix, " smulh_flag"));
}

template<typename FieldT>
void ALU_smul_gadget<FieldT>::generate_r1cs_witness()
{
    /* do multiplication */
    /*
      from two's complement: (packed - 2^w * bits[w-1])
      to two's complement: lower order bits of (2^{2w} + result_of_mul)
    */
    self.pb.val(mul_result.packed) =
        (self.pb.val(self.arg1val.packed) - (self.pb.val(self.arg1val.bits[self.pb.ap.w-1])*(FieldT(2)^self.pb.ap.w))) *
        (self.pb.val(self.arg2val.packed) - (self.pb.val(self.arg2val.bits[self.pb.ap.w-1])*(FieldT(2)^self.pb.ap.w))) +
        (FieldT(2)^(2*self.pb.ap.w));

    mul_result.generate_r1cs_witness_from_packed();

    /* pack result */
    pack_smulh_result->generate_r1cs_witness_from_bits();

    /* compute flag */
    pack_top->generate_r1cs_witness_from_bits();
    size_t topval = self.pb.val(top).as_ulong();

    if (topval == 0)
    {
        self.pb.val(is_top_empty) = FieldT::one();
        self.pb.val(is_top_empty_aux) = FieldT::zero();
    }
    else
    {
        self.pb.val(is_top_empty) = FieldT::zero();
        self.pb.val(is_top_empty_aux) = self.pb.val(top).inverse();
    }

    if (topval == ((1ul<<(self.pb.ap.w+1))-1))
    {
        self.pb.val(is_top_full) = FieldT::one();
        self.pb.val(is_top_full_aux) = FieldT::zero();
    }
    else
    {
        self.pb.val(is_top_full) = FieldT::zero();
        self.pb.val(is_top_full_aux) = (self.pb.val(top)-FieldT((1ul<<(self.pb.ap.w+1))-1)).inverse();
    }

    /* smulh_flag = 1 - (is_top_full + is_top_empty) */
    self.pb.val(smulh_flag) = FieldT::one() - (self.pb.val(is_top_full) + self.pb.val(is_top_empty));
}

template<typename FieldT>
void test_ALU_smulh_gadget(const size_t w)
{
    ffec::print_time("starting smulh test");
    brute_force_arithmetic_gadget<ALU_smul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_SMULH,
                                                                   [] (tinyram_protoboard<FieldT> &pb,
                                                                       pb_variable_array<FieldT> &opcode_indicators,
                                                                       word_variable_gadget<FieldT> &desval,
                                                                       word_variable_gadget<FieldT> &arg1val,
                                                                       word_variable_gadget<FieldT> &arg2val,
                                                                       pb_variable<FieldT> &flag,
                                                                       pb_variable<FieldT> &result,
                                                                       pb_variable<FieldT> &result_flag) ->
                                                                   ALU_smul_gadget<FieldT>* {
                                                                       return new ALU_smul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          "ALU_smul_gadget");
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                       const size_t res = ffec::to_twos_complement((ffec::from_twos_complement(x, w) * ffec::from_twos_complement(y, w)), 2*w);
                                                                       return res >> w;
                                                                   },
                                                                   [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                       const int res = ffec::from_twos_complement(x, w) * ffec::from_twos_complement(y, w);
                                                                       const int truncated_res = ffec::from_twos_complement(ffec::to_twos_complement(res, 2*w) & ((1ul<<w)-1), w);
                                                                       return (res != truncated_res);
                                                                   });
    ffec::print_time("smulh tests successful");
}

template<typename FieldT>
void ALU_divmod_gadget<FieldT>::generate_r1cs_constraints()
{
    /* B_inv * B = B_nonzero */
    linear_combination<FieldT> a1, b1, c1;
    a1.add_term(B_inv, 1);
    b1.add_term(self.arg2val.packed, 1);
    c1.add_term(B_nonzero, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a1, b1, c1), FMT(self.annotation_prefix, " B_inv*B=B_nonzero"));

    /* (1-B_nonzero) * B = 0 */
    linear_combination<FieldT> a2, b2, c2;
    a2.add_term(ONE, 1);
    a2.add_term(B_nonzero, -1);
    b2.add_term(self.arg2val.packed, 1);
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a2, b2, c2), FMT(self.annotation_prefix, " (1-B_nonzero)*B=0"));

    /* B * q + r = A_aux = A * B_nonzero */
    linear_combination<FieldT> a3, b3, c3;
    a3.add_term(self.arg2val.packed, 1);
    b3.add_term(udiv_result, 1);
    c3.add_term(A_aux, 1);
    c3.add_term(umod_result, -1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a3, b3, c3), FMT(self.annotation_prefix, " B*q+r=A_aux"));

    linear_combination<FieldT> a4, b4, c4;
    a4.add_term(self.arg1val.packed, 1);
    b4.add_term(B_nonzero, 1);
    c4.add_term(A_aux, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a4, b4, c4), FMT(self.annotation_prefix, " A_aux=A*B_nonzero"));

    /* q * (1-B_nonzero) = 0 */
    linear_combination<FieldT> a5, b5, c5;
    a5.add_term(udiv_result, 1);
    b5.add_term(ONE, 1);
    b5.add_term(B_nonzero, -1);
    c5.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a5, b5, c5), FMT(self.annotation_prefix, " q*B_nonzero=0"));

    /* A<B_gadget<FieldT>(B, r, less=B_nonzero, leq=ONE) */
    r_less_B->generate_r1cs_constraints();
}

template<typename FieldT>
void ALU_divmod_gadget<FieldT>::generate_r1cs_witness()
{
    if (self.pb.val(self.arg2val.packed) == FieldT::zero())
    {
        self.pb.val(B_inv) = FieldT::zero();
        self.pb.val(B_nonzero) = FieldT::zero();

        self.pb.val(A_aux) = FieldT::zero();

        self.pb.val(udiv_result) = FieldT::zero();
        self.pb.val(umod_result) = FieldT::zero();

        self.pb.val(udiv_flag) = FieldT::one();
        self.pb.val(umod_flag) = FieldT::one();
    }
    else
    {
        self.pb.val(B_inv) = self.pb.val(self.arg2val.packed).inverse();
        self.pb.val(B_nonzero) = FieldT::one();

        const size_t A = self.pb.val(self.arg1val.packed).as_ulong();
        const size_t B = self.pb.val(self.arg2val.packed).as_ulong();

        self.pb.val(A_aux) = self.pb.val(self.arg1val.packed);

        self.pb.val(udiv_result) = FieldT(A / B);
        self.pb.val(umod_result) = FieldT(A % B);

        self.pb.val(udiv_flag) = FieldT::zero();
        self.pb.val(umod_flag) = FieldT::zero();
    }

    r_less_B->generate_r1cs_witness();
}

template<typename FieldT>
void test_ALU_udiv_gadget(const size_t w)
{
    ffec::print_time("starting udiv test");
    brute_force_arithmetic_gadget<ALU_divmod_gadget<FieldT>, FieldT>(w,
                                                                     tinyram_opcode_UDIV,
                                                                     [] (tinyram_protoboard<FieldT> &pb,
                                                                         pb_variable_array<FieldT> &opcode_indicators,
                                                                         word_variable_gadget<FieldT> &desval,
                                                                         word_variable_gadget<FieldT> &arg1val,
                                                                         word_variable_gadget<FieldT> &arg2val,
                                                                         pb_variable<FieldT> &flag,
                                                                         pb_variable<FieldT> &result,
                                                                         pb_variable<FieldT> &result_flag) ->
                                                                     ALU_divmod_gadget<FieldT>* {
                                                                         pb_variable<FieldT> umod_result; umod_result.allocate(pb, "umod_result");
                                                                         pb_variable<FieldT> umod_flag; umod_flag.allocate(pb, "umod_flag");
                                                                         return new ALU_divmod_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                              result, result_flag,
                                                                                                              umod_result, umod_flag,
                                                                                                              "ALU_divmod_gadget");
                                                                     },
                                                                     [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                         return (y == 0 ? 0 : x / y);
                                                                     },
                                                                     [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                         return (y == 0);
                                                                     });
    ffec::print_time("udiv tests successful");
}

template<typename FieldT>
void test_ALU_umod_gadget(const size_t w)
{
    ffec::print_time("starting umod test");
    brute_force_arithmetic_gadget<ALU_divmod_gadget<FieldT>, FieldT>(w,
                                                                     tinyram_opcode_UMOD,
                                                                     [] (tinyram_protoboard<FieldT> &pb,
                                                                         pb_variable_array<FieldT> &opcode_indicators,
                                                                         word_variable_gadget<FieldT> &desval,
                                                                         word_variable_gadget<FieldT> &arg1val,
                                                                         word_variable_gadget<FieldT> &arg2val,
                                                                         pb_variable<FieldT> &flag,
                                                                         pb_variable<FieldT> &result,
                                                                         pb_variable<FieldT> &result_flag) ->
                                                                     ALU_divmod_gadget<FieldT>* {
                                                                         pb_variable<FieldT> udiv_result; udiv_result.allocate(pb, "udiv_result");
                                                                         pb_variable<FieldT> udiv_flag; udiv_flag.allocate(pb, "udiv_flag");
                                                                         return new ALU_divmod_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                              udiv_result, udiv_flag,
                                                                                                              result, result_flag,
                                                                                                              "ALU_divmod_gadget");
                                                                     },
                                                                     [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                         return (y == 0 ? 0 : x % y);
                                                                     },
                                                                     [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                         return (y == 0);
                                                                     });
    ffec::print_time("umod tests successful");
}

template<typename FieldT>
void ALU_shr_shl_gadget<FieldT>::generate_r1cs_constraints()
{
    /*
      select the input for barrel shifter:

      r = arg1val * opcode_indicators[SHR] + reverse(arg1val) * (1-opcode_indicators[SHR])
      r - reverse(arg1val) = (arg1val - reverse(arg1val)) * opcode_indicators[SHR]
    */
    pack_reversed_input->generate_r1cs_constraints(false);

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.arg1val.packed, reversed_input * (-1) },
            { self.opcode_indicators[tinyram_opcode_SHR] },
            { barrel_right_internal[0], reversed_input * (-1) }),
        FMT(self.annotation_prefix, " select_arg1val_or_reversed"));

    /*
      do logw iterations of barrel shifts
    */
    for (size_t i = 0; i < logw; ++i)
    {
        /* assert that shifted out part is bits */
        for (size_t j = 0; j < 1ul<<i; ++j)
        {
            generate_boolean_r1cs_constraint<FieldT>(self.pb, shifted_out_bits[i][j], FMT(self.annotation_prefix, " shifted_out_bits_%zu_{}", i, j));
        }

        /*
          add main shifting constraint


          old_result =
          (shifted_result * 2^(i+1) + shifted_out_part) * need_to_shift +
          (shfited_result) * (1-need_to_shift)

          old_result - shifted_result = (shifted_result * (2^(i+1) - 1) + shifted_out_part) * need_to_shift
        */
        linear_combination<FieldT> a, b, c;

        a.add_term(barrel_right_internal[i+1], (FieldT(2)^(i+1)) - FieldT::one());
        for (size_t j = 0; j < 1ul<<i; ++j)
        {
            a.add_term(shifted_out_bits[i][j], (FieldT(2)^j));
        }

        b.add_term(self.arg2val.bits[i], 1);

        c.add_term(barrel_right_internal[i], 1);
        c.add_term(barrel_right_internal[i+1], -1);

        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), FMT(self.annotation_prefix, " barrel_shift_{}", i));
    }

    /*
      get result as the logw iterations or zero if shift was oversized

      result = (1-is_oversize_shift) * barrel_right_internal[logw]
    */
    check_oversize_shift->generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE, is_oversize_shift * (-1) },
            { barrel_right_internal[logw] },
            { self.result }),
        FMT(self.annotation_prefix, " result"));

    /*
      get reversed result for SHL
    */
    unpack_result->generate_r1cs_constraints(true);
    pack_reversed_result->generate_r1cs_constraints(false);

    /*
      select the correct output:
      r = result * opcode_indicators[SHR] + reverse(result) * (1-opcode_indicators[SHR])
      r - reverse(result) = (result - reverse(result)) * opcode_indicators[SHR]
    */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.result, reversed_result * (-1) },
            { self.opcode_indicators[tinyram_opcode_SHR] },
            { shr_result, reversed_result * (-1) }),
        FMT(self.annotation_prefix, " shr_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.result, reversed_result * (-1) },
            { self.opcode_indicators[tinyram_opcode_SHR] },
            { shr_result, reversed_result * (-1) }),
        FMT(self.annotation_prefix, " shl_result"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.bits[0] },
            { shr_flag }),
        FMT(self.annotation_prefix, " shr_flag"));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.bits[self.pb.ap.w-1] },
            { shl_flag }),
        FMT(self.annotation_prefix, " shl_flag"));
}

template<typename FieldT>
void ALU_shr_shl_gadget<FieldT>::generate_r1cs_witness()
{
    /* select the input for barrel shifter */
    pack_reversed_input->generate_r1cs_witness_from_bits();

    self.pb.val(barrel_right_internal[0]) =
        (self.pb.val(self.opcode_indicators[tinyram_opcode_SHR]) == FieldT::one() ?
         self.pb.val(self.arg1val.packed) : self.pb.val(reversed_input));

    /*
      do logw iterations of barrel shifts.

      old_result =
      (shifted_result * 2^i + shifted_out_part) * need_to_shift +
      (shfited_result) * (1-need_to_shift)
    */

    for (size_t i = 0; i < logw; ++i)
    {
        self.pb.val(barrel_right_internal[i+1]) =
            (self.pb.val(self.arg2val.bits[i]) == FieldT::zero()) ? self.pb.val(barrel_right_internal[i]) :
            FieldT(self.pb.val(barrel_right_internal[i]).as_ulong() >> (i+1));

        shifted_out_bits[i].fill_with_bits_of_ulong(self.pb, self.pb.val(barrel_right_internal[i]).as_ulong() % (2u<<i));
    }

    /*
      get result as the logw iterations or zero if shift was oversized

      result = (1-is_oversize_shift) * barrel_right_internal[logw]
    */
    check_oversize_shift->generate_r1cs_witness();
    self.pb.val(self.result) = (FieldT::one() - self.pb.val(is_oversize_shift)) * self.pb.val(barrel_right_internal[logw]);

    /*
      get reversed result for SHL
    */
    unpack_result->generate_r1cs_witness_from_packed();
    pack_reversed_result->generate_r1cs_witness_from_bits();

    /*
      select the correct output:
      r = result * opcode_indicators[SHR] + reverse(result) * (1-opcode_indicators[SHR])
      r - reverse(result) = (result - reverse(result)) * opcode_indicators[SHR]
    */
    self.pb.val(shr_result) = (self.pb.val(self.opcode_indicators[tinyram_opcode_SHR]) == FieldT::one()) ?
        self.pb.val(self.result) : self.pb.val(reversed_result);

    self.pb.val(shl_result) = self.pb.val(shr_result);
    self.pb.val(shr_flag) = self.pb.val(self.arg1val.bits[0]);
    self.pb.val(shl_flag) = self.pb.val(self.arg1val.bits[self.pb.ap.w-1]);
}

template<typename FieldT>
void test_ALU_shr_gadget(const size_t w)
{
    ffec::print_time("starting shr test");
    brute_force_arithmetic_gadget<ALU_shr_shl_gadget<FieldT>, FieldT>(w,
                                                                      tinyram_opcode_SHR,
                                                                      [] (tinyram_protoboard<FieldT> &pb,
                                                                          pb_variable_array<FieldT> &opcode_indicators,
                                                                          word_variable_gadget<FieldT> &desval,
                                                                          word_variable_gadget<FieldT> &arg1val,
                                                                          word_variable_gadget<FieldT> &arg2val,
                                                                          pb_variable<FieldT> &flag,
                                                                          pb_variable<FieldT> &result,
                                                                          pb_variable<FieldT> &result_flag) ->
                                                                      ALU_shr_shl_gadget<FieldT>* {
                                                                          pb_variable<FieldT> shl_result; shl_result.allocate(pb, "shl_result");
                                                                          pb_variable<FieldT> shl_flag; shl_flag.allocate(pb, "shl_flag");
                                                                          return new ALU_shr_shl_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                                result, result_flag,
                                                                                                                shl_result, shl_flag,
                                                                                                                "ALU_shr_shl_gadget");
                                                                      },
                                                                      [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                          return (x >> y);
                                                                      },
                                                                      [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                          return (x & 1);
                                                                      });
    ffec::print_time("shr tests successful");
}

template<typename FieldT>
void test_ALU_shl_gadget(const size_t w)
{
    ffec::print_time("starting shl test");
    brute_force_arithmetic_gadget<ALU_shr_shl_gadget<FieldT>, FieldT>(w,
                                                                      tinyram_opcode_SHL,
                                                                      [] (tinyram_protoboard<FieldT> &pb,
                                                                          pb_variable_array<FieldT> &opcode_indicators,
                                                                          word_variable_gadget<FieldT> &desval,
                                                                          word_variable_gadget<FieldT> &arg1val,
                                                                          word_variable_gadget<FieldT> &arg2val,
                                                                          pb_variable<FieldT> &flag,
                                                                          pb_variable<FieldT> &result,
                                                                          pb_variable<FieldT> &result_flag) ->
                                                                      ALU_shr_shl_gadget<FieldT>* {
                                                                          pb_variable<FieldT> shr_result; shr_result.allocate(pb, "shr_result");
                                                                          pb_variable<FieldT> shr_flag; shr_flag.allocate(pb, "shr_flag");
                                                                          return new ALU_shr_shl_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                                shr_result, shr_flag,
                                                                                                                result, result_flag,
                                                                                                                "ALU_shr_shl_gadget");
                                                                      },
                                                                      [w] (size_t des, bool f, size_t x, size_t y) -> size_t {
                                                                          return (x << y) & ((1ul<<w)-1);
                                                                      },
                                                                      [w] (size_t des, bool f, size_t x, size_t y) -> bool {
                                                                          return (x >> (w-1));
                                                                      });
    ffec::print_time("shl tests successful");
}



//#endif
