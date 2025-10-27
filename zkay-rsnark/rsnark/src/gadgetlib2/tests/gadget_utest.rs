/** @file
 *****************************************************************************
 Unit tests for gadgetlib2 - tests for specific gadgets
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <iostream>
use  <sstream>

use  <gtest/gtest.h>

use crate::gadgetlib2::gadget;
use crate::gadgetlib2::pp;
use crate::gadgetlib2::protoboard;

using ::std::cerr;
using ::std::cout;
using ::std::endl;
using ::std::stringstream;
using namespace gadgetlib2;

// #define EXHAUSTIVE_N 4

namespace {

TEST(gadgetLib2,R1P_AND_Gadget_SimpleTest) {
    initPublicParamsFromDefaultPp();
    auto pb = Protoboard::create(R1P);

    VariableArray x(3, "x");
    Variable y("y");
    auto andGadget = AND_Gadget::create(pb, x, y);
    andGadget.generateConstraints();

    pb->val(x[0]) = 0;
    pb->val(x[1]) = 1;
    pb->val(x[2]) = 1;
    andGadget.generateWitness();
    EXPECT_TRUE(pb->val(y) == 0);
    EXPECT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    pb->val(y) = 1;
    EXPECT_FALSE(pb->isSatisfied());

    pb->val(x[0]) = 1;
    andGadget.generateWitness();
    EXPECT_TRUE(pb->val(y) == 1);
    EXPECT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));

    pb->val(y) = 0;
    EXPECT_FALSE(pb->isSatisfied());
}

pub struct LogicGadgetExhaustiveTester {

    ProtoboardPtr pb;
    numInputs:usize;
    inputs:VariableArray,
    output:Variable,
    GadgetPtr logicGadget;
    usize currentInputValues;

    LogicGadgetExhaustiveTester(ProtoboardPtr pb, usize numInputs);
    pub fn  setInputValsTo(val:usize);
    pub fn  runCompletenessCheck();
    virtual pub fn  ruinOutputVal() = 0;
    pub fn  runSoundnessCheck();

    DISALLOW_COPY_AND_ASSIGN(LogicGadgetExhaustiveTester);

    pub fn  runExhaustiveTest();
};

pub struct AndGadgetExhaustiveTester {//LogicGadgetExhaustiveTester
   virtual pub fn  ruinOutputVal();
     AndGadgetExhaustiveTester(ProtoboardPtr pb, usize numInputs);
};

pub struct OrGadgetExhaustiveTester {//LogicGadgetExhaustiveTester
   virtual pub fn  ruinOutputVal();
     OrGadgetExhaustiveTester(ProtoboardPtr pb, usize numInputs);
};


TEST(gadgetLib2,R1P_ANDGadget_ExhaustiveTest) {
    initPublicParamsFromDefaultPp();
    for inputSize in 1..=EXHAUSTIVE_N {
        SCOPED_TRACE(GADGETLIB2_FMT("n = %u \n", inputSize));
        auto pb = Protoboard::create(R1P);
        AndGadgetExhaustiveTester tester(pb, inputSize);
        tester.runExhaustiveTest();
    }
}

TEST(gadgetLib2,BinaryAND_Gadget) {
    auto pb = Protoboard::create(R1P);
    Variable input1("input1");
    Variable input2("input2");
    Variable result("result");
    auto andGadget = AND_Gadget::create(pb, input1, input2, result);
    andGadget.generateConstraints();
    pb->val(input1) = pb->val(input2) = 0;
    andGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(result), 0);
    pb->val(result) = 1;
    ASSERT_FALSE(pb->isSatisfied());
    pb->val(result) = 0;
    pb->val(input1) = 1;
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    pb->val(input2) = 1;
    ASSERT_FALSE(pb->isSatisfied());
    andGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(result), 1);
}

TEST(gadgetLib2,R1P_ORGadget_Exhaustive) {
    initPublicParamsFromDefaultPp();
    for n in 1..=EXHAUSTIVE_N {
        SCOPED_TRACE(GADGETLIB2_FMT("n = %u \n", n));
        auto pb = Protoboard::create(R1P);
        OrGadgetExhaustiveTester tester(pb, n);
        tester.runExhaustiveTest();
    }
}

TEST(gadgetLib2,BinaryOR_Gadget) {
    auto pb = Protoboard::create(R1P);
    Variable input1("input1");
    Variable input2("input2");
    Variable result("result");
    auto orGadget = OR_Gadget::create(pb, input1, input2, result);
    orGadget.generateConstraints();
    pb->val(input1) = pb->val(input2) = 0;
    orGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(result), 0);
    pb->val(result) = 1;
    ASSERT_FALSE(pb->isSatisfied());
    pb->val(result) = 0;
    pb->val(input1) = 1;
    ASSERT_FALSE(pb->isSatisfied());
    pb->val(result) = 1;
    ASSERT_CONSTRAINTS_SATISFIED(pb);
    pb->val(input2) = 1;
    orGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(result), 1);
}

// TODO refactor this test --Shaul
TEST(gadgetLib2,R1P_InnerProductGadget_Exhaustive) {
    initPublicParamsFromDefaultPp();
    let n = EXHAUSTIVE_N;
    auto pb = Protoboard::create(R1P);
    VariableArray A(n, "A");
    VariableArray B(n, "B");
    Variable result("result");
    auto g = InnerProduct_Gadget::create(pb, A, B, result);
    g.generateConstraints();
    for i in 0..1u<<n {
        for j in 0..1u<<n {
            usize correct = 0;
            for k in 0..n {
                pb->val(A[k])=  if i & (1u<<k) {1} else{0};
                pb->val(B[k])=  if j & (1u<<k) {1} else{0};
                correct += if (i & (1u<<k)) && (j & (1u<<k)) {1} else {0};
            }
            g.generateWitness();
            EXPECT_EQ(pb->val(result) , FElem(correct));
            EXPECT_TRUE(pb->isSatisfied());
            // negative test
            pb->val(result) = 100*n+19;
            EXPECT_FALSE(pb->isSatisfied());
        }
    }
}

// TODO refactor this test --Shaul
TEST(gadgetLib2,R1P_LooseMUX_Gadget_Exhaustive) {
initPublicParamsFromDefaultPp();
let n = EXHAUSTIVE_N;
    auto pb = Protoboard::create(R1P);
    VariableArray arr(1<<n, "arr");
    Variable index("index");
    Variable result("result");
    Variable success_flag("success_flag");
    auto g = LooseMUX_Gadget::create(pb, arr, index, result, success_flag);
    g.generateConstraints();
    for i in 0..1u<<n {
        pb->val(arr[i]) = (19*i) % (1u<<n);
    }
    for idx in -1..=(1<<n) {
        pb->val(index) = idx;
        g.generateWitness();
        if 0 <= idx && idx <= (1<<n) - 1 {
            EXPECT_EQ(pb->val(result) , (19*idx) % (1u<<n));
            EXPECT_EQ(pb->val(success_flag) , 1);
            EXPECT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
            pb->val(result) -= 1;
            EXPECT_FALSE(pb->isSatisfied());
        }
        else {
            EXPECT_EQ(pb->val(success_flag) , 0);
            EXPECT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
            pb->val(success_flag) = 1;
            EXPECT_FALSE(pb->isSatisfied());
        }
    }
}

// Forward declaration
pub fn  packing_Gadget_R1P_ExhaustiveTest(ProtoboardPtr unpackingPB, ProtoboardPtr packingPB,
                                       n:int, VariableArray packed, VariableArray unpacked,
                                       GadgetPtr packingGadget, GadgetPtr unpackingGadget);

// TODO refactor this test --Shaul
TEST(gadgetLib2,R1P_Packing_Gadgets) {
    initPublicParamsFromDefaultPp();
    auto unpackingPB = Protoboard::create(R1P);
    auto packingPB = Protoboard::create(R1P);
    let n= EXHAUSTIVE_N;
    { // test CompressionPacking_Gadget
        SCOPED_TRACE("testing CompressionPacking_Gadget");
        VariableArray packed(1, "packed");
        VariableArray unpacked(n, "unpacked");
        auto packingGadget = CompressionPacking_Gadget::create(packingPB, unpacked, packed,
                                                               PackingMode::PACK);
        auto unpackingGadget = CompressionPacking_Gadget::create(unpackingPB, unpacked, packed,
                                                                 PackingMode::UNPACK);
        packing_Gadget_R1P_ExhaustiveTest(unpackingPB, packingPB, n, packed, unpacked, packingGadget,
                                          unpackingGadget);
    }
    { // test IntegerPacking_Gadget
        SCOPED_TRACE("testing IntegerPacking_Gadget");
        VariableArray packed(1, "packed");
        VariableArray unpacked(n, "unpacked");
        auto packingGadget = IntegerPacking_Gadget::create(packingPB, unpacked, packed,
                                                           PackingMode::PACK);
        auto unpackingGadget = IntegerPacking_Gadget::create(unpackingPB, unpacked, packed,
                                                             PackingMode::UNPACK);
        packing_Gadget_R1P_ExhaustiveTest(unpackingPB, packingPB, n, packed, unpacked, packingGadget,
                                          unpackingGadget);
    }
}

TEST(gadgetLib2,R1P_EqualsConst_Gadget) {
    initPublicParamsFromDefaultPp();
    auto pb = Protoboard::create(R1P);
    Variable input("input");
    Variable result("result");
    auto gadget = EqualsConst_Gadget::create(pb, 0, input, result);
    gadget.generateConstraints();
    pb->val(input) = 0;
    gadget.generateWitness();
    // Positive test for input == n
    EXPECT_EQ(pb->val(result), 1);
    EXPECT_TRUE(pb->isSatisfied());
    // Negative test
    pb->val(result) = 0;
    EXPECT_FALSE(pb->isSatisfied());
    // Positive test for input != n
    pb->val(input) = 1;
    gadget.generateWitness();
    EXPECT_EQ(pb->val(result), 0);
    EXPECT_TRUE(pb->isSatisfied());
    // Negative test
    pb->val(input) = 0;
    EXPECT_FALSE(pb->isSatisfied());
}

TEST(gadgetLib2,ConditionalFlag_Gadget) {
    initPublicParamsFromDefaultPp();
    auto pb = Protoboard::create(R1P);
    FlagVariable flag;
    Variable condition("condition");
    auto cfGadget = ConditionalFlag_Gadget::create(pb, condition, flag);
    cfGadget.generateConstraints();
    pb->val(condition) = 1;
    cfGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    pb->val(condition) = 42;
    cfGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(flag),1);
    pb->val(condition) = 0;
    ASSERT_FALSE(pb->isSatisfied());
    cfGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(flag),0);
    pb->val(flag) = 1;
    ASSERT_FALSE(pb->isSatisfied());
}

TEST(gadgetLib2,LogicImplication_Gadget) {
    auto pb = Protoboard::create(R1P);
    FlagVariable flag;
    Variable condition("condition");
    auto implyGadget = LogicImplication_Gadget::create(pb, condition, flag);
    implyGadget.generateConstraints();
    pb->val(condition) = 1;
    pb->val(flag) = 0;
    ASSERT_FALSE(pb->isSatisfied());
    implyGadget.generateWitness();
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    ASSERT_EQ(pb->val(flag), 1);
    pb->val(condition) = 0;
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    implyGadget.generateWitness();
    ASSERT_EQ(pb->val(flag), 1);
    pb->val(flag) = 0;
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
}

// TODO refactor this test --Shaul
pub fn  packing_Gadget_R1P_ExhaustiveTest(ProtoboardPtr unpackingPB, ProtoboardPtr packingPB,
                                       n:int, VariableArray packed, VariableArray unpacked,
                                       GadgetPtr packingGadget, GadgetPtr unpackingGadget) {
    packingGadget.generateConstraints();
    unpackingGadget.generateConstraints();
    for i in 0..1l<<n {
        ::Vec<int> bits(n);
        for j in 0..n {
            bits[j]=  if i & 1u<<j {1} else{0 };
            packingPB->val(unpacked[j]) = bits[j]; // set unpacked bits in the packing protoboard
        }
        unpackingPB->val(packed[0]) = i; // set the packed value in the unpacking protoboard
        unpackingGadget.generateWitness();
        packingGadget.generateWitness();
        ASSERT_TRUE(unpackingPB->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        ASSERT_TRUE(packingPB->isSatisfied());
        ASSERT_EQ(packingPB->val(packed[0]), i); // check packed value is correct
        for j in 0..n {
            // Tests for unpacking gadget
            SCOPED_TRACE(GADGETLIB2_FMT("\nValue being packed/unpacked: %u, bits[%u] = %u" , i, j, bits[j]));
            ASSERT_EQ(unpackingPB->val(unpacked[j]), bits[j]); // check bit correctness
            packingPB->val(unpacked[j]) = unpackingPB->val(unpacked[j]) = 1-bits[j]; // flip bit
            ASSERT_FALSE(unpackingPB->isSatisfied());
            ASSERT_FALSE(packingPB->isSatisfied());
            packingPB->val(unpacked[j]) = unpackingPB->val(unpacked[j]) = bits[j]; // restore bit
            // special case to test booleanity checks. Cause arithmetic constraints to stay
            // satisfied while ruining Booleanity
            if j > 0 && bits[j]==1 && bits[j-1]==0  {
                packingPB->val(unpacked[j-1]) = unpackingPB->val(unpacked[j-1]) = 2;
                packingPB->val(unpacked[j]) = unpackingPB->val(unpacked[j]) = 0;
                ASSERT_FALSE(unpackingPB->isSatisfied());
                ASSERT_TRUE(packingPB->isSatisfied()); // packing should not enforce Booleanity
                // restore correct state
                packingPB->val(unpacked[j-1]) = unpackingPB->val(unpacked[j-1]) = 0;
                packingPB->val(unpacked[j]) = unpackingPB->val(unpacked[j]) = 1;
            }
        }
    }
}


pub fn setInputValsTo(val:usize) {
    for maskBit in 0..numInputs {
        pb->val(inputs[maskBit])=  if (val & (1u << maskBit)) {1} else{0};
    }
}

pub fn runCompletenessCheck() {
    SCOPED_TRACE(GADGETLIB2_FMT("Positive (completeness) test failed. curInput: %u", currentInputValues));
    EXPECT_TRUE(pb->isSatisfied());
}

pub fn runSoundnessCheck() {
    SCOPED_TRACE(pb->annotation());
    SCOPED_TRACE(GADGETLIB2_FMT("Negative (soundness) test failed. curInput: %u, Constraints "
        "are:", currentInputValues));
    EXPECT_FALSE(pb->isSatisfied());
}
pub fn new(ProtoboardPtr pb, usize numInputs)
    :pb,numInputs, inputs(numInputs, "inputs"), output("output"),
    currentInputValues(0) {}

pub fn runExhaustiveTest() {
    logicGadget.generateConstraints();
    for (currentInputValues = 0; currentInputValues < (1u << numInputs); ++currentInputValues) {
        setInputValsTo(currentInputValues);
        logicGadget.generateWitness();
        runCompletenessCheck();
        ruinOutputVal();
        runSoundnessCheck();
    }
}

pub fn ruinOutputVal() {
    pb->val(output)=  if (currentInputValues == ((1u << numInputs) - 1)) {0} else{1};
}

pub fn new(ProtoboardPtr pb, usize numInputs)
    : LogicGadgetExhaustiveTester(pb, numInputs) {
    logicGadget = AND_Gadget::create(pb, inputs, output);
}

pub fn ruinOutputVal() {
    pb->val(output)=  if (currentInputValues == 0) {1} else{0};
}

pub fn new(ProtoboardPtr pb, usize numInputs)
    : LogicGadgetExhaustiveTester(pb, numInputs) {
    logicGadget = OR_Gadget::create(pb, inputs, output);
}


} // namespace
