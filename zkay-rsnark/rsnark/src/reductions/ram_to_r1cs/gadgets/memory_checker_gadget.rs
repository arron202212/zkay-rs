/** @file
 *****************************************************************************

 Declaration of interfaces for memory_checker_gadget, a gadget that verifies the
 consistency of two accesses to memory that are adjacent in a "memory sort".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_CHECKER_GADGET_HPP_
// #define MEMORY_CHECKER_GADGET_HPP_

use libsnark/reductions/ram_to_r1cs/gadgets/trace_lines;



template<typename ramT>
class memory_checker_gadget : public ram_gadget_base<ramT> {
private:

    type ram_base_field<ramT> FieldT;

    pb_variable<FieldT> timestamps_leq;
    pb_variable<FieldT> timestamps_less;
    std::shared_ptr<comparison_gadget<FieldT> > compare_timestamps;

    pb_variable<FieldT> addresses_eq;
    pb_variable<FieldT> addresses_leq;
    pb_variable<FieldT> addresses_less;
    std::shared_ptr<comparison_gadget<FieldT> > compare_addresses;

    pb_variable<FieldT> loose_contents_after1_equals_contents_before2;
    pb_variable<FieldT> loose_contents_before2_equals_zero;
    pb_variable<FieldT> loose_timestamp2_is_zero;

public:

    memory_line_variable_gadget<ramT> line1;
    memory_line_variable_gadget<ramT> line2;

    memory_checker_gadget(ram_protoboard<ramT> &pb,
                          const size_t timestamp_size,
                          const memory_line_variable_gadget<ramT> &line1,
                          const memory_line_variable_gadget<ramT> &line2,
                          const std::string& annotation_prefix="");

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use libsnark/reductions/ram_to_r1cs/gadgets/memory_checker_gadget;

//#endif // MEMORY_CHECKER_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for memory_checker_gadget.

 See memory_checker_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_CHECKER_GADGET_TCC_
// #define MEMORY_CHECKER_GADGET_TCC_



template<typename ramT>
memory_checker_gadget<ramT>::memory_checker_gadget(ram_protoboard<ramT> &pb,
                                                   const size_t timestamp_size,
                                                   const memory_line_variable_gadget<ramT> &line1,
                                                   const memory_line_variable_gadget<ramT> &line2,
                                                   const std::string& annotation_prefix) :
    ram_gadget_base<ramT>(pb, annotation_prefix), line1(line1), line2(line2)
{
    /* compare the two timestamps */
    timestamps_leq.allocate(pb, FMT(self.annotation_prefix, " timestamps_leq"));
    timestamps_less.allocate(pb, FMT(self.annotation_prefix, " timestamps_less"));
    compare_timestamps.reset(new comparison_gadget<FieldT>(pb, timestamp_size, line1.timestamp->packed, line2.timestamp->packed, timestamps_less, timestamps_leq,
                                                           FMT(self.annotation_prefix, " compare_ts")));


    /* compare the two addresses */
    const size_t address_size = pb.ap.address_size();
    addresses_eq.allocate(pb, FMT(self.annotation_prefix, " addresses_eq"));
    addresses_leq.allocate(pb, FMT(self.annotation_prefix, " addresses_leq"));
    addresses_less.allocate(pb, FMT(self.annotation_prefix, " addresses_less"));
    compare_addresses.reset(new comparison_gadget<FieldT>(pb, address_size, line1.address->packed, line2.address->packed, addresses_less, addresses_leq,
                                                          FMT(self.annotation_prefix, " compare_addresses")));

    /*
      Add variables that will contain flags representing the following relations:
      - "line1.contents_after = line2.contents_before" (to check that contents do not change between instructions);
      - "line2.contents_before = 0" (for the first access at an address); and
      - "line2.timestamp = 0" (for wrap-around checks to ensure only one 'cycle' in the memory sort).

      More precisely, each of the above flags is "loose" (i.e., it equals 0 if
      the relation holds, but can be either 0 or 1 if the relation does not hold).
     */
    loose_contents_after1_equals_contents_before2.allocate(pb, FMT(self.annotation_prefix, " loose_contents_after1_equals_contents_before2"));
    loose_contents_before2_equals_zero.allocate(pb, FMT(self.annotation_prefix, " loose_contents_before2_equals_zero"));
    loose_timestamp2_is_zero.allocate(pb, FMT(self.annotation_prefix, " loose_timestamp2_is_zero"));
}

template<typename ramT>
void memory_checker_gadget<ramT>::generate_r1cs_constraints()
{
    /* compare the two timestamps */
    compare_timestamps->generate_r1cs_constraints();

    /* compare the two addresses */
    compare_addresses->generate_r1cs_constraints();
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(addresses_leq, 1 - addresses_less, addresses_eq), FMT(self.annotation_prefix, " addresses_eq"));

    /*
      Add constraints for the following three flags:
       - loose_contents_after1_equals_contents_before2;
       - loose_contents_before2_equals_zero;
       - loose_timestamp2_is_zero.
     */
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(loose_contents_after1_equals_contents_before2,
                                                         line1.contents_after->packed - line2.contents_before->packed, 0),
                                 FMT(self.annotation_prefix, " loose_contents_after1_equals_contents_before2"));
    generate_boolean_r1cs_constraint<FieldT>(self.pb, loose_contents_after1_equals_contents_before2, FMT(self.annotation_prefix, " loose_contents_after1_equals_contents_before2"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(loose_contents_before2_equals_zero,
                                                         line2.contents_before->packed, 0),
                                 FMT(self.annotation_prefix, " loose_contents_before2_equals_zero"));
    generate_boolean_r1cs_constraint<FieldT>(self.pb, loose_contents_before2_equals_zero, FMT(self.annotation_prefix, " loose_contents_before2_equals_zero"));

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(loose_timestamp2_is_zero,
                                                         line2.timestamp->packed, 0),
                                 FMT(self.annotation_prefix, " loose_timestamp2_is_zero"));
    generate_boolean_r1cs_constraint<FieldT>(self.pb, loose_timestamp2_is_zero, FMT(self.annotation_prefix, " loose_timestamp2_is_zero"));

    /*
      The three cases that need to be checked are:

      line1.address = line2.address => line1.contents_after = line2.contents_before
      (i.e. contents do not change between accesses to the same address)

      line1.address < line2.address => line2.contents_before = 0
      (i.e. access to new address has the "before" value set to 0)

      line1.address > line2.address => line2.timestamp = 0
      (i.e. there is only one cycle with non-decreasing addresses, except
      for the case where we go back to a unique pre-set timestamp; we choose
      timestamp 0 to be the one that touches address 0)

      As usual, we implement "A => B" as "NOT (A AND (NOT B))".
    */
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(addresses_eq, 1 - loose_contents_after1_equals_contents_before2, 0),
                                 FMT(self.annotation_prefix, " memory_retains_contents_between_accesses"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(addresses_less, 1 - loose_contents_before2_equals_zero, 0),
                                 FMT(self.annotation_prefix, " new_address_starts_at_zero"));
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1 - addresses_leq, 1 - loose_timestamp2_is_zero, 0),
                                 FMT(self.annotation_prefix, " only_one_cycle"));
}

template<typename ramT>
void memory_checker_gadget<ramT>::generate_r1cs_witness()
{
    /* compare the two addresses */
    compare_addresses->generate_r1cs_witness();
    self.pb.val(addresses_eq) = self.pb.val(addresses_leq) * (FieldT::one() - self.pb.val(addresses_less));

    /* compare the two timestamps */
    compare_timestamps->generate_r1cs_witness();

    /*
      compare the values of:
      - loose_contents_after1_equals_contents_before2;
      - loose_contents_before2_equals_zero;
      - loose_timestamp2_is_zero.
     */
    self.pb.val(loose_contents_after1_equals_contents_before2) = (self.pb.val(line1.contents_after->packed) == self.pb.val(line2.contents_before->packed)) ? FieldT::one() : FieldT::zero();
    self.pb.val(loose_contents_before2_equals_zero) = self.pb.val(line2.contents_before->packed).is_zero() ? FieldT::one() : FieldT::zero();
    self.pb.val(loose_timestamp2_is_zero) = (self.pb.val(line2.timestamp->packed) == FieldT::zero() ? FieldT::one() : FieldT::zero());
}



//#endif // MEMORY_CHECKER_GADGET_TCC_
