//  Functions to profile the gadgetlib1 implementations of Benes and AS-Waksman routing networks.
use crate::gadgetlib1::gadgets::routing::as_waksman_routing_gadget::as_waksman_routing_gadget;
use crate::gadgetlib1::gadgets::routing::benes_routing_gadget::benes_routing_gadget;
use crate::gadgetlib1::pb_variable::pb_variable_array;
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use ffec::FieldTConfig;
use ffec::common::utils::log2;
use rccell::RcCell;

pub fn get_as_waksman_size<FieldT: FieldTConfig, PB: PBConfig>(
    n: usize,
    l: usize,
    num_constraints: &mut usize,
    num_variables: &mut usize,
) {
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let (mut randbits, mut outbits) = (
        vec![pb_variable_array::<FieldT, PB>::default(); n],
        vec![pb_variable_array::<FieldT, PB>::default(); n],
    );
    for y in 0..n {
        randbits[y].allocate(&pb, l, prefix_format!("", "randbits_{}", y));
        outbits[y].allocate(&pb, l, prefix_format!("", "outbits_{}", y));
    }

    let mut r = as_waksman_routing_gadget::<FieldT, PB>::new(
        pb.clone(),
        n,
        randbits,
        outbits,
        "main_routing_gadget".to_owned(),
    );
    r.generate_r1cs_constraints();

    *num_constraints = pb.borrow().num_constraints();
    *num_variables = pb.borrow().num_variables();
}

pub fn get_benes_size<FieldT: FieldTConfig, PB: PBConfig>(
    n: usize,
    l: usize,
    num_constraints: &mut usize,
    num_variables: &mut usize,
) {
    let t = log2(n);
    assert!(n == 1usize << t);

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let (mut randbits, mut outbits) = (
        vec![pb_variable_array::<FieldT, PB>::default(); 1usize << t],
        vec![pb_variable_array::<FieldT, PB>::default(); 1usize << t],
    );
    for y in 0..1usize << t {
        randbits[y].allocate(&pb, l, prefix_format!("", "randbits_{}", y));
        outbits[y].allocate(&pb, l, prefix_format!("", "outbits_{}", y));
    }

    let mut r = benes_routing_gadget::<FieldT, PB>::new(
        pb.clone(),
        n,
        randbits,
        outbits,
        n,
        "main_routing_gadget".to_owned(),
    );
    r.generate_r1cs_constraints();

    *num_constraints = pb.borrow().num_constraints();
    *num_variables = pb.borrow().num_variables();
}

pub fn profile_routing_gadgets<FieldT: FieldTConfig, PB: PBConfig>(l: usize) {
    print!("profiling number of constraints for powers-of-2\n");
    for n in 2..=65 {
        let (mut as_waksman_constr, mut as_waksman_vars) = (0, 0);
        get_as_waksman_size::<FieldT, PB>(n, l, &mut as_waksman_constr, &mut as_waksman_vars);

        let rounded_n = 1usize << log2(n);
        let (mut benes_constr, mut benes_vars) = (0, 0);
        get_benes_size::<FieldT, PB>(rounded_n, l, &mut benes_constr, &mut benes_vars);

        print!(
            "n = {} (rounded = {}), l = {}, benes_constr = {}, benes_vars = {}, as_waksman_constr = {}, as_waksman_vars = {}, constr_ratio = {:.3}, var_ratio = {:.3}\n",
            n,
            rounded_n,
            l,
            benes_constr,
            benes_vars,
            as_waksman_constr,
            as_waksman_vars,
            benes_constr as f64 / as_waksman_constr as f64,
            benes_vars as f64 / as_waksman_vars as f64
        );
    }
}

pub fn profile_num_switches<FieldT: FieldTConfig, PB: PBConfig>(l: usize) {
    print!("profiling number of switches in arbitrary size networks (and rounded-up for Benes)\n");
    for n in 2..=65 {
        let (mut as_waksman_constr, mut as_waksman_vars) = (0, 0);
        get_as_waksman_size::<FieldT, PB>(n, l, &mut as_waksman_constr, &mut as_waksman_vars);

        let rounded_n = 1usize << log2(n);
        let (mut benes_constr, mut benes_vars) = (0, 0);
        get_benes_size::<FieldT, PB>(rounded_n, l, &mut benes_constr, &mut benes_vars);

        let as_waksman_switches = (as_waksman_constr - n * (2 + l)) / 2;
        let benes_switches = (benes_constr - rounded_n * (2 + l)) / 2;
        // let benes_expected = log2(rounded_n)*rounded_n; // switch-Benes has (-rounded_n/2) term
        print!(
            "n = {} (rounded_n = {}), l = {}, benes_switches = {}, as_waksman_switches = {}, ratio = {:.3}\n",
            n,
            rounded_n,
            l,
            benes_switches,
            as_waksman_switches,
            benes_switches as f64 / as_waksman_switches as f64
        );
    }
}

pub fn main() -> i32 {
    // start_profiling();
    // default_ec_pp::init_public_params();
    // profile_routing_gadgets::<Fr::<default_ec_pp> >(32+16+3+2);
    // profile_num_switches::<Fr::<default_ec_pp> >(1);
    0
}
