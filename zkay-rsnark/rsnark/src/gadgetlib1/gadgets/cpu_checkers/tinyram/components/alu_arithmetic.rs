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
// use  <memory>

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget;



/* arithmetic gadgets */
// 
pub struct  ALU_arithmetic_gadget<FieldT>   {
// : public tinyram_standard_gadget<FieldT>
    opcode_indicators:pb_variable_array<FieldT>,
    desval:word_variable_gadget<FieldT>,
    arg1val:word_variable_gadget<FieldT>,
    arg2val:word_variable_gadget<FieldT>,
    flag:pb_variable<FieldT>,
    result:pb_variable<FieldT>,
    result_flag:pb_variable<FieldT>,
}
impl<FieldT>  ALU_arithmetic_gadget<FieldT>   {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                          opcode_indicators:pb_variable_array<FieldT>,
                          desval:word_variable_gadget<FieldT>,
                          arg1val:word_variable_gadget<FieldT>,
                          arg2val:word_variable_gadget<FieldT>,
                          flag:pb_variable<FieldT>,
                          result:pb_variable<FieldT>,
                          result_flag:pb_variable<FieldT>,
                          annotation_prefix:std::string) ->Self
       {
//  tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
       Self{ opcode_indicators,
        desval,
        arg1val,
        arg2val,
        flag,
        result,
        result_flag }
}
}

// 
pub struct  ALU_and_gadget  {
// : public ALU_arithmetic_gadget<FieldT>
res_word:    pb_variable_array<FieldT>,
pack_result:    std::shared_ptr<packing_gadget<FieldT> >,
not_all_zeros:    std::shared_ptr<disjunction_gadget<FieldT> >,
not_all_zeros_result:    pb_variable<FieldT>,
}
impl<FieldT>  ALU_and_gadget<FieldT>   {
// 
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
        
    {
        res_word.allocate(pb, pb.ap.w, format!("{} res_bit",self.annotation_prefix));
        not_all_zeros_result.allocate(pb, format!("{} not_all_zeros_result",self.annotation_prefix));

        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
        not_all_zeros.reset(
            disjunction_gadget::<FieldT>::new(pb, res_word, not_all_zeros_result,
                                           format!("{}not_all_zeros",self.annotation_prefix)));
// ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
        Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_ALU_and_gadget(w:usize);

// 
pub struct  ALU_or_gadget<FieldT>   {
// : public ALU_arithmetic_gadget<FieldT>
res_word:    pb_variable_array<FieldT>,
pack_result:    std::shared_ptr<packing_gadget<FieldT> >,
not_all_zeros:    std::shared_ptr<disjunction_gadget<FieldT> >,
not_all_zeros_result:    pb_variable<FieldT>,
}
impl<FieldT>  ALU_or_gadget<FieldT>   {
// 
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                  opcode_indicators:pb_variable_array<FieldT>,
                  desval:word_variable_gadget<FieldT>,
                  arg1val:word_variable_gadget<FieldT>,
                  arg2val:word_variable_gadget<FieldT>,
                  flag:pb_variable<FieldT>,
                  result:pb_variable<FieldT>,
                  result_flag:pb_variable<FieldT>,
                  annotation_prefix:std::string)->Self
        
    {
        res_word.allocate(pb, pb.ap.w, format!("{} res_bit",self.annotation_prefix));
        not_all_zeros_result.allocate(pb, format!("{} not_all_zeros_result",self.annotation_prefix));

        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
        not_all_zeros.reset(
            disjunction_gadget::<FieldT>::new(pb, res_word, not_all_zeros_result,
                                           format!("{}not_all_zeros",self.annotation_prefix)));
// ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_ALU_or_gadget(w:usize);

// 
pub struct  ALU_xor_gadget {
// : public ALU_arithmetic_gadget<FieldT> 
res_word:    pb_variable_array<FieldT>,
pack_result:    std::shared_ptr<packing_gadget<FieldT> >,
not_all_zeros:    std::shared_ptr<disjunction_gadget<FieldT> >,
not_all_zeros_result:    pb_variable<FieldT>,
}
impl ALU_xor_gadget<FieldT>  {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
        
    {
        res_word.allocate(pb, pb.ap.w, format!("{} res_bit",self.annotation_prefix));
        not_all_zeros_result.allocate(pb, format!("{} not_all_zeros_result",self.annotation_prefix));

        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
        not_all_zeros.reset(
            disjunction_gadget::<FieldT>::new(pb, res_word, not_all_zeros_result,
                                           format!("{}not_all_zeros",self.annotation_prefix)));
// ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_ALU_xor_gadget(w:usize);


pub struct  ALU_not_gadget {
/* we do bitwise not, because we need to compute flag */

res_word:    pb_variable_array<FieldT>,
pack_result:    std::shared_ptr<packing_gadget<FieldT> >,
not_all_zeros:    std::shared_ptr<disjunction_gadget<FieldT> >,
not_all_zeros_result:    pb_variable<FieldT>,
}
impl ALU_not_gadget {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
       
    {
        res_word.allocate(pb, pb.ap.w, format!("{} res_bit",self.annotation_prefix));
        not_all_zeros_result.allocate(pb, format!("{} not_all_zeros_result",self.annotation_prefix));

        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
        not_all_zeros.reset(
            disjunction_gadget::<FieldT>::new(pb, res_word, not_all_zeros_result,
                                           format!("{}not_all_zeros",self.annotation_prefix)));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_ALU_not_gadget(w:usize);


pub struct  ALU_add_gadget {

addition_result:    pb_variable<FieldT>,
res_word:    pb_variable_array<FieldT>,
res_word_and_flag:    pb_variable_array<FieldT>,
pack_result:    std::shared_ptr<packing_gadget<FieldT> > unpack_addition,,

    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
       
    {
        addition_result.allocate(pb, format!("{} addition_result",self.annotation_prefix));
        res_word.allocate(pb, pb.ap.w, format!("{} res_word",self.annotation_prefix));

        res_word_and_flag = res_word;
        res_word_and_flag.push(result_flag);

        unpack_addition.reset(
            packing_gadget::<FieldT>::new(pb, res_word_and_flag, addition_result,
                                       format!("{} unpack_addition",self.annotation_prefix)));
        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

 
}

// pub fn test_ALU_add_gadget(w:usize);


pub struct  ALU_sub_gadget {

intermediate_result:    pb_variable<FieldT>,
negated_flag:    pb_variable<FieldT>,
res_word:    pb_variable_array<FieldT>,
res_word_and_negated_flag:    pb_variable_array<FieldT>,

pack_result:    std::shared_ptr<packing_gadget<FieldT> > unpack_intermediate,,
}
 impl ALU_sub_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
        
    {
        intermediate_result.allocate(pb, format!("{} intermediate_result",self.annotation_prefix));
        negated_flag.allocate(pb, format!("{} negated_flag",self.annotation_prefix));
        res_word.allocate(pb, pb.ap.w, format!("{} res_word",self.annotation_prefix));

        res_word_and_negated_flag = res_word;
        res_word_and_negated_flag.push(negated_flag);

        unpack_intermediate.reset(
            packing_gadget::<FieldT>::new(pb, res_word_and_negated_flag, intermediate_result,
                                       format!("{} unpack_intermediate",self.annotation_prefix)));
        pack_result.reset(
            packing_gadget::<FieldT>::new(pb, res_word, result,
                                       format!("{} pack_result",self.annotation_prefix)));
// ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
Self{res_word,not_all_zeros_result,pack_result,not_all_zeros}
    }

 
}

// pub fn test_ALU_sub_gadget(w:usize);


pub struct  ALU_mov_gadget {
}
 impl ALU_mov_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   result:pb_variable<FieldT>,
                   result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
        {
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
        Self{}
}

 
}


// pub fn test_ALU_mov_gadget(w:usize);


pub struct  ALU_cmov_gadget {
}
 impl ALU_cmov_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                    opcode_indicators:pb_variable_array<FieldT>,
                    desval:word_variable_gadget<FieldT>,
                    arg1val:word_variable_gadget<FieldT>,
                    arg2val:word_variable_gadget<FieldT>,
                    flag:pb_variable<FieldT>,
                    result:pb_variable<FieldT>,
                    result_flag:pb_variable<FieldT>,
                    annotation_prefix:std::string)->Self
   
    {
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, annotation_prefix)
    Self{}
    }

 
}


// pub fn test_ALU_cmov_gadget(w:usize);


pub struct  ALU_cmp_gadget {

comparator:    comparison_gadget<FieldT>,

    cmpe_result:pb_variable<FieldT>,
    cmpe_result_flag:pb_variable<FieldT>,
    cmpa_result:pb_variable<FieldT>,
    cmpa_result_flag:pb_variable<FieldT>,
    cmpae_result:pb_variable<FieldT>,
    cmpae_result_flag:pb_variable<FieldT>,
}
 impl ALU_cmp_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                   opcode_indicators:pb_variable_array<FieldT>,
                   desval:word_variable_gadget<FieldT>,
                   arg1val:word_variable_gadget<FieldT>,
                   arg2val:word_variable_gadget<FieldT>,
                   flag:pb_variable<FieldT>,
                   cmpe_result:pb_variable<FieldT>,
                   cmpe_result_flag:pb_variable<FieldT>,
                   cmpa_result:pb_variable<FieldT>,
                   cmpa_result_flag:pb_variable<FieldT>,
                   cmpae_result:pb_variable<FieldT>,
                   cmpae_result_flag:pb_variable<FieldT>,
                   annotation_prefix:std::string)->Self
    { 
        // ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, cmpa_result, cmpa_result_flag, annotation_prefix),
        Self{comparator:comparison_gadget::<FieldT>::new(pb, pb.ap.w, arg2val.packed, arg1val.packed, cmpa_result_flag, cmpae_result_flag,
                   format!("{} comparator",self.annotation_prefix)),
        cmpe_result, cmpe_result_flag,
        cmpa_result, cmpa_result_flag,
        cmpae_result, cmpae_result_flag}
    }

 
}


// pub fn test_ALU_cmpe_gadget(w:usize);


// pub fn test_ALU_cmpa_gadget(w:usize);


// pub fn test_ALU_cmpae_gadget(w:usize);


pub struct  ALU_cmps_gadget {

negated_arg1val_sign:    pb_variable<FieldT>,
negated_arg2val_sign:    pb_variable<FieldT>,
modified_arg1:    pb_variable_array<FieldT>,
modified_arg2:    pb_variable_array<FieldT>,
packed_modified_arg1:    pb_variable<FieldT>,
packed_modified_arg2:    pb_variable<FieldT>,
pack_modified_arg1:    std::shared_ptr<packing_gadget<FieldT> >,
pack_modified_arg2:    std::shared_ptr<packing_gadget<FieldT> >,
comparator:    std::shared_ptr<comparison_gadget<FieldT> >,

    cmpg_result:pb_variable<FieldT>,
    cmpg_result_flag:pb_variable<FieldT>,
    cmpge_result:pb_variable<FieldT>,
    cmpge_result_flag:pb_variable<FieldT>,
}
 impl ALU_cmps_gadget<FieldT>  {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                    opcode_indicators:pb_variable_array<FieldT>,
                    desval:word_variable_gadget<FieldT>,
                    arg1val:word_variable_gadget<FieldT>,
                    arg2val:word_variable_gadget<FieldT>,
                    flag:pb_variable<FieldT>,
                    cmpg_result:pb_variable<FieldT>,
                    cmpg_result_flag:pb_variable<FieldT>,
                    cmpge_result:pb_variable<FieldT>,
                    cmpge_result_flag:pb_variable<FieldT>,
                    annotation_prefix:std::string)->Self
   
    {
        negated_arg1val_sign.allocate(pb, format!("{} negated_arg1val_sign",self.annotation_prefix));
        negated_arg2val_sign.allocate(pb, format!("{} negated_arg2val_sign",self.annotation_prefix));

        modified_arg1 = pb_variable_array<FieldT>(arg1val.bits.begin(), --arg1val.bits.end());
        modified_arg1.push(negated_arg1val_sign);

        modified_arg2 = pb_variable_array<FieldT>(arg2val.bits.begin(), --arg2val.bits.end());
        modified_arg2.push(negated_arg2val_sign);

        packed_modified_arg1.allocate(pb, format!("{} packed_modified_arg1",self.annotation_prefix));
        packed_modified_arg2.allocate(pb, format!("{} packed_modified_arg2",self.annotation_prefix));

        pack_modified_arg1.reset(packing_gadget::<FieldT>::new(pb, modified_arg1, packed_modified_arg1,
                                                            format!("{} pack_modified_arg1",self.annotation_prefix)));
        pack_modified_arg2.reset(packing_gadget::<FieldT>::new(pb, modified_arg2, packed_modified_arg2,
                                                            format!("{} pack_modified_arg2",self.annotation_prefix)));

        comparator.reset(comparison_gadget::<FieldT>::new(pb, pb.ap.w,
                                                       packed_modified_arg2, packed_modified_arg1,
                                                       cmpg_result_flag, cmpge_result_flag,
                                                       format!("{} comparator",self.annotation_prefix)));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, cmpg_result, cmpg_result_flag, annotation_prefix),
        Self{cmpg_result, cmpg_result_flag,
        cmpge_result, cmpge_result_flag}
    }
 
}


// pub fn test_ALU_cmpg_gadget(w:usize);


// pub fn test_ALU_cmpge_gadget(w:usize);


pub struct  ALU_umul_gadget {
mul_result:    dual_variable_gadget<FieldT>,
mull_bits:    pb_variable_array<FieldT>,
umulh_bits:    pb_variable_array<FieldT>,
result_flag:    pb_variable<FieldT>,
pack_mull_result:    std::shared_ptr<packing_gadget<FieldT> >,
pack_umulh_result:    std::shared_ptr<packing_gadget<FieldT> >,
compute_flag:    std::shared_ptr<disjunction_gadget<FieldT> >,

    mull_result:pb_variable<FieldT>,
    mull_flag:pb_variable<FieldT>,
    umulh_result:pb_variable<FieldT>,
    umulh_flag:pb_variable<FieldT>,
}
 impl ALU_umul_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                    opcode_indicators:pb_variable_array<FieldT>,
                    desval:word_variable_gadget<FieldT>,
                    arg1val:word_variable_gadget<FieldT>,
                    arg2val:word_variable_gadget<FieldT>,
                    flag:pb_variable<FieldT>,
                    mull_result:pb_variable<FieldT>,
                    mull_flag:pb_variable<FieldT>,
                    umulh_result:pb_variable<FieldT>,
                    umulh_flag:pb_variable<FieldT>,
                    annotation_prefix:std::string)->Self
    
    {
        mull_bits.insert(mull_bits.end(), mul_result.bits.begin(), mul_result.bits.begin()+pb.ap.w);
        umulh_bits.insert(umulh_bits.end(), mul_result.bits.begin()+pb.ap.w, mul_result.bits.begin()+2*pb.ap.w);

        pack_mull_result.reset(packing_gadget::<FieldT>::new(pb, mull_bits, mull_result, format!("{} pack_mull_result",self.annotation_prefix)));
        pack_umulh_result.reset(packing_gadget::<FieldT>::new(pb, umulh_bits, umulh_result, format!("{} pack_umulh_result",self.annotation_prefix)));

        result_flag.allocate(pb, format!("{} result_flag",self.annotation_prefix));
        compute_flag.reset(disjunction_gadget::<FieldT>::new(pb, umulh_bits, result_flag, format!("{} compute_flag",self.annotation_prefix)));
        // ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, mull_result, mull_flag, annotation_prefix),
        Self{mul_result:dual_variable_gadget::<FieldT>::new(pb, 2*pb.ap.w, format!("{} mul_result",self.annotation_prefix)),
        mull_result, mull_flag, umulh_result, umulh_flag}
    }
 
}


// pub fn test_ALU_mull_gadget(w:usize);


// pub fn test_ALU_umulh_gadget(w:usize);


pub struct  ALU_smul_gadget {

mul_result:    dual_variable_gadget<FieldT>,
smulh_bits:    pb_variable_array<FieldT>,

top:    pb_variable<FieldT>,
pack_top:    std::shared_ptr<packing_gadget<FieldT> >,

is_top_empty_aux:    pb_variable<FieldT> is_top_empty,,
is_top_full_aux:    pb_variable<FieldT> is_top_full,,

result_flag:    pb_variable<FieldT>,
pack_smulh_result:    std::shared_ptr<packing_gadget<FieldT> >,

    smulh_result:pb_variable<FieldT>,
    smulh_flag:pb_variable<FieldT>,
}
 impl ALU_smul_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                    opcode_indicators:pb_variable_array<FieldT>,
                    desval:word_variable_gadget<FieldT>,
                    arg1val:word_variable_gadget<FieldT>,
                    arg2val:word_variable_gadget<FieldT>,
                    flag:pb_variable<FieldT>,
                    smulh_result:pb_variable<FieldT>,
                    smulh_flag:pb_variable<FieldT>,
                    annotation_prefix:std::string)->Self
   
    {
        smulh_bits.insert(smulh_bits.end(), mul_result.bits.begin()+pb.ap.w, mul_result.bits.begin()+2*pb.ap.w);

        pack_smulh_result.reset(packing_gadget::<FieldT>::new(pb, smulh_bits, smulh_result, format!("{} pack_smulh_result",self.annotation_prefix)));

        top.allocate(pb, format!("{} top",self.annotation_prefix));
        pack_top.reset(packing_gadget::<FieldT>::new(pb, pb_variable_array<FieldT>(mul_result.bits.begin() + pb.ap.w-1, mul_result.bits.begin() + 2*pb.ap.w), top,
                                                  format!("{} pack_top",self.annotation_prefix)));

        is_top_empty.allocate(pb, format!("{} is_top_empty",self.annotation_prefix));
        is_top_empty_aux.allocate(pb, format!("{} is_top_empty_aux",self.annotation_prefix));

        is_top_full.allocate(pb, format!("{} is_top_full",self.annotation_prefix));
        is_top_full_aux.allocate(pb, format!("{} is_top_full_aux",self.annotation_prefix));

        result_flag.allocate(pb, format!("{} result_flag",self.annotation_prefix));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, smulh_result, smulh_flag, annotation_prefix),
        Self{mul_result:dual_variable_gadget::<FieldT>::new(pb, 2*pb.ap.w+1, format!("{} mul_result",self.annotation_prefix)), /* see witness map for explanation for 2w+1 */
        smulh_result, smulh_flag}
    }
 
}


// pub fn test_ALU_smulh_gadget(w:usize);


pub struct  ALU_divmod_gadget {
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

B_inv:    pb_variable<FieldT>,
B_nonzero:    pb_variable<FieldT>,
A_aux:    pb_variable<FieldT>,
r_less_B:    std::shared_ptr<comparison_gadget<FieldT> >,

    udiv_result:pb_variable<FieldT>,
    udiv_flag:pb_variable<FieldT>,
    umod_result:pb_variable<FieldT>,
    umod_flag:pb_variable<FieldT>,
}
 impl ALU_divmod_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                      opcode_indicators:pb_variable_array<FieldT>,
                      desval:word_variable_gadget<FieldT>,
                      arg1val:word_variable_gadget<FieldT>,
                      arg2val:word_variable_gadget<FieldT>,
                      flag:pb_variable<FieldT>,
                      udiv_result:pb_variable<FieldT>,
                      udiv_flag:pb_variable<FieldT>,
                      umod_result:pb_variable<FieldT>,
                      umod_flag:pb_variable<FieldT>,
                      annotation_prefix:std::string)->Self
   
    {
        B_inv.allocate(pb, format!("{} B_inv",self.annotation_prefix));
        B_nonzero.allocate(pb, format!("{} B_nonzer",self.annotation_prefix));
        A_aux.allocate(pb, format!("{} A_aux",self.annotation_prefix));
        r_less_B.reset(comparison_gadget::<FieldT>::new(pb, pb.ap.w, umod_result, arg2val.packed,
                                                     B_nonzero, ONE, format!("{} r_less_B",self.annotation_prefix)));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, udiv_result, udiv_flag, annotation_prefix),
        Self{udiv_result, udiv_flag, umod_result, umod_flag}
    }
 
}


// pub fn test_ALU_udiv_gadget(w:usize);


// pub fn test_ALU_umod_gadget(w:usize);


pub struct  ALU_shr_shl_gadget {

reversed_input:    pb_variable<FieldT>,
pack_reversed_input:    std::shared_ptr<packing_gadget<FieldT> >,

barrel_right_internal:    pb_variable_array<FieldT>,
shifted_out_bits:    std::vector<pb_variable_array<FieldT> >,

is_oversize_shift:    pb_variable<FieldT>,
check_oversize_shift:    std::shared_ptr<disjunction_gadget<FieldT> >,
result:    pb_variable<FieldT>,

result_bits:    pb_variable_array<FieldT>,
unpack_result:    std::shared_ptr<packing_gadget<FieldT> >,
reversed_result:    pb_variable<FieldT>,
pack_reversed_result:    std::shared_ptr<packing_gadget<FieldT> >,

shr_result:    pb_variable<FieldT>,
shr_flag:    pb_variable<FieldT>,
shl_result:    pb_variable<FieldT>,
shl_flag:    pb_variable<FieldT>,

logw:    usize,
}
 impl ALU_shr_shl_gadget<FieldT> {
    pub fn new(pb :tinyram_protoboard<FieldT> ,
                       opcode_indicators:pb_variable_array<FieldT>,
                       desval:word_variable_gadget<FieldT>,
                       arg1val:word_variable_gadget<FieldT>,
                       arg2val:word_variable_gadget<FieldT>,
                       flag:pb_variable<FieldT>,
                       shr_result:pb_variable<FieldT>,
                       shr_flag:pb_variable<FieldT>,
                       shl_result:pb_variable<FieldT>,
                       shl_flag:pb_variable<FieldT>,
                       annotation_prefix:std::string)->Self
   
    {
        logw = ffec::log2(pb.ap.w);

        reversed_input.allocate(pb, format!("{} reversed_input",self.annotation_prefix));
        pack_reversed_input.reset(
            packing_gadget::<FieldT>::new(pb, pb_variable_array<FieldT>(arg1val.bits.rbegin(), arg1val.bits.rend()),
                                       reversed_input,
                                       format!("{} pack_reversed_input",self.annotation_prefix)));

        barrel_right_internal.allocate(pb, logw+1, format!("{} barrel_right_internal",self.annotation_prefix));

        shifted_out_bits.resize(logw);
        for i in 0..logw
        {
            shifted_out_bits[i].allocate(pb, 1u64<<i, format!("{} shifted_out_bits_{}",self.annotation_prefix, i));
        }

        is_oversize_shift.allocate(pb, format!("{} is_oversize_shift",self.annotation_prefix));
        check_oversize_shift.reset(
            disjunction_gadget::<FieldT>::new(pb,
                                           pb_variable_array<FieldT>(arg2val.bits.begin()+logw, arg2val.bits.end()),
                                           is_oversize_shift,
                                           format!("{} check_oversize_shift",self.annotation_prefix)));
        result.allocate(pb, format!("{} result",self.annotation_prefix));

        result_bits.allocate(pb, pb.ap.w, format!("{} result_bits",self.annotation_prefix));
        unpack_result.reset(
            packing_gadget::<FieldT>::new(pb, result_bits, result, //barrel_right_internal[logw],
                                       format!("{} unpack_result",self.annotation_prefix)));

        reversed_result.allocate(pb, format!("{} reversed_result",self.annotation_prefix));
        pack_reversed_result.reset(
            packing_gadget::<FieldT>::new(pb, pb_variable_array<FieldT>(result_bits.rbegin(), result_bits.rend()),
                                       reversed_result,
                                       format!("{} pack_reversed_result",self.annotation_prefix)));
//  ALU_arithmetic_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, shr_result, shr_flag, annotation_prefix),
       Self{ shr_result, shr_flag, shl_result, shl_flag}
    }
 
}

// 
// pub fn test_ALU_shr_gadget(w:usize);

// 
// pub fn test_ALU_shl_gadget(w:usize);



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_arithmetic;

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

// use  <functional>

use ffec::common::profiling;
use ffec::common::utils;



/* the code here is full of template lambda magic, but it is better to
   have limited presence of such code than to have code duplication in
   testing functions, which basically do the same thing: brute force
   the range of inputs which different success predicates */

// template<class T, typename FieldT>
type initializer_fn< T,  FieldT> =
    fn(tinyram_protoboard<FieldT>&,    // pb
                   pb_variable_array<FieldT>&,       // opcode_indicators
                   word_variable_gadget<FieldT>&, // desval
                   word_variable_gadget<FieldT>&, // arg1val
                   word_variable_gadget<FieldT>&, // arg2val
                   pb_variable<FieldT>&,             // flag
                   pb_variable<FieldT>&,             // result
                   pb_variable<FieldT>&              // result_flag
                  )->T;

// template<class T, typename FieldT>
pub fn brute_force_arithmetic_gadget< T,  FieldT>( w:usize,
                                    opcode:usize,
                                    initializer:initializer_fn<T, FieldT>,
                                   res_function:fn(usize,bool,usize,usize)->usize,
                                   flag_function:fn(usize,bool,usize,usize)->bool )
{
/* parameters for res_function and flag_function are both desval, flag, arg1val, arg2val */

    print!("testing on all {} bit inputs\n", w);

    tinyram_architecture_params ap(w, 16);
    let mut  P=tinyram_program::new(); 
    P.instructions = generate_tinyram_prelude(ap);
    let mut  pb=tinyram_protoboard::<FieldT>::new(ap, P.size(), 0, 10);

    let mut  opcode_indicators=pb_variable_array::<FieldT>::new();
    opcode_indicators.allocate(pb, 1u64<<ap.opcode_width(), "opcode_indicators");
    for i in 0..1u64<<ap.opcode_width()
    {
        pb.val(opcode_indicators[i]) = if i == opcode {FieldT::one()} else{FieldT::zero()}
    }

     let mut desval=word_variable_gadget::<FieldT>::new(pb, "desval");
    desval.generate_r1cs_constraints(true);
    let mut  arg1val=word_variable_gadget::<FieldT>::new(pb, "arg1val");
    arg1val.generate_r1cs_constraints(true);
    let mut  arg2val=word_variable_gadget::<FieldT>::new(pb, "arg2val");
    arg2val.generate_r1cs_constraints(true);
     let mut flag=pb_variable::<FieldT>::new(); 
    flag.allocate(pb, "flag");
     let mut  result=pb_variable::<FieldT>::new(); 
     result.allocate(pb, "result");
     let mut  result_flag=pb_variable::<FieldT>::new(); 
     result_flag.allocate(pb, "result_flag");

    let mut g=T::new();
    g.reset(initializer(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag));
    g.generate_r1cs_constraints();

    for des in 0..(1u32 << w)
    {
        pb.val(desval.packed) = FieldT(des);
        desval.generate_r1cs_witness_from_packed();

        for f in 0..=1
        {
            pb.val(flag) = if f {FieldT::one()} else{FieldT::zero()}

            for arg1 in 0..(1u32 << w)
            {
                pb.val(arg1val.packed) = FieldT(arg1);
                arg1val.generate_r1cs_witness_from_packed();

                for arg2 in 0..(1u32 << w)
                {
                    pb.val(arg2val.packed) = FieldT(arg2);
                    arg2val.generate_r1cs_witness_from_packed();

                    let  res = res_function(des, f, arg1, arg2);
                    booletl res_f = flag_function(des, f, arg1, arg2);
// #ifdef DEBUG
                    print!("with the following parameters: flag = {}"
                           ", desval = {} ({})"
                           ", arg1val = {} ({})"
                           ", arg2val = {} ({})"
                           ". expected result: {} ({}), expected flag: {}\n",
                           f,
                           des, ffec::from_twos_complement(des, w),
                           arg1, ffec::from_twos_complement(arg1, w),
                           arg2, ffec::from_twos_complement(arg2, w),
                           res, ffec::from_twos_complement(res, w), res_f);
//#endif
                    g.generate_r1cs_witness();
// #ifdef DEBUG
                    print!("result: ");
                    pb.val(result).print();
                    print!("flag: ");
                    pb.val(result_flag).print();
//#endif
                    assert!(pb.is_satisfied());
                    assert!(pb.val(result) == FieldT(res));
                    assert!(pb.val(result_flag) == (if res_f  {FieldT::one()} else {FieldT::zero()}));
                }
            }
        }
    }
}

/* and */
impl ALU_and_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    for i in 0..self.pb.ap.w
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { self.arg1val.bits[i] },
                { self.arg2val.bits[i] },
                { self.res_word[i] }),
            format!("{} res_word_{}",self.annotation_prefix, i));
    }

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
    not_all_zeros.generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        format!("{} result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    for i in 0..self.pb.ap.w
    {
        let  b1 = self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        let b2 = self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = if b1 && b2 {FieldT::one()} else{FieldT::zero()}
    }

    pack_result.generate_r1cs_witness_from_bits();
    not_all_zeros.generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(not_all_zeros_result);
}

}

pub fn test_ALU_and_gadget(w:usize)
{
    ffec::print_time("starting and test");
    brute_force_arithmetic_gadget::<ALU_and_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_AND,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_and_gadget<FieldT>{
                                                                      return new ALU_and_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_and_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return x & y; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return (x & y) == 0; });
    ffec::print_time("and tests successful");
}

/* or */
impl ALU_or_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    for i in 0..self.pb.ap.w
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { ONE, self.arg1val.bits[i] * (-1) },
                { ONE, self.arg2val.bits[i] * (-1) },
                { ONE, self.res_word[i] * (-1) }),
            format!("{} res_word_{}",self.annotation_prefix, i));
    }

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
    not_all_zeros.generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint::<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        format!("{} result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    for i in 0..self.pb.ap.w
    {
        let b1= self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        let b2= self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = if b1 || b2 {FieldT::one()} else{FieldT::zero()}
    }

    pack_result.generate_r1cs_witness_from_bits();
    not_all_zeros.generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}

}
pub fn test_ALU_or_gadget(w:usize)
{
    ffec::print_time("starting or test");
    brute_force_arithmetic_gadget<ALU_or_gadget<FieldT>, FieldT>(w,
                                                                 tinyram_opcode_OR,
                                                                 |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                     pb_variable_array<FieldT>,
desval:&                                                                     word_variable_gadget<FieldT>,
arg1val:&                                                                     word_variable_gadget<FieldT>,
arg2val:&                                                                     word_variable_gadget<FieldT>,
flag:&                                                                     pb_variable<FieldT>,
result:&                                                                     pb_variable<FieldT>,
result_flag:&                                                                     pb_variable<FieldT>| ->
                                                                 ALU_or_gadget<FieldT>{
                                                                     return new ALU_or_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_or_gadget");
                                                                 },
                                                                 |des:usize, f:bool, x:usize, y:usize| -> usize { return x | y; },
                                                                 |des:usize, f:bool, x:usize, y:usize| -> bool { return (x | y) == 0; });
    ffec::print_time("or tests successful");
}

/* xor */
impl ALU_xor_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    for i in 0..self.pb.ap.w
    {
        /* a = b ^ c <=> a = b + c - 2*b*c, (2*b)*c = b+c - a */
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { self.arg1val.bits[i] * 2},
                { self.arg2val.bits[i] },
                { self.arg1val.bits[i], self.arg2val.bits[i], self.res_word[i] * (-1) }),
            format!("{} res_word_{}",self.annotation_prefix, i));
    }

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
    not_all_zeros.generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        format!("{} result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    for i in 0..self.pb.ap.w
    {
        let b1= self.pb.val(self.arg1val.bits[i]) == FieldT::one();
        let b2= self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = if b1 ^ b2 {FieldT::one()} else{FieldT::zero()}
    }

    pack_result.generate_r1cs_witness_from_bits();
    not_all_zeros.generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}

}
pub fn test_ALU_xor_gadget(w:usize)
{
    ffec::print_time("starting xor test");
    brute_force_arithmetic_gadget<ALU_xor_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_XOR,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_xor_gadget<FieldT>{
                                                                      return new ALU_xor_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_xor_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return x ^ y; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return (x ^ y) == 0; });
    ffec::print_time("xor tests successful");
}

/* not */
impl ALU_not_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    for i in 0..self.pb.ap.w
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint<FieldT>(
                { ONE },
                { ONE, self.arg2val.bits[i] * (-1) },
                { self.res_word[i] }),
            format!("{} res_word_{}",self.annotation_prefix, i));
    }

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
    not_all_zeros.generate_r1cs_constraints();

    /* result_flag = 1 - not_all_zeros = result is 0^w */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.not_all_zeros_result * (-1) },
            { self.result_flag }),
        format!("{} result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    for i in 0..self.pb.ap.w
    {
        let b2= self.pb.val(self.arg2val.bits[i]) == FieldT::one();

        self.pb.val(self.res_word[i]) = if !b2 {FieldT::one()} else{FieldT::zero()}
    }

    pack_result.generate_r1cs_witness_from_bits();
    not_all_zeros.generate_r1cs_witness();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.not_all_zeros_result);
}
}

pub fn test_ALU_not_gadget(w:usize)
{
    ffec::print_time("starting not test");
    brute_force_arithmetic_gadget<ALU_not_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_NOT,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_not_gadget<FieldT>{
                                                                      return new ALU_not_gadget<FieldT>(pb, opcode_indicators,desval, arg1val, arg2val, flag, result, result_flag, "ALU_not_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return (1u64<<w)-1-y; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return ((1u64<<w)-1-y) == 0; });
    ffec::print_time("not tests successful");
}

/* add */
impl ALU_add_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* addition_result = 1 * (arg1val + arg2val) */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.packed, self.arg2val.packed },
            { self.addition_result }),
        format!("{} addition_result",self.annotation_prefix));

    /* unpack into bits */
    unpack_addition.generate_r1cs_constraints(true);

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
}


pub fn generate_r1cs_witness()
{
    self.pb.val(addition_result) = self.pb.val(self.arg1val.packed) + self.pb.val(self.arg2val.packed);
    unpack_addition.generate_r1cs_witness_from_packed();
    pack_result.generate_r1cs_witness_from_bits();
}

}
pub fn test_ALU_add_gadget(w:usize)
{
    ffec::print_time("starting add test");
    brute_force_arithmetic_gadget<ALU_add_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_ADD,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_add_gadget<FieldT>{
                                                                      return new ALU_add_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_add_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return (x+y) % (1u64<<w); },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return (x+y) >= (1u64<<w); });
    ffec::print_time("add tests successful");
}

/* sub */
impl ALU_sub_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* intermediate_result = 2^w + (arg1val - arg2val) */
    FieldT twoi = FieldT::one();

    let (mut a,mut b,mut c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());

    a.add_term(0, 1);
    for i in 0..self.pb.ap.w
    {
        twoi = twoi + twoi;
    }
    b.add_term(0, twoi);
    b.add_term(self.arg1val.packed, 1);
    b.add_term(self.arg2val.packed, -1);
    c.add_term(intermediate_result, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), format!("{} main_constraint",self.annotation_prefix));

    /* unpack into bits */
    unpack_intermediate.generate_r1cs_constraints(true);

    /* generate result */
    pack_result.generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.negated_flag * (-1) },
            { self.result_flag }),
        format!("{} result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    FieldT twoi = FieldT::one();
    for i in 0..self.pb.ap.w
    {
        twoi = twoi + twoi;
    }

    self.pb.val(intermediate_result) = twoi + self.pb.val(self.arg1val.packed) - self.pb.val(self.arg2val.packed);
    unpack_intermediate.generate_r1cs_witness_from_packed();
    pack_result.generate_r1cs_witness_from_bits();
    self.pb.val(self.result_flag) = FieldT::one() - self.pb.val(self.negated_flag);
}
}

pub fn test_ALU_sub_gadget(w:usize)
{
    ffec::print_time("starting sub test");
    brute_force_arithmetic_gadget<ALU_sub_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_SUB,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
                                                                      pb_variable<FieldT> &result_flag) ->
                                                                  ALU_sub_gadget<FieldT>{
                                                                      return new ALU_sub_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_sub_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                      let unsigned_result = ((1u64<<w) + x - y) % (1u64<<w);
                                                                      return unsigned_result;
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                      let  msb = ((1u64<<w) + x - y) >> w;
                                                                      return (msb == 0);
                                                                  });
    ffec::print_time("sub tests successful");
}

/* mov */
impl ALU_mov_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg2val.packed },
            { self.result }),
        format!("{} mov_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.flag },
            { self.result_flag }),
        format!("{} mov_result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(self.result) = self.pb.val(self.arg2val.packed);
    self.pb.val(self.result_flag) = self.pb.val(self.flag);
}
}

pub fn test_ALU_mov_gadget(w:usize)
{
    ffec::print_time("starting mov test");
    brute_force_arithmetic_gadget<ALU_mov_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_MOV,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_mov_gadget<FieldT>{
                                                                      return new ALU_mov_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_mov_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return y; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return f; });
    ffec::print_time("mov tests successful");
}

/* cmov */
impl ALU_cmov_gadget<FieldT>{
pub fn generate_r1cs_constraints()
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
        format!("{} cmov_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.flag },
            { self.result_flag }),
        format!("{} cmov_result_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(self.result) = ((self.pb.val(self.flag) == FieldT::one()) ?
                                  self.pb.val(self.arg2val.packed) :
                                  self.pb.val(self.desval.packed));
    self.pb.val(self.result_flag) = self.pb.val(self.flag);
}
}

pub fn test_ALU_cmov_gadget(w:usize)
{
    ffec::print_time("starting cmov test");
    brute_force_arithmetic_gadget<ALU_cmov_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMOV,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_cmov_gadget<FieldT>{
                                                                       return new ALU_cmov_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag, "ALU_cmov_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize { return if f  {y} else {des} },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool { return f; });
    ffec::print_time("cmov tests successful");
}

/* unsigned comparison */
impl ALU_cmp_gadget<FieldT>{
pub fn generate_r1cs_constraints()
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
        format!("{} cmpa_result_flag",self.annotation_prefix));

    /* copy over results */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpe_result }),
        format!("{} cmpe_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpa_result }),
        format!("{} cmpa_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpae_result }),
        format!("{} cmpae_result",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
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
}

pub fn test_ALU_cmpe_gadget(w:usize)
{
    ffec::print_time("starting cmpe test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPE,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_cmp_gadget<FieldT>{
                                                                      pb_variable<FieldT> cmpa_result; cmpa_result.allocate(pb, "cmpa_result");
                                                                      pb_variable<FieldT> cmpa_result_flag; cmpa_result_flag.allocate(pb, "cmpa_result_flag");
                                                                      pb_variable<FieldT> cmpae_result; cmpae_result.allocate(pb, "cmpae_result");
                                                                      pb_variable<FieldT> cmpae_result_flag; cmpae_result_flag.allocate(pb, "cmpae_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        result, result_flag,
                                                                                                        cmpa_result, cmpa_result_flag,
                                                                                                        cmpae_result, cmpae_result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return des; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return x == y; });
    ffec::print_time("cmpe tests successful");
}


pub fn test_ALU_cmpa_gadget(w:usize)
{
    ffec::print_time("starting cmpa test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPA,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_cmp_gadget<FieldT>{
                                                                      pb_variable<FieldT> cmpe_result; cmpe_result.allocate(pb, "cmpe_result");
                                                                      pb_variable<FieldT> cmpe_result_flag; cmpe_result_flag.allocate(pb, "cmpe_result_flag");
                                                                      pb_variable<FieldT> cmpae_result; cmpae_result.allocate(pb, "cmpae_result");
                                                                      pb_variable<FieldT> cmpae_result_flag; cmpae_result_flag.allocate(pb, "cmpae_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        cmpe_result, cmpe_result_flag,
                                                                                                        result, result_flag,
                                                                                                        cmpae_result, cmpae_result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return des; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return x > y; });
    ffec::print_time("cmpa tests successful");
}


pub fn test_ALU_cmpae_gadget(w:usize)
{
    ffec::print_time("starting cmpae test");
    brute_force_arithmetic_gadget<ALU_cmp_gadget<FieldT>, FieldT>(w,
                                                                  tinyram_opcode_CMPAE,
                                                                  |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                      pb_variable_array<FieldT>,
desval:&                                                                      word_variable_gadget<FieldT>,
arg1val:&                                                                      word_variable_gadget<FieldT>,
arg2val:&                                                                      word_variable_gadget<FieldT>,
flag:&                                                                      pb_variable<FieldT>,
result:&                                                                      pb_variable<FieldT>,
result_flag:&                                                                      pb_variable<FieldT>| ->
                                                                  ALU_cmp_gadget<FieldT>{
                                                                      pb_variable<FieldT> cmpe_result; cmpe_result.allocate(pb, "cmpe_result");
                                                                      pb_variable<FieldT> cmpe_result_flag; cmpe_result_flag.allocate(pb, "cmpe_result_flag");
                                                                      pb_variable<FieldT> cmpa_result; cmpa_result.allocate(pb, "cmpa_result");
                                                                      pb_variable<FieldT> cmpa_result_flag; cmpa_result_flag.allocate(pb, "cmpa_result_flag");
                                                                      return new ALU_cmp_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                        cmpe_result, cmpe_result_flag,
                                                                                                        cmpa_result, cmpa_result_flag,
                                                                                                        result, result_flag, "ALU_cmp_gadget");
                                                                  },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> usize { return des; },
                                                                  |des:usize, f:bool, x:usize, y:usize| -> bool { return x >= y; });
    ffec::print_time("cmpae tests successful");
}

/* signed comparison */
impl ALU_cmps_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* negate sign bits */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.arg1val.bits[self.pb.ap.w-1] * (-1) },
            { negated_arg1val_sign }),
        format!("{} negated_arg1val_sign",self.annotation_prefix));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, self.arg2val.bits[self.pb.ap.w-1] * (-1) },
            { negated_arg2val_sign }),
        format!("{} negated_arg2val_sign",self.annotation_prefix));

    /* pack */
    pack_modified_arg1.generate_r1cs_constraints(false);
    pack_modified_arg2.generate_r1cs_constraints(false);

    /* compare */
    comparator.generate_r1cs_constraints();

    /* copy over results */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpg_result }),
        format!("{} cmpg_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.desval.packed },
            { cmpge_result }),
        format!("{} cmpge_result",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    /* negate sign bits */
    self.pb.val(negated_arg1val_sign) = FieldT::one() - self.pb.val(self.arg1val.bits[self.pb.ap.w-1]);
    self.pb.val(negated_arg2val_sign) = FieldT::one() - self.pb.val(self.arg2val.bits[self.pb.ap.w-1]);

    /* pack */
    pack_modified_arg1.generate_r1cs_witness_from_bits();
    pack_modified_arg2.generate_r1cs_witness_from_bits();

    /* produce result */
    comparator.generate_r1cs_witness();

    self.pb.val(cmpg_result) = self.pb.val(self.desval.packed);
    self.pb.val(cmpge_result) = self.pb.val(self.desval.packed);
}

}
pub fn test_ALU_cmpg_gadget(w:usize)
{
    ffec::print_time("starting cmpg test");
    brute_force_arithmetic_gadget<ALU_cmps_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMPG,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_cmps_gadget<FieldT>{
                                                                       pb_variable<FieldT> cmpge_result; cmpge_result.allocate(pb, "cmpge_result");
                                                                       pb_variable<FieldT> cmpge_result_flag; cmpge_result_flag.allocate(pb, "cmpge_result_flag");
                                                                       return new ALU_cmps_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          cmpge_result, cmpge_result_flag, "ALU_cmps_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize { return des; },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                       return (ffec::from_twos_complement(x, w) >
                                                                               ffec::from_twos_complement(y, w));
                                                                   });
    ffec::print_time("cmpg tests successful");
}


pub fn test_ALU_cmpge_gadget(w:usize)
{
    ffec::print_time("starting cmpge test");
    brute_force_arithmetic_gadget<ALU_cmps_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_CMPGE,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_cmps_gadget<FieldT>{
                                                                       pb_variable<FieldT> cmpg_result; cmpg_result.allocate(pb, "cmpg_result");
                                                                       pb_variable<FieldT> cmpg_result_flag; cmpg_result_flag.allocate(pb, "cmpg_result_flag");
                                                                       return new ALU_cmps_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          cmpg_result, cmpg_result_flag,
                                                                                                          result, result_flag, "ALU_cmps_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize { return des; },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                       return (ffec::from_twos_complement(x, w) >=
                                                                               ffec::from_twos_complement(y, w));
                                                                   });
    ffec::print_time("cmpge tests successful");
}

impl ALU_umul_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* do multiplication */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.arg1val.packed },
            { self.arg2val.packed },
            { mul_result.packed }),
        format!("{} main_constraint",self.annotation_prefix));
    mul_result.generate_r1cs_constraints(true);

    /* pack result */
    pack_mull_result.generate_r1cs_constraints(false);
    pack_umulh_result.generate_r1cs_constraints(false);

    /* compute flag */
    compute_flag.generate_r1cs_constraints();

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.result_flag },
            { mull_flag }),
        format!("{} mull_flag",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.result_flag },
            { umulh_flag }),
        format!("{} umulh_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    /* do multiplication */
    self.pb.val(mul_result.packed) = self.pb.val(self.arg1val.packed) * self.pb.val(self.arg2val.packed);
    mul_result.generate_r1cs_witness_from_packed();

    /* pack result */
    pack_mull_result.generate_r1cs_witness_from_bits();
    pack_umulh_result.generate_r1cs_witness_from_bits();

    /* compute flag */
    compute_flag.generate_r1cs_witness();

    self.pb.val(mull_flag) = self.pb.val(self.result_flag);
    self.pb.val(umulh_flag) = self.pb.val(self.result_flag);
}

}
pub fn test_ALU_mull_gadget(w:usize)
{
    ffec::print_time("starting mull test");
    brute_force_arithmetic_gadget<ALU_umul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_MULL,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_umul_gadget<FieldT>{
                                                                       pb_variable<FieldT> umulh_result; umulh_result.allocate(pb, "umulh_result");
                                                                       pb_variable<FieldT> umulh_flag; umulh_flag.allocate(pb, "umulh_flag");
                                                                       return new ALU_umul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          umulh_result, umulh_flag,
                                                                                                          "ALU_umul_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize { return (x*y) % (1u64<<w); },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                       return ((x*y) >> w) != 0;
                                                                   });
    ffec::print_time("mull tests successful");
}


pub fn test_ALU_umulh_gadget(w:usize)
{
    ffec::print_time("starting umulh test");
    brute_force_arithmetic_gadget<ALU_umul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_UMULH,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_umul_gadget<FieldT>{
                                                                       pb_variable<FieldT> mull_result; mull_result.allocate(pb, "mull_result");
                                                                       pb_variable<FieldT> mull_flag; mull_flag.allocate(pb, "mull_flag");
                                                                       return new ALU_umul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          mull_result, mull_flag,
                                                                                                          result, result_flag,
                                                                                                          "ALU_umul_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize { return (x*y) >> w; },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                       return ((x*y) >> w) != 0;
                                                                   });
    ffec::print_time("umulh tests successful");
}

impl ALU_smul_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* do multiplication */
    /*
      from two's complement: (packed - 2^w * bits[w-1])
      to two's complement: lower order bits of 2^{2w} + result_of_*
    */

    let (mut a,mut b,mut c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a.add_term(self.arg1val.packed, 1);
    a.add_term(self.arg1val.bits[self.pb.ap.w-1], -(FieldT(2)^self.pb.ap.w));
    b.add_term(self.arg2val.packed, 1);
    b.add_term(self.arg2val.bits[self.pb.ap.w-1], -(FieldT(2)^self.pb.ap.w));
    c.add_term(mul_result.packed, 1);
    c.add_term(ONE, -(FieldT(2)^(2*self.pb.ap.w)));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), format!("{} main_constraint",self.annotation_prefix));

    mul_result.generate_r1cs_constraints(true);

    /* pack result */
    pack_smulh_result.generate_r1cs_constraints(false);

    /* compute flag */
    pack_top.generate_r1cs_constraints(false);

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
        format!("{} I*X=1-R (is_top_empty)",self.annotation_prefix));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_empty },
            { top },
            { ONE * 0 }),
        format!("{} R*X=0 (is_top_full)",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_full_aux },
            { top, ONE * (1l-(1u64<<(self.pb.ap.w+1))) },
            { ONE, is_top_full * (-1) }),
        format!("{} I*X=1-R (is_top_full)",self.annotation_prefix));
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { is_top_full },
            { top, ONE * (1l-(1u64<<(self.pb.ap.w+1))) },
            { ONE * 0 }),
        format!("{} R*X=0 (is_top_full)",self.annotation_prefix));

    /* smulh_flag = 1 - (is_top_full + is_top_empty) */
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { ONE, is_top_full * (-1), is_top_empty * (-1) },
            { smulh_flag }),
        format!("{} smulh_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
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
    pack_smulh_result.generate_r1cs_witness_from_bits();

    /* compute flag */
    pack_top.generate_r1cs_witness_from_bits();
    let  topval = self.pb.val(top).as_ulong();

    if topval == 0
    {
        self.pb.val(is_top_empty) = FieldT::one();
        self.pb.val(is_top_empty_aux) = FieldT::zero();
    }
    else
    {
        self.pb.val(is_top_empty) = FieldT::zero();
        self.pb.val(is_top_empty_aux) = self.pb.val(top).inverse();
    }

    if topval == ((1u64<<(self.pb.ap.w+1))-1)
    {
        self.pb.val(is_top_full) = FieldT::one();
        self.pb.val(is_top_full_aux) = FieldT::zero();
    }
    else
    {
        self.pb.val(is_top_full) = FieldT::zero();
        self.pb.val(is_top_full_aux) = (self.pb.val(top)-FieldT((1u64<<(self.pb.ap.w+1))-1)).inverse();
    }

    /* smulh_flag = 1 - (is_top_full + is_top_empty) */
    self.pb.val(smulh_flag) = FieldT::one() - (self.pb.val(is_top_full) + self.pb.val(is_top_empty));
}

}
pub fn test_ALU_smulh_gadget(w:usize)
{
    ffec::print_time("starting smulh test");
    brute_force_arithmetic_gadget<ALU_smul_gadget<FieldT>, FieldT>(w,
                                                                   tinyram_opcode_SMULH,
                                                                   |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                       pb_variable_array<FieldT>,
desval:&                                                                       word_variable_gadget<FieldT>,
arg1val:&                                                                       word_variable_gadget<FieldT>,
arg2val:&                                                                       word_variable_gadget<FieldT>,
flag:&                                                                       pb_variable<FieldT>,
result:&                                                                       pb_variable<FieldT>,
result_flag:&                                                                       pb_variable<FieldT>| ->
                                                                   ALU_smul_gadget<FieldT>{
                                                                       return new ALU_smul_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                          result, result_flag,
                                                                                                          "ALU_smul_gadget");
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                       const usize res = ffec::to_twos_complement((ffec::from_twos_complement(x, w) * ffec::from_twos_complement(y, w)), 2*w);
                                                                       return res >> w;
                                                                   },
                                                                   |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                       const int res = ffec::from_twos_complement(x, w) * ffec::from_twos_complement(y, w);
                                                                       const int truncated_res = ffec::from_twos_complement(ffec::to_twos_complement(res, 2*w) & ((1u64<<w)-1), w);
                                                                       return (res != truncated_res);
                                                                   });
    ffec::print_time("smulh tests successful");
}

impl ALU_divmod_gadget<FieldT>{
pub fn generate_r1cs_constraints()
{
    /* B_inv * B = B_nonzero */
    let (mut a1,mut b1,mut c1)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a1.add_term(B_inv, 1);
    b1.add_term(self.arg2val.packed, 1);
    c1.add_term(B_nonzero, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a1, b1, c1), format!("{} B_inv*B=B_nonzero",self.annotation_prefix));

    /* (1-B_nonzero) * B = 0 */
    let (mut a2,mut b2,mut c2)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a2.add_term(ONE, 1);
    a2.add_term(B_nonzero, -1);
    b2.add_term(self.arg2val.packed, 1);
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a2, b2, c2), format!("{} (1-B_nonzero)*B=0",self.annotation_prefix));

    /* B * q + r = A_aux = A * B_nonzero */
    let (mut a3,mut b3,mut c3)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a3.add_term(self.arg2val.packed, 1);
    b3.add_term(udiv_result, 1);
    c3.add_term(A_aux, 1);
    c3.add_term(umod_result, -1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a3, b3, c3), format!("{} B*q+r=A_aux",self.annotation_prefix));

    let (mut a4,mut b4,mut c4)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a4.add_term(self.arg1val.packed, 1);
    b4.add_term(B_nonzero, 1);
    c4.add_term(A_aux, 1);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a4, b4, c4), format!("{} A_aux=A*B_nonzero",self.annotation_prefix));

    /* q * (1-B_nonzero) = 0 */
    let (mut a5,mut b5,mut c5)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a5.add_term(udiv_result, 1);
    b5.add_term(ONE, 1);
    b5.add_term(B_nonzero, -1);
    c5.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a5, b5, c5), format!("{} q*B_nonzero=0",self.annotation_prefix));

    /* A<B_gadget<FieldT>(B, r, less=B_nonzero, leq=ONE) */
    r_less_B.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    if self.pb.val(self.arg2val.packed) == FieldT::zero()
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

        const usize A = self.pb.val(self.arg1val.packed).as_ulong();
        const usize B = self.pb.val(self.arg2val.packed).as_ulong();

        self.pb.val(A_aux) = self.pb.val(self.arg1val.packed);

        self.pb.val(udiv_result) = FieldT(A / B);
        self.pb.val(umod_result) = FieldT(A % B);

        self.pb.val(udiv_flag) = FieldT::zero();
        self.pb.val(umod_flag) = FieldT::zero();
    }

    r_less_B.generate_r1cs_witness();
}

}
pub fn test_ALU_udiv_gadget(w:usize)
{
    ffec::print_time("starting udiv test");
    brute_force_arithmetic_gadget<ALU_divmod_gadget<FieldT>, FieldT>(w,
                                                                     tinyram_opcode_UDIV,
                                                                     |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                         pb_variable_array<FieldT>,
desval:&                                                                         word_variable_gadget<FieldT>,
arg1val:&                                                                         word_variable_gadget<FieldT>,
arg2val:&                                                                         word_variable_gadget<FieldT>,
flag:&                                                                         pb_variable<FieldT>,
result:&                                                                         pb_variable<FieldT>,
result_flag:&                                                                         pb_variable<FieldT>| ->
                                                                     ALU_divmod_gadget<FieldT>{
                                                                         pb_variable<FieldT> umod_result; umod_result.allocate(pb, "umod_result");
                                                                         pb_variable<FieldT> umod_flag; umod_flag.allocate(pb, "umod_flag");
                                                                         return new ALU_divmod_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                              result, result_flag,
                                                                                                              umod_result, umod_flag,
                                                                                                              "ALU_divmod_gadget");
                                                                     },
                                                                     |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                         return if y == 0 {0} else{x / y}
                                                                     },
                                                                     |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                         return (y == 0);
                                                                     });
    ffec::print_time("udiv tests successful");
}


pub fn test_ALU_umod_gadget(w:usize)
{
    ffec::print_time("starting umod test");
    brute_force_arithmetic_gadget<ALU_divmod_gadget<FieldT>, FieldT>(w,
                                                                     tinyram_opcode_UMOD,
                                                                     |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                         pb_variable_array<FieldT>,
desval:&                                                                         word_variable_gadget<FieldT>,
arg1val:&                                                                         word_variable_gadget<FieldT>,
arg2val:&                                                                         word_variable_gadget<FieldT>,
flag:&                                                                         pb_variable<FieldT>,
result:&                                                                         pb_variable<FieldT>,
result_flag:&                                                                         pb_variable<FieldT>| ->
                                                                     ALU_divmod_gadget<FieldT>{
                                                                         pb_variable<FieldT> udiv_result; udiv_result.allocate(pb, "udiv_result");
                                                                         pb_variable<FieldT> udiv_flag; udiv_flag.allocate(pb, "udiv_flag");
                                                                         return new ALU_divmod_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                              udiv_result, udiv_flag,
                                                                                                              result, result_flag,
                                                                                                              "ALU_divmod_gadget");
                                                                     },
                                                                     |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                         return if y == 0 {0} else{x % y}
                                                                     },
                                                                     |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                         return (y == 0);
                                                                     });
    ffec::print_time("umod tests successful");
}
impl   ALU_shr_shl_gadget<FieldT>{

pub fn generate_r1cs_constraints()
{
    /*
      select the input for barrel shifter:

      r = arg1val * opcode_indicators[SHR] + reverse(arg1val) * (1-opcode_indicators[SHR])
      r - reverse(arg1val) = (arg1val - reverse(arg1val)) * opcode_indicators[SHR]
    */
    pack_reversed_input.generate_r1cs_constraints(false);

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.arg1val.packed, reversed_input * (-1) },
            { self.opcode_indicators[tinyram_opcode_SHR] },
            { barrel_right_internal[0], reversed_input * (-1) }),
        format!("{} select_arg1val_or_reversed",self.annotation_prefix));

    /*
      do logw iterations of barrel shifts
    */
    for i in 0..logw
    {
        /* assert that shifted out part is bits */
        for j in 0..1u64<<i
        {
            generate_boolean_r1cs_constraint<FieldT>(self.pb, shifted_out_bits[i][j], format!("{} shifted_out_bits_%zu_{}",self.annotation_prefix, i, j));
        }

        /*
          add main shifting constraint


          old_result =
          (shifted_result * 2^(i+1) + shifted_out_part) * need_to_shift +
          (shfited_result) * (1-need_to_shift)

          old_result - shifted_result = (shifted_result * (2^(i+1) - 1) + shifted_out_part) * need_to_shift
        */
        let (mut a,mut b,mut c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());

        a.add_term(barrel_right_internal[i+1], (FieldT(2)^(i+1)) - FieldT::one());
        for j in 0..1u64<<i
        {
            a.add_term(shifted_out_bits[i][j], (FieldT(2)^j));
        }

        b.add_term(self.arg2val.bits[i], 1);

        c.add_term(barrel_right_internal[i], 1);
        c.add_term(barrel_right_internal[i+1], -1);

        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(a, b, c), format!("{} barrel_shift_{}",self.annotation_prefix, i));
    }

    /*
      get result as the logw iterations or zero if shift was oversized

      result = (1-is_oversize_shift) * barrel_right_internal[logw]
    */
    check_oversize_shift.generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE, is_oversize_shift * (-1) },
            { barrel_right_internal[logw] },
            { self.result }),
        format!("{} result",self.annotation_prefix));

    /*
      get reversed result for SHL
    */
    unpack_result.generate_r1cs_constraints(true);
    pack_reversed_result.generate_r1cs_constraints(false);

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
        format!("{} shr_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { self.result, reversed_result * (-1) },
            { self.opcode_indicators[tinyram_opcode_SHR] },
            { shr_result, reversed_result * (-1) }),
        format!("{} shl_result",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.bits[0] },
            { shr_flag }),
        format!("{} shr_flag",self.annotation_prefix));

    self.pb.add_r1cs_constraint(
        r1cs_constraint<FieldT>(
            { ONE },
            { self.arg1val.bits[self.pb.ap.w-1] },
            { shl_flag }),
        format!("{} shl_flag",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    /* select the input for barrel shifter */
    pack_reversed_input.generate_r1cs_witness_from_bits();

    self.pb.val(barrel_right_internal[0]) =
        (self.pb.val(self.opcode_indicators[tinyram_opcode_SHR]) == FieldT::one() ?
         self.pb.val(self.arg1val.packed) : self.pb.val(reversed_input));

    /*
      do logw iterations of barrel shifts.

      old_result =
      (shifted_result * 2^i + shifted_out_part) * need_to_shift +
      (shfited_result) * (1-need_to_shift)
    */

    for i in 0..logw
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
    check_oversize_shift.generate_r1cs_witness();
    self.pb.val(self.result) = (FieldT::one() - self.pb.val(is_oversize_shift)) * self.pb.val(barrel_right_internal[logw]);

    /*
      get reversed result for SHL
    */
    unpack_result.generate_r1cs_witness_from_packed();
    pack_reversed_result.generate_r1cs_witness_from_bits();

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

}
pub fn test_ALU_shr_gadget(w:usize)
{
    ffec::print_time("starting shr test");
    brute_force_arithmetic_gadget<ALU_shr_shl_gadget<FieldT>, FieldT>(w,
                                                                      tinyram_opcode_SHR,
                                                                      |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                          pb_variable_array<FieldT>,
desval:&                                                                          word_variable_gadget<FieldT>,
arg1val:&                                                                          word_variable_gadget<FieldT>,
arg2val:&                                                                          word_variable_gadget<FieldT>,
flag:&                                                                          pb_variable<FieldT>,
result:&                                                                          pb_variable<FieldT>,
result_flag:&                                                                          pb_variable<FieldT>| ->
                                                                      ALU_shr_shl_gadget<FieldT>{
                                                                          pb_variable<FieldT> shl_result; shl_result.allocate(pb, "shl_result");
                                                                          pb_variable<FieldT> shl_flag; shl_flag.allocate(pb, "shl_flag");
                                                                          return new ALU_shr_shl_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                                result, result_flag,
                                                                                                                shl_result, shl_flag,
                                                                                                                "ALU_shr_shl_gadget");
                                                                      },
                                                                      |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                          return (x >> y);
                                                                      },
                                                                      |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                          return (x & 1);
                                                                      });
    ffec::print_time("shr tests successful");
}


pub fn test_ALU_shl_gadget(w:usize)
{
    ffec::print_time("starting shl test");
    brute_force_arithmetic_gadget<ALU_shr_shl_gadget<FieldT>, FieldT>(w,
                                                                      tinyram_opcode_SHL,
                                                                      |pb :tinyram_protoboard<FieldT> ,
opcode_indicators:&                                                                          pb_variable_array<FieldT>,
desval:&                                                                          word_variable_gadget<FieldT>,
arg1val:&                                                                          word_variable_gadget<FieldT>,
arg2val:&                                                                          word_variable_gadget<FieldT>,
flag:&                                                                          pb_variable<FieldT>,
result:&                                                                          pb_variable<FieldT>,
result_flag:&                                                                          pb_variable<FieldT>| ->
                                                                      ALU_shr_shl_gadget<FieldT>{
                                                                          pb_variable<FieldT> shr_result; shr_result.allocate(pb, "shr_result");
                                                                          pb_variable<FieldT> shr_flag; shr_flag.allocate(pb, "shr_flag");
                                                                          return new ALU_shr_shl_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag,
                                                                                                                shr_result, shr_flag,
                                                                                                                result, result_flag,
                                                                                                                "ALU_shr_shl_gadget");
                                                                      },
                                                                      |des:usize, f:bool, x:usize, y:usize| -> usize {
                                                                          return (x << y) & ((1u64<<w)-1);
                                                                      },
                                                                      |des:usize, f:bool, x:usize, y:usize| -> bool {
                                                                          return (x >> (w-1));
                                                                      });
    ffec::print_time("shl tests successful");
}



//#endif
