use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use ffec::FieldTConfig;
use rccell::RcCell;
#[derive(Clone, Default)]
pub struct gadget<FieldT: FieldTConfig, PB: PBConfig, T: Clone + Default> {
    pub pb: RcCell<protoboard<FieldT, PB>>,
    pub annotation_prefix: String,
    pub t: T,
}

impl<FieldT: FieldTConfig, PB: PBConfig, T: Clone + Default> gadget<FieldT, PB, T> {
    pub fn new(pb: RcCell<protoboard<FieldT, PB>>, annotation_prefix: String, t: T) -> Self {
        // #ifdef DEBUG
        // assert!(annotation_prefix != "");

        Self {
            pb,
            annotation_prefix,
            t,
        }
    }
}
