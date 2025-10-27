/** @file
 *****************************************************************************

 Declaration of interfaces for functionality for routing on an arbitrary-size (AS) Waksman network.

 AS-Waksman networks were introduced in \[BD02]. An AS-Waksman network for
 N packets is recursively defined as follows: place a column of floor(N/2) switches on
 the left, and a column of floor(N/2) switches on the right; then place two AS-Waksman
 sub-networks, for floor(N/2) and ceil(N/2) packets respectively, in the middle.

 Note that unlike for Beneš networks where each switch handles routing
 of one packet to one of its two possible destinations, AS-Waksman
 network employs switches with two input ports and two output ports
 and operate either in "straight" or "cross mode".

 Routing is performed in a way that is similar to routing on Benes networks:
 one first computes the switch settings for the left and right columns,
 and then one recursively computes routings for the top and bottom sub-networks.
 More precisely, as in \[BD02], we treat the problem of determining the switch
 settings of the left and right columns as a 2-coloring problem on a certain
 bipartite graph. The coloring is found by performing a depth-first search on
 the graph and alternating the color at every step. For performance reasons
 the graph in our implementation is implicitly represented.

 References:

 \[BD02]:
 "On arbitrary size {W}aksman networks and their vulnerability",
 Bruno Beauquier, Eric Darrot,
 Parallel Processing Letters 2002

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef AS_WAKSMAN_ROUTING_ALGORITHM_HPP_
// #define AS_WAKSMAN_ROUTING_ALGORITHM_HPP_

// use  <cstddef>
// use  <map>
// 

use ffec::common::utils;

use crate::common::data_structures::integer_permutation;



/**
 * When laid out on num_packets \times num_columns grid, each switch
 * occupies two positions: its top input and output ports are at
 * position (column_idx, row_idx) and the bottom input and output
 * ports are at position (column_idx, row_idx+1).
 *
 * We call the position assigned to the top ports of a switch its
 * "canonical" position.
 */

/**
 * A data structure that stores the topology of an AS-Waksman network.
 *
 * For a given column index column_idx and packet index packet_idx,
 * as_waksman_topology[column_idx][packet_idx] specifies the two
 * possible destinations at column_idx+1-th column where the
 * packet_idx-th packet in the column_idx-th column could be routed
 * after passing the switch, which has (column_idx, packet_idx) as one
 * of its occupied positions.
 *
 * This information is stored as a pair of indices, where:
 * - the first index denotes the destination when the switch is
 *   operated in "straight" setting, and
 * - the second index denotes the destination when the switch is
 *   operated in "cross" setting.
 *
 * If no switch occupies a position (column_idx, packet_idx),
 * i.e. there is just a wire passing through that position, then the
 * two indices are set to be equal and the packet is always routed to
 * the specified destination at the column_idx+1-th column.
 */
type as_waksman_topology=Vec<Vec<std::pair<usize, usize> > > ;

/**
 * A routing assigns a bit to each switch in the AS-Waksman routing network.
 *
 * More precisely:
 *
 * - as_waksman_routing[column_idx][packet_idx]=false, if switch with
 *   canonical position of (column_idx,packet_idx) is set to
 *   "straight" setting, and
 *
 * - as_waksman_routing[column_idx][packet_idx]=true, if switch with
 *   canonical position of (column_idx,packet_idx) is set to "cross"
 *   setting.
 *
 * Note that as_waksman_routing[column_idx][packet_idx] does contain
 * entries for the positions associated with the bottom ports of the
 * switches, i.e. only canonical positions are present.
 */
type as_waksman_routing=Vec<BTreeMap<usize, bool> > ;

/**
 * Return the number of (switch) columns in a AS-Waksman network for a given number of packets.
 *
 * For example:
 * - as_waksman_num_columns(2) = 1,
 * - as_waksman_num_columns(3) = 3,
 * - as_waksman_num_columns(4) = 3,
 * and so on.
 */
// usize as_waksman_num_columns(num_packets:usize);

// /**
//  * Return the topology of an AS-Waksman network for a given number of packets.
//  *
//  * See as_waksman_topology (above) for details.
//  */
// as_waksman_topology generate_as_waksman_topology(num_packets:usize);

// /**
//  * Route the given permutation on an AS-Waksman network of suitable size.
//  */
// as_waksman_routing get_as_waksman_routing(permutation:&integer_permutation);

// /**
//  * Check if a routing "implements" the given permutation.
//  */
// bool valid_as_waksman_routing(permutation:&integer_permutation, routing:&as_waksman_routing);



//#endif // AS_WAKSMAN_ROUTING_ALGORITHM_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for functionality for routing on an arbitrary-size (AS) Waksman network.

 See as_waksman_routing_algorithm.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <cassert>

// use crate::common::routing_algorithms::as_waksman_routing_algorithm;



/**
 * Return the height of the AS-Waksman network's top sub-network.
 */
 pub fn as_waksman_top_height(num_packets:usize)->usize
{
    return num_packets/2;
}

/**
 * Return the input wire of a left-hand side switch of an AS-Waksman network for
 * a given number of packets.
 *
 * A switch is specified by a row index row_idx, relative to a "row_offset" that
 * records the level of recursion. (The corresponding column index column_idx
 * can be inferred from row_offset and num_packets, and it is easier to reason about
 * implicitly.)
 *
 * If top = true, return the top wire, otherwise return bottom wire.
 */
 pub fn as_waksman_switch_output(num_packets:usize, row_offset:usize, row_idx:usize, use_top:bool)->usize
{
    let  relpos = row_idx - row_offset;
    assert!(relpos % 2 == 0 && relpos + 1 < num_packets);
    return row_offset + (relpos / 2) + (if use_top { 0 }else {as_waksman_top_height(num_packets)});
}

/**
 * Return the input wire of a right-hand side switch of an AS-Waksman network for
 * a given number of packets.
 *
 * This function is analogous to as_waksman_switch_output above.
 */
 pub fn as_waksman_switch_input(num_packets:usize, row_offset:usize, row_idx:usize, use_top:bool)->usize
{
    /* Due to symmetry, this function equals as_waksman_switch_output. */
    return as_waksman_switch_output(num_packets, row_offset, row_idx, use_top);
}

pub fn  as_waksman_num_columns(num_packets:usize)->usize
{
    return if num_packets > 1 {2*ffec::log2(num_packets)-1} else{0};
}

/**
 * Construct AS-Waksman subnetwork occupying switch columns
 *           [left,left+1, ..., right]
 * that will route
 * - from left-hand side inputs [lo,lo+1,...,hi]
 * - to right-hand side destinations rhs_dests[0],rhs_dests[1],...,rhs_dests[hi-lo+1].
 * That is, rhs_dests are 0-indexed w.r.t. row_offset of lo.
 *
 * Note that rhs_dests is *not* a permutation of [lo, lo+1, ... hi].
 *
 * This function fills out neighbors[left] and neighbors[right-1].
 */
pub fn construct_as_waksman_inner(left:usize,
                                right:usize,
                                lo:usize,
                                hi:usize,
                                rhs_dests:Vec<usize>,
                                 neighbors:&as_waksman_topology)
{
    if left > right
    {
        return;
    }

    let  subnetwork_size = (hi - lo + 1);
    assert!(rhs_dests.len() == subnetwork_size);
    let subnetwork_width = as_waksman_num_columns(subnetwork_size);
    assert!(right - left + 1 >= subnetwork_width);

    if right - left + 1 > subnetwork_width
    {
        /**
         * If there is more space for the routing network than needed,
         * just add straight edges. This also handles the size-1 base case.
         */
        for packet_idx in lo..=hi
        {
            neighbors[left][packet_idx].first = neighbors[left][packet_idx].second = packet_idx;
            neighbors[right][packet_idx].first = neighbors[right][packet_idx].second = rhs_dests[packet_idx - lo];
        }

        let mut  new_rhs_dests=vec![-1;subnetwork_size];
        for packet_idx in lo..=hi
        {
            new_rhs_dests[packet_idx-lo] = packet_idx;
        }

        construct_as_waksman_inner(left+1, right-1, lo, hi, new_rhs_dests, neighbors);
    }
    else if subnetwork_size == 2
    {
        /* Non-trivial base case: routing a 2-element permutation. */
        neighbors[left][lo].first = neighbors[left][hi].second = rhs_dests[0];
        neighbors[left][lo].second = neighbors[left][hi].first = rhs_dests[1];
    }
    else
    {
        /**
         * Networks of size sz > 2 are handled by adding two columns of
         * switches alongside the network and recursing.
         */
        let mut new_rhs_dests=vec![-1;subnetwork_size];

        /**
         * This adds floor(sz/2) switches alongside the network.
         *
         * As per the AS-Waksman construction, one of the switches in the
         * even case can be eliminated (i.e., set to a constant). We handle
         * this later.
         */
        let mut  new_rhs_dests=vec![-1;subnetwork_size];
        for row_idx  in (lo..hi + 1-subnetwork_size%2).step_by(2)
        {
            neighbors[left][row_idx].first = as_waksman_switch_output(subnetwork_size, lo, row_idx, true);
            neighbors[left][row_idx+1].second = neighbors[left][row_idx].first;
            neighbors[left][row_idx].second = as_waksman_switch_output(subnetwork_size, lo, row_idx, false);
            neighbors[left][row_idx+1].first = neighbors[left][row_idx].second;

            new_rhs_dests[as_waksman_switch_input(subnetwork_size, lo, row_idx, true)-lo] = row_idx;
            new_rhs_dests[as_waksman_switch_input(subnetwork_size, lo, row_idx, false)-lo] = row_idx + 1;

            neighbors[right][row_idx].first = rhs_dests[row_idx-lo];
            neighbors[right][row_idx+1].second = rhs_dests[row_idx-lo];
            neighbors[right][row_idx].second = rhs_dests[row_idx+1-lo];
            neighbors[right][row_idx+1].first = rhs_dests[row_idx+1-lo];
        }

        if subnetwork_size % 2 == 1
        {
           
            //  * Odd special case:
            //  * the last wire is not connected to any switch,
            //  * and the wire is merely routed "straight".
            
            (neighbors[left][hi].first , neighbors[left][hi].second )= (hi,hi);
            (neighbors[right][hi].first , neighbors[right][hi].second )= (rhs_dests[hi-lo],rhs_dests[hi-lo]);
            new_rhs_dests[hi-lo] = hi;
        }
        else
        {
           
            //  * Even special case:
            //  * fix the bottom-most left-hand-side switch
            //  * to a constant "straight" setting.
            
            neighbors[left][hi-1].second = neighbors[left][hi-1].first;
            neighbors[left][hi].second = neighbors[left][hi].first;
        }

        let  d = as_waksman_top_height(subnetwork_size);
        let  new_rhs_dests_top=&new_rhs_dests.begin()[..d];
        let  new_rhs_dests_bottom=&new_rhs_dests[d..];

        construct_as_waksman_inner(left+1, right-1, lo, lo+d-1, new_rhs_dests_top, neighbors);
        construct_as_waksman_inner(left+1, right-1, lo+d, hi, new_rhs_dests_bottom, neighbors);
    }
}

 pub fn generate_as_waksman_topology(num_packets:usize)->as_waksman_topology
{
    assert!(num_packets > 1);
    let  width = as_waksman_num_columns(num_packets);

    let  neighbors=as_waksman_topology(width, vec![(-1,-1);num_packets]);

    let mut rhs_dests=vec![0;num_packets];
    for packet_idx in 0..num_packets
    {
        rhs_dests[packet_idx] = packet_idx;
    }

    construct_as_waksman_inner(0, width-1, 0, num_packets-1, rhs_dests, neighbors);

    return neighbors;
}

/**
 * Given either a position occupied either by its top or bottom ports,
 * return the row index of its canonical position.
 *
 * This function is agnostic to column_idx, given row_offset, so we omit
 * column_idx.
 */
 pub fn s_waksman_get_canonical_row_idx(row_offset:usize, row_idx:usize)->usize
{
    /* translate back relative to row_offset, clear LSB, and then translate forward */
    return (((row_idx - row_offset) & !1) + row_offset);
}

/**
 * Return a switch value that makes switch row_idx =
 * as_waksman_switch_position_from_wire_position(row_offset, packet_idx) to
 * route the wire packet_idx via the top (if top = true), resp.,
 * bottom (if top = false) subnetwork.
 *
 * NOTE: pos is assumed to be
 * - the input position for the LHS switches, and
 * - the output position for the RHS switches.
 */
 pub fn as_waksman_get_switch_setting_from_top_bottom_decision(row_offset:usize, packet_idx:usize, use_top:bool)->bool
{
    let  row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
    return (packet_idx == row_idx) ^ use_top;
}

/**
 * Return true if the switch with input port at (column_idx, row_idx)
 * when set to "straight" (if top = true), resp., "cross" (if top =
 * false), routes the packet at (column_idx, row_idx) via the top
 * subnetwork.
 *
 * NOTE: packet_idx is assumed to be
 * - the input position for the RHS switches, and
 * - the output position for the LHS switches.
 */
 pub fn as_waksman_get_top_bottom_decision_from_switch_setting(row_offset:usize, packet_idx:usize, switch_setting:bool)->bool
{
    let  row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
    return (row_idx == packet_idx) ^ switch_setting;
}

/**
 * Given an output wire of a RHS switch, compute and return the output
 * position of the other wire also connected to this switch.
 */
 pub fn as_waksman_other_output_position(row_offset:usize, packet_idx:usize)->usize
{
    let  row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
    return (1 - (packet_idx - row_idx)) + row_idx;
}

/**
 * Given an input wire of a LHS switch, compute and return the input
 * position of the other wire also connected to this switch.
 */
 pub fn as_waksman_other_input_position(row_offset:usize, packet_idx:usize)->usize
{
    /* Due to symmetry, this function equals as_waksman_other_output_position. */
    return as_waksman_other_output_position(row_offset, packet_idx);
}

/**
 * Compute AS-Waksman switch settings for the subnetwork occupying switch columns
 *         [left,left+1,...,right]
 * that will route
 * - from left-hand side inputs [lo,lo+1,...,hi]
 * - to right-hand side destinations pi[lo],pi[lo+1],...,pi[hi].
 *
 * The permutation
 * - pi maps [lo, lo+1, ... hi] to itself, offset by lo, and
 * - piinv is the inverse of pi.
 *
 * NOTE: due to offsets, neither pi or piinv are instances of integer_permutation.
 */
pub fn as_waksman_route_inner(left:usize,
                            right:usize,
                            lo:usize,
                            hi:usize,
                            permutation:&integer_permutation,
                            permutation_inv:&integer_permutation,
                            routing:&as_waksman_routing)
{
    if left > right
    {
        return;
    }

    let  subnetwork_size = (hi - lo + 1);
    let  subnetwork_width = as_waksman_num_columns(subnetwork_size);
    assert!(right - left + 1 >= subnetwork_width);

// #ifdef DEBUG
    assert!(permutation.min_element == lo);
    assert!(permutation.max_element == hi);
    assert!(permutation.len() == subnetwork_size);
    assert!(permutation.is_valid());
    assert!(permutation.inverse() == permutation_inv);
//#endif

    if right - left + 1 > subnetwork_width
    {
        /**
         * If there is more space for the routing network than required,
         * then the topology for this subnetwork includes straight edges
         * along its sides and no switches, so it suffices to recurse.
         */
        as_waksman_route_inner(left+1, right-1, lo, hi, permutation, permutation_inv, routing);
    }
    else if subnetwork_size == 2
    {
        /**
         * Non-trivial base case: switch settings for a 2-element permutation
         */
        assert!(permutation.get(lo) == lo || permutation.get(lo) == lo+1);
        assert!(permutation.get(lo+1) == lo || permutation.get(lo+1) == lo + 1);
        assert!(permutation.get(lo) != permutation.get(lo+1));

        routing[left][lo] = (permutation.get(lo) != lo);
    }
    else
    {
        /**
         * The algorithm first assigns a setting to a LHS switch,
         * route its target to RHS, which will enforce a RHS switch setting.
         * Then, it back-routes the RHS value back to LHS.
         * If this enforces a LHS switch setting, then forward-route that;
         * otherwise we will select the next value from LHS to route.
         */
        let mut  new_permutation=integer_permutation::new(lo, hi);
        let mut  new_permutation_inv=integer_permutation::new(lo, hi);
        let mut  lhs_routed=vec![false;subnetwork_size]; /* offset by lo, i.e. lhs_routed[packet_idx-lo] is set if packet packet_idx is routed */

        let mut  to_route;
        let mut  max_unrouted;
        let mut  route_left;

        if subnetwork_size % 2 == 1
        {
            /**
             * ODD CASE: we first deal with the bottom-most straight wire,
             * which is not connected to any of the switches at this level
             * of recursion and just passed into the lower subnetwork.
             */
            if permutation.get(hi) == hi
            {
                /**
                 * Easy sub-case: it is routed directly to the bottom-most
                 * wire on RHS, so no switches need to be touched.
                 */
                new_permutation.set(hi, hi);
                new_permutation_inv.set(hi, hi);
                to_route = hi - 1;
                route_left = true;
            }
            else
            {
                /**
                 * Other sub-case: the straight wire is routed to a switch
                 * on RHS, so route the other value from that switch
                 * using the lower subnetwork.
                 */
                let  rhs_switch = as_waksman_get_canonical_row_idx(lo, permutation.get(hi));
                let  rhs_switch_setting = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation.get(hi), false);
                routing[right][rhs_switch] = rhs_switch_setting;
                let mut  tprime = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, false);
                new_permutation.set(hi, tprime);
                new_permutation_inv.set(tprime, hi);

                to_route = as_waksman_other_output_position(lo, permutation.get(hi));
                route_left = false;
            }

            lhs_routed[hi-lo] = true;
            max_unrouted = hi - 1;
        }
        else
        {

            //  * EVEN CASE: the bottom-most switch is fixed to a constant
            //  * straight setting. So we route wire hi accordingly.

            routing[left][hi-1] = false;
            to_route = hi;
            route_left = true;
            max_unrouted = hi;
        }

        while (1)
        {
            /**
             * INVARIANT: the wire `to_route' on LHS (if route_left = true),
             * resp., RHS (if route_left = false) can be routed.
             */
            if route_left
            {
                /* If switch value has not been assigned, assign it arbitrarily. */
                let  lhs_switch = as_waksman_get_canonical_row_idx(lo, to_route);
                if routing[left].find(lhs_switch) == routing[left].end()
                {
                    routing[left][lhs_switch] = false;
                }
                let  lhs_switch_setting = routing[left][lhs_switch];
                let  use_top = as_waksman_get_top_bottom_decision_from_switch_setting(lo, to_route, lhs_switch_setting);
                let  t = as_waksman_switch_output(subnetwork_size, lo, lhs_switch, use_top);
                if permutation.get(to_route) == hi
                {
                    /**
                     * We have routed to the straight wire for the odd case,
                     * so now we back-route from it.
                     */
                    new_permutation.set(t, hi);
                    new_permutation_inv.set(hi, t);
                    lhs_routed[to_route-lo] = true;
                    to_route = max_unrouted;
                    route_left = true;
                }
                else
                {
                    let rhs_switch = as_waksman_get_canonical_row_idx(lo, permutation.get(to_route));
                    /**
                     * We know that the corresponding switch on the right-hand side
                     * cannot be set, so we set it according to the incoming wire.
                     */
                    assert!(routing[right].find(rhs_switch) == routing[right].end());
                    routing[right][rhs_switch] = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation.get(to_route), use_top);
                    let tprime = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, use_top);
                    new_permutation.set(t, tprime);
                    new_permutation_inv.set(tprime, t);

                    lhs_routed[to_route-lo] = true;
                    to_route = as_waksman_other_output_position(lo, permutation.get(to_route));
                    route_left = false;
                }
            }
            else
            {
                /**
                 * We have arrived on the right-hand side, so the switch setting is fixed.
                 * Next, we back route from here.
                 */
                let rhs_switch = as_waksman_get_canonical_row_idx(lo, to_route);
                let lhs_switch = as_waksman_get_canonical_row_idx(lo, permutation_inv.get(to_route));
                assert!(routing[right].find(rhs_switch) != routing[right].end());
                let rhs_switch_setting = routing[right][rhs_switch];
                let use_top = as_waksman_get_top_bottom_decision_from_switch_setting(lo, to_route, rhs_switch_setting);
                let lhs_switch_setting = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation_inv.get(to_route), use_top);

                /* The value on the left-hand side is either the same or not set. */
                let it = routing[left].find(lhs_switch);
                assert!(it == routing[left].end() || it.1 == lhs_switch_setting);
                routing[left][lhs_switch] = lhs_switch_setting;

                let t = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, use_top);
                let tprime = as_waksman_switch_output(subnetwork_size, lo, lhs_switch, use_top);
                new_permutation.set(tprime, t);
                new_permutation_inv.set(t, tprime);

                lhs_routed[permutation_inv.get(to_route)-lo] = true;
                to_route = as_waksman_other_input_position(lo, permutation_inv.get(to_route));
                route_left = true;
            }

            /* If the next packet to be routed hasn't been routed before, then try routing it. */
            if !route_left || !lhs_routed[to_route-lo]
            {
                continue;
            }

            /* Otherwise just find the next unrouted packet. */
            while (max_unrouted > lo && lhs_routed[max_unrouted-lo])
            {
                max_unrouted-=1;
            }

            if max_unrouted < lo || (max_unrouted == lo && lhs_routed[0]) /* lhs_routed[0] = corresponds to lo shifted by lo */
            {
                /* All routed! */
                break;
            }
            else
            {
                to_route = max_unrouted;
                route_left = true;
            }
        }

        if subnetwork_size % 2 == 0
        {
            /* Remove the AS-Waksman switch with the fixed value. */
            routing[left].erase(hi-1);
        }

        let d = as_waksman_top_height(subnetwork_size);
        let new_permutation_upper = new_permutation.slice(lo, lo + d - 1);
        let new_permutation_lower = new_permutation.slice(lo + d, hi);

        let new_permutation_inv_upper = new_permutation_inv.slice(lo, lo + d - 1);
        let new_permutation_inv_lower = new_permutation_inv.slice(lo + d, hi);

        as_waksman_route_inner(left+1, right-1, lo, lo + d - 1, new_permutation_upper, new_permutation_inv_upper, routing);
        as_waksman_route_inner(left+1, right-1, lo + d, hi, new_permutation_lower, new_permutation_inv_lower, routing);
    }
}

 pub fn get_as_waksman_routing(permutation:&integer_permutation)->as_waksman_routing
{
    let num_packets = permutation.len();
    let width = as_waksman_num_columns(num_packets);

     let routing=as_waksman_routing(width);
    as_waksman_route_inner(0, width-1, 0, num_packets-1, permutation, permutation.inverse(), routing);
    return routing;
}

 pub fn valid_as_waksman_routing(permutation:&integer_permutation, routing:&as_waksman_routing)->bool
{
    let num_packets = permutation.len();
    let width = as_waksman_num_columns(num_packets);
    let  neighbors = generate_as_waksman_topology(num_packets);

    let mut  curperm=integer_permutation::new(num_packets);

    for column_idx in 0..width
    {
         let mut nextperm=integer_permutation::new(num_packets);
        for packet_idx in 0..num_packets
        {
            let mut  routed_packet_idx;
            if neighbors[column_idx][packet_idx].first == neighbors[column_idx][packet_idx].second
            {
                routed_packet_idx = neighbors[column_idx][packet_idx].first;
            }
            else
            {
                let it = routing[column_idx].find(packet_idx);
                let it2 = routing[column_idx].find(packet_idx-1);
                assert!((it != routing[column_idx].end()) ^ (it2 != routing[column_idx].end()));
                let switch_setting = if it != routing[column_idx].end() {it.1} else{it2.1};

                routed_packet_idx = if switch_setting {neighbors[column_idx][packet_idx].second} else{neighbors[column_idx][packet_idx].first};
            }

            nextperm.set(routed_packet_idx, curperm.get(packet_idx));
        }

        curperm = nextperm;
    }

    return (curperm == permutation.inverse());
}


