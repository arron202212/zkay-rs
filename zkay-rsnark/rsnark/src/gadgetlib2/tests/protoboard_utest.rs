//  Unit tests for gadgetlib2 protoboard

use crate::gadgetlib2::constraint::PrintOptions;
use crate::gadgetlib2::pp::{Fp, initPublicParamsFromDefaultPp};
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{
    DualWord, FElem, FieldType, MultiPackedWord, UnpackedWord, Variable, VariableArray, VariableSet,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_R1P_enforceBooleanity() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let x = Variable::default();
        pb.borrow_mut().enforceBooleanity(&x);
        *pb.borrow_mut().val(&x) = 0.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&x) = 1.into();
        assert!(
            pb.borrow()
                .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
        );
        *pb.borrow_mut().val(&x) = FElem::froms(Fp::from(2));
        assert!(!pb.borrow().isSatisfied(&&PrintOptions::NO_DBG_PRINT));
    }

    #[test]
    fn test_Protoboard_unpackedWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let unpacked = UnpackedWord::new(8, "unpacked".to_owned());
        pb.borrow_mut().setValuesAsBitArray(&unpacked, 42);
        assert!(pb.borrow_mut().unpackedWordAssignmentEqualsValue(
            &unpacked,
            42,
            &PrintOptions::NO_DBG_PRINT
        ));
        assert!(!pb.borrow_mut().unpackedWordAssignmentEqualsValue(
            &unpacked,
            43,
            &PrintOptions::NO_DBG_PRINT
        ));
        assert!(!pb.borrow_mut().unpackedWordAssignmentEqualsValue(
            &unpacked,
            1024 + 42,
            &PrintOptions::NO_DBG_PRINT
        ));
    }

    #[test]
    fn test_Protoboard_multipackedWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let multipacked = MultiPackedWord::new(8, FieldType::R1P, "multipacked".to_owned());
        *pb.borrow_mut().val(&multipacked[0]) = 42.into();
        assert!(pb.borrow_mut().multipackedWordAssignmentEqualsValue(
            &multipacked,
            42,
            &PrintOptions::NO_DBG_PRINT
        ));
        assert!(!pb.borrow_mut().multipackedWordAssignmentEqualsValue(
            &multipacked,
            43,
            &PrintOptions::NO_DBG_PRINT
        ));
        let multipackedAgnostic = VariableArray::<MultiPackedWord>::from(FieldType::AGNOSTIC);
        // ASSERT_THROW(pb.multipackedWordAssignmentEqualsValue(multipackedAgnostic, 43),
        //              ::std::runtime_error);
    }

    #[test]
    fn test_Protoboard_dualWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        let dualword = DualWord::new(8, FieldType::R1P, "dualword".to_owned());
        pb.borrow_mut().setDualWordValue(&dualword, 42);
        assert!(pb.borrow_mut().dualWordAssignmentEqualsValue(
            &dualword,
            42,
            &PrintOptions::NO_DBG_PRINT
        ));
        assert!(!pb.borrow_mut().dualWordAssignmentEqualsValue(
            &dualword,
            43,
            &PrintOptions::NO_DBG_PRINT
        ));
        assert!(!pb.borrow_mut().dualWordAssignmentEqualsValue(
            &dualword,
            42 + 1024,
            &PrintOptions::NO_DBG_PRINT
        ));
    }
}
