// /** @file
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef BASIC_GADGETS_HPP_
// // #define BASIC_GADGETS_HPP_

// // use  <cassert>
// // use  <memory>

use crate::gadgetlib1::gadget;

fn FMT(s:&String,c:&str){

}

/* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
// 
// pub fn generate_boolean_r1cs_constraint(pb:&protoboard<FieldT> , lc:&pb_linear_combination<FieldT>, annotation_prefix:&String);

// 
// pub fn generate_r1cs_equals_const_constraint(pb:&protoboard<FieldT> , lc:&pb_linear_combination<FieldT>, annotation_prefix:&FieldT& c,  String);

// 
pub struct packing_gadget {
// private:: public gadget<FieldT> 
    /* no internal variables */
// public:
      bits:pb_linear_combination_array<FieldT> ,
      packed:pb_linear_combination<FieldT>,
}

 impl packing_gadget {
    pub fn new(pb:protoboard<FieldT> ,
                   bits:pb_linear_combination_array<FieldT>,
                   packed:pb_linear_combination<FieldT>,
                   annotation_prefix:&String) ->Self
        { 
        // gadget<FieldT>(pb, annotation_prefix),
        Self{ bits, packed}
        }

    // pub fn generate_r1cs_constraints(enforce_bitness:bool);
    // /* adds constraint result = \sum  bits[i] * 2^i */

    // pub fn generate_r1cs_witness_from_packed();
    // pub fn generate_r1cs_witness_from_bits();
}

// 
pub struct  multipacking_gadget {
// private:: public gadget<FieldT> 
packers:    Vec<packing_gadget<FieldT> >,
// public:
bits:     pb_linear_combination_array<FieldT>,
packed_vars:     pb_linear_combination_array<FieldT>,

chunk_size:     usize,
num_chunks:     usize,
last_chunk_size:   usize,
}
// impl multipacking_gadget {
//     multipacking_gadget(pb:&protoboard<FieldT> ,
//                         bits:&pb_linear_combination_array<FieldT>,
//                         packed_vars:&pb_linear_combination_array<FieldT>,
//                         chunk_size:usize,
//                         annotation_prefix:&String);
//     // pub fn generate_r1cs_constraints(enforce_bitness:bool);
//     // pub fn generate_r1cs_witness_from_packed();
//     // pub fn generate_r1cs_witness_from_bits();
// }

// 
pub struct  field_vector_copy_gadget {
// public:: public gadget<FieldT> 
source:     pb_variable_array<FieldT>,
target:     pb_variable_array<FieldT>,
do_copy:     pb_linear_combination<FieldT>,

    // field_vector_copy_gadget(pb:&protoboard<FieldT> ,
    //                          source:&pb_variable_array<FieldT>,
    //                          target:&pb_variable_array<FieldT>,
    //                          do_copy:&pb_linear_combination<FieldT>,
    //                          annotation_prefix:&String);
    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
pub struct  bit_vector_copy_gadget  {
// public:: public gadget<FieldT>
source_bits:     pb_variable_array<FieldT>,
target_bits:     pb_variable_array<FieldT>,
do_copy:     pb_linear_combination<FieldT>,

packed_source:    pb_variable_array<FieldT>,
packed_target:    pb_variable_array<FieldT>,

pack_source:    RcCell<multipacking_gadget<FieldT> >,
pack_target:    RcCell<multipacking_gadget<FieldT> >,
copier:    RcCell<field_vector_copy_gadget<FieldT> >,

chunk_size:     usize,
num_chunks:     usize,

    // bit_vector_copy_gadget(pb:&protoboard<FieldT> ,
    //                        source_bits:&pb_variable_array<FieldT>,
    //                        target_bits:&pb_variable_array<FieldT>,
    //                        do_copy:&pb_linear_combination<FieldT>,
    //                        chunk_size:usize,
    //                        annotation_prefix:&String);
    // pub fn generate_r1cs_constraints(enforce_source_bitness:bool, enforce_target_bitness:bool);
    // pub fn generate_r1cs_witness();
}

// 
pub struct  dual_variable_gadget  {
// private:: public gadget<FieldT>
consistency_check:    RcCell<packing_gadget<FieldT> >,
// public:
packed:    pb_variable<FieldT>,
bits:    pb_variable_array<FieldT>,
}
impl dual_variable_gadget  {
    pub fn new(pb:protoboard<FieldT> ,
                         width:usize,
                         annotation_prefix:&String) ->Self
     
    {
//    gadget<FieldT>(pb, annotation_prefix)
        packed.allocate(pb, FMT(annotation_prefix, " packed"));
        bits.allocate(pb, width, FMT(annotation_prefix, " bits"));
        // consistency_check.reset(packing_gadget::<FieldT>::new(pb,
        //                                                    bits,
        //                                                    packed,
        //                                                    FMT(annotation_prefix, " consistency_check")));
        Self{packed,bits}
    }

    pub fn new2(pb:&protoboard<FieldT> ,
                         bits:&pb_variable_array<FieldT>,
                         annotation_prefix:&String)->Self
        
    {
// gadget<FieldT>(pb, annotation_prefix), bits(bits)
        packed.allocate(pb, FMT(annotation_prefix, " packed"));
        consistency_check.reset( packing_gadget::<FieldT>::new(pb,
                                                           bits,
                                                           packed,
                                                           FMT(annotation_prefix, " consistency_check")));
    }

    pub fn new3(pb:&protoboard<FieldT> ,
                         packed:&pb_variable<FieldT>,
                         width:usize,
                         annotation_prefix:&String) ->Self
       
    {
//  gadget<FieldT>(pb, annotation_prefix), packed(packed)
        bits.allocate(pb, width, FMT(annotation_prefix, " bits"));
        consistency_check.reset( packing_gadget::<FieldT>::new(pb,
                                                           bits,
                                                           packed,
                                                           FMT(annotation_prefix, " consistency_check")));
    }

    // pub fn generate_r1cs_constraints(enforce_bitness:bool);
    // pub fn generate_r1cs_witness_from_packed();
    // pub fn generate_r1cs_witness_from_bits();
}

/*
  the gadgets below are Fp specific:
  I * X = R
  (1-R) * X = 0

  if X = 0 then R = 0
  if X != 0 then R = 1 and I = X^{-1}
*/

// 
pub struct  disjunction_gadget {
// private:: public gadget<FieldT> 
inv:    pb_variable<FieldT>,
// public:
inputs:     pb_variable_array<FieldT>,
output:     pb_variable<FieldT>,
}
impl disjunction_gadget {
    pub fn new(pb:protoboard<FieldT>,
                       inputs:pb_variable_array<FieldT>,
                       output:pb_variable<FieldT>,
                       annotation_prefix:&String) ->Self
        
    {
// gadget<FieldT>(pb, annotation_prefix), inputs(inputs), output(output)
        assert!(inputs.size() >= 1);
        inv.allocate(pb, FMT(annotation_prefix, " inv"));
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_disjunction_gadget(n:usize);

// 
pub struct  conjunction_gadget {
// private:: public gadget<FieldT> 
inv:    pb_variable<FieldT>,
// public:
inputs:     pb_variable_array<FieldT>,
output:     pb_variable<FieldT>,
}
impl conjunction_gadget {
    pub fn new(pb:protoboard<FieldT>,
                       inputs:pb_variable_array<FieldT>,
                       output:pb_variable<FieldT>,
                       annotation_prefix:&String) ->Self
       
    {
//  gadget<FieldT>(pb, annotation_prefix), inputs(inputs), output(output)
        assert!(inputs.size() >= 1);
        inv.allocate(pb, FMT(annotation_prefix, " inv"));
    }

//     pub fn generate_r1cs_constraints();
//     pub fn generate_r1cs_witness();
}

// 
// pub fn test_conjunction_gadget(n:usize);

// 
pub struct  comparison_gadget {
// private:: public gadget<FieldT> 
alpha:    pb_variable_array<FieldT>,
alpha_packed:    pb_variable<FieldT>,
pack_alpha:    RcCell<packing_gadget<FieldT> >,

all_zeros_test:    RcCell<disjunction_gadget<FieldT> >,
not_all_zeros:    pb_variable<FieldT>,
// public:
n:     usize,
A:     pb_linear_combination<FieldT>,
B:     pb_linear_combination<FieldT>,
less:     pb_variable<FieldT>,
less_or_eq:     pb_variable<FieldT>,
}
impl comparison_gadget {
    pub fn new(pb:protoboard<FieldT>,
                      n:usize,
                      A:pb_linear_combination<FieldT>,
                      B:pb_linear_combination<FieldT>,
                      less:pb_variable<FieldT>,
                      less_or_eq:pb_variable<FieldT>,
                      annotation_prefix:&String) ->Self
    {
//  gadget<FieldT>(pb, annotation_prefix), n(n), A(A), B(B), less(less), less_or_eq(less_or_eq)
        alpha.allocate(pb, n, FMT(annotation_prefix, " alpha"));
        alpha.push(less_or_eq); // alpha[n] is less_or_eq

        alpha_packed.allocate(pb, FMT(annotation_prefix, " alpha_packed"));
        not_all_zeros.allocate(pb, FMT(annotation_prefix, " not_all_zeros"));

        pack_alpha.reset( packing_gadget::<FieldT>::new(pb, alpha, alpha_packed,
                                                    FMT(annotation_prefix, " pack_alpha")));

        all_zeros_test.reset(disjunction_gadget::<FieldT>::new(pb,
                                                            pb_variable_array::<FieldT>(alpha.begin(), alpha.begin() + n),
                                                            not_all_zeros,
                                                            FMT(annotation_prefix, " all_zeros_test")));
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_comparison_gadget(n:usize);

// 
pub struct  inner_product_gadget {
// private:: public gadget<FieldT> 
    /* S_i = \sum_{k=0}^{i+1} A[i] * B[i] */
S:    pb_variable_array<FieldT>,
// public:
A:     pb_linear_combination_array<FieldT>,
B:     pb_linear_combination_array<FieldT>,
result:     pb_variable<FieldT>,
}
 impl inner_product_gadget {
    pub fn new(pb:protoboard<FieldT>,
                         A:pb_linear_combination_array<FieldT>,
                         B:pb_linear_combination_array<FieldT>,
                         result:pb_variable<FieldT>,
                         annotation_prefix:&String)->Self
       
    {
//  gadget<FieldT>(pb, annotation_prefix), A(A), B(B), result(result)
        assert!(A.size() >= 1);
        assert!(A.size() == B.size());

        S.allocate(pb, A.size()-1, FMT(annotation_prefix, " S"));
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_inner_product_gadget(n:usize);

// 
pub struct  loose_multiplexing_gadget {
/*
  this implements loose multiplexer:
  index not in bounds -> success_flag = 0
  index in bounds && success_flag = 1 -> result is correct
  however if index is in bounds we can also set success_flag to 0 (and then result will be forced to be 0)
*/
// public:: public gadget<FieldT> 
alpha:    pb_variable_array<FieldT>,
// private:
compute_result:    RcCell<inner_product_gadget<FieldT> >,
// public:
arr:     pb_linear_combination_array<FieldT>,
index:     pb_variable<FieldT>,
result:     pb_variable<FieldT>,
success_flag:     pb_variable<FieldT>,
}
impl loose_multiplexing_gadget {
    pub fn new(pb:protoboard<FieldT>,
                              arr:pb_linear_combination_array<FieldT>,
                              index:pb_variable<FieldT>,
                              result:pb_variable<FieldT>,
                              success_flag:pb_variable<FieldT>,
                              annotation_prefix:&String) ->Self
        
    {
// gadget<FieldT>(pb, annotation_prefix), arr(arr), index(index), result(result), success_flag(success_flag)
        alpha.allocate(pb, arr.size(), FMT(annotation_prefix, " alpha"));
        compute_result.reset(inner_product_gadget::<FieldT>::new(pb, alpha, arr, result, FMT(annotation_prefix, " compute_result")));
    }

    // pub fn generate_r1cs_constraints();
    // pub fn generate_r1cs_witness();
}

// 
// pub fn test_loose_multiplexing_gadget(n:usize);

// < FieldT,  VarT>
// pub fn create_linear_combination_constraints(pb:&protoboard<FieldT> ,
//                                            base:&Vec<FieldT>,
//                                            v:&Vec<(VarT,FieldT) >,
//                                            target:&VarT,
//                                            annotation_prefix:&String);

// < FieldT,  VarT>
// pub fn create_linear_combination_witness(pb:&protoboard<FieldT> ,
//                                        base:&Vec<FieldT>,
//                                        v:&Vec<(VarT,FieldT) >,
//                                        target:&VarT);


// use crate::gadgetlib1::gadgets/basic_gadgets;

//#endif // BASIC_GADGETS_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_GADGETS_TCC_
// #define BASIC_GADGETS_TCC_

use common::profiling;
use common::utils;



// 
pub fn generate_boolean_r1cs_constraint(pb:&protoboard<FieldT> , lc:&pb_linear_combination<FieldT>, annotation_prefix:&String)
/* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
{
    pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(lc, 1-lc, 0),
                           FMT(annotation_prefix, " boolean_r1cs_constraint"));
}


pub fn generate_r1cs_equals_const_constraint(pb:&protoboard<FieldT> , lc:&pb_linear_combination<FieldT>, c:&FieldT, annotation_prefix:&String)
{
    pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, lc, c),
                           FMT(annotation_prefix, " constness_constraint"));
}

impl packing_gadget<FieldT>{

pub fn  generate_r1cs_constraints(enforce_bitness:bool)
{/* adds constraint result = \sum  bits[i] * 2^i */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, pb_packing_sum::<FieldT>(bits), packed), FMT(annotation_prefix, " packing_constraint"));

    if enforce_bitness
    {
        for i in 0..bits.size()
        {
            generate_boolean_r1cs_constraint::<FieldT>(self.pb, bits[i], FMT(annotation_prefix, " bitness_{}", i));
        }
    }
}


pub fn  generate_r1cs_witness_from_packed()
{
    packed.evaluate(self.pb);
    assert!(self.pb.lc_val(packed).as_bigint().num_bits() <= bits.size()); // `bits` is large enough to represent this packed value
    bits.fill_with_bits_of_field_element(self.pb, self.pb.lc_val(packed));
}


pub fn  generate_r1cs_witness_from_bits()
{
    bits.evaluate(self.pb);
    self.pb.lc_val(packed) = bits.get_field_element_from_bits(self.pb);
}
}

impl multipacking_gadget<FieldT>{

pub fn new(pb:&protoboard<FieldT> ,
                                                 bits:&pb_linear_combination_array<FieldT>,
                                                 packed_vars:&pb_linear_combination_array<FieldT>,
                                                 chunk_size:usize,
                                                 annotation_prefix:&String) ->Self
  
{
//   gadget<FieldT>(pb, annotation_prefix), bits(bits), packed_vars(packed_vars),
//     chunk_size(chunk_size),
//     num_chunks(div_ceil(bits.size(), chunk_size))
    // last_chunk_size(bits.size() - (num_chunks-1) * chunk_size)
    assert!(packed_vars.size() == num_chunks);
    for i in 0..num_chunks
    {
        packers.push(packing_gadget::<FieldT>(self.pb, pb_linear_combination_array::<FieldT>(bits.begin() + i * chunk_size,
                                                                                                  bits.begin() + std::cmp::min((i+1) * chunk_size, bits.size())),
                                                    packed_vars[i], FMT(annotation_prefix, " packers_{}", i)));
    }
}


pub fn  generate_r1cs_constraints(enforce_bitness:bool)
{
    for i in 0..num_chunks
    {
        packers[i].generate_r1cs_constraints(enforce_bitness);
    }
}


pub fn  generate_r1cs_witness_from_packed()
{
    for i in 0..num_chunks
    {
        packers[i].generate_r1cs_witness_from_packed();
    }
}


pub fn  generate_r1cs_witness_from_bits()
{
    for i in 0..num_chunks
    {
        packers[i].generate_r1cs_witness_from_bits();
    }
}
}

 pub fn multipacking_num_chunks(num_bits:usize)->usize
{
    return div_ceil(num_bits, FieldT::capacity());
}
impl field_vector_copy_gadget<FieldT>{

pub fn new(pb:protoboard<FieldT> ,
                                                           source:pb_variable_array<FieldT>,
                                                           target:pb_variable_array<FieldT>,
                                                           do_copy:pb_linear_combination<FieldT>,
                                                           annotation_prefix:&String) ->Self

{
// gadget<FieldT>(pb, annotation_prefix), source(source), target(target), do_copy(do_copy)
    assert!(source.size() == target.size());
}


pub fn  generate_r1cs_constraints()
{
    for i in 0..source.size()
    {
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(do_copy, source[i] - target[i], 0),
                                     FMT(annotation_prefix, " copying_check_{}", i));
    }
}


pub fn  generate_r1cs_witness()
{
    do_copy.evaluate(self.pb);
    assert!(self.pb.lc_val(do_copy) == FieldT::one() || self.pb.lc_val(do_copy) == FieldT::zero());
    if self.pb.lc_val(do_copy) != FieldT::zero()
    {
        for i in 0..source.size()
        {
            self.pb.val(target[i]) = self.pb.val(source[i]);
        }
    }
}
}

impl bit_vector_copy_gadget<FieldT>{

pub fn new(pb:protoboard<FieldT> ,
                                                       source_bits:pb_variable_array<FieldT>,
                                                       target_bits:pb_variable_array<FieldT>,
                                                       do_copy:pb_linear_combination<FieldT>,
                                                       chunk_size:usize,
                                                       annotation_prefix:&String) ->Self
  
{
//   gadget<FieldT>(pb, annotation_prefix), source_bits(source_bits), target_bits(target_bits), do_copy(do_copy),
//     chunk_size(chunk_size), num_chunks(div_ceil(source_bits.size(), chunk_size))
    assert!(source_bits.size() == target_bits.size());

    packed_source.allocate(pb, num_chunks, FMT(annotation_prefix, " packed_source"));
    pack_source.reset(multipacking_gadget::<FieldT>::new(pb, source_bits, packed_source, chunk_size, FMT(annotation_prefix, " pack_source")));

    packed_target.allocate(pb, num_chunks, FMT(annotation_prefix, " packed_target"));
    pack_target.reset(multipacking_gadget::<FieldT>::new(pb, target_bits, packed_target, chunk_size, FMT(annotation_prefix, " pack_target")));

    copier.reset(field_vector_copy_gadget::<FieldT>::new(pb, packed_source, packed_target, do_copy, FMT(annotation_prefix, " copier")));
}


pub fn  generate_r1cs_constraints(enforce_source_bitness:bool, enforce_target_bitness:bool)
{
    pack_source.generate_r1cs_constraints(enforce_source_bitness);
    pack_target.generate_r1cs_constraints(enforce_target_bitness);

    copier.generate_r1cs_constraints();
}


pub fn  generate_r1cs_witness()
{
    do_copy.evaluate(self.pb);
    assert!(self.pb.lc_val(do_copy) == FieldT::zero() || self.pb.lc_val(do_copy) == FieldT::one());
    if self.pb.lc_val(do_copy) == FieldT::one()
    {
        for i in 0..source_bits.size()
        {
            self.pb.val(target_bits[i]) = self.pb.val(source_bits[i]);
        }
    }

    pack_source.generate_r1cs_witness_from_bits();
    pack_target.generate_r1cs_witness_from_bits();
}
}

impl dual_variable_gadget<FieldT>{

pub fn  generate_r1cs_constraints(enforce_bitness:bool)
{
    consistency_check.generate_r1cs_constraints(enforce_bitness);
}


pub fn  generate_r1cs_witness_from_packed()
{
    consistency_check.generate_r1cs_witness_from_packed();
}


pub fn  generate_r1cs_witness_from_bits()
{
    consistency_check.generate_r1cs_witness_from_bits();
}
}
impl disjunction_gadget<FieldT>{

pub fn  generate_r1cs_constraints()
{
    /* inv * sum = output */
    let (mut  a1,mut b1,mut c1)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a1.add_term(inv);
    for i in 0..inputs.size()
    {
        b1.add_term(inputs[i]);
    }
    c1.add_term(output);

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a1, b1, c1), FMT(annotation_prefix, " inv*sum=output"));

    /* (1-output) * sum = 0 */
    let (mut  a2,mut b2,mut c2)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a2.add_term(ONE);
    a2.add_term(output, -1);
    for i in 0..inputs.size()
    {
        b2.add_term(inputs[i]);
    }
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a2, b2, c2), FMT(annotation_prefix, " (1-output)*sum=0"));
}


pub fn  generate_r1cs_witness()
{
    let mut  sum = FieldT::zero();

    for i in 0..inputs.size()
    {
        sum += self.pb.val(inputs[i]);
    }

    if sum.is_zero()
    {
        self.pb.val(inv) = FieldT::zero();
        self.pb.val(output) = FieldT::zero();
    }
    else
    {
        self.pb.val(inv) = sum.inverse();
        self.pb.val(output) = FieldT::one();
    }
}
}


pub fn test_disjunction_gadget(n:usize)
{
    print!("testing disjunction_gadget on all {} bit strings\n", n);

    let mut  pb=protoboard::<FieldT>::new();
   let mut  inputs= pb_variable_array::<FieldT>::new();
    inputs.allocate(pb, n, "inputs");

    let mut  output=pb_variable::<FieldT>::new();
    output.allocate(pb, "output");

    let mut  d=disjunction_gadget::<FieldT>::new(pb, inputs, output, "d");
    d.generate_r1cs_constraints();

    for w in 0..1u64<<n
    {
        for j in 0..n
        {
            pb.val(inputs[j]) = FieldT::from(if w & (1u64<<j)!=0  {1} else {0});
        }

        d.generate_r1cs_witness();

// #ifdef DEBUG
        print!("positive test for {}\n", w);
//#endif
        assert!(pb.val(output) == if w {FieldT::one()}else{FieldT::zero()});
        assert!(pb.is_satisfied());

// #ifdef DEBUG
        print!("negative test for {}\n", w);
//#endif
        pb.val(output) = (if w  {FieldT::zero()} else {FieldT::one()});
        assert!(!pb.is_satisfied());
    }

    print_time("disjunction tests successful");
}

impl conjunction_gadget<FieldT>{

pub fn  generate_r1cs_constraints()
{
    /* inv * (n-sum) = 1-output */
     let (a1, b1, c1)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a1.add_term(inv);
    b1.add_term(ONE, inputs.size());
    for i in 0..inputs.size()
    {
        b1.add_term(inputs[i], -1);
    }
    c1.add_term(ONE);
    c1.add_term(output, -1);

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a1, b1, c1), FMT(annotation_prefix, " inv*(n-sum)=(1-output)"));

    /* output * (n-sum) = 0 */
         let (a2, b2, c2)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());

    a2.add_term(output);
    b2.add_term(ONE, inputs.size());
    for i in 0..inputs.size()
    {
        b2.add_term(inputs[i], -1);
    }
    c2.add_term(ONE, 0);

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a2, b2, c2), FMT(annotation_prefix, " output*(n-sum)=0"));
}


pub fn  generate_r1cs_witness()
{
    let mut  sum = FieldT::from(inputs.size());

    for i in 0..inputs.size()
    {
        sum -= self.pb.val(inputs[i]);
    }

    if sum.is_zero()
    {
        self.pb.val(inv) = FieldT::zero();
        self.pb.val(output) = FieldT::one();
    }
    else
    {
        self.pb.val(inv) = sum.inverse();
        self.pb.val(output) = FieldT::zero();
    }
}
}

pub fn test_conjunction_gadget(n:usize)
{
    print!("testing conjunction_gadget on all {} bit strings\n", n);

    let mut pb=protoboard::<FieldT>::new() ;
    let mut  inputs=pb_variable_array::<FieldT>::new();
    inputs.allocate(pb, n, "inputs");

    let mut  output=pb_variable::<FieldT>::new();
    output.allocate(pb, "output");

     let mut c=conjunction_gadget::<FieldT>::new(pb, inputs, output, "c");
    c.generate_r1cs_constraints();

    for w in 0..1u64<<n
    {
        for j in 0..n
        {
            pb.val(inputs[j]) = if w & (1u64<<j)!=0 { FieldT::one()} else {FieldT::zero()};
        }

        c.generate_r1cs_witness();

// #ifdef DEBUG
        print!("positive test for {}\n", w);
//#endif
        assert!(pb.val(output) == if w == (1u64<<n) - 1 {FieldT::one()}else{FieldT::zero()});
        assert!(pb.is_satisfied());

// #ifdef DEBUG
        print!("negative test for {}\n", w);
//#endif
        pb.val(output) = if w == (1u64<<n) - 1 {FieldT::zero()}else{FieldT::one()};
        assert!(!pb.is_satisfied());
    }

    print_time("conjunction tests successful");
}

impl comparison_gadget<FieldT>{

pub fn  generate_r1cs_constraints()
{
    /*
      packed(alpha) = 2^n + B - A

      not_all_zeros = \bigvee_{i=0}^{n-1} alpha_i

      if B - A > 0, then 2^n + B - A > 2^n,
          so alpha_n = 1 and not_all_zeros = 1
      if B - A = 0, then 2^n + B - A = 2^n,
          so alpha_n = 1 and not_all_zeros = 0
      if B - A < 0, then 2^n + B - A \in {0, 1, \ldots, 2^n-1},
          so alpha_n = 0

      therefore alpha_n = less_or_eq and alpha_n * not_all_zeros = less
     */

    /* not_all_zeros to be Boolean, alpha_i are Boolean by packing gadget */
    generate_boolean_r1cs_constraint::<FieldT>(self.pb, not_all_zeros,
                                     FMT(annotation_prefix, " not_all_zeros"));

    /* constraints for packed(alpha) = 2^n + B - A */
    pack_alpha.generate_r1cs_constraints(true);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, (FieldT(2)^n) + B - A, alpha_packed), FMT(annotation_prefix, " main_constraint"));

    /* compute result */
    all_zeros_test.generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(less_or_eq, not_all_zeros, less),
                                 FMT(annotation_prefix, " less"));
}


pub fn  generate_r1cs_witness()
{
    A.evaluate(self.pb);
    B.evaluate(self.pb);

    /* unpack 2^n + B - A into alpha_packed */
    self.pb.val(alpha_packed) = (FieldT(2)^n) + self.pb.lc_val(B) - self.pb.lc_val(A);
    pack_alpha.generate_r1cs_witness_from_packed();

    /* compute result */
    all_zeros_test.generate_r1cs_witness();
    self.pb.val(less) = self.pb.val(less_or_eq) * self.pb.val(not_all_zeros);
}
}


pub fn test_comparison_gadget(n:usize)
{
    print!("testing comparison_gadget on all {} bit inputs\n", n);

    let mut pb=protoboard::<FieldT> ::new();

    let (A, B, less, less_or_eq)=(pb_variable::<FieldT> ::new(),pb_variable::<FieldT> ::new(),pb_variable::<FieldT> ::new(),pb_variable::<FieldT> ::new());
    A.allocate(pb, "A");
    B.allocate(pb, "B");
    less.allocate(pb, "less");
    less_or_eq.allocate(pb, "less_or_eq");

    let mut cmp=comparison_gadget::<FieldT>::new(pb, n, A, B, less, less_or_eq, "cmp");
    cmp.generate_r1cs_constraints();

    for a in 0..1u64<<n
    {
        for b in 0..1u64<<n
        {
            pb.val(A) = FieldT(a);
            pb.val(B) = FieldT(b);

            cmp.generate_r1cs_witness();

// #ifdef DEBUG
            print!("positive test for {} < {}\n", a, b);
//#endif
            assert!(pb.val(less) == if a < b {FieldT::one()}else{FieldT::zero()});
            assert!(pb.val(less_or_eq) == if a <= b {FieldT::one()}else{FieldT::zero()});
            assert!(pb.is_satisfied());
        }
    }

    print_time("comparison tests successful");
}

impl inner_product_gadget<FieldT>{

pub fn  generate_r1cs_constraints()
{
    /*
      S_i = \sum_{k=0}^{i+1} A[i] * B[i]
      S[0] = A[0] * B[0]
      S[i+1] - S[i] = A[i] * B[i]
    */
    for i in 0..A.size()
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint::<FieldT>(A[i], B[i],
                                    (if i == A.size()-1  {result} else {S[i] + ( if i == 0  {0 * ONE} else {-S[i-1]})})),
            FMT(annotation_prefix, " S_{}", i));
    }
}


pub fn  generate_r1cs_witness()
{
    let mut  total = FieldT::zero();
    for i in 0..A.size()
    {
        A[i].evaluate(self.pb);
        B[i].evaluate(self.pb);

        total += self.pb.lc_val(A[i]) * self.pb.lc_val(B[i]);
        self.pb.val(if i == A.size()-1  {result }else  {S[i]}) = total;
    }
}
}

pub fn test_inner_product_gadget(n:usize)
{
    print!("testing inner_product_gadget on all {} bit strings\n", n);

    let mut  pb=protoboard::<FieldT>::new();
   let mut  A= pb_variable_array::<FieldT>::new();
    A.allocate(pb, n, "A");
   let mut  B= pb_variable_array::<FieldT>::new();
    B.allocate(pb, n, "B");

    let mut  result=pb_variable::<FieldT>::new();
    result.allocate(pb, "result");

    let mut g=inner_product_gadget::<FieldT> ::new(pb, A, B, result, "g");
    g.generate_r1cs_constraints();

    for i in 0..1u64<<n
    {
        for j in 0..1u64<<n
        {
            let mut  correct = 0;
            for k in 0..n
            {
                pb.val(A[k]) = if i & (1u64<<k) {FieldT::one()}else{FieldT::zero()};
                pb.val(B[k]) = if j & (1u64<<k) {FieldT::one()}else{FieldT::zero()};
                correct += if (i & (1u64<<k)) && (j & (1u64<<k)) {1}else{0};
            }

            g.generate_r1cs_witness();
// #ifdef DEBUG
            print!("positive test for ({}, {})\n", i, j);
//#endif
            assert!(pb.val(result) == FieldT::from(correct));
            assert!(pb.is_satisfied());

// #ifdef DEBUG
            print!("negative test for ({}, {})\n", i, j);
//#endif
            pb.val(result) = FieldT::from(100*n+19);
            assert!(!pb.is_satisfied());
        }
    }

    print_time("inner_product_gadget tests successful");
}

impl loose_multiplexing_gadget<FieldT>{

pub fn  generate_r1cs_constraints()
{
    /* \alpha_i (index - i) = 0 */
    for i in 0..arr.size()
    {
        self.pb.add_r1cs_constraint(
            r1cs_constraint::<FieldT>(alpha[i], index - i, 0),
            FMT(annotation_prefix, " alpha_{}", i));
    }

    /* 1 * (\sum \alpha_i) = success_flag */
    let ( a, b, c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());
    a.add_term(ONE);
    for i in 0..arr.size()
    {
        b.add_term(alpha[i]);
    }
    c.add_term(success_flag);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a, b, c), FMT(annotation_prefix, " main_constraint"));

    /* now success_flag is constrained to either 0 (if index is out of
       range) or \alpha_i. constrain it and \alpha_i to zero */
    generate_boolean_r1cs_constraint::<FieldT>(self.pb, success_flag, FMT(annotation_prefix, " success_flag"));

    /* compute result */
    compute_result.generate_r1cs_constraints();
}


pub fn  generate_r1cs_witness()
{
    /* assumes that idx can be fit in ulong; true for our purposes for now */
     let mut  valint = self.pb.val(index).as_bigint();
    let mut  idx = valint.as_ulong();
    let  arrsize= bigint::<FieldT::num_limbs>::new(arr.size());

    if idx >= arr.size() || mpn_cmp(valint.data, arrsize.data, FieldT::num_limbs) >= 0
    {
        for i in 0..arr.size()
        {
            self.pb.val(alpha[i]) = FieldT::zero();
        }

        self.pb.val(success_flag) = FieldT::zero();
    }
    else
    {
        for i in 0..arr.size()
        {
            self.pb.val(alpha[i]) = if i == idx {FieldT::one()}else{FieldT::zero()};
        }

        self.pb.val(success_flag) = FieldT::one();
    }

    compute_result.generate_r1cs_witness();
}}


pub fn test_loose_multiplexing_gadget(n:usize)
{
    print!("testing loose_multiplexing_gadget on 2**{} pb_variable<FieldT> array inputs\n", n);
    let mut  pb=protoboard::<FieldT>::new();

    let mut  arr=pb_variable_array::<FieldT>::new();
    arr.allocate(pb, 1u64<<n, "arr");
   let ( index, result, success_flag)=( pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new(),pb_variable::<FieldT>::new());
    index.allocate(pb, "index");
    result.allocate(pb, "result");
    success_flag.allocate(pb, "success_flag");

    let mut  g=loose_multiplexing_gadget::<FieldT>::new(pb, arr, index, result, success_flag, "g");
    g.generate_r1cs_constraints();

    for i in 0..1u64<<n
    {
        pb.val(arr[i]) = FieldT::from((19*i) % (1u64<<n));
    }

    for idx in -1..=(int)(1u64<<n)
    {
        pb.val(index) = FieldT::from(idx);
        g.generate_r1cs_witness();

        if 0 <= idx && idx <= (1u64<<n) - 1
        {
            print!("demuxing element {} (in bounds)\n", idx);
            assert!(pb.val(result) == FieldT((19*idx) % (1u64<<n)));
            assert!(pb.val(success_flag) == FieldT::one());
            assert!(pb.is_satisfied());
            pb.val(result) -= FieldT::one();
            assert!(!pb.is_satisfied());
        }
        else
        {
            print!("demuxing element {} (out of bounds)\n", idx);
            assert!(pb.val(success_flag) == FieldT::zero());
            assert!(pb.is_satisfied());
            pb.val(success_flag) = FieldT::one();
            assert!(!pb.is_satisfied());
        }
    }
    print!("loose_multiplexing_gadget tests successful\n");
}

// < FieldT,  VarT>
pub fn create_linear_combination_constraints< FieldT,  VarT>(pb:&protoboard<FieldT> ,
                                           base:&Vec<FieldT>,
                                           v:&Vec<(VarT,FieldT) >,
                                           target:&VarT,
                                           annotation_prefix:&String)
{
    for i in 0..base.size()
    {
        let ( a, b, c)=(linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new(),linear_combination::<FieldT>::new());

        a.add_term(ONE);
        b.add_term(ONE, base[i]);

        for p in &v
        {
            b.add_term(p.first.all_vars[i], p.1);
        }

        c.add_term(target.all_vars[i]);

        pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(a, b, c), FMT(annotation_prefix, " linear_combination_{}", i));
    }
}


pub fn create_linear_combination_witness< FieldT,  VarT>(pb:&protoboard<FieldT> ,
                                       base:&Vec<FieldT>,
                                       v:&Vec<(VarT,FieldT) >,
                                       target:&VarT)
{
    for i in 0..base.size()
    {
        pb.val(target.all_vars[i]) = base[i];

        for p in &v
        {
            pb.val(target.all_vars[i]) += p.second * pb.val(p.first.all_vars[i]);
        }
    }
}


//#endif // BASIC_GADGETS_TCC_
