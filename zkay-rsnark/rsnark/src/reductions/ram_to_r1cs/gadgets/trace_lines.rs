/** @file
*****************************************************************************

Declaration of interfaces for trace-line variables.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TRACE_LINES_HPP_
// #define TRACE_LINES_HPP_

//
use ffec::common::utils;

// use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::relations::ram_computations::rams::ram_params;

/**
 * A memory line contains variables for the following:
 * - timestamp
 * - address
 * - contents_before
 * - contents_after
 *
 * Memory lines are used by memory_checker_gadget.
 */
//

// type FieldT = ram_base_field<ramT>;
pub struct memory_line_variable_gadget<FieldT:FieldTConfig,ramT> {
    //: public ram_gadget_base
    timestamp: RcCell<dual_variable_gadget<FieldT>>,
    address: RcCell<dual_variable_gadget<FieldT>>,
    contents_before: RcCell<dual_variable_gadget<FieldT>>,
    contents_after: RcCell<dual_variable_gadget<FieldT>>,
}

/**
 * An execution line inherits from a memory line and, in addition, contains
 * variables for a CPU state and for a flag denoting if the machine has accepted.
 *
 * Execution lines are used by execution_checker_gadget.
 */
// type FieldT=ram_base_field<ramT> ;
pub struct execution_line_variable_gadget<FieldT:FieldTConfig,ramT> {
    // / : public memory_line_variable_gadget
    cpu_state: pb_variable_array<FieldT>,
    has_accepted: pb_variable<FieldT>,
}


impl<FieldT:FieldTConfig,ramT> memory_line_variable_gadget<FieldT,ramT> {
    pub fn new(
        pb: ram_protoboard<ramT>,
        timestamp_size: usize,
        ap: ram_architecture_params<ramT>,
        annotation_prefix: String,
    ) -> Self {
        let address_size = ap.address_size();
        let value_size = ap.value_size();

       let  timestamp=RcCell::new(dual_variable_gadget::<FieldT>::new(
            pb,
            timestamp_size,
            FMT(self.annotation_prefix, " timestamp"),
        ));
       let address=RcCell::new(dual_variable_gadget::<FieldT>::new(
            pb,
            address_size,
            FMT(self.annotation_prefix, " address"),
        ));
        let contents_before=RcCell::new(dual_variable_gadget::<FieldT>::new(
            pb,
            value_size,
            FMT(self.annotation_prefix, " contents_before"),
        ));
        let contents_after=RcCell::new(dual_variable_gadget::<FieldT>::new(
            pb,
            value_size,
            FMT(self.annotation_prefix, " contents_after"),
        ));
        // ram_gadget_base::<ramT>(&pb, annotation_prefix)
        Self{timestamp,address,contents_before,contents_after}
    }

    //
    pub fn generate_r1cs_constraints(&self,enforce_bitness: bool) {
        self.timestamp.generate_r1cs_constraints(enforce_bitness);
        self.address.generate_r1cs_constraints(enforce_bitness);
        self.contents_before.generate_r1cs_constraints(enforce_bitness);
        self.contents_after.generate_r1cs_constraints(enforce_bitness);
    }

    pub fn generate_r1cs_witness_from_bits() {
        self.timestamp.generate_r1cs_witness_from_bits();
        self.address.generate_r1cs_witness_from_bits();
        self.contents_before.generate_r1cs_witness_from_bits();
        self.contents_after.generate_r1cs_witness_from_bits();
    }

    pub fn generate_r1cs_witness_from_packed() {
        self.timestamp.generate_r1cs_witness_from_packed();
        self.address.generate_r1cs_witness_from_packed();
        self.contents_before.generate_r1cs_witness_from_packed();
        self.contents_after.generate_r1cs_witness_from_packed();
    }

    pub fn all_vars(&self) -> pb_variable_array<FieldT> {
        let mut r = pb_variable_array::<FieldT>::new();
        r.extend(&self.timestamp);
        r.extend(&self.address);
        r.extend(&self.contents_before);
        r.extend(&self.contents_after);

        return r;
    }
}
impl<FieldT:FieldTConfig,ramT> execution_line_variable_gadget<FieldT,ramT> {
    pub fn new(
        pb: ram_protoboard<ramT>,
        timestamp_size: usize,
        ap: ram_architecture_params<ramT>,
        annotation_prefix: String,
    ) -> Self {
        let cpu_state_size = ap.cpu_state_size();

        cpu_state.allocate(&pb, cpu_state_size, FMT(annotation_prefix, " cpu_state"));
        has_accepted.allocate(&pb, FMT(annotation_prefix, " has_accepted"));
        // memory_line_variable_gadget<ramT>(&pb, timestamp_size, ap, annotation_prefix)
        Self {}
    }
}

//#endif // TRACE_LINES_TCC_
