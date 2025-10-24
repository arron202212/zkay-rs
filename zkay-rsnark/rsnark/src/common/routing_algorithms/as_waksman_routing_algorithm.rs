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

use  <cstddef>
use  <map>
use  <vector>

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
type std::vector<std::vector<std::pair<size_t, size_t> > > as_waksman_topology;

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
type std::vector<std::map<size_t, bool> > as_waksman_routing;

/**
 * Return the number of (switch) columns in a AS-Waksman network for a given number of packets.
 *
 * For example:
 * - as_waksman_num_columns(2) = 1,
 * - as_waksman_num_columns(3) = 3,
 * - as_waksman_num_columns(4) = 3,
 * and so on.
 */
size_t as_waksman_num_columns(const size_t num_packets);

/**
 * Return the topology of an AS-Waksman network for a given number of packets.
 *
 * See as_waksman_topology (above) for details.
 */
as_waksman_topology generate_as_waksman_topology(const size_t num_packets);

/**
 * Route the given permutation on an AS-Waksman network of suitable size.
 */
as_waksman_routing get_as_waksman_routing(const integer_permutation &permutation);

/**
 * Check if a routing "implements" the given permutation.
 */
bool valid_as_waksman_routing(const integer_permutation &permutation, const as_waksman_routing &routing);



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

use  <cassert>

use crate::common::routing_algorithms::as_waksman_routing_algorithm;



/**
 * Return the height of the AS-Waksman network's top sub-network.
 */
size_t as_waksman_top_height(const size_t num_packets)
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
size_t as_waksman_switch_output(const size_t num_packets, const size_t row_offset, const size_t row_idx, const bool use_top)
{
    size_t relpos = row_idx - row_offset;
    assert!(relpos % 2 == 0 && relpos + 1 < num_packets);
    return row_offset + (relpos / 2) + (if use_top { 0 }else {as_waksman_top_height(num_packets)});
}

/**
 * Return the input wire of a right-hand side switch of an AS-Waksman network for
 * a given number of packets.
 *
 * This function is analogous to as_waksman_switch_output above.
 */
size_t as_waksman_switch_input(const size_t num_packets, const size_t row_offset, const size_t row_idx, const bool use_top)
{
    /* Due to symmetry, this function equals as_waksman_switch_output. */
    return as_waksman_switch_output(num_packets, row_offset, row_idx, use_top);
}

size_t as_waksman_num_columns(const size_t num_packets)
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
void construct_as_waksman_inner(const size_t left,
                                const size_t right,
                                const size_t lo,
                                const size_t hi,
                                const std::vector<size_t> rhs_dests,
                                as_waksman_topology &neighbors)
{
    if left > right
    {
        return;
    }

    const size_t subnetwork_size = (hi - lo + 1);
    assert!(rhs_dests.len() == subnetwork_size);
    const size_t subnetwork_width = as_waksman_num_columns(subnetwork_size);
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

        std::vector<size_t> new_rhs_dests(subnetwork_size, -1);
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
        std::vector<size_t> new_rhs_dests(subnetwork_size, -1);

        /**
         * This adds floor(sz/2) switches alongside the network.
         *
         * As per the AS-Waksman construction, one of the switches in the
         * even case can be eliminated (i.e., set to a constant). We handle
         * this later.
         */
        for hi in &hi + 1; row_idx += 2)
        {
            neighbors[left][row_idx].first = neighbors[left][row_idx+1].second = as_waksman_switch_output(subnetwork_size, lo, row_idx, true);
            neighbors[left][row_idx].second = neighbors[left][row_idx+1].first = as_waksman_switch_output(subnetwork_size, lo, row_idx, false);

            new_rhs_dests[as_waksman_switch_input(subnetwork_size, lo, row_idx, true)-lo] = row_idx;
            new_rhs_dests[as_waksman_switch_input(subnetwork_size, lo, row_idx, false)-lo] = row_idx + 1;

            neighbors[right][row_idx].first = neighbors[right][row_idx+1].second = rhs_dests[row_idx-lo];
            neighbors[right][row_idx].second = neighbors[right][row_idx+1].first = rhs_dests[row_idx+1-lo];
        }

        if subnetwork_size % 2 == 1
        {
            /**
             * Odd special case:
             * the last wire is not connected to any switch,
             * and the wire is merely routed "straight".
             */
            neighbors[left][hi].first = neighbors[left][hi].second = hi;
            neighbors[right][hi].first = neighbors[right][hi].second = rhs_dests[hi-lo];
            new_rhs_dests[hi-lo] = hi;
        }
        else
        {
            /**
             * Even special case:
             * fix the bottom-most left-hand-side switch
             * to a constant "straight" setting.
             */
            neighbors[left][hi-1].second = neighbors[left][hi-1].first;
            neighbors[left][hi].second = neighbors[left][hi].first;
        }

        const size_t d = as_waksman_top_height(subnetwork_size);
        const std::vector<size_t> new_rhs_dests_top(new_rhs_dests.begin(), new_rhs_dests.begin()+d);
        const std::vector<size_t> new_rhs_dests_bottom(new_rhs_dests.begin()+d, new_rhs_dests.end());

        construct_as_waksman_inner(left+1, right-1, lo, lo+d-1, new_rhs_dests_top, neighbors);
        construct_as_waksman_inner(left+1, right-1, lo+d, hi, new_rhs_dests_bottom, neighbors);
    }
}

as_waksman_topology generate_as_waksman_topology(const size_t num_packets)
{
    assert!(num_packets > 1);
    const size_t width = as_waksman_num_columns(num_packets);

    as_waksman_topology neighbors(width, std::vector<std::pair<size_t, size_t> >(num_packets, std::make_pair<size_t, size_t>(-1, -1)));

    std::vector<size_t> rhs_dests(num_packets);
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
size_t as_waksman_get_canonical_row_idx(const size_t row_offset, const size_t row_idx)
{
    /* translate back relative to row_offset, clear LSB, and then translate forward */
    return (((row_idx - row_offset) & ~1) + row_offset);
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
bool as_waksman_get_switch_setting_from_top_bottom_decision(const size_t row_offset, const size_t packet_idx, const bool use_top)
{
    const size_t row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
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
bool as_waksman_get_top_bottom_decision_from_switch_setting(const size_t row_offset, const size_t packet_idx, const bool switch_setting)
{
    const size_t row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
    return (row_idx == packet_idx) ^ switch_setting;
}

/**
 * Given an output wire of a RHS switch, compute and return the output
 * position of the other wire also connected to this switch.
 */
size_t as_waksman_other_output_position(const size_t row_offset, const size_t packet_idx)
{
    const size_t row_idx = as_waksman_get_canonical_row_idx(row_offset, packet_idx);
    return (1 - (packet_idx - row_idx)) + row_idx;
}

/**
 * Given an input wire of a LHS switch, compute and return the input
 * position of the other wire also connected to this switch.
 */
size_t as_waksman_other_input_position(const size_t row_offset, const size_t packet_idx)
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
void as_waksman_route_inner(const size_t left,
                            const size_t right,
                            const size_t lo,
                            const size_t hi,
                            const integer_permutation &permutation,
                            const integer_permutation &permutation_inv,
                            as_waksman_routing &routing)
{
    if left > right
    {
        return;
    }

    const size_t subnetwork_size = (hi - lo + 1);
    const size_t subnetwork_width = as_waksman_num_columns(subnetwork_size);
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
        integer_permutation new_permutation(lo, hi);
        integer_permutation new_permutation_inv(lo, hi);
        std::vector<bool> lhs_routed(subnetwork_size, false); /* offset by lo, i.e. lhs_routed[packet_idx-lo] is set if packet packet_idx is routed */

        size_t to_route;
        size_t max_unrouted;
        bool route_left;

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
                const size_t rhs_switch = as_waksman_get_canonical_row_idx(lo, permutation.get(hi));
                const bool rhs_switch_setting = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation.get(hi), false);
                routing[right][rhs_switch] = rhs_switch_setting;
                size_t tprime = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, false);
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
            /**
             * EVEN CASE: the bottom-most switch is fixed to a constant
             * straight setting. So we route wire hi accordingly.
             */
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
                const size_t lhs_switch = as_waksman_get_canonical_row_idx(lo, to_route);
                if routing[left].find(lhs_switch) == routing[left].end()
                {
                    routing[left][lhs_switch] = false;
                }
                const bool lhs_switch_setting = routing[left][lhs_switch];
                const bool use_top = as_waksman_get_top_bottom_decision_from_switch_setting(lo, to_route, lhs_switch_setting);
                const size_t t = as_waksman_switch_output(subnetwork_size, lo, lhs_switch, use_top);
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
                    const size_t rhs_switch = as_waksman_get_canonical_row_idx(lo, permutation.get(to_route));
                    /**
                     * We know that the corresponding switch on the right-hand side
                     * cannot be set, so we set it according to the incoming wire.
                     */
                    assert!(routing[right].find(rhs_switch) == routing[right].end());
                    routing[right][rhs_switch] = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation.get(to_route), use_top);
                    const size_t tprime = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, use_top);
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
                const size_t rhs_switch = as_waksman_get_canonical_row_idx(lo, to_route);
                const size_t lhs_switch = as_waksman_get_canonical_row_idx(lo, permutation_inv.get(to_route));
                assert!(routing[right].find(rhs_switch) != routing[right].end());
                const bool rhs_switch_setting = routing[right][rhs_switch];
                const bool use_top = as_waksman_get_top_bottom_decision_from_switch_setting(lo, to_route, rhs_switch_setting);
                const bool lhs_switch_setting = as_waksman_get_switch_setting_from_top_bottom_decision(lo, permutation_inv.get(to_route), use_top);

                /* The value on the left-hand side is either the same or not set. */
                auto it = routing[left].find(lhs_switch);
                assert!(it == routing[left].end() || it->second == lhs_switch_setting);
                routing[left][lhs_switch] = lhs_switch_setting;

                const size_t t = as_waksman_switch_input(subnetwork_size, lo, rhs_switch, use_top);
                const size_t tprime = as_waksman_switch_output(subnetwork_size, lo, lhs_switch, use_top);
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

        const size_t d = as_waksman_top_height(subnetwork_size);
        const integer_permutation new_permutation_upper = new_permutation.slice(lo, lo + d - 1);
        const integer_permutation new_permutation_lower = new_permutation.slice(lo + d, hi);

        const integer_permutation new_permutation_inv_upper = new_permutation_inv.slice(lo, lo + d - 1);
        const integer_permutation new_permutation_inv_lower = new_permutation_inv.slice(lo + d, hi);

        as_waksman_route_inner(left+1, right-1, lo, lo + d - 1, new_permutation_upper, new_permutation_inv_upper, routing);
        as_waksman_route_inner(left+1, right-1, lo + d, hi, new_permutation_lower, new_permutation_inv_lower, routing);
    }
}

as_waksman_routing get_as_waksman_routing(const integer_permutation &permutation)
{
    const size_t num_packets = permutation.len();
    const size_t width = as_waksman_num_columns(num_packets);

    as_waksman_routing routing(width);
    as_waksman_route_inner(0, width-1, 0, num_packets-1, permutation, permutation.inverse(), routing);
    return routing;
}

bool valid_as_waksman_routing(const integer_permutation &permutation, const as_waksman_routing &routing)
{
    const size_t num_packets = permutation.len();
    const size_t width = as_waksman_num_columns(num_packets);
    as_waksman_topology neighbors = generate_as_waksman_topology(num_packets);

    integer_permutation curperm(num_packets);

    for column_idx in 0..width
    {
        integer_permutation nextperm(num_packets);
        for packet_idx in 0..num_packets
        {
            size_t routed_packet_idx;
            if neighbors[column_idx][packet_idx].first == neighbors[column_idx][packet_idx].second
            {
                routed_packet_idx = neighbors[column_idx][packet_idx].first;
            }
            else
            {
                auto it = routing[column_idx].find(packet_idx);
                auto it2 = routing[column_idx].find(packet_idx-1);
                assert!((it != routing[column_idx].end()) ^ (it2 != routing[column_idx].end()));
                const bool switch_setting = if it != routing[column_idx].end() {it->second} else{it2->second};

                routed_packet_idx = if switch_setting {neighbors[column_idx][packet_idx].second} else{neighbors[column_idx][packet_idx].first};
            }

            nextperm.set(routed_packet_idx, curperm.get(packet_idx));
        }

        curperm = nextperm;
    }

    return (curperm == permutation.inverse());
}


