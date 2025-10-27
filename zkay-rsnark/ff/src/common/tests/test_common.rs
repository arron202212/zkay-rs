/**
 *****************************************************************************
 Some tests for the functions in this directory.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#include <cstdint>
//#include <gtest/gtest.h>

use crate::common::utils;
use crate::algebra::fields::binary::gf32;
use crate::algebra::curves::edwards::edwards_pp;
use crate::algebra::curves::mnt::mnt6::mnt6_pp;

using namespace libff;

TEST(Log2Test, SimpleTest) {
    // There seems to be a second log2 function that operates on floats so we added .
    EXPECT_EQ(log2(0), 0ULL);
    EXPECT_EQ(log2(1), 0ULL);
    EXPECT_EQ(log2(2), 1ULL);
    EXPECT_EQ(log2(3), 2ULL);
    EXPECT_EQ(log2(4), 2ULL);
    EXPECT_EQ(log2(5), 3ULL);
    EXPECT_EQ(log2(6), 3ULL);
    EXPECT_EQ(log2(7), 3ULL);
    EXPECT_EQ(log2(8), 3ULL);
    EXPECT_EQ(log2(9), 4ULL);
}

TEST(Log2Test, PowersOfTwo) {
    for i in 10..20{
    {
        const std::usize k = (1ULL<<i);
        EXPECT_EQ(log2(k-1), i);
        EXPECT_EQ(log2(k), i);
        EXPECT_EQ(log2(k+1), i+1);
    }
}


pub fn  test_random_element()
{
    FieldT x = random_element_non_zero_one<FieldT>();
    EXPECT_NE(x, FieldT::zero());
    EXPECT_NE(x, FieldT::one());

    x = random_element_non_zero<FieldT>();
    EXPECT_NE(x, FieldT::zero());

    FieldT y = random_element_exclude(x);
    EXPECT_NE(x, y);
}

TEST(UtilsTest, RandomElementTest)
{
    init_edwards_fields();
    test_random_element<edwards_Fq3>();
    test_random_element<gf32>();
}

TEST(UtilsTest, CurveVectorSizeTest)
{
    init_edwards_params();
    init_mnt6_params();

    Vec<edwards_G1> vec;

    vec.push_back(edwards_G1::G1_one);
    vec.push_back(edwards_G1::G1_zero);
    vec.push_back(edwards_G1::G1_one);

    EXPECT_EQ(curve_size_in_bits(vec), 552);

    Vec<mnt6_G2> vec2;

    vec2.push_back(mnt6_G2::G2_zero);
    vec2.push_back(mnt6_G2::G2_one);
    vec2.push_back(mnt6_G2::G2_one);
    vec2.push_back(mnt6_G2::G2_zero);
    vec2.push_back(mnt6_G2::G2_zero);

    EXPECT_EQ(curve_size_in_bits(vec2), 4475);
}
