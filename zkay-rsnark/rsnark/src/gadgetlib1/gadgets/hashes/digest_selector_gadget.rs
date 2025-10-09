/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef DIGEST_SELECTOR_GADGET_HPP_
// #define DIGEST_SELECTOR_GADGET_HPP_

use  <vector>

use libsnark/gadgetlib1/gadgets/basic_gadgets;
use libsnark/gadgetlib1/gadgets/hashes/hash_io;



template<typename FieldT>
class digest_selector_gadget : public gadget<FieldT> {
public:
    size_t digest_size;
    digest_variable<FieldT> input;
    pb_linear_combination<FieldT> is_right;
    digest_variable<FieldT> left;
    digest_variable<FieldT> right;

    digest_selector_gadget(protoboard<FieldT> &pb,
                           const size_t digest_size,
                           const digest_variable<FieldT> &input,
                           const pb_linear_combination<FieldT> &is_right,
                           const digest_variable<FieldT> &left,
                           const digest_variable<FieldT> &right,
                           const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};



use libsnark/gadgetlib1/gadgets/hashes/digest_selector_gadget;

//#endif // DIGEST_SELECTOR_GADGET_HPP_
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef DIGEST_SELECTOR_GADGET_TCC_
// #define DIGEST_SELECTOR_GADGET_TCC_



template<typename FieldT>
digest_selector_gadget<FieldT>::digest_selector_gadget(protoboard<FieldT> &pb,
                                                       const size_t digest_size,
                                                       const digest_variable<FieldT> &input,
                                                       const pb_linear_combination<FieldT> &is_right,
                                                       const digest_variable<FieldT> &left,
                                                       const digest_variable<FieldT> &right,
                                                       const std::string &annotation_prefix) :
gadget<FieldT>(pb, annotation_prefix), digest_size(digest_size), input(input), is_right(is_right), left(left), right(right)
{
}

template<typename FieldT>
void digest_selector_gadget<FieldT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < digest_size; ++i)
    {
        /*
          input = is_right * right + (1-is_right) * left
          input - left = is_right(right - left)
        */
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(is_right, right.bits[i] - left.bits[i], input.bits[i] - left.bits[i]),
                                     FMT(self.annotation_prefix, " propagate_{}", i));
    }
}

template<typename FieldT>
void digest_selector_gadget<FieldT>::generate_r1cs_witness()
{
    is_right.evaluate(self.pb);

    assert!(self.pb.lc_val(is_right) == FieldT::one() || self.pb.lc_val(is_right) == FieldT::zero());
    if (self.pb.lc_val(is_right) == FieldT::one())
    {
        for (size_t i = 0; i < digest_size; ++i)
        {
            self.pb.val(right.bits[i]) = self.pb.val(input.bits[i]);
        }
    }
    else
    {
        for (size_t i = 0; i < digest_size; ++i)
        {
            self.pb.val(left.bits[i]) = self.pb.val(input.bits[i]);
        }
    }
}



//#endif // DIGEST_SELECTOR_GADGET_TCC_
