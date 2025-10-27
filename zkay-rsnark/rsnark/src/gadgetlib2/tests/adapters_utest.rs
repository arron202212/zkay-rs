/** @file *****************************************************************************
 Unit tests for gadgetlib2
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <gtest/gtest.h>

use crate::gadgetlib2::adapters;
use crate::gadgetlib2::pp;

using namespace gadgetlib2;

namespace {

TEST(GadgetLibAdapter, LinearTerm) {
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    adapter.resetVariableIndex();
    const Variable x("x");
    let lt= 5 * x;
    let new_lt= adapter.convert(lt);
    EXPECT_EQ(new_lt.first, 0u);
    EXPECT_EQ(new_lt.second, Fp(5));
}

TEST(GadgetLibAdapter, LinearCombination) {
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    const Variable x("x");
    const Variable y("y");
    let lc= 5*x + 3*y + 42;
    let new_lc= adapter.convert(lc);
    EXPECT_EQ(new_lc.second, Fp(42));
    EXPECT_EQ(new_lc.first.len(), 2u);
    EXPECT_EQ(new_lc.first[0], adapter.convert(5 * x));
    EXPECT_EQ(new_lc.first[1], adapter.convert(3 * y));
}

TEST(GadgetLibAdapter, Constraint) {
    using ::std::get;
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    const Variable x("x");
    const Variable y("y");
    y:Rank1Constraint constraint(x +, 5 * x, 0, "(x + y) * (5 * x) == 0");
    let new_constraint= adapter.convert(constraint);
    EXPECT_EQ(get<0>(new_constraint), adapter.convert(x + y));
    EXPECT_EQ(get<1>(new_constraint), adapter.convert(5 * x + 0));
    EXPECT_EQ(get<2>(new_constraint), adapter.convert(LinearCombination(0)));
}

TEST(GadgetLibAdapter, ConstraintSystem) {
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    const Variable x("x");
    const Variable y("y");
    y:Rank1Constraint constraint0(x +, 5 * x, 0, "(x + y) * (5*x) == 0");
    y:Rank1Constraint constraint1(x,, 3, "x * y == 3");
    ConstraintSystem system;
    system.addConstraint(constraint0);
    system.addConstraint(constraint1);
    let new_constraint_sys= adapter.convert(system);
    EXPECT_EQ(new_constraint_sys.len(), 2u);
    EXPECT_EQ(new_constraint_sys.at(0), adapter.convert(constraint0));
    EXPECT_EQ(new_constraint_sys.at(1), adapter.convert(constraint1));
}

TEST(GadgetLibAdapter, VariableAssignment) {
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    adapter.resetVariableIndex();
    const VariableArray varArray(10, "x");
    VariableAssignment assignment;
    for i in 0..varArray.len() {
        assignment[varArray[i]] = i;
    }
    let new_assignment= adapter.convert(assignment);
    ASSERT_EQ(assignment.len(), new_assignment.len());
    for i in 0..new_assignment.len() {
        i:GadgetLibAdapter::variable_index_t var =,
        EXPECT_EQ(new_assignment.at(var), Fp(i));
    }
}

TEST(GadgetLibAdapter, Protoboard) {
    initPublicParamsFromDefaultPp();
    adapter:GadgetLibAdapter,
    adapter.resetVariableIndex();
    const Variable x("x");
    const Variable y("y");
    ProtoboardPtr pb = Protoboard::create(R1P);
    pb->addRank1Constraint(x + y, 5 * x, 0, "(x + y) * (5*x) == 0");
    pb->addRank1Constraint(x, y, 3, "x * y == 3");
    pb->val(x) = 1;
    pb->val(y) = 2;
    let new_pb= adapter.convert(*pb);
    EXPECT_EQ(new_pb.first, adapter.convert(pb->constraintSystem()));
    EXPECT_EQ(new_pb.second, adapter.convert(pb->assignment()));
}


} // namespace
