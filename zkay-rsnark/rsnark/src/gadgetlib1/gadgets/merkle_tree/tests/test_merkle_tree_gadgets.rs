/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// #ifdef CURVE_BN128
use ffec::algebra::curves::bn128/bn128_pp;
//#endif
use ffec::algebra::curves::edwards/edwards_pp;
use ffec::algebra::curves::mnt::mnt4::mnt4_pp;
use ffec::algebra::curves::mnt::mnt6::mnt6_pp;

use crate::gadgetlib1::gadgets::hashes::sha256/sha256_gadget;
use crate::gadgetlib1::gadgets::merkle_tree/merkle_tree_check_read_gadget;
use crate::gadgetlib1::gadgets::merkle_tree/merkle_tree_check_update_gadget;



template<typename ppT>
void test_all_merkle_tree_gadgets()
{
    type ffec::Fr<ppT> FieldT;
    test_merkle_tree_check_read_gadget<FieldT, CRH_with_bit_out_gadget<FieldT> >();
    test_merkle_tree_check_read_gadget<FieldT, sha256_two_to_one_hash_gadget<FieldT> >();

    test_merkle_tree_check_update_gadget<FieldT, CRH_with_bit_out_gadget<FieldT> >();
    test_merkle_tree_check_update_gadget<FieldT, sha256_two_to_one_hash_gadget<FieldT> >();
}

int main(void)
{
    ffec::start_profiling();

// #ifdef CURVE_BN128       // BN128 has fancy dependencies so it may be disabled
    ffec::bn128_pp::init_public_params();
    test_all_merkle_tree_gadgets<ffec::bn128_pp>();
//#endif

    ffec::edwards_pp::init_public_params();
    test_all_merkle_tree_gadgets<ffec::edwards_pp>();

    ffec::mnt4_pp::init_public_params();
    test_all_merkle_tree_gadgets<ffec::mnt4_pp>();

    ffec::mnt6_pp::init_public_params();
    test_all_merkle_tree_gadgets<ffec::mnt6_pp>();
}
