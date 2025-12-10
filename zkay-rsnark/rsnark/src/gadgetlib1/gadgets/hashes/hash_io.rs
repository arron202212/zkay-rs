use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::generate_boolean_r1cs_constraint;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::variable::variable;
use ffec::common::utils::bit_vector;
use rccell::RcCell;
#[derive(Clone, Default)]
pub struct digest_variable<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>
    pub digest_size: usize,
    pub bits: pb_variable_array<FieldT, PB>,
}
#[derive(Clone, Default)]
pub struct block_variable<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>
    pub block_size: usize,
    pub bits: pb_variable_array<FieldT, PB>,
}

pub type digest_variables<FieldT, PB> = gadget<FieldT, PB, digest_variable<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> digest_variable<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        digest_size: usize,
        annotation_prefix: String,
    ) -> digest_variables<FieldT, PB> {
        let mut bits = pb_variable_array::<FieldT, PB>::default();
        bits.allocate(
            &pb,
            digest_size,
            &prefix_format!(annotation_prefix, " bits"),
        );
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { digest_size, bits })
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        digest_size: usize,
        partial_bits: pb_variable_array<FieldT, PB>,
        padding: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> digest_variables<FieldT, PB> {
        // assert!(bits.len() <= digest_size);
        let mut bits = partial_bits;
        while (bits.len() != digest_size) {
            bits.contents.push(padding.clone());
        }
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { digest_size, bits })
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> digest_variables<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.digest_size {
            generate_boolean_r1cs_constraint::<FieldT, PB>(
                &self.pb,
                &(self.t.bits[i].clone().into()),
                prefix_format!(self.annotation_prefix, " bits_{}", i),
            );
        }
    }

    pub fn generate_r1cs_witness(&self, contents: &bit_vector) {
        self.t.bits.fill_with_bits(&self.pb, contents);
    }

    pub fn get_digest(&self) -> bit_vector {
        return self.t.bits.get_bits(&self.pb);
    }
}

pub type block_variables<FieldT, PB> = gadget<FieldT, PB, block_variable<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> block_variable<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        block_size: usize,
        annotation_prefix: String,
    ) -> block_variables<FieldT, PB> {
        let mut bits = pb_variable_array::<FieldT, PB>::default();
        bits.allocate(&pb, block_size, &prefix_format!(annotation_prefix, " bits"));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { block_size, bits })
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        parts: Vec<pb_variable_array<FieldT, PB>>,
        annotation_prefix: String,
    ) -> block_variables<FieldT, PB> {
        let mut bits = pb_variable_array::<FieldT, PB>::default();
        for part in parts {
            bits.contents.extend(part.contents.clone());
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                block_size: bits.len(),
                bits,
            },
        )
    }

    pub fn new3(
        pb: RcCell<protoboard<FieldT, PB>>,
        left: digest_variables<FieldT, PB>,
        right: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> block_variables<FieldT, PB> {
        assert!(left.t.bits.len() == right.t.bits.len());
        let block_size = 2 * left.t.bits.len();
        let bits = pb_variable_array::<FieldT, PB>::new(
            left.t
                .bits
                .iter()
                .chain(right.t.bits.iter())
                .cloned()
                .collect(),
        );
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { block_size, bits })
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> block_variables<FieldT, PB> {
    pub fn generate_r1cs_witness(&self, contents: &bit_vector) {
        self.t.bits.fill_with_bits(&self.pb, contents);
    }

    pub fn get_block(&self) -> bit_vector {
        return self.t.bits.get_bits(&self.pb);
    }
}
