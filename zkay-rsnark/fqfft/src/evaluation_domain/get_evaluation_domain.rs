// A convenience method for choosing an evaluation domain
// Returns an evaluation domain object in which the domain S has size
// |S| >= min_size.
// The function chooses from different supported domains, depending on min_size.

use crate::evaluation_domain::domains::arithmetic_sequence_domain::arithmetic_sequence_domain;
use crate::evaluation_domain::domains::basic_radix2_domain::basic_radix2_domain;
use crate::evaluation_domain::domains::extended_radix2_domain::extended_radix2_domain;
use crate::evaluation_domain::domains::geometric_sequence_domain::geometric_sequence_domain;
use crate::evaluation_domain::domains::step_radix2_domain::step_radix2_domain;
use crate::evaluation_domain::evaluation_domain::EvaluationDomainType;
use crate::evaluation_domain::evaluation_domain::evaluation_domain;
use crate::polynomial_arithmetic::basic_operations::_polynomial_multiplication;
use crate::polynomial_arithmetic::basis_change::compute_subproduct_tree;
use crate::tools::exceptions;
use ffec::{FieldTConfig, common::utils::log2};
use rccell::RcCell;

pub fn get_evaluation_domain<FieldT: FieldTConfig>(
    min_size: usize,
) -> eyre::Result<RcCell<EvaluationDomainType<FieldT>>> {
    // let mut  result; //RcCell::new( );//evaluation_domain::<FieldT>

    let big = 1usize << (log2(min_size) - 1);
    let small = min_size - big;
    let rounded_small = (1usize << log2(small));

    //         result=basic_radix2_domain::<FieldT>::new(min_size)).map_err(||
    //         result=extended_radix2_domain::<FieldT>::new(min_size)) .map_err(||
    //         result=step_radix2_domain::<FieldT>::new(min_size)).map_err(||
    // result=basic_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    // result=extended_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    // result=step_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    //  result=geometric_sequence_domain::<FieldT>::new(min_size)).map_err(||
    //         result=arithmetic_sequence_domain::<FieldT>::new(min_size)).map_err(||
    //      eyre::bail!("get_evaluation_domain: no matching domain")))))))))
    eyre::bail!("get_evaluation_domain: no matching domain")
}
