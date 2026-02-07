use crate::common::data_structures::merkle_tree::HashTConfig;
use crate::gadgetlib1::gadgets::hashes::hash_io::digest_variable;
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components::SHA256_digest_size;
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_gadget::sha256_two_to_one_hash_gadget;
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use ff_curves::{Fr, PublicParams, default_ec_pp};
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use ffec::{FieldTConfig, int_list_to_bits};
use rccell::RcCell;

pub fn test_two_to_one<FieldT: FieldTConfig, PB: PBConfig>() {
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let mut left =
        digest_variable::<FieldT, PB>::new(pb.clone(), SHA256_digest_size, "left".to_owned());
    let mut right =
        digest_variable::<FieldT, PB>::new(pb.clone(), SHA256_digest_size, "right".to_owned());
    let mut output =
        digest_variable::<FieldT, PB>::new(pb.clone(), SHA256_digest_size, "output".to_owned());

    let mut f = sha256_two_to_one_hash_gadget::<FieldT, PB>::new(
        pb.clone(),
        left.clone(),
        right.clone(),
        output.clone(),
        "f".to_owned(),
    );
    f.generate_r1cs_constraints(true);
    print!(
        "Number of constraints for sha256_two_to_one_hash_gadget: {}\n",
        pb.borrow().num_constraints()
    );

    let left_bv = int_list_to_bits(
        &[
            0x426bc2d8, 0x4dc86782, 0x81e8957a, 0x409ec148, 0xe6cffbe8, 0xafe6ba4f, 0x9c6f1978,
            0xdd7af7e9,
        ],
        32,
    );
    let right_bv = int_list_to_bits(
        &[
            0x038cce42, 0xabd366b8, 0x3ede7e00, 0x9130de53, 0x72cdf73d, 0xee825114, 0x8cb48d1b,
            0x9af68ad0,
        ],
        32,
    );
    let hash_bv = int_list_to_bits(
        &[
            0xeffd0b7f, 0x1ccba116, 0x2ee816f7, 0x31c62b48, 0x59305141, 0x990e5c0a, 0xce40d33d,
            0x0b1167d1,
        ],
        32,
    );

    left.generate_r1cs_witness(&left_bv);
    right.generate_r1cs_witness(&right_bv);

    f.generate_r1cs_witness();
    output.generate_r1cs_witness(&hash_bv);

    assert!(pb.borrow().is_satisfied());
}

pub fn main<PB: PBConfig>() -> i32 {
    start_profiling();
    default_ec_pp::init_public_params();
    test_two_to_one::<Fr<default_ec_pp>, PB>();
    0
}
