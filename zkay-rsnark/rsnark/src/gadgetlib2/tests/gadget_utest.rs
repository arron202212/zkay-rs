//  Unit tests for gadgetlib2 - tests for specific gadgets

// use crate::gadgetlib2::gadget;
// use crate::gadgetlib2::pp;

use crate::gadgetlib2::constraint::PrintOptions;
use crate::gadgetlib2::gadget::{
    AND_Gadget, CompressionPacking_Gadget, ConditionalFlag_Gadget, EqualsConst_Gadget,
    GadgetConfig, GadgetPtr, InnerProduct_Gadget, IntegerPacking_Gadget, LogicImplication_Gadget,
    LooseMUX_Gadget, OR_Gadget, PackingMode,
};
use crate::gadgetlib2::pp::initPublicParamsFromDefaultPp;
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{
    FElem, FieldType, FlagVariable, ProtoboardPtr, Variable, VariableArray, VariableArrayBase,
    VariableArrayType,
};
use std::marker::PhantomData;

const EXHAUSTIVE_N: usize = 4;

fn SCOPED_TRACE(s: String) {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_R1P_AND_Gadget_SimpleTest() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();

        let x = VariableArray::new(3, "x".to_owned(), VariableArrayBase);
        let y = Variable::from("y");
        let andGadget =
            AND_Gadget::create_r1p(Some(pb.clone()), x.clone().into(), y.clone().into());
        andGadget.borrow().generateConstraints();

        *pb.borrow_mut().val(&x[0]) = 0.into();
        *pb.borrow_mut().val(&x[1]) = 1.into();
        *pb.borrow_mut().val(&x[2]) = 1.into();
        andGadget.borrow().generateWitness();
        assert!(*pb.borrow_mut().val(&y) == 0);
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&y) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));

        *pb.borrow_mut().val(&x[0]) = 1.into();
        andGadget.borrow().generateWitness();
        assert!(*pb.borrow_mut().val(&y) == 1);
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );

        *pb.borrow_mut().val(&y) = 0.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
    }

    pub struct LogicGadgetExhaustiveTester<T> {
        pb: ProtoboardPtr,
        numInputs: usize,
        inputs: VariableArrayType,
        output: Variable,
        logicGadget: GadgetPtr,
        currentInputValues: usize,
        _t: PhantomData<T>,
        // LogicGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
        // pub fn test_ setInputValsTo(val:usize);
        // pub fn test_ runCompletenessCheck();
        // virtual pub fn test_ ruinOutputVal() = 0;
        // pub fn test_ runSoundnessCheck();

        // DISALLOW_COPY_AND_ASSIGN(LogicGadgetExhaustiveTester);

        // pub fn test_ runExhaustiveTest();
    }

    pub struct AndGadgetExhaustiveTester;
    //  {
    //     //LogicGadgetExhaustiveTester
    //     //    virtual pub fn test_ ruinOutputVal();
    //     //      AndGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
    // }

    pub struct OrGadgetExhaustiveTester;
    //  {
    //     //LogicGadgetExhaustiveTester
    //     //    virtual pub fn test_ ruinOutputVal();
    //     //      OrGadgetExhaustiveTester(pb:ProtoboardPtr, numInputs:usize);
    // }

    #[test]
    fn test_R1P_ANDGadget_ExhaustiveTest() {
        initPublicParamsFromDefaultPp();
        for inputSize in 1..=EXHAUSTIVE_N {
            SCOPED_TRACE(format!("n = {} \n", inputSize));
            let pb = Protoboard::create(FieldType::R1P, None);
            let mut tester = AndGadgetExhaustiveTester::new(pb, inputSize);
            tester.runExhaustiveTest();
        }
    }

    #[test]
    fn test_BinaryAND_Gadget() {
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let input1 = Variable::from("input1");
        let input2 = Variable::from("input2");
        let result = Variable::from("result");
        let andGadget = AND_Gadget::create(
            Some(pb.clone()),
            input1.clone().into(),
            input2.clone().into(),
            result.clone(),
        );
        andGadget.borrow().generateConstraints();
        *pb.borrow_mut().val(&input1) = 0.into();
        *pb.borrow_mut().val(&input2) = 0.into();
        andGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&result), 0);
        *pb.borrow_mut().val(&result) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        *pb.borrow_mut().val(&result) = 0.into();
        *pb.borrow_mut().val(&input1) = 1.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&input2) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        andGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&result), 1);
    }

    #[test]
    fn test_R1P_ORGadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        for n in 1..=EXHAUSTIVE_N {
            SCOPED_TRACE(format!("n = {} \n", n));
            let pb = Protoboard::create(FieldType::R1P, None);
            let mut tester = OrGadgetExhaustiveTester::new(pb.clone(), n);
            tester.runExhaustiveTest();
        }
    }

    #[test]
    fn test_BinaryOR_Gadget() {
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let input1 = Variable::from("input1");
        let input2 = Variable::from("input2");
        let result = Variable::from("result");
        let orGadget = OR_Gadget::create(
            Some(pb.clone()),
            input1.clone().into(),
            input2.clone().into(),
            result.clone(),
        );
        orGadget.borrow().generateConstraints();
        *pb.borrow_mut().val(&input1) = 0.into();
        *pb.borrow_mut().val(&input2) = 0.into();
        orGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&result), 0);
        *pb.borrow_mut().val(&result) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        *pb.borrow_mut().val(&result) = 0.into();
        *pb.borrow_mut().val(&input1) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        *pb.borrow_mut().val(&result) = 1.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&input2) = 1.into();
        orGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&result), 1);
    }

    // TODO refactor this test --Shaul
    #[test]
    fn test_R1P_InnerProductGadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        let n = EXHAUSTIVE_N;
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let A = VariableArray::new(n, "A".to_owned(), VariableArrayBase);
        let B = VariableArray::new(n, "B".to_owned(), VariableArrayBase);
        let result = Variable::from("result");
        let g = InnerProduct_Gadget::create(
            Some(pb.clone()),
            A.clone().into(),
            B.clone().into(),
            result.clone(),
        );
        g.borrow().generateConstraints();
        for i in 0..1usize << n {
            for j in 0..1usize << n {
                let mut correct = 0;
                for k in 0..n {
                    *pb.borrow_mut().val(&A[k]) = ((i >> k) & 1).into();
                    *pb.borrow_mut().val(&B[k]) = ((j >> k) & 1).into();
                    correct += (((i >> k) & 1) & ((j >> k) & 1));
                }
                g.borrow().generateWitness();
                assert_eq!(*pb.borrow_mut().val(&result), FElem::from(correct));
                assert!(pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
                // negative test
                *pb.borrow_mut().val(&result) = (100 * n + 19).into();
                assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
            }
        }
    }

    // TODO refactor this test --Shaul
    #[test]
    fn test_R1P_LooseMUX_Gadget_Exhaustive() {
        initPublicParamsFromDefaultPp();
        let n = EXHAUSTIVE_N;
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let arr = VariableArray::new(1 << n, "arr".to_owned(), VariableArrayBase);
        let index = Variable::from("index");
        let result = Variable::from("result");
        let success_flag = Variable::from("success_flag");
        let g = LooseMUX_Gadget::create(
            Some(pb.clone()),
            arr.clone().into(),
            index.clone(),
            result.clone(),
            success_flag.clone(),
        );
        g.borrow().generateConstraints();
        for i in 0..1usize << n {
            *pb.borrow_mut().val(&arr[i]) = ((19 * i) % (1usize << n)).into();
        }
        for idx in -1i64..=(1 << n) {
            *pb.borrow_mut().val(&index) = idx.into();
            g.borrow().generateWitness();
            if 0 <= idx && idx <= (1 << n) - 1 {
                assert_eq!(
                    *pb.borrow_mut().val(&result),
                    FElem::from((19 * idx) % (1i64 << n))
                );
                assert_eq!(*pb.borrow_mut().val(&success_flag), 1);
                assert!(
                    pb.borrow()
                        .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
                );
                *pb.borrow_mut().val(&result) -= &(1.into());
                assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
            } else {
                assert_eq!(*pb.borrow_mut().val(&success_flag), 0);
                assert!(
                    pb.borrow()
                        .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
                );
                *pb.borrow_mut().val(&success_flag) = 1.into();
                assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
            }
        }
    }

    // Forward declaration
    // pub fn test_ packing_Gadget_R1P_ExhaustiveTest(ProtoboardPtr unpackingPB, ProtoboardPtr packingPB,
    //                                        n:int, VariableArray packed, VariableArray unpacked,
    //                                        GadgetPtr packingGadget, GadgetPtr unpackingGadget);

    // TODO refactor this test --Shaul
    #[test]
    fn test_R1P_Packing_Gadgets() {
        initPublicParamsFromDefaultPp();
        let unpackingPB = Protoboard::create(FieldType::R1P, None);
        let packingPB = Protoboard::create(FieldType::R1P, None);
        let n = EXHAUSTIVE_N;
        {
            // test CompressionPacking_Gadget
            SCOPED_TRACE("testing CompressionPacking_Gadget".to_owned());
            let packed = VariableArray::new(1, "packed".to_owned(), VariableArrayBase);
            let unpacked = VariableArray::new(n, "unpacked".to_owned(), VariableArrayBase);
            let packingGadget = CompressionPacking_Gadget::create(
                packingPB.clone(),
                unpacked.clone().into(),
                packed.clone().into(),
                PackingMode::PACK,
            );
            let unpackingGadget = CompressionPacking_Gadget::create(
                unpackingPB.clone(),
                unpacked.clone().into(),
                packed.clone().into(),
                PackingMode::UNPACK,
            );
            packing_Gadget_R1P_ExhaustiveTest(
                unpackingPB.clone(),
                packingPB.clone(),
                n,
                packed.clone().into(),
                unpacked.clone().into(),
                packingGadget,
                unpackingGadget,
            );
        }
        {
            // test IntegerPacking_Gadget
            SCOPED_TRACE("testing IntegerPacking_Gadget".to_owned());
            let packed = VariableArray::new(1, "packed".to_owned(), VariableArrayBase);
            let unpacked = VariableArray::new(n, "unpacked".to_owned(), VariableArrayBase);
            let packingGadget = IntegerPacking_Gadget::create(
                packingPB.clone(),
                unpacked.clone().into(),
                packed.clone().into(),
                PackingMode::PACK,
            );
            let unpackingGadget = IntegerPacking_Gadget::create(
                unpackingPB.clone(),
                unpacked.clone().into(),
                packed.clone().into(),
                PackingMode::UNPACK,
            );
            packing_Gadget_R1P_ExhaustiveTest(
                unpackingPB,
                packingPB,
                n,
                packed.into(),
                unpacked.into(),
                packingGadget,
                unpackingGadget,
            );
        }
    }

    #[test]
    fn test_R1P_EqualsConst_Gadget() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let input = Variable::from("input");
        let result = Variable::from("result");
        let gadget = EqualsConst_Gadget::create(
            Some(pb.clone()),
            0.into(),
            input.clone().into(),
            result.clone(),
        );
        gadget.borrow().generateConstraints();
        *pb.borrow_mut().val(&input) = 0.into();
        gadget.borrow().generateWitness();
        // Positive test for input == n
        assert_eq!(*pb.borrow_mut().val(&result), 1);
        assert!(pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        // Negative test
        *pb.borrow_mut().val(&result) = 0.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        // Positive test for input != n
        *pb.borrow_mut().val(&input) = 1.into();
        gadget.borrow().generateWitness();
        assert_eq!(*pb.borrow_mut().val(&result), 0);
        assert!(pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        // Negative test
        *pb.borrow_mut().val(&input) = 0.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
    }

    #[test]
    fn test_ConditionalFlag_Gadget() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let flag = FlagVariable::default();
        let condition = Variable::from("condition");
        let cfGadget = ConditionalFlag_Gadget::create(
            Some(pb.clone()),
            condition.clone().into(),
            flag.clone(),
        );
        cfGadget.borrow().generateConstraints();
        *pb.borrow_mut().val(&condition) = 1.into();
        cfGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&condition) = 42.into();
        cfGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&flag), 1);
        *pb.borrow_mut().val(&condition) = 0.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        cfGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&flag), 0);
        *pb.borrow_mut().val(&flag) = 1.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
    }

    #[test]
    fn test_LogicImplication_Gadget() {
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let flag = FlagVariable::default();
        let condition = Variable::from("condition");
        let implyGadget = LogicImplication_Gadget::create(
            Some(pb.clone()),
            condition.clone().into(),
            flag.clone(),
        );
        implyGadget.borrow().generateConstraints();
        *pb.borrow_mut().val(&condition) = 1.into();
        *pb.borrow_mut().val(&flag) = 0.into();
        assert!(!pb.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
        implyGadget.borrow().generateWitness();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        assert_eq!(*pb.borrow_mut().val(&flag), 1);
        *pb.borrow_mut().val(&condition) = 0.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        implyGadget.borrow().generateWitness();
        assert_eq!(*pb.borrow_mut().val(&flag), 1);
        *pb.borrow_mut().val(&flag) = 0.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
    }

    // TODO refactor this test --Shaul
    pub fn packing_Gadget_R1P_ExhaustiveTest(
        unpackingPB: ProtoboardPtr,
        packingPB: ProtoboardPtr,
        n: usize,
        packed: VariableArrayType,
        unpacked: VariableArrayType,
        packingGadget: GadgetPtr,
        unpackingGadget: GadgetPtr,
    ) {
        let (unpackingPB, packingPB) = (unpackingPB.unwrap(), packingPB.unwrap());
        packingGadget.borrow().generateConstraints();
        unpackingGadget.borrow().generateConstraints();
        for i in 0..1 << n {
            let mut bits = vec![0; n];
            for j in 0..n {
                bits[j] = (i >> j & 1);
                *packingPB.borrow_mut().val(&unpacked[j]) = bits[j].clone().into(); // set unpacked bits in the packing protoboard
            }
            *unpackingPB.borrow_mut().val(&packed[0]) = i.into(); // set the packed value in the unpacking protoboard
            unpackingGadget.borrow().generateWitness();
            packingGadget.borrow().generateWitness();
            assert!(
                unpackingPB
                    .borrow()
                    .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
            );
            assert!(packingPB.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
            assert_eq!(*packingPB.borrow_mut().val(&packed[0]), i); // check packed value is correct
            for j in 0..n {
                // Tests for unpacking gadget
                SCOPED_TRACE(format!(
                    "\nValue being packed/unpacked: {}, bits[{}] = {}",
                    i, j, bits[j],
                ));
                assert_eq!(*unpackingPB.borrow_mut().val(&unpacked[j]), bits[j]); // check bit correctness
                *packingPB.borrow_mut().val(&unpacked[j]) = (1 - bits[j]).into(); // flip bi
                *unpackingPB.borrow_mut().val(&unpacked[j]) = (1 - bits[j]).into(); // flip bit
                assert!(
                    !unpackingPB
                        .borrow()
                        .isSatisfied(&PrintOptions::NO_DBG_PRINT)
                );
                assert!(!packingPB.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT));
                *packingPB.borrow_mut().val(&unpacked[j]) = bits[j].into(); // restore bit
                *unpackingPB.borrow_mut().val(&unpacked[j]) = bits[j].into(); // restore bit
                // special case to test booleanity checks. Cause arithmetic constraints to stay
                // satisfied while ruining Booleanity
                if j > 0 && bits[j] == 1 && bits[j - 1] == 0 {
                    *packingPB.borrow_mut().val(&unpacked[j - 1]) = 2.into();
                    *unpackingPB.borrow_mut().val(&unpacked[j - 1]) = 2.into();
                    *packingPB.borrow_mut().val(&unpacked[j]) = 0.into();
                    *unpackingPB.borrow_mut().val(&unpacked[j]) = 0.into();
                    assert!(
                        !unpackingPB
                            .borrow()
                            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
                    );
                    assert!(packingPB.borrow().isSatisfied(&PrintOptions::NO_DBG_PRINT)); // packing should not enforce Booleanity
                    // restore correct state
                    *packingPB.borrow_mut().val(&unpacked[j - 1]) = 0.into();
                    *unpackingPB.borrow_mut().val(&unpacked[j - 1]) = 0.into();
                    *packingPB.borrow_mut().val(&unpacked[j]) = 1.into();
                    *unpackingPB.borrow_mut().val(&unpacked[j]) = 1.into();
                }
            }
        }
    }

    impl<T> LogicGadgetExhaustiveTester<T> {
        pub fn setInputValsTo(&mut self, val: usize) {
            for maskBit in 0..self.numInputs {
                *self
                    .pb
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .val(&self.inputs[maskBit]) = ((val >> maskBit) & 1).into();
            }
        }

        pub fn runCompletenessCheck(&self) {
            SCOPED_TRACE(format!(
                "Positive (completeness) test failed. curInput: {}",
                self.currentInputValues,
            ));
            assert!(
                self.pb
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .isSatisfied(&PrintOptions::NO_DBG_PRINT)
            );
        }

        pub fn runSoundnessCheck(&self) {
            SCOPED_TRACE(self.pb.as_ref().unwrap().borrow().annotation());
            SCOPED_TRACE(format!(
                "Negative (soundness) test failed. curInput: {}, Constraints are:",
                self.currentInputValues,
            ));
            assert!(
                !self
                    .pb
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .isSatisfied(&PrintOptions::NO_DBG_PRINT)
            );
        }

        pub fn new(pb: ProtoboardPtr, numInputs: usize) -> Self {
            Self {
                pb,
                numInputs,
                inputs: VariableArray::new(numInputs, "inputs".to_owned(), VariableArrayBase)
                    .into(),
                output: Variable::from("output"),
                logicGadget: GadgetPtr::default(),
                currentInputValues: 0,
                _t: PhantomData,
            }
        }
    }

    impl LogicGadgetExhaustiveTester<AndGadgetExhaustiveTester> {
        pub fn runExhaustiveTest(&mut self) {
            self.logicGadget.borrow().generateConstraints();
            for currentInputValues in 0..(1usize << self.numInputs) {
                self.setInputValsTo(currentInputValues);
                self.logicGadget.borrow().generateWitness();
                self.runCompletenessCheck();
                self.ruinOutputVal();
                self.runSoundnessCheck();
            }
        }
        pub fn ruinOutputVal(&self) {
            *self.pb.as_ref().unwrap().borrow_mut().val(&self.output) =
                ((self.currentInputValues != ((1usize << self.numInputs) - 1)) as i32).into();
        }
    }
    impl AndGadgetExhaustiveTester {
        pub fn new(
            pb: ProtoboardPtr,
            numInputs: usize,
        ) -> LogicGadgetExhaustiveTester<AndGadgetExhaustiveTester> {
            let mut _self = LogicGadgetExhaustiveTester::<AndGadgetExhaustiveTester>::new(
                pb.clone(),
                numInputs,
            );
            _self.logicGadget =
                AND_Gadget::create_r1p(pb.clone(), _self.inputs.clone(), _self.output.clone());
            _self
        }
    }

    impl LogicGadgetExhaustiveTester<OrGadgetExhaustiveTester> {
        pub fn runExhaustiveTest(&mut self) {
            self.logicGadget.borrow().generateConstraints();
            for currentInputValues in 0..(1usize << self.numInputs) {
                self.setInputValsTo(currentInputValues);
                self.logicGadget.borrow().generateWitness();
                self.runCompletenessCheck();
                self.ruinOutputVal();
                self.runSoundnessCheck();
            }
        }
        pub fn ruinOutputVal(&self) {
            *self.pb.as_ref().unwrap().borrow_mut().val(&self.output) =
                ((self.currentInputValues == 0) as i32).into();
        }
    }

    impl OrGadgetExhaustiveTester {
        pub fn new(
            pb: ProtoboardPtr,
            numInputs: usize,
        ) -> LogicGadgetExhaustiveTester<OrGadgetExhaustiveTester> {
            let mut _self =
                LogicGadgetExhaustiveTester::<OrGadgetExhaustiveTester>::new(pb.clone(), numInputs);
            _self.logicGadget = OR_Gadget::create_r1p(
                pb.clone(),
                _self.inputs.clone().into(),
                _self.output.clone().into(),
            );
            _self
        }
    }
}
