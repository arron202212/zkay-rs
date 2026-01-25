/** @file
*****************************************************************************

Declaration of interfaces for a protoboard for TinyRAM.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TINYRAM_PROTOBOARD_HPP_
// #define TINYRAM_PROTOBOARD_HPP_
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::tinyram_packing_gadget;
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::relations::ram_computations::rams::ram_params;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_architecture_params;
use ffec::FieldTConfig;
use rccell::RcCell;
use std::marker::PhantomData;
#[derive(Clone, Default)]
pub struct tinyram_protoboard<FieldT: FieldTConfig> {
    // : public RcCell<protoboard<FieldT>>
    pub ap: tinyram_architecture_params,
    _t: PhantomData<FieldT>,
    // tinyram_protoboard(ap:tinyram_architecture_params);
}
pub trait SubTinyRamConfig: Default + Clone {}

#[derive(Clone, Default)]
pub struct tinyram_gadget<FieldT: FieldTConfig, T: SubTinyRamConfig> {
    // : public gadget<FieldT>
    // pb: RcCell<protoboard<FieldT,tinyram_protoboard<FieldT>>>,
    _t: PhantomData<FieldT>,
    pub t: T,
    //     tinyram_gadget(pb:tinyram_protoboard<FieldT>, annotation_prefix:String="");
}

// standard gadgets provide two methods: generate_r1cs_constraints and generate_r1cs_witness
pub trait SubTinyRamGadgetConfig: Default + Clone {
    // fn generate_r1cs_constraints(&self);
    // fn generate_r1cs_witness(&self);
}

#[derive(Clone, Default)]
pub struct tinyram_standard_gadget<FieldT: FieldTConfig, T: SubTinyRamGadgetConfig> {
    // : public tinyram_gadget<FieldT>
    _t: PhantomData<FieldT>,
    pub t: T,
    //     tinyram_standard_gadget(pb:tinyram_protoboard<FieldT>, annotation_prefix:String="");
}

impl<FieldT: FieldTConfig> tinyram_protoboard<FieldT> {
    pub fn new(ap: tinyram_architecture_params) -> protoboard<FieldT, Self> {
        protoboard::<FieldT, Self>::new(Self {
            ap,
            _t: PhantomData,
        })
    }
}
impl<FieldT: FieldTConfig> PBConfig for tinyram_protoboard<FieldT> {}

impl<FieldT: FieldTConfig, T: SubTinyRamConfig> tinyram_gadget<FieldT, T> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, tinyram_protoboard<FieldT>, Self> {
        // gadget<FieldT>(&pb, annotation_prefix),pb
        gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new(
            pb,
            annotation_prefix,
            Self { _t: PhantomData, t },
        )
    }
}
pub type tinyram_standard_gadgets<FieldT, T> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, T>>,
>;
impl<FieldT: FieldTConfig, T: SubTinyRamGadgetConfig> tinyram_standard_gadget<FieldT, T> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, tinyram_protoboard<FieldT>, tinyram_gadget<FieldT, Self>> {
        // tinyram_gadget<FieldT>(&pb, annotation_prefix)
        tinyram_gadget::<FieldT, Self>::new(pb, annotation_prefix, Self { _t: PhantomData, t })
    }
}
impl<FieldT: FieldTConfig, T: SubTinyRamGadgetConfig> SubTinyRamConfig
    for tinyram_standard_gadget<FieldT, T>
{
}
