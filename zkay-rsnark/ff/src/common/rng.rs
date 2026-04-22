// Declaration of functions for generating randomness.

use crate::{
    algebra::field_utils::bigint::{GMP_NUMB_BITS, bigint},
    common::{rng, utils::is_little_endian},
};
