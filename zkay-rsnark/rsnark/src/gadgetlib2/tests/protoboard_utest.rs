//  Unit tests for gadgetlib2 protoboard

// use crate::gadgetlib2::pp;
// use crate::gadgetlib2::protoboard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn R1P_enforceBooleanity() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let x = Variable::default();
        pb.enforceBooleanity(x);
        pb.val(x) = 0;
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        pb.val(x) = 1;
        assert!(pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED));
        pb.val(x) = Fp(2);
        assert!(!pb.isSatisfied());
    }

    #[test]
    fn Protoboard_unpackedWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let unpacked = UnpackedWord::new(8, "unpacked");
        pb.setValuesAsBitArray(unpacked, 42);
        assert!(pb.unpackedWordAssignmentEqualsValue(unpacked, 42));
        assert!(!pb.unpackedWordAssignmentEqualsValue(unpacked, 43));
        assert!(!pb.unpackedWordAssignmentEqualsValue(unpacked, 1024 + 42));
    }

    #[test]
    fn Protoboard_multipackedWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let multipacked = MultiPackedWord::new(8, R1P, "multipacked");
        pb.val(multipacked[0]) = 42;
        assert!(pb.multipackedWordAssignmentEqualsValue(multipacked, 42));
        assert!(!pb.multipackedWordAssignmentEqualsValue(multipacked, 43));
        let multipackedAgnostic = MultiPackedWord::from(AGNOSTIC);
        // ASSERT_THROW(pb.multipackedWordAssignmentEqualsValue(multipackedAgnostic, 43),
        //              ::std::runtime_error);
    }

    #[test]
    fn Protoboard_dualWordAssignmentEqualsValue_R1P() {
        initPublicParamsFromDefaultPp();
        let pb = Protoboard::create(R1P);
        let dualword = DualWord::new(8, R1P, "dualword");
        pb.setDualWordValue(dualword, 42);
        assert!(pb.dualWordAssignmentEqualsValue(dualword, 42));
        assert!(!pb.dualWordAssignmentEqualsValue(dualword, 43));
        assert!(!pb.dualWordAssignmentEqualsValue(dualword, 42 + 1024));
    }
}
