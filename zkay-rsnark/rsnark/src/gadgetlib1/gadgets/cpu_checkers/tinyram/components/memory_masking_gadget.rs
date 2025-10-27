/** @file
 *****************************************************************************

 Declaration of interfaces for the TinyRAM memory masking gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_MASKING_GADGET_HPP_
// #define MEMORY_MASKING_GADGET_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget;



/**
 * The memory masking gadget checks if a specified part of a double
 * word is correctly modified. In TinyRAM CPU checker we use this to
 * implement byte addressing and word addressing for the memory that
 * consists of double words.
 *
 * More precisely, memory masking gadgets takes the following
 * arguments:
 *
 * dw_contents_prev, dw_contents_next -- the contents of the memory
 *
 * double word before and after the access
 *
 * access_is_word -- a boolean indicating if access is word
 *
 * access_is_byte -- a boolean indicating if access is byte
 *
 * subaddress -- an integer specifying which byte (if access_is_byte=1)
 * or word (if access_is_byte=1) this access is operating on
 *
 * subcontents -- contents of the byte, resp., word to be operated on
 *
 * Memory masking gadget enforces that dw_contents_prev is equal to
 * dw_contents_next everywhere, except subaddres-th byte (if
 * access_is_byte = 1), or MSB(subaddress)-th word (if access_is_word =
 * 1). The corresponding byte, resp., word in dw_contents_next is
 * required to equal subcontents.
 *
 * Note that indexing MSB(subaddress)-th word is the same as indexing
 * the word specified by subaddress expressed in bytes and aligned to
 * the word boundary by rounding the subaddress down.
 *
 * Requirements: The caller is required to perform bounds checks on
 * subcontents. The caller is also required to ensure that exactly one
 * of access_is_word and access_is_byte is set to 1.
 */
// 
pub struct memory_masking_gadget  {
// : public tinyram_standard_gadget<FieldT>
shift:    pb_linear_combination<FieldT>,
is_word0:    pb_variable<FieldT>,
is_word1:    pb_variable<FieldT>,
is_subaddress:    pb_variable_array<FieldT>,
is_byte:    pb_variable_array<FieldT>,

masked_out_word0:    pb_linear_combination<FieldT>,
masked_out_word1:    pb_linear_combination<FieldT>,
masked_out_bytes:    pb_linear_combination_array<FieldT>,

get_masked_out_dw_contents_prev:    RcCell<inner_product_gadget<FieldT> >,

masked_out_dw_contents_prev:    pb_variable<FieldT>,
expected_dw_contents_next:    pb_variable<FieldT>,
// 
dw_contents_prev:    doubleword_variable_gadget<FieldT>,
subaddress:    dual_variable_gadget<FieldT>,
subcontents:    pb_linear_combination<FieldT>,
access_is_word:    pb_linear_combination<FieldT>,
access_is_byte:    pb_linear_combination<FieldT>,
dw_contents_next:    doubleword_variable_gadget<FieldT>,

    // memory_masking_gadget(tinyram_protoboard<FieldT> &pb,
    //                       dw_contents_prev:doubleword_variable_gadget<FieldT>,
    //                       subaddress:dual_variable_gadget<FieldT>,
    //                       subcontents:pb_linear_combination<FieldT>,
    //                       access_is_word:pb_linear_combination<FieldT>,
    //                       access_is_byte:pb_linear_combination<FieldT>,
    //                       dw_contents_next:doubleword_variable_gadget<FieldT>,
    //                       annotation_prefix:String="");
    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness();
}



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::memory_masking_gadget;

//#endif // MEMORY_MASKING_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the TinyRAM memory masking gadget.

 See memory_masking_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_MASKING_GADGET_TCC_
// #define MEMORY_MASKING_GADGET_TCC_


impl memory_masking_gadget<FieldT>{

pub fn new(
pb:tinyram_protoboard<FieldT>,
                                                     dw_contents_prev:doubleword_variable_gadget<FieldT>,
                                                     subaddress:dual_variable_gadget<FieldT>,
                                                     subcontents:pb_linear_combination<FieldT>,
                                                     access_is_word:pb_linear_combination<FieldT>,
                                                     access_is_byte:pb_linear_combination<FieldT>,
                                                     dw_contents_next:doubleword_variable_gadget<FieldT>,
                                                     annotation_prefix:String) ->Self
   
{
    /*
      Indicator variables for access being to word_0, word_1, and
      byte_0, byte_1, ...

      We use little-endian indexing here (least significant
      bit/byte/word has the smallest address).
    */
    is_word0.allocate(pb, format!("{} is_word0",self.annotation_prefix));
    is_word1.allocate(pb, format!("{} is_word1",self.annotation_prefix));
    is_subaddress.allocate(pb, 2 * pb.ap.bytes_in_word(), format!("{} is_sub_address",self.annotation_prefix));
    is_byte.allocate(pb, 2 * pb.ap.bytes_in_word(), format!("{} is_byte",self.annotation_prefix));

    /*
      Get value of the dw_contents_prev for which the specified entity
      is masked out to be zero. E.g. the value of masked_out_bytes[3]
      will be the same as the value of dw_contents_prev, when 3rd
      (0-indexed) byte is set to all zeros.
    */
    masked_out_word0.assign(pb, (FieldT(2)^pb.ap.w) * pb_packing_sum::<FieldT>(
                                pb_variable_array::<FieldT>(dw_contents_prev.bits.begin() + pb.ap.w,
                                                          dw_contents_prev.bits.begin() + 2 * pb.ap.w)));
    masked_out_word1.assign(pb, pb_packing_sum::<FieldT>(
                                pb_variable_array::<FieldT>(dw_contents_prev.bits.begin(),
                                                          dw_contents_prev.bits.begin() + pb.ap.w)));
    masked_out_bytes.resize(2 * pb.ap.bytes_in_word());

    for i in 0..2 * pb.ap.bytes_in_word()
    {
        /* just subtract out the byte to be masked */
        masked_out_bytes[i].assign(pb, (dw_contents_prev.packed -
                                        (FieldT(2)^(8*i)) * pb_packing_sum::<FieldT>(
                                            pb_variable_array::<FieldT>(dw_contents_prev.bits.begin() + 8*i,
                                                                      dw_contents_prev.bits.begin() + 8*(i+1)))));
    }

    /*
      Define masked_out_dw_contents_prev to be the correct masked out
      contents for the current access type.
    */

    let mut masked_out_indicators=pb_linear_combination_array::<FieldT> ::new();
    masked_out_indicators.push(is_word0);
    masked_out_indicators.push(is_word1);
    masked_out_indicators.insert(masked_out_indicators.end(), is_byte.begin(), is_byte.end());

    let mut masked_out_results=pb_linear_combination_array::<FieldT> ::new();
    masked_out_results.push(masked_out_word0);
    masked_out_results.push(masked_out_word1);
    masked_out_results.insert(masked_out_results.end(), masked_out_bytes.begin(), masked_out_bytes.end());

    masked_out_dw_contents_prev.allocate(pb, format!("{} masked_out_dw_contents_prev",self.annotation_prefix));
    get_masked_out_dw_contents_prev.reset( inner_product_gadget::<FieldT>::new(pb, masked_out_indicators, masked_out_results, masked_out_dw_contents_prev,
                                                                           format!("{} get_masked_out_dw_contents_prev",self.annotation_prefix)));

    /*
      Define shift so that masked_out_dw_contents_prev + shift * subcontents = dw_contents_next
     */
    let mut  shift_lc = is_word0 * 1 + is_word1 * (FieldT(2)^self.pb.ap.w);
    for i in 0..2 * self.pb.ap.bytes_in_word()
    {
        shift_lc = shift_lc + is_byte[i] * (FieldT(2)^(8*i));
    }
    shift.assign(pb, shift_lc);
    //  tinyram_standard_gadget<FieldT>(pb, annotation_prefix),
    Self{dw_contents_prev,
    subaddress,
    subcontents,
    access_is_word,
    access_is_byte,
    dw_contents_next}
}


pub fn generate_r1cs_constraints()
{
    /* get indicator variables for is_subaddress[i] by adding constraints
       is_subaddress[i] * (subaddress - i) = 0 and \sum_i is_subaddress[i] = 1 */
    for i in 0..2 * self.pb.ap.bytes_in_word()
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(is_subaddress[i], subaddress.packed - i, 0),
                                     format!("{} is_subaddress_{}",self.annotation_prefix, i));
    }
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, pb_sum::<FieldT>(is_subaddress), 1), format!("{} is_subaddress",self.annotation_prefix));

    /* get indicator variables is_byte_X */
    for i in 0..2 * self.pb.ap.bytes_in_word()
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(access_is_byte, is_subaddress[i], is_byte[i]),
                                     format!("{} is_byte_{}",self.annotation_prefix, i));
    }

    /* get indicator variables is_word_0/is_word_1 */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(access_is_word, 1 - subaddress.bits[self.pb.ap.subaddr_len()-1], is_word0),
                                 format!("{} is_word_0",self.annotation_prefix));
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(access_is_word, subaddress.bits[self.pb.ap.subaddr_len()-1], is_word1),
                                 format!("{} is_word_1",self.annotation_prefix));

    /* compute masked_out_dw_contents_prev */
    get_masked_out_dw_contents_prev.generate_r1cs_constraints();

    /*
       masked_out_dw_contents_prev + shift * subcontents = dw_contents_next
     */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(shift, subcontents, dw_contents_next.packed - masked_out_dw_contents_prev),
                                 format!("{} mask_difference",self.annotation_prefix));
}


pub fn generate_r1cs_witness()
{
    /* get indicator variables is_subaddress */
    for i in 0..2 * self.pb.ap.bytes_in_word()
    {
        self.pb.val(is_subaddress[i])=  if (self.pb.val(subaddress.packed) == FieldT(i)) {FieldT::one()} else{FieldT::zero()};
    }

    /* get indicator variables is_byte_X */
    for i in 0..2 * self.pb.ap.bytes_in_word()
    {
        self.pb.val(is_byte[i]) = self.pb.val(is_subaddress[i]) * self.pb.lc_val(access_is_byte);
    }

    /* get indicator variables is_word_0/is_word_1 */
    self.pb.val(is_word0) = (FieldT::one() - self.pb.val(subaddress.bits[self.pb.ap.subaddr_len()-1])) * self.pb.lc_val(access_is_word);
    self.pb.val(is_word1) = self.pb.val(subaddress.bits[self.pb.ap.subaddr_len()-1]) * self.pb.lc_val(access_is_word);

    /* calculate shift and masked out words/bytes */
    shift.evaluate(self.pb);
    masked_out_word0.evaluate(self.pb);
    masked_out_word1.evaluate(self.pb);
    masked_out_bytes.evaluate(self.pb);

    /* get masked_out dw/word0/word1/bytes */
    get_masked_out_dw_contents_prev.generate_r1cs_witness();

    /* compute dw_contents_next */
    self.pb.val(dw_contents_next.packed) = self.pb.val(masked_out_dw_contents_prev) + self.pb.lc_val(shift) * self.pb.lc_val(subcontents);
    dw_contents_next.generate_r1cs_witness_from_packed();
}

}

//#endif // MEMORY_MASKING_GADGET_TCC_
