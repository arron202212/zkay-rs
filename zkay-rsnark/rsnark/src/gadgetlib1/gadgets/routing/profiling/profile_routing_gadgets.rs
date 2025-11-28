/** @file
 *****************************************************************************

 Functions to profile the gadgetlib1 implementations of Benes and AS-Waksman routing networks.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <algorithm>

use ffec::common::default_types::ec_pp;
use ffec::common::profiling;

use crate::gadgetlib1::gadgets::routing::as_waksman_routing_gadget;
use crate::gadgetlib1::gadgets::routing::benes_routing_gadget;




pub fn  get_as_waksman_size( n:usize, l:usize, num_constraints:usize, num_variables:usize)
{
    let mut  pb=protoboard::<FieldT> ::new();

    let  (mut randbits, mut outbits)=(vec![vec![];n],vec![vec![];n]);
    for y in 0..n
    {
        randbits[y].allocate(&pb, l, FMT("", "randbits_{}", y));
        outbits[y].allocate(&pb, l, FMT("", "outbits_{}", y));
    }

    let mut r=as_waksman_routing_gadget::<FieldT>::new(pb, n, randbits, outbits, "main_routing_gadget");
    r.generate_r1cs_constraints();

    num_constraints = pb.num_constraints();
    num_variables = pb.num_variables();
}


pub fn  get_benes_size( n:usize, l:usize,  num_constraints:&mut usize, num_variables:&mut usize)
{
    let t = ffec::log2(n);
    assert!(n == 1u64<<t);

    let mut  pb=protoboard::<FieldT> ::new();

    let (mut  randbits,outbits)=(vec![vec![];1usize<<t],vec![vec![];1usize<<t]);
    for y in 0..1u64<<t
    {
        randbits[y].allocate(&pb, l, FMT("", "randbits_{}", y));
        outbits[y].allocate(&pb, l, FMT("", "outbits_{}", y));
    }

    let mut r=benes_routing_gadget::<FieldT>::new(pb, n, randbits, outbits, n, "main_routing_gadget");
    r.generate_r1cs_constraints();

    num_constraints = pb.num_constraints();
    num_variables = pb.num_variables();
}


pub fn  profile_routing_gadgets(l:usize)
{
    print!("profiling number of constraints for powers-of-2\n");
    for n in 2..=65
    {
        let  (as_waksman_constr, as_waksman_vars);
        get_as_waksman_size::<FieldT>(n, l, as_waksman_constr, as_waksman_vars);

        let rounded_n = 1u64<<ffec::log2(n);
        let  (benes_constr, benes_vars);
        get_benes_size::<FieldT>(rounded_n, l, benes_constr, benes_vars);

        print!("n = {} (rounded = {}), l = {}, benes_constr = {}, benes_vars = {}, as_waksman_constr = {}, as_waksman_vars = {}, constr_ratio = {:.3}, var_ratio = {:.3}\n",
               n, rounded_n, l, benes_constr, benes_vars, as_waksman_constr, as_waksman_vars, 1.benes_constr/as_waksman_constr, 1.benes_vars/as_waksman_vars);
    }
}


pub fn  profile_num_switches(l:usize)
{
    print!("profiling number of switches in arbitrary size networks (and rounded-up for Benes)\n");
    for n in 2..=65
    {
        let (as_waksman_constr, as_waksman_vars);
        get_as_waksman_size::<FieldT>(n, l, as_waksman_constr, as_waksman_vars);

        let rounded_n = 1u64<<ffec::log2(n);
        let (benes_constr, benes_vars);
        get_benes_size::<FieldT>(rounded_n, l, benes_constr, benes_vars);

        let as_waksman_switches = (as_waksman_constr - n*(2+l))/2;
        let benes_switches = (benes_constr - rounded_n*(2+l))/2;
        // let benes_expected = ffec::log2(rounded_n)*rounded_n; // switch-Benes has (-rounded_n/2) term
        print!("n = {} (rounded_n = {}), l = {}, benes_switches = {}, as_waksman_switches = {}, ratio = {:.3}\n",
               n, rounded_n, l, benes_switches, as_waksman_switches, 1.*benes_switches/as_waksman_switches);
    }
}

pub fn  main()->i32
{
    ffec::start_profiling();
    ffec::default_ec_pp::init_public_params();
    profile_routing_gadgets::<ffec::Fr::<ffec::default_ec_pp> >(32+16+3+2);
    profile_num_switches::<ffec::Fr::<ffec::default_ec_pp> >(1);
0
}
