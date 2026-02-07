//  Unit tests for gadgetlib2 - tests for specific gadgets

// use crate::gadgetlib2::gadget;
// use crate::gadgetlib2::pp;
// use crate::gadgetlib2::protoboard;

// using ::std::cerr;
// using ::std::cout;
// using ::std::endl;
// using ::std::stringstream;
// using namespace gadgetlib2;

// #define EXHAUSTIVE_N 4

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[test]
    fn R1P_AND_Gadget_SimpleTest() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);

        let x = VariableArray::new(3, "x");
        let y = Variable::from("y");
        let andGadget = AND_Gadget::create(&pb, x, y);
        andGadget.generateConstraints();

        pb.val(x[0]) = 0;
        pb.val(x[1]) = 1;
        pb.val(x[2]) = 1;
        andGadget.generateWitness();
        assert!(pb.val(y) == 0);
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        pb.val(y) = 1;
        assert!(!pb.isSatisfied());

        pb.val(x[0]) = 1;
        andGadget.generateWitness();
        assert!(pb.val(y) == 1);
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));

        pb.val(y) = 0;
        assert!(!pb.isSatisfied());
    }

    pub struct LogicGadgetExhaustiveTester<T> {
        pb: ProtoboardPtr,
        numInputs: usize,
        inputs: VariableArray,
        output: Variable,
        logicGadget: GadgetPtr,
        currentInputValues: usize,
        _t: PhantomData<T>,
        // LogicGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
        // pub fn  setInputValsTo(val:usize);
        // pub fn  runCompletenessCheck();
        // virtual pub fn  ruinOutputVal() = 0;
        // pub fn  runSoundnessCheck();

        // DISALLOW_COPY_AND_ASSIGN(LogicGadgetExhaustiveTester);

        // pub fn  runExhaustiveTest();
    }

    pub struct AndGadgetExhaustiveTester {
        //LogicGadgetExhaustiveTester
        //    virtual pub fn  ruinOutputVal();
        //      AndGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
    }

    pub struct OrGadgetExhaustiveTester {
        //LogicGadgetExhaustiveTester
        //    virtual pub fn  ruinOutputVal();
        //      OrGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
    }

    #[test]
    fn R1P_ANDGadget_ExhaustiveTest() {
        initPublicParamsFromDefaultPp();
        for inputSize in 1..=EXHAUSTIVE_N {
            SCOPED_TRACE(GADGETLIB2_FMT("n = %u \n", inputSize));
            let pb = Protoboard::create(R1P);
            let mut tester = AndGadgetExhaustiveTester::new(pb, inputSize);
            tester.runExhaustiveTest();
        }
    }

    #[test]
    fn BinaryAND_Gadget() {
        let pb = Protoboard::create(R1P);
        let input1 = Variable::from("input1");
        let input2 = Variable::from("input2");
        let result = Variable::from("result");
        let andGadget = AND_Gadget::create(&pb, input1, input2, result);
        andGadget.generateConstraints();
        pb.val(input1) = pb.val(input2) = 0;
        andGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(result), 0);
        pb.val(result) = 1;
        assert!(!pb.isSatisfied());
        pb.val(result) = 0;
        pb.val(input1) = 1;
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        pb.val(input2) = 1;
        assert!(!pb.isSatisfied());
        andGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(result), 1);
    }

    #[test]
    fn R1P_ORGadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        for n in 1..=EXHAUSTIVE_N {
            SCOPED_TRACE(GADGETLIB2_FMT("n = %u \n", n));
            let pb = Protoboard::create(R1P);
            let tester = OrGadgetExhaustiveTester::new(pb, n);
            tester.runExhaustiveTest();
        }
    }

    #[test]
    fn BinaryOR_Gadget() {
        let pb = Protoboard::create(R1P);
        let input1 = Variable::from("input1");
        let input2 = Variable::from("input2");
        let result = Variable::from("result");
        let orGadget = OR_Gadget::create(&pb, input1, input2, result);
        orGadget.generateConstraints();
        pb.val(input1) = pb.val(input2) = 0;
        orGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(result), 0);
        pb.val(result) = 1;
        assert!(!pb.isSatisfied());
        pb.val(result) = 0;
        pb.val(input1) = 1;
        assert!(!pb.isSatisfied());
        pb.val(result) = 1;
        ASSERT_CONSTRAINTS_SATISFIED(pb);
        pb.val(input2) = 1;
        orGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(result), 1);
    }

    // TODO refactor this test --Shaul
    #[test]
    fn R1P_InnerProductGadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        let n = EXHAUSTIVE_N;
        let pb = Protoboard::create(R1P);
        let A = VariableArray::new(n, "A");
        let B = VariableArray::new(n, "B");
        let result = Variable::from("result");
        let g = InnerProduct_Gadget::create(&pb, A, B, result);
        g.generateConstraints();
        for i in 0..1usize << n {
            for j in 0..1usize << n {
                let mut correct = 0;
                for k in 0..n {
                    pb.val(A[k]) = if i & (1usize << k) { 1 } else { 0 };
                    pb.val(B[k]) = if j & (1usize << k) { 1 } else { 0 };
                    correct += if (i & (1usize << k)) && (j & (1usize << k)) {
                        1
                    } else {
                        0
                    };
                }
                g.generateWitness();
                assert_eq!(pb.val(result), FElem(correct));
                assert!(pb.isSatisfied());
                // negative test
                pb.val(result) = 100 * n + 19;
                assert!(!pb.isSatisfied());
            }
        }
    }

    // TODO refactor this test --Shaul
    #[test]
    fn R1P_LooseMUX_Gadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        let n = EXHAUSTIVE_N;
        let pb = Protoboard::create(R1P);
        let arr = VariableArray::new(1 << n, "arr");
        let index = Variable::from("index");
        let result = Variable::from("result");
        let success_flag = Variable::from("success_flag");
        let g = LooseMUX_Gadget::create(&pb, arr, index, result, success_flag);
        g.generateConstraints();
        for i in 0..1usize << n {
            pb.val(arr[i]) = (19 * i) % (1usize << n);
        }
        for idx in -1..=(1 << n) {
            pb.val(index) = idx;
            g.generateWitness();
            if 0 <= idx && idx <= (1 << n) - 1 {
                assert_eq!(pb.val(result), (19 * idx) % (1usize << n));
                assert_eq!(pb.val(success_flag), 1);
                assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
                pb.val(result) -= 1;
                assert!(!pb.isSatisfied());
            } else {
                assert_eq!(pb.val(success_flag), 0);
                assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
                pb.val(success_flag) = 1;
                assert!(!pb.isSatisfied());
            }
        }
    }

    // Forward declaration
    // pub fn  packing_Gadget_R1P_ExhaustiveTest(ProtoboardPtr unpackingPB, ProtoboardPtr packingPB,
    //                                        n:int, VariableArray packed, VariableArray unpacked,
    //                                        GadgetPtr packingGadget, GadgetPtr unpackingGadget);

    // TODO refactor this test --Shaul
    #[test]
    fn R1P_Packing_Gadgets() {
        initPublicParamsFromDefaultPp();
        let unpackingPB = Protoboard::create(R1P);
        let packingPB = Protoboard::create(R1P);
        let n = EXHAUSTIVE_N;
        {
            // test CompressionPacking_Gadget
            SCOPED_TRACE("testing CompressionPacking_Gadget");
            let packed = VariableArray::new(1, "packed");
            let unpacked = VariableArray::new(n, "unpacked");
            let packingGadget =
                CompressionPacking_Gadget::create(packingPB, unpacked, packed, PackingMode::PACK);
            let unpackingGadget = CompressionPacking_Gadget::create(
                unpackingPB,
                unpacked,
                packed,
                PackingMode::UNPACK,
            );
            packing_Gadget_R1P_ExhaustiveTest(
                unpackingPB,
                packingPB,
                n,
                packed,
                unpacked,
                packingGadget,
                unpackingGadget,
            );
        }
        {
            // test IntegerPacking_Gadget
            SCOPED_TRACE("testing IntegerPacking_Gadget");
            let packed = VariableArray::new(1, "packed");
            let unpacked = VariableArray::new(n, "unpacked");
            let packingGadget =
                IntegerPacking_Gadget::create(packingPB, unpacked, packed, PackingMode::PACK);
            let unpackingGadget =
                IntegerPacking_Gadget::create(unpackingPB, unpacked, packed, PackingMode::UNPACK);
            packing_Gadget_R1P_ExhaustiveTest(
                unpackingPB,
                packingPB,
                n,
                packed,
                unpacked,
                packingGadget,
                unpackingGadget,
            );
        }
    }

    #[test]
    fn R1P_EqualsConst_Gadget() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let input = Variable::from("input");
        let result = Variable::from("result");
        let gadget = EqualsConst_Gadget::create(&pb, 0, input, result);
        gadget.generateConstraints();
        pb.val(input) = 0;
        gadget.generateWitness();
        // Positive test for input == n
        assert_eq!(pb.val(result), 1);
        assert!(pb.isSatisfied());
        // Negative test
        pb.val(result) = 0;
        assert!(!pb.isSatisfied());
        // Positive test for input != n
        pb.val(input) = 1;
        gadget.generateWitness();
        assert_eq!(pb.val(result), 0);
        assert!(pb.isSatisfied());
        // Negative test
        pb.val(input) = 0;
        assert!(!pb.isSatisfied());
    }

    #[test]
    fn ConditionalFlag_Gadget() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let flag = FlagVariable::default();
        let condition = Variable::from("condition");
        let cfGadget = ConditionalFlag_Gadget::create(&pb, condition, flag);
        cfGadget.generateConstraints();
        pb.val(condition) = 1;
        cfGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        pb.val(condition) = 42;
        cfGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(flag), 1);
        pb.val(condition) = 0;
        assert!(!pb.isSatisfied());
        cfGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(flag), 0);
        pb.val(flag) = 1;
        assert!(!pb.isSatisfied());
    }

    #[test]
    fn LogicImplication_Gadget() {
        let pb = Protoboard::create(R1P);
        let flag = FlagVariable::default();
        let condition = Variable::from("condition");
        let implyGadget = LogicImplication_Gadget::create(&pb, condition, flag);
        implyGadget.generateConstraints();
        pb.val(condition) = 1;
        pb.val(flag) = 0;
        assert!(!pb.isSatisfied());
        implyGadget.generateWitness();
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        assert_eq!(pb.val(flag), 1);
        pb.val(condition) = 0;
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        implyGadget.generateWitness();
        assert_eq!(pb.val(flag), 1);
        pb.val(flag) = 0;
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
    }

    // TODO refactor this test --Shaul
    pub fn packing_Gadget_R1P_ExhaustiveTest(
        unpackingPB: ProtoboardPtr,
        packingPB: ProtoboardPtr,
        n: i32,
        packed: VariableArray,
        unpacked: VariableArray,
        packingGadget: GadgetPtr,
        unpackingGadget: GadgetPtr,
    ) {
        packingGadget.generateConstraints();
        unpackingGadget.generateConstraints();
        for i in 0..1 << n {
            let mut bits = vec![0; n];
            for j in 0..n {
                bits[j] = if i & 1usize << j { 1 } else { 0 };
                packingPB.val(unpacked[j]) = bits[j]; // set unpacked bits in the packing protoboard
            }
            unpackingPB.val(packed[0]) = i; // set the packed value in the unpacking protoboard
            unpackingGadget.generateWitness();
            packingGadget.generateWitness();
            assert!(unpackingPB.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
            assert!(packingPB.isSatisfied());
            assert_eq!(packingPB.val(packed[0]), i); // check packed value is correct
            for j in 0..n {
                // Tests for unpacking gadget
                SCOPED_TRACE(GADGETLIB2_FMT(
                    "\nValue being packed/unpacked: %u, bits[%u] = %u",
                    i,
                    j,
                    bits[j],
                ));
                assert_eq!(unpackingPB.val(unpacked[j]), bits[j]); // check bit correctness
                packingPB.val(unpacked[j]) = unpackingPB.val(unpacked[j]) = 1 - bits[j]; // flip bit
                assert!(!unpackingPB.isSatisfied());
                assert!(!packingPB.isSatisfied());
                packingPB.val(unpacked[j]) = unpackingPB.val(unpacked[j]) = bits[j]; // restore bit
                // special case to test booleanity checks. Cause arithmetic constraints to stay
                // satisfied while ruining Booleanity
                if j > 0 && bits[j] == 1 && bits[j - 1] == 0 {
                    packingPB.val(unpacked[j - 1]) = unpackingPB.val(unpacked[j - 1]) = 2;
                    packingPB.val(unpacked[j]) = unpackingPB.val(unpacked[j]) = 0;
                    assert!(!unpackingPB.isSatisfied());
                    assert!(packingPB.isSatisfied()); // packing should not enforce Booleanity
                    // restore correct state
                    packingPB.val(unpacked[j - 1]) = unpackingPB.val(unpacked[j - 1]) = 0;
                    packingPB.val(unpacked[j]) = unpackingPB.val(unpacked[j]) = 1;
                }
            }
        }
    }

    impl<T> LogicGadgetExhaustiveTester<T> {
        pub fn setInputValsTo(val: usize) {
            for maskBit in 0..numInputs {
                pb.val(inputs[maskBit]) = if (val & (1usize << maskBit)) { 1 } else { 0 };
            }
        }

        pub fn runCompletenessCheck() {
            SCOPED_TRACE(GADGETLIB2_FMT(
                "Positive (completeness) test failed. curInput: %u",
                currentInputValues,
            ));
            assert!(pb.isSatisfied());
        }

        pub fn runSoundnessCheck() {
            SCOPED_TRACE(pb.annotation());
            SCOPED_TRACE(GADGETLIB2_FMT(
                "Negative (soundness) test failed. curInput: %u, Constraints are:",
                currentInputValues,
            ));
            assert!(!pb.isSatisfied());
        }

        pub fn new(pb: ProtoboardPtr, numInputs: usize) -> Self {
            Self {
                pb,
                numInputs,
                inputs: VariableArray::new(numInputs, "inputs"),
                output: Variable::from("output"),
                logicGadget: GadgetPtr::default(),
                currentInputValues: 0,
                _t: PhantomData,
            }
        }

        pub fn runExhaustiveTest() {
            logicGadget.generateConstraints();
            for currentInputValues in 0..(1usize << numInputs) {
                setInputValsTo(currentInputValues);
                logicGadget.generateWitness();
                runCompletenessCheck();
                ruinOutputVal();
                runSoundnessCheck();
            }
        }
    }
    impl LogicGadgetExhaustiveTester<AndGadgetExhaustiveTester> {
        pub fn ruinOutputVal() {
            pb.val(output) = if (currentInputValues == ((1usize << numInputs) - 1)) {
                0
            } else {
                1
            };
        }

        pub fn new(pb: ProtoboardPtr, numInputs: usize) -> Self {
            let mut _self = LogicGadgetExhaustiveTester::new(pb.clone(), numInputs);
            _self = logicGadget = AND_Gadget::create(pb.clone(), inputs, output);
            _self
        }
    }

    impl LogicGadgetExhaustiveTester<OrGadgetExhaustiveTester> {
        pub fn ruinOutputVal() {
            pb.val(output) = if (currentInputValues == 0) { 1 } else { 0 };
        }

        pub fn new(pb: ProtoboardPtr, numInputs: usize) -> Self {
            let mut _self = LogicGadgetExhaustiveTester::new(pb.clone(), numInputs);
            _self.logicGadget = OR_Gadget::create(&pb, inputs, output);
            _self
        }
    }
}
