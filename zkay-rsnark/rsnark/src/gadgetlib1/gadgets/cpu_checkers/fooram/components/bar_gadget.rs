/** @file
 *****************************************************************************

 Declaration of interfaces for an auxiliarry gadget for the FOORAM CPU.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef BAR_GADGET_HPP_
#define BAR_GADGET_HPP_

use  <libsnark/gadgetlib1/gadget.hpp>
use  <libsnark/gadgetlib1/gadgets/basic_gadgets.hpp>

namespace libsnark {

/**
 * The bar gadget checks linear combination
 *                   Z = aX + bY (mod 2^w)
 * for a, b - const, X, Y - vectors of w bits,
 * where w is implicitly inferred, Z - a packed variable.
 *
 * This gadget is used four times in fooram:
 * - PC' = PC + 1
 * - load_addr = 2 * x + PC'
 * - store_addr = x + PC
 */
template<typename FieldT>
class bar_gadget : public gadget<FieldT> {
public:
    pb_linear_combination_array<FieldT> X;
    FieldT a;
    pb_linear_combination_array<FieldT> Y;
    FieldT b;
    pb_linear_combination<FieldT> Z_packed;
    pb_variable_array<FieldT> Z_bits;

    pb_variable<FieldT> result;
    pb_variable_array<FieldT> overflow;
    pb_variable_array<FieldT> unpacked_result;

    std::shared_ptr<packing_gadget<FieldT> > unpack_result;
    std::shared_ptr<packing_gadget<FieldT> > pack_Z;

    size_t width;
    bar_gadget(protoboard<FieldT> &pb,
               const pb_linear_combination_array<FieldT> &X,
               const FieldT &a,
               const pb_linear_combination_array<FieldT> &Y,
               const FieldT &b,
               const pb_linear_combination<FieldT> &Z_packed,
               const std::string &annotation_prefix);
    void generate_r1cs_constraints();
    void generate_r1cs_witness();
};

} // libsnark

use  <libsnark/gadgetlib1/gadgets/cpu_checkers/fooram/components/bar_gadget.tcc>

#endif // BAR_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for an auxiliary gadget for the FOORAM CPU.

 See bar_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef BAR_GADGET_TCC_
#define BAR_GADGET_TCC_

namespace libsnark {

template<typename FieldT>
bar_gadget<FieldT>::bar_gadget(protoboard<FieldT> &pb,
                               const pb_linear_combination_array<FieldT> &X,
                               const FieldT &a,
                               const pb_linear_combination_array<FieldT> &Y,
                               const FieldT &b,
                               const pb_linear_combination<FieldT> &Z_packed,
                               const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    X(X),
    a(a),
    Y(Y),
    b(b),
    Z_packed(Z_packed)
{
    assert(X.size() == Y.size());
    width = X.size();

    result.allocate(pb, FMT(annotation_prefix, " result"));
    Z_bits.allocate(pb, width, FMT(annotation_prefix, " Z_bits"));
    overflow.allocate(pb, 2*width, FMT(annotation_prefix, " overflow"));

    unpacked_result.insert(unpacked_result.end(), Z_bits.begin(), Z_bits.end());
    unpacked_result.insert(unpacked_result.end(), overflow.begin(), overflow.end());

    unpack_result.reset(new packing_gadget<FieldT>(pb, unpacked_result, result, FMT(annotation_prefix, " unpack_result")));
    pack_Z.reset(new packing_gadget<FieldT>(pb, Z_bits, Z_packed, FMT(annotation_prefix, " pack_Z")));
}

template<typename FieldT>
void bar_gadget<FieldT>::generate_r1cs_constraints()
{
    unpack_result->generate_r1cs_constraints(true);
    pack_Z->generate_r1cs_constraints(false);

    this->pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, a * pb_packing_sum<FieldT>(X) + b * pb_packing_sum<FieldT>(Y), result), FMT(this->annotation_prefix, " compute_result"));
}

template<typename FieldT>
void bar_gadget<FieldT>::generate_r1cs_witness()
{
    this->pb.val(result) = X.get_field_element_from_bits(this->pb) * a + Y.get_field_element_from_bits(this->pb) * b;
    unpack_result->generate_r1cs_witness_from_packed();

    pack_Z->generate_r1cs_witness_from_bits();
}

} // libsnark

#endif // BAR_GADGET_TCC_
