/** @file
 *****************************************************************************

 Declaration of interfaces for the FOORAM CPU checker gadget.

 The gadget checks the correct operation for the CPU of the FOORAM architecture.

 In FOORAM, the only instruction is FOO(x) and its encoding is x.
 The instruction FOO(x) has the following semantics:
 - if x is odd: reg <- [2*x+(pc+1)]
 - if x is even: [pc+x] <- reg+pc
 - increment pc by 1

 Starting from empty memory, FOORAM performs non-trivial pseudo-random computation
 that exercises both loads, stores, and instruction fetches.

 E.g. for the first 200 steps on 16 cell machine we get 93 different memory configurations.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_CPU_CHECKER_HPP_
// #define FOORAM_CPU_CHECKER_HPP_

// use  <cstddef>
// use  <memory>

use ffec::common::serialization;

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::fooram::components::bar_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::fooram::components::fooram_protoboard;
use crate::relations::ram_computations::memory::memory_interface;



// 
pub struct fooram_cpu_checker {
//  : public fooram_gadget<FieldT>
prev_pc_addr:    pb_variable_array<FieldT>,
prev_pc_val:    pb_variable_array<FieldT>,
prev_state:    pb_variable_array<FieldT>,
guess:    pb_variable_array<FieldT>,
ls_addr:    pb_variable_array<FieldT>,
ls_prev_val:    pb_variable_array<FieldT>,
ls_next_val:    pb_variable_array<FieldT>,
next_state:    pb_variable_array<FieldT>,
next_pc_addr:    pb_variable_array<FieldT>,
next_has_accepted:    pb_variable<FieldT>,

zero:    pb_variable<FieldT>,
packed_next_pc_addr:    pb_variable<FieldT>,
one_as_addr:    pb_linear_combination_array<FieldT>,
pack_next_pc_addr:    std::shared_ptr<packing_gadget<FieldT> >,

packed_load_addr:    pb_variable<FieldT>,
packed_store_addr:    pb_variable<FieldT>,
packed_store_val:    pb_variable<FieldT>,

increment_pc:    std::shared_ptr<bar_gadget<FieldT> >,
compute_packed_load_addr:    std::shared_ptr<bar_gadget<FieldT> >,
compute_packed_store_addr:    std::shared_ptr<bar_gadget<FieldT> >,
compute_packed_store_val:    std::shared_ptr<bar_gadget<FieldT> >,

packed_ls_addr:    pb_variable<FieldT>,
packed_ls_prev_val:    pb_variable<FieldT>,
packed_ls_next_val:    pb_variable<FieldT>,
packed_prev_state:    pb_variable<FieldT>,
packed_next_state:    pb_variable<FieldT>,
pack_ls_addr:    std::shared_ptr<packing_gadget<FieldT> >,
pack_ls_prev_val:    std::shared_ptr<packing_gadget<FieldT> >,
pack_ls_next_val:    std::shared_ptr<packing_gadget<FieldT> >,
pack_prev_state:    std::shared_ptr<packing_gadget<FieldT> >,
pack_next_state:    std::shared_ptr<packing_gadget<FieldT> >,

    // fooram_cpu_checker(
// fooram_protoboard<FieldT> &pb,
    //                    pb_variable_array<FieldT> &prev_pc_addr,
    //                    pb_variable_array<FieldT> &prev_pc_val,
    //                    pb_variable_array<FieldT> &prev_state,
    //                    pb_variable_array<FieldT> &ls_addr,
    //                    pb_variable_array<FieldT> &ls_prev_val,
    //                    pb_variable_array<FieldT> &ls_next_val,
    //                    pb_variable_array<FieldT> &next_state,
    //                    pb_variable_array<FieldT> &next_pc_addr,
    //                    pb_variable<FieldT> &next_has_accepted,
    //                    const std::string &annotation_prefix);

    // void generate_r1cs_constraints();

    // void generate_r1cs_witness() { assert!(0); }

    // void generate_r1cs_witness_address();

    // void generate_r1cs_witness_other(fooram_input_tape_iterator &aux_it,
    //                                  const fooram_input_tape_iterator &aux_end);

    // void dump() const;
}



// use crate::gadgetlib1::gadgets::cpu_checkers/fooram/fooram_cpu_checker;

//#endif // FORAM_CPU_CHECKER_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the FOORAM CPU checker gadget.

 See fooram_cpu_checker.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_CPU_CHECKER_TCC_
// #define FOORAM_CPU_CHECKER_TCC_


impl fooram_cpu_checker<FieldT>{

pub fn new(
pb:    fooram_protoboard<FieldT>,
prev_pc_addr:                                               pb_variable_array<FieldT>,
prev_pc_val:                                               pb_variable_array<FieldT>,
prev_state:                                               pb_variable_array<FieldT>,
ls_addr:                                               pb_variable_array<FieldT>,
ls_prev_val:                                               pb_variable_array<FieldT>,
ls_next_val:                                               pb_variable_array<FieldT>,
next_state:                                               pb_variable_array<FieldT>,
next_pc_addr:                                               pb_variable_array<FieldT>,
next_has_accepted:                                               pb_variable<FieldT>,
annotation_prefix:                                                std::string,
) ->Self
   
{
    /* increment PC */
    packed_next_pc_addr.allocate(pb, format!("{annotation_prefix} packed_next_pc_addr"));
    pack_next_pc_addr.reset( packing_gadget::<FieldT>::new(pb, next_pc_addr, packed_next_pc_addr, format!("{annotation_prefix} pack_next_pc_addr")));

    one_as_addr.resize(next_pc_addr.len());
    one_as_addr[0].assign(self.pb, 1);
    for i in 1..next_pc_addr.len()
    {
        one_as_addr[i].assign(self.pb, 0);
    }

    /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
    increment_pc.reset( bar_gadget::<FieldT>::new(pb, prev_pc_addr, FieldT::one(), one_as_addr, FieldT::one(), packed_next_pc_addr, format!("{annotation_prefix} increment_pc")));

    /* packed_store_addr = prev_pc_addr + prev_pc_val */
    packed_store_addr.allocate(pb, format!("{annotation_prefix} packed_store_addr"));
    compute_packed_store_addr.reset(bar_gadget::<FieldT>::new(pb, prev_pc_addr, FieldT::one(), prev_pc_val, FieldT::one(), packed_store_addr, format!("{annotation_prefix} compute_packed_store_addr")));

    /* packed_load_addr = 2 * x + next_pc_addr */
    packed_load_addr.allocate(pb, format!("{annotation_prefix} packed_load_addr"));
    compute_packed_load_addr.reset(bar_gadget::<FieldT>::new(pb, prev_pc_val, FieldT(2), next_pc_addr, FieldT::one(), packed_load_addr, format!("{annotation_prefix} compute_packed_load_addr")));

    /*
      packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
      packed_ls_addr ~ ls_addr
    */
    packed_ls_addr.allocate(pb, format!("{annotation_prefix} packed_ls_addr"));
    pack_ls_addr.reset(packing_gadget::<FieldT>::new(pb, ls_addr, packed_ls_addr, " pack_ls_addr"));

    /* packed_store_val = prev_state_bits + prev_pc_addr */
    packed_store_val.allocate(pb, format!("{annotation_prefix} packed_store_val"));
    compute_packed_store_val.reset(bar_gadget::<FieldT>::new(pb, prev_state, FieldT::one(), prev_pc_addr, FieldT::one(), packed_store_val, format!("{annotation_prefix} compute_packed_store_val")));

    /*
      packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
      packed_ls_next_val ~ ls_next_val
    */
    packed_ls_prev_val.allocate(pb, format!("{annotation_prefix} packed_ls_prev_val"));
    pack_ls_prev_val.reset(packing_gadget::<FieldT>::new(self.pb, ls_prev_val, packed_ls_prev_val, format!("{annotation_prefix} pack_ls_prev_val")));
    packed_ls_next_val.allocate(pb, format!("{annotation_prefix} packed_ls_next_val"));
    pack_ls_next_val.reset(packing_gadget::<FieldT>::new(self.pb, ls_next_val, packed_ls_next_val, format!("{annotation_prefix} pack_ls_next_val")));

    /*
      packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
      packed_next_state ~ next_state
      packed_prev_state ~ prev_state
    */
    packed_prev_state.allocate(pb, format!("{annotation_prefix} packed_prev_state"));
    pack_prev_state.reset(packing_gadget::<FieldT>::new(pb, prev_state, packed_prev_state, " pack_prev_state"));

    packed_next_state.allocate(pb, format!("{annotation_prefix} packed_next_state"));
    pack_next_state.reset(packing_gadget::<FieldT>::new(pb, next_state, packed_next_state, " pack_next_state"));

    /* next_has_accepted = 1 */
    //  fooram_gadget<FieldT>(pb, annotation_prefix),
    Self{prev_pc_addr,
    prev_pc_val,
    prev_state,
    ls_addr,
    ls_prev_val,
    ls_next_val,
    next_state,
    next_pc_addr,
    next_has_accepted}
}


pub fn generate_r1cs_constraints()
{
    /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
    pack_next_pc_addr.generate_r1cs_constraints(false);
    increment_pc.generate_r1cs_constraints();

    /* packed_store_addr = prev_pc_addr + prev_pc_val */
    compute_packed_store_addr.generate_r1cs_constraints();

    /* packed_load_addr = 2 * x + next_pc_addr */
    compute_packed_load_addr.generate_r1cs_constraints();

    /*
      packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
      packed_ls_addr - packed_store_addr = x0 * (packed_load_addr - packed_store_addr)
      packed_ls_addr ~ ls_addr
    */
    pack_ls_addr.generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(prev_pc_val[0],
                                                         packed_load_addr - packed_store_addr,
                                                         packed_ls_addr - packed_store_addr),
                                 format!( "{} compute_ls_addr_packed",self.annotation_prefix));

    /* packed_store_val = prev_state_bits + prev_pc_addr */
    compute_packed_store_val.generate_r1cs_constraints();

    /*
      packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
      packed_ls_next_val - packed_store_val = x0 * (packed_ls_prev_val - packed_store_val)
      packed_ls_next_val ~ ls_next_val
    */
    pack_ls_prev_val.generate_r1cs_constraints(false);
    pack_ls_next_val.generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(prev_pc_val[0],
                                                         packed_ls_prev_val - packed_store_val,
                                                         packed_ls_next_val - packed_store_val),
                                 format!("{} compute_packed_ls_next_val",self.annotation_prefix));

    /*
      packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
      packed_next_state - packed_prev_state = x0 * (packed_ls_prev_val - packed_prev_state)
      packed_next_state ~ next_state
      packed_prev_state ~ prev_state
    */
    pack_prev_state.generate_r1cs_constraints(false);
    pack_next_state.generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(prev_pc_val[0],
                                                         packed_ls_prev_val - packed_prev_state,
                                                         packed_next_state - packed_prev_state),
                                 format!( "{} compute_packed_next_state",self.annotation_prefix));

    /* next_has_accepted = 1 */
    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, next_has_accepted, 1), format!("{} always_accepted",self.annotation_prefix));
}


pub fn generate_r1cs_witness_address()
{
    one_as_addr.evaluate(self.pb);

    /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
    increment_pc.generate_r1cs_witness();
    pack_next_pc_addr.generate_r1cs_witness_from_packed();

    /* packed_store_addr = prev_pc_addr + prev_pc_val */
    compute_packed_store_addr.generate_r1cs_witness();

    /* packed_load_addr = 2 * x + next_pc_addr */
    compute_packed_load_addr.generate_r1cs_witness();

    /*
      packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
      packed_ls_addr - packed_store_addr = x0 * (packed_load_addr - packed_store_addr)
      packed_ls_addr ~ ls_addr
    */
    self.pb.val(packed_ls_addr) = (self.pb.val(prev_pc_val[0]) * self.pb.val(packed_load_addr) +
                                    (FieldT::one()-self.pb.val(prev_pc_val[0])) * self.pb.val(packed_store_addr));
    pack_ls_addr.generate_r1cs_witness_from_packed();
}


pub fn generate_r1cs_witness_other(
aux_it:&fooram_input_tape_iterator,
aux_end:&                                                              fooram_input_tape_iterator)
{
    /* fooram memory contents do not depend on program/input. */
    // ffec::UNUSED(aux_it, aux_end);
    /* packed_store_val = prev_state_bits + prev_pc_addr */
    compute_packed_store_val.generate_r1cs_witness();

    /*
      packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
      packed_ls_next_val - packed_store_val = x0 * (packed_ls_prev_val - packed_store_val)
      packed_ls_next_val ~ ls_next_val
    */
    pack_ls_prev_val.generate_r1cs_witness_from_bits();
    self.pb.val(packed_ls_next_val) = (self.pb.val(prev_pc_val[0]) * self.pb.val(packed_ls_prev_val) +
                                        (FieldT::one() - self.pb.val(prev_pc_val[0])) * self.pb.val(packed_store_val));
    pack_ls_next_val.generate_r1cs_witness_from_packed();

    /*
      packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
      packed_next_state - packed_prev_state = x0 * (packed_ls_prev_val - packed_prev_state)
      packed_next_state ~ next_state
      packed_prev_state ~ prev_state
    */
    pack_prev_state.generate_r1cs_witness_from_bits();
    self.pb.val(packed_next_state) = (self.pb.val(prev_pc_val[0]) * self.pb.val(packed_ls_prev_val) +
                                       (FieldT::one() - self.pb.val(prev_pc_val[0])) * self.pb.val(packed_prev_state));
    pack_next_state.generate_r1cs_witness_from_packed();

    /* next_has_accepted = 1 */
    self.pb.val(next_has_accepted) = FieldT::one();
}


pub fn dump() 
{
    print!("packed_store_addr: ");
    self.pb.val(packed_store_addr).print();
    print!("packed_load_addr: ");
    self.pb.val(packed_load_addr).print();
    print!("packed_ls_addr: ");
    self.pb.val(packed_ls_addr).print();
    print!("packed_store_val: ");
    self.pb.val(packed_store_val).print();
    print!("packed_ls_next_val: ");
    self.pb.val(packed_ls_next_val).print();
    print!("packed_next_state: ");
    self.pb.val(packed_next_state).print();
}

}

//#endif // FOORAM_CPU_CHECKER_TCC
