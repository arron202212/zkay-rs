/** @file
 *****************************************************************************

 Functions to profile the gadgetlib1 implementations of Benes and AS-Waksman routing networks.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <algorithm>

use ffec::common::default_types::ec_pp;
use ffec::common::profiling;

use crate::gadgetlib1::gadgets::routing::as_waksman_routing_gadget;
use crate::gadgetlib1::gadgets::routing::benes_routing_gadget;



template<typename FieldT>
void get_as_waksman_size(const size_t n, const size_t l, size_t &num_constraints, size_t &num_variables)
{
    protoboard<FieldT> pb;

    std::vector<pb_variable_array<FieldT> > randbits(n), outbits(n);
    for y in 0..n
    {
        randbits[y].allocate(pb, l, FMT("", "randbits_{}", y));
        outbits[y].allocate(pb, l, FMT("", "outbits_{}", y));
    }

    as_waksman_routing_gadget<FieldT> r(pb, n, randbits, outbits, "main_routing_gadget");
    r.generate_r1cs_constraints();

    num_constraints = pb.num_constraints();
    num_variables = pb.num_variables();
}

template<typename FieldT>
void get_benes_size(const size_t n, const size_t l, size_t &num_constraints, size_t &num_variables)
{
    const size_t t = ffec::log2(n);
    assert!(n == 1u64<<t);

    protoboard<FieldT> pb;

    std::vector<pb_variable_array<FieldT> > randbits(1u64<<t), outbits(1u64<<t);
    for y in 0..1u64<<t
    {
        randbits[y].allocate(pb, l, FMT("", "randbits_{}", y));
        outbits[y].allocate(pb, l, FMT("", "outbits_{}", y));
    }

    benes_routing_gadget<FieldT> r(pb, n, randbits, outbits, n, "main_routing_gadget");
    r.generate_r1cs_constraints();

    num_constraints = pb.num_constraints();
    num_variables = pb.num_variables();
}

template<typename FieldT>
void profile_routing_gadgets(const size_t l)
{
    print!("profiling number of constraints for powers-of-2\n");
    for n in 2..=65
    {
        size_t as_waksman_constr, as_waksman_vars;
        get_as_waksman_size<FieldT>(n, l, as_waksman_constr, as_waksman_vars);

        const size_t rounded_n = 1u64<<ffec::log2(n);
        size_t benes_constr, benes_vars;
        get_benes_size<FieldT>(rounded_n, l, benes_constr, benes_vars);

        print!("n = {} (rounded = {}), l = {}, benes_constr = {}, benes_vars = {}, as_waksman_constr = {}, as_waksman_vars = {}, constr_ratio = %0.3f, var_ratio = %0.3f\n",
               n, rounded_n, l, benes_constr, benes_vars, as_waksman_constr, as_waksman_vars, 1.*benes_constr/as_waksman_constr, 1.*benes_vars/as_waksman_vars);
    }
}

template<typename FieldT>
void profile_num_switches(const size_t l)
{
    print!("profiling number of switches in arbitrary size networks (and rounded-up for Benes)\n");
    for n in 2..=65
    {
        size_t as_waksman_constr, as_waksman_vars;
        get_as_waksman_size<FieldT>(n, l, as_waksman_constr, as_waksman_vars);

        const size_t rounded_n = 1u64<<ffec::log2(n);
        size_t benes_constr, benes_vars;
        get_benes_size<FieldT>(rounded_n, l, benes_constr, benes_vars);

        const size_t as_waksman_switches = (as_waksman_constr - n*(2+l))/2;
        const size_t benes_switches = (benes_constr - rounded_n*(2+l))/2;
        // const size_t benes_expected = ffec::log2(rounded_n)*rounded_n; // switch-Benes has (-rounded_n/2) term
        print!("n = {} (rounded_n = {}), l = {}, benes_switches = {}, as_waksman_switches = {}, ratio = %0.3f\n",
               n, rounded_n, l, benes_switches, as_waksman_switches, 1.*benes_switches/as_waksman_switches);
    }
}

int main()
{
    ffec::start_profiling();
    ffec::default_ec_pp::init_public_params();
    profile_routing_gadgets<ffec::Fr<ffec::default_ec_pp> >(32+16+3+2);
    profile_num_switches<ffec::Fr<ffec::default_ec_pp> >(1);
}
