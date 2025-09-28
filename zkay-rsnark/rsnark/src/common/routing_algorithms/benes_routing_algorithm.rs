/** @file
 *****************************************************************************

 Declaration of interfaces for functionality for routing on a Benes network.

 Routing is performed via the standard algorithm that computes a
 routing by first computing the switch settings for the left and right
 columns of the network and then recursively computing routings for
 the top half and the bottom half of the network (each of which is a
 Benes network of smaller size).

 References:

 \[Ben65]:
 "Mathematical theory of connecting networks and telephone traffic",
 Václav E. Beneš,
 Academic Press 1965

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef BENES_ROUTING_ALGORITHM_HPP_
#define BENES_ROUTING_ALGORITHM_HPP_

use  <vector>

use  <libff/common/utils.hpp>

use  <libsnark/common/data_structures/integer_permutation.hpp>

namespace libsnark {

/**
 * A data structure that stores the topology of a Benes network.
 *
 * For a given column index column_idx and packet index packet_idx,
 * benes_topology[column_idx][packet_idx] specifies the two possible
 * destinations where the packet_idx-th packet in the column_idx-th column
 * could be routed. This information is stored as a pair of indices, where:
 * - the first index denotes the destination when the switch is in "straight" mode, and
 * - the second index denotes the destination when the switch is in "cross" mode.
 *
 * (The topology has a very succinct description and can be easily
 * queried at an arbitrary position, see implementation of
 * generate_benes_topology for details.)
 */
type std::vector<std::vector<std::pair<size_t, size_t> > > benes_topology;

/**
 * A routing assigns a bit to each switch in a Benes network.
 *
 * For a d-dimensional Benes network, the switch bits are stored in a
 * vector consisting of 2*d entries, and each entry contains 2^d bits.
 * That is, we have one switch per packet, but switch settings are not
 * independent.
 */
type std::vector<libff::bit_vector> benes_routing;

/**
 * Return the number of (switch) columns in a Benes network for a given number of packets.
 *
 * For example:
 * - benes_num_columns(2) = 2,
 * - benes_num_columns(4) = 4,
 * - benes_num_columns(8) = 6,
 * and so on.
 */
size_t benes_num_columns(const size_t num_packets);

/**
 * Return the topology of a Benes network for a given number of packets.
 *
 * See benes_topology (above) for details.
 */
benes_topology generate_benes_topology(const size_t num_packets);

/**
 * Route the given permutation on a Benes network of suitable size.
 */
benes_routing get_benes_routing(const integer_permutation &permutation);

/**
 * Check if a routing "implements" the given permutation.
 */
bool valid_benes_routing(const integer_permutation &permutation, const benes_routing &routing);

} // libsnark

#endif // BENES_ROUTING_ALGORITHM_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for functionality for routing on a Benes network.

 See benes_routing_algorithm.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cassert>

use  <libsnark/common/routing_algorithms/benes_routing_algorithm.hpp>

namespace libsnark {

/**
 * Compute the mask for all the cross edges originating at a
 * particular column.
 *
 * Namely, the packet (column_idx, row_idx) (with column_idx <
 * num_columns) can be routed to two destinations:
 *
 * - (column_idx+1, row_idx), if the switch handling that packet is
 *    set to the "straight" setting, and
 *
 * - (column_idx+1, row_idx XOR benes_cross_edge_mask(dimension,
 *   column_idx)) if the switch handling that packet is set to "cross"
 *   setting.
 *
 * For example, all cross edges in the 0-th column flip the most
 * significant bit of row_idx.
 */
size_t benes_cross_edge_mask(const size_t dimension, const size_t column_idx)
{
    return (column_idx < dimension ? 1ul<<(dimension-1-column_idx) : 1ul<<(column_idx-dimension));
}

/**
 * Return the specified destination of packet of the left-hand side of
 * the routing network, based on the subnetwork (recall that each
 * packet has two possible destinations -- one at the top subnetwork
 * and one at the bottom subnetwork).
 *
 * That is for a packet located at column_idx-th column and row_idx-th
 * row, return:
 *
 * - row_idx' of the destination packet (column_idx+1, row_idx') at
 *   the top subnetwork (if use_top = true)
 *
 * - row_idx' of the destination packet (column_idx+1, row_idx') at
 *   the bottom subnetwork (if use_top = false)
 */
size_t benes_lhs_packet_destination(const size_t dimension, const size_t column_idx, const size_t row_idx, const bool use_top)
{
    const size_t mask = benes_cross_edge_mask(dimension, column_idx);
    return (use_top ? row_idx & ~mask : row_idx | mask);
}

/**
 * Return the specified source of packet of the right-hand side of the
 * routing network, based on the subnetwork (recall that each packet
 * has two possible source packets -- one at the top subnetwork and
 * one at the bottom subnetwork).
 *
 * That is for a packet located at column_idx-th column and row_idx-th
 * row, return:
 *
 * - row_idx' of the destination packet (column_idx-1, row_idx') at
 *   the top subnetwork (if use_top = true)
 *
 * - row_idx' of the destination packet (column_idx-1, row_idx') at
 *   the bottom subnetwork (if use_top = false)
 */
size_t benes_rhs_packet_source(const size_t dimension, const size_t column_idx, const size_t row_idx, const bool use_top)
{
    return benes_lhs_packet_destination(dimension, column_idx-1, row_idx, use_top); /* by symmetry */
}

/**
 * For a switch located at column_idx-th column and row_idx-th row,
 * return the switch setting that would route its packet using the top
 * subnetwork.
 */
bool benes_get_switch_setting_from_subnetwork(const size_t dimension, const size_t column_idx, const size_t row_idx, const bool use_top)
{
    return (row_idx != benes_lhs_packet_destination(dimension, column_idx, row_idx, use_top));
}

/**
 * A packet column_idx-th column and row_idx-th row of the routing
 * network has two destinations (see comment by
 * benes_cross_edge_mask), this returns row_idx' of the "cross"
 * destination.
 */
size_t benes_packet_cross_destination(const size_t dimension, const size_t column_idx, const size_t row_idx)
{
    const size_t mask = benes_cross_edge_mask(dimension, column_idx);
    return row_idx ^ mask;
}

/**
 * A packet column_idx-th column and row_idx-th row of the routing
 * network has two source packets that could give rise to it (see
 * comment by benes_cross_edge_mask), this returns row_idx' of the
 * "cross" source packet.
 */
size_t benes_packet_cross_source(const size_t dimension, const size_t column_idx, const size_t packet_idx)
{
    return benes_packet_cross_destination(dimension, column_idx-1, packet_idx); /* by symmetry */
}

size_t benes_num_columns(const size_t num_packets)
{
    const size_t dimension = libff::log2(num_packets);
    assert(num_packets == 1ul<<dimension);

    return 2*dimension;
}

benes_topology generate_benes_topology(const size_t num_packets)
{
    const size_t num_columns = benes_num_columns(num_packets);
    const size_t dimension = libff::log2(num_packets);
    assert(num_packets == 1ul<<dimension);

    benes_topology result(num_columns);

    for (size_t column_idx = 0; column_idx < num_columns; ++column_idx)
    {
        result[column_idx].resize(num_packets);
        for (size_t packet_idx = 0; packet_idx < num_packets; ++packet_idx)
        {
            result[column_idx][packet_idx].first = packet_idx;
            result[column_idx][packet_idx].second = benes_packet_cross_destination(dimension, column_idx, packet_idx);
        }
    }

    return result;
}

/**
 * Auxiliary function used in get_benes_routing (see below).
 *
 * The network from t_start to t_end is the part of the Benes network
 * that needs to be routed according to the permutation pi.
 *
 * The permutation
 * - pi maps [subnetwork_offset..subnetwork_offset+subnetwork_size-1] to itself, offset by subnetwork_offset, and
 * - piinv is the inverse of pi.
 */
void route_benes_inner(const size_t dimension,
                       const integer_permutation &permutation,
                       const integer_permutation &permutation_inv,
                       const size_t column_idx_start,
                       const size_t column_idx_end,
                       const size_t subnetwork_offset,
                       const size_t subnetwork_size,
                       benes_routing &routing)
{
#ifdef DEBUG
    assert(permutation.size() == subnetwork_size);
    assert(permutation.is_valid());
    assert(permutation.inverse() == permutation_inv);
#endif

    if (column_idx_start == column_idx_end)
    {
        /* nothing to route */
        return;
    }
    libff::bit_vector lhs_routed(subnetwork_size, false); /* adjusted by subnetwork_offset */

    size_t w = subnetwork_offset; /* left-hand-side vertex to be routed. */
    size_t last_unrouted = subnetwork_offset;

    integer_permutation new_permutation(subnetwork_offset, subnetwork_offset + subnetwork_size - 1);
    integer_permutation new_permutation_inv(subnetwork_offset, subnetwork_offset + subnetwork_size - 1);

    while (true)
    {
        /**
         * INVARIANT:
         * node w from left hand side can always be routed
         * to the right-hand side using the upper network.
         */

        /* route w to its target on RHS, wprime = pi[w], using upper network */
        size_t wprime = permutation.get(w);

        /* route (column_idx_start, w) forward via top subnetwork */
        routing[column_idx_start][w] = benes_get_switch_setting_from_subnetwork(dimension, column_idx_start, w, true);
        new_permutation.set(benes_lhs_packet_destination(dimension, column_idx_start, w, true), benes_rhs_packet_source(dimension, column_idx_end, wprime, true));
        lhs_routed[w-subnetwork_offset] = true;

        /* route (column_idx_end, wprime) backward via top subnetwork */
        routing[column_idx_end-1][benes_rhs_packet_source(dimension, column_idx_end, wprime, true)] = benes_get_switch_setting_from_subnetwork(dimension, column_idx_end-1, wprime, true);
        new_permutation_inv.set(benes_rhs_packet_source(dimension, column_idx_end, wprime, true), benes_lhs_packet_destination(dimension, column_idx_start, w, true));

        /* now the other neighbor of wprime must be back-routed via the lower network, so get vprime, the neighbor on RHS and v, its target on LHS */
        const size_t vprime = benes_packet_cross_source(dimension, column_idx_end, wprime);
        const size_t v = permutation_inv.get(vprime);
        assert(!lhs_routed[v-subnetwork_offset]);

        /* back-route (column_idx_end, vprime) using the lower subnetwork */
        routing[column_idx_end-1][benes_rhs_packet_source(dimension, column_idx_end, vprime, false)] = benes_get_switch_setting_from_subnetwork(dimension, column_idx_end-1, vprime, false);
        new_permutation_inv.set(benes_rhs_packet_source(dimension, column_idx_end, vprime, false), benes_lhs_packet_destination(dimension, column_idx_start, v, false));

        /* forward-route (column_idx_start, v) using the lower subnetwork */
        routing[column_idx_start][v] = benes_get_switch_setting_from_subnetwork(dimension, column_idx_start, v, false);
        new_permutation.set(benes_lhs_packet_destination(dimension, column_idx_start, v, false), benes_rhs_packet_source(dimension, column_idx_end, vprime, false));
        lhs_routed[v-subnetwork_offset] = true;

        /* if the other neighbor of v is not routed, route it; otherwise, find the next unrouted node  */
        if (!lhs_routed[benes_packet_cross_destination(dimension, column_idx_start, v) - subnetwork_offset])
        {
            w = benes_packet_cross_destination(dimension, column_idx_start, v);
        }
        else
        {
            while ((last_unrouted < subnetwork_offset + subnetwork_size) && lhs_routed[last_unrouted-subnetwork_offset])
            {
                ++last_unrouted;
            }

            if (last_unrouted == subnetwork_offset + subnetwork_size)
            {
                break; /* all routed! */
            }
            else
            {
                w = last_unrouted;
            }
        }
    }

    const integer_permutation new_permutation_upper = new_permutation.slice(subnetwork_offset, subnetwork_offset + subnetwork_size/2 - 1);
    const integer_permutation new_permutation_lower = new_permutation.slice(subnetwork_offset + subnetwork_size/2, subnetwork_offset + subnetwork_size - 1);

    const integer_permutation new_permutation_inv_upper = new_permutation_inv.slice(subnetwork_offset, subnetwork_offset + subnetwork_size/2 - 1);
    const integer_permutation new_permutation_inv_lower = new_permutation_inv.slice(subnetwork_offset + subnetwork_size/2, subnetwork_offset + subnetwork_size - 1);

    /* route upper part */
    route_benes_inner(dimension, new_permutation_upper, new_permutation_inv_upper, column_idx_start+1, column_idx_end-1,
                      subnetwork_offset, subnetwork_size/2, routing);

    /* route lower part */
    route_benes_inner(dimension, new_permutation_lower, new_permutation_inv_lower, column_idx_start+1, column_idx_end-1,
                      subnetwork_offset+subnetwork_size/2, subnetwork_size/2, routing);
}

benes_routing get_benes_routing(const integer_permutation &permutation)
{
    const size_t num_packets = permutation.size();
    const size_t num_columns = benes_num_columns(num_packets);
    const size_t dimension = libff::log2(num_packets);

    benes_routing routing(num_columns, libff::bit_vector(num_packets));

    route_benes_inner(dimension, permutation, permutation.inverse(), 0, num_columns, 0, num_packets, routing);

    return routing;
}

/* auxiliary function that is used in valid_benes_routing below */
template<typename T>
std::vector<std::vector<T> > route_by_benes(const benes_routing &routing, const std::vector<T> &start)
{
    const size_t num_packets = start.size();
    const size_t num_columns = benes_num_columns(num_packets);
    const size_t dimension = libff::log2(num_packets);

    std::vector<std::vector<T> > res(num_columns+1, std::vector<T>(num_packets));
    res[0] = start;

    for (size_t column_idx = 0; column_idx < num_columns; ++column_idx)
    {
        const size_t mask = benes_cross_edge_mask(dimension, column_idx);

        for (size_t packet_idx = 0; packet_idx < num_packets; ++packet_idx)
        {
            size_t next_packet_idx = (routing[column_idx][packet_idx] == false) ? packet_idx : packet_idx ^ mask;
            res[column_idx+1][next_packet_idx] = res[column_idx][packet_idx];
        }
    }

    return res;
}

bool valid_benes_routing(const integer_permutation &permutation, const benes_routing &routing)
{
    const size_t num_packets = permutation.size();
    const size_t num_columns = benes_num_columns(num_packets);

    std::vector<size_t> input_packets(num_packets);
    for (size_t packet_idx = 0; packet_idx < num_packets; ++packet_idx)
    {
        input_packets[packet_idx] = packet_idx;
    }

    const std::vector<std::vector<size_t> > routed_packets = route_by_benes(routing, input_packets);

    for (size_t packet_idx = 0; packet_idx < num_packets; ++packet_idx)
    {
        if (routed_packets[num_columns][permutation.get(packet_idx)] != input_packets[packet_idx])
        {
            return false;
        }
    }

    return true;
}

} // libsnark
