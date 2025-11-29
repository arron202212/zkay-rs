// Declaration of interfaces for a protoboard for the FOORAM CPU.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::FieldTConfig;
use crate::relations::ram_computations::rams::fooram::fooram_aux;
use crate::relations::ram_computations::rams::fooram::fooram_aux::fooram_architecture_params;
use rccell::RcCell;
use std::marker::PhantomData;
#[derive(Clone, Default)]
pub struct fooram_protoboard<FieldT: FieldTConfig> {
    pub ap: fooram_architecture_params,
    _t: PhantomData<FieldT>,
    // fooram_protoboard(ap:&fooram_architecture_params);
}
pub trait SubFooRamConfig: Default + Clone {}
pub struct fooram_gadget<FieldT: FieldTConfig, T: SubFooRamConfig> {
    _t: PhantomData<FieldT>,
    pub t: T,
    //     fooram_gadget(fooram_protoboard<FieldT> &pb, annotation_prefix:&String="");
}

// Implementation of interfaces for a protoboard for the FOORAM CPU.

impl<FieldT: FieldTConfig> fooram_protoboard<FieldT> {
    pub fn new(ap: fooram_architecture_params) -> protoboard<FieldT, Self> {
        protoboard::<FieldT, Self>::new(Self {
            ap,
            _t: PhantomData,
        })
    }
}
impl<FieldT: FieldTConfig> PBConfig for fooram_protoboard<FieldT> {}
impl<FieldT: FieldTConfig, T: SubFooRamConfig> fooram_gadget<FieldT, T> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, fooram_protoboard<FieldT>>>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, fooram_protoboard<FieldT>, Self> {
        gadget::<FieldT, fooram_protoboard<FieldT>, Self>::new(
            pb,
            annotation_prefix,
            Self { _t: PhantomData, t },
        )
    }
}
