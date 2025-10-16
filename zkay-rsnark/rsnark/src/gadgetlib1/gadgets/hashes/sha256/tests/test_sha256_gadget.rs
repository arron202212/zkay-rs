/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use ffec::common::default_types::ec_pp;
use ffec::common::profiling;
use ffec::common::utils;

use crate::gadgetlib1::gadgets::hashes::sha256/sha256_gadget;



template<typename FieldT>
void test_two_to_one()
{
    protoboard<FieldT> pb;

    digest_variable<FieldT> left(pb, SHA256_digest_size, "left");
    digest_variable<FieldT> right(pb, SHA256_digest_size, "right");
    digest_variable<FieldT> output(pb, SHA256_digest_size, "output");

    sha256_two_to_one_hash_gadget<FieldT> f(pb, left, right, output, "f");
    f.generate_r1cs_constraints();
    print!("Number of constraints for sha256_two_to_one_hash_gadget: {}\n", pb.num_constraints());

    const ffec::bit_vector left_bv = ffec::int_list_to_bits({0x426bc2d8, 0x4dc86782, 0x81e8957a, 0x409ec148, 0xe6cffbe8, 0xafe6ba4f, 0x9c6f1978, 0xdd7af7e9}, 32);
    const ffec::bit_vector right_bv = ffec::int_list_to_bits({0x038cce42, 0xabd366b8, 0x3ede7e00, 0x9130de53, 0x72cdf73d, 0xee825114, 0x8cb48d1b, 0x9af68ad0}, 32);
    const ffec::bit_vector hash_bv = ffec::int_list_to_bits({0xeffd0b7f, 0x1ccba116, 0x2ee816f7, 0x31c62b48, 0x59305141, 0x990e5c0a, 0xce40d33d, 0x0b1167d1}, 32);

    left.generate_r1cs_witness(left_bv);
    right.generate_r1cs_witness(right_bv);

    f.generate_r1cs_witness();
    output.generate_r1cs_witness(hash_bv);

    assert!(pb.is_satisfied());
}

int main(void)
{
    ffec::start_profiling();
    ffec::default_ec_pp::init_public_params();
    test_two_to_one<ffec::Fr<ffec::default_ec_pp> >();
}
