use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::FieldTConfig;
use rccell::RcCell;
pub struct gadget<FieldT: FieldTConfig, T> {
    pub pb: RcCell<protoboard<FieldT>>,
    pub annotation_prefix: String,
    pub t: T,
}

//
impl<FieldT: FieldTConfig, T> gadget<FieldT, T> {
    pub fn new(pb: RcCell<protoboard<FieldT>>, annotation_prefix: String, t: T) -> Self {
        // #ifdef DEBUG
        // assert!(annotation_prefix != "");
        //#endif
        Self {
            pb,
            annotation_prefix,
            t,
        }
    }
}
