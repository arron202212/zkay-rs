/** @file
 *****************************************************************************

 Declaration of interfaces for a RAM-to-R1CS reduction, that is, constructing
 a R1CS ("Rank-1 Constraint System") from a RAM ("Random-Access Machine").

 The implementation is a thin layer around a "RAM universal gadget", which is
 where most of the work is done. See gadgets::ram_universal_gadget.hpp for details.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_TO_R1CS_HPP_
// #define RAM_TO_R1CS_HPP_

// ram_universal_gadget;



type FieldT=ram_base_field<ramT>;
pub struct ram_to_r1cs<ramT> {


    

boot_trace_size_bound:    usize,

main_protoboard:    ram_protoboard<ramT>,
r1cs_input:    pb_variable_array<FieldT>,
universal_gadget:    RcCell<ram_universal_gadget<ramT> >,
}





// use crate::reductions::ram_to_r1cs::ram_to_r1cs;

//#endif // RAM_TO_R1CS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a RAM-to-R1CS reduction, that is, constructing
 a R1CS ("Rank-1 Constraint System") from a RAM ("Random-Access Machine").

 See ram_to_r1cs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_TO_R1CS_TCC_
// #define RAM_TO_R1CS_TCC_

// use  <set>


impl ram_to_r1cs<ramT>{

pub fn new(ap:&ram_architecture_params<ramT>,
                               boot_trace_size_bound:usize,
                               time_bound:usize) ->Self
   
{
    let  r1cs_input_size = ram_universal_gadget::<ramT>::packed_input_size(ap, boot_trace_size_bound);
    r1cs_input.allocate(main_protoboard, r1cs_input_size, "r1cs_input");
    universal_gadget.reset( ram_universal_gadget::<ramT>::new(main_protoboard,
                                                          boot_trace_size_bound,
                                                          time_bound,
                                                          r1cs_input,
                                                          "universal_gadget"));
    main_protoboard.set_input_sizes(r1cs_input_size);
    Self {boot_trace_size_bound,
    main_protoboard}
}


pub fn instance_map()
{
    ffec::enter_block("Call to instance_map of ram_to_r1cs");
    universal_gadget.generate_r1cs_constraints();
    ffec::leave_block("Call to instance_map of ram_to_r1cs");
}


 pub fn get_constraint_system() ->r1cs_constraint_system<ram_base_field<ramT> >
{
    return main_protoboard.get_constraint_system();
}


pub fn auxiliary_input_map(boot_trace:&ram_boot_trace<ramT>,
                                                                                 auxiliary_input:&ram_input_tape<ramT>)->r1cs_primary_input<ram_base_field<ramT> > 
{
    ffec::enter_block("Call to witness_map of ram_to_r1cs");
    universal_gadget.generate_r1cs_witness(boot_trace, auxiliary_input);
// #ifdef DEBUG
   let  primary_input_from_input_map = Self::primary_input_map(main_protoboard.ap, boot_trace_size_bound, boot_trace);
    let primary_input_from_witness_map = main_protoboard.primary_input();
    assert!(primary_input_from_input_map == primary_input_from_witness_map);
//#endif
    ffec::leave_block("Call to witness_map of ram_to_r1cs");
    return main_protoboard.auxiliary_input();
}


pub fn print_execution_trace() 
{
    universal_gadget.print_execution_trace();
}


pub fn print_memory_trace() 
{
    universal_gadget.print_memory_trace();
}


 pub fn pack_primary_input_address_and_value(ap:&ram_architecture_params<ramT>,
                                                                                           av:&address_and_value)->Vec<ram_base_field<ramT> >
{
    type FieldT=ram_base_field<ramT>;

    let address = av.0;
    let contents = av.1;

    let address_bits = ffec::convert_field_element_to_bit_vector::<FieldT>(FieldT(address, true), ap.address_size());
    let contents_bits = ffec::convert_field_element_to_bit_vector::<FieldT>(FieldT(contents, true), ap.value_size());

    let mut  trace_element_bits=vec![];
    trace_element_bits.insert(trace_element_bits.end(), address_bits.begin(), address_bits.end());
    trace_element_bits.insert(trace_element_bits.end(), contents_bits.begin(), contents_bits.end());

    let trace_element = ffec::pack_bit_vector_into_field_element_vector::<FieldT>(trace_element_bits);

    return trace_element;
}



 pub fn primary_input_map(ap:&ram_architecture_params<ramT>,
                                                                               boot_trace_size_bound:&usize,
                                                                               boot_trace:&ram_boot_trace<ramT>)->r1cs_primary_input<ram_base_field<ramT> >
{
    type FieldT=ram_base_field<ramT>;

    let packed_input_element_size = ram_universal_gadget::<ramT>::packed_input_element_size(ap);
     let mut result=r1cs_primary_input::<FieldT >::new(ram_universal_gadget::<ramT>::packed_input_size(ap, boot_trace_size_bound));

    let mut  bound_input_locations=BTreeSet::new();

    for it in &boot_trace.get_all_trace_entries()
    {
        let input_pos = it.0;
        let av = it.1;

        assert!(input_pos < boot_trace_size_bound);
        assert!(bound_input_locations.find(input_pos).is_none);

        let packed_input_element = Self::pack_primary_input_address_and_value(ap, av);
        // std::copy(packed_input_element.begin(), packed_input_element.end(), result.begin() + ));
        result.splice(packed_input_element_size * (boot_trace_size_bound - 1 - input_pos,packed_input_element));
        bound_input_locations.insert(input_pos);
    }

    return result;
}

}

//#endif // RAM_TO_R1CS_TCC_
