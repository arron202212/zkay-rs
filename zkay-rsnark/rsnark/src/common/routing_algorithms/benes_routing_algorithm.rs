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
//#ifndef BENES_ROUTING_ALGORITHM_HPP_
// #define BENES_ROUTING_ALGORITHM_HPP_

//
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;

use crate::common::data_structures::integer_permutation::integer_permutation;

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
type benes_topology = Vec<Vec<(usize, usize)>>;

/**
 * A routing assigns a bit to each switch in a Benes network.
 *
 * For a d-dimensional Benes network, the switch bits are stored in a
 * vector consisting of 2*d entries, and each entry contains 2^d bits.
 * That is, we have one switch per packet, but switch settings are not
 * independent.
 */
pub type benes_routing = Vec<bit_vector>;

/**
 * Return the number of (switch) columns in a Benes network for a given number of packets.
 *
 * For example:
 * - benes_num_columns(2) = 2,
 * - benes_num_columns(4) = 4,
 * - benes_num_columns(8) = 6,
 * and so on.
 */
// usize benes_num_columns(num_packets:usize);

// /**
//  * Return the topology of a Benes network for a given number of packets.
//  *
//  * See benes_topology (above) for details.
//  */
// benes_topology generate_benes_topology(num_packets:usize);

// /**
//  * Route the given permutation on a Benes network of suitable size.
//  */
// benes_routing get_benes_routing(permutation:&integer_permutation);

// /**
//  * Check if a routing "implements" the given permutation.
//  */
// bool valid_benes_routing(permutation:&integer_permutation, routing:&benes_routing);

// //#endif // BENES_ROUTING_ALGORITHM_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for functionality for routing on a Benes network.

//  See benes_routing_algorithm.hpp .

//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use  <cassert>

// use crate::common::routing_algorithms::benes_routing_algorithm;

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
pub fn benes_cross_edge_mask(dimension: usize, column_idx: usize) -> usize {
    return if column_idx < dimension {
        1usize << (dimension - 1 - column_idx)
    } else {
        1usize << (column_idx - dimension)
    };
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
pub fn benes_lhs_packet_destination(
    dimension: usize,
    column_idx: usize,
    row_idx: usize,
    use_top: bool,
) -> usize {
    let mask = benes_cross_edge_mask(dimension, column_idx);
    return if use_top {
        row_idx & !mask
    } else {
        row_idx | mask
    };
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
pub fn benes_rhs_packet_source(
    dimension: usize,
    column_idx: usize,
    row_idx: usize,
    use_top: bool,
) -> usize {
    return benes_lhs_packet_destination(dimension, column_idx - 1, row_idx, use_top); /* by symmetry */
}

/**
 * For a switch located at column_idx-th column and row_idx-th row,
 * return the switch setting that would route its packet using the top
 * subnetwork.
 */
pub fn benes_get_switch_setting_from_subnetwork(
    dimension: usize,
    column_idx: usize,
    row_idx: usize,
    use_top: bool,
) -> bool {
    return (row_idx != benes_lhs_packet_destination(dimension, column_idx, row_idx, use_top));
}

/**
 * A packet column_idx-th column and row_idx-th row of the routing
 * network has two destinations (see comment by
 * benes_cross_edge_mask), this returns row_idx' of the "cross"
 * destination.
 */
pub fn benes_packet_cross_destination(
    dimension: usize,
    column_idx: usize,
    row_idx: usize,
) -> usize {
    let mask = benes_cross_edge_mask(dimension, column_idx);
    return row_idx ^ mask;
}

/**
 * A packet column_idx-th column and row_idx-th row of the routing
 * network has two source packets that could give rise to it (see
 * comment by benes_cross_edge_mask), this returns row_idx' of the
 * "cross" source packet.
 */
pub fn benes_packet_cross_source(dimension: usize, column_idx: usize, packet_idx: usize) -> usize {
    return benes_packet_cross_destination(dimension, column_idx - 1, packet_idx); /* by symmetry */
}

pub fn benes_num_columns(num_packets: usize) -> usize {
    let dimension = log2(num_packets);
    assert!(num_packets == 1usize << dimension);

    return 2 * dimension;
}

pub fn generate_benes_topology(num_packets: usize) -> benes_topology {
    let num_columns = benes_num_columns(num_packets);
    let dimension = log2(num_packets);
    assert!(num_packets == 1usize << dimension);

    let mut result: Vec<Vec<_>> = Vec::with_capacity(num_columns);

    for column_idx in 0..num_columns {
        result[column_idx].resize(num_packets, (usize::MIN, usize::MIN));
        for packet_idx in 0..num_packets {
            result[column_idx][packet_idx].0 = packet_idx;
            result[column_idx][packet_idx].1 =
                benes_packet_cross_destination(dimension, column_idx, packet_idx);
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
pub fn route_benes_inner(
    dimension: usize,
    permutation: &integer_permutation,
    permutation_inv: &integer_permutation,
    column_idx_start: usize,
    column_idx_end: usize,
    subnetwork_offset: usize,
    subnetwork_size: usize,
    routing: &mut benes_routing,
) {
    // #ifdef DEBUG
    assert!(permutation.size() == subnetwork_size);
    assert!(permutation.is_valid());
    assert!(&permutation.inverse() == permutation_inv);
    //#endif

    if column_idx_start == column_idx_end {
        /* nothing to route */
        return;
    }
    let mut lhs_routed = vec![false; subnetwork_size]; /* adjusted by subnetwork_offset */

    let mut w = subnetwork_offset; /* left-hand-side vertex to be routed. */
    let mut last_unrouted = subnetwork_offset;

    let mut new_permutation =
        integer_permutation::new2(subnetwork_offset, subnetwork_offset + subnetwork_size - 1);
    let mut new_permutation_inv =
        integer_permutation::new2(subnetwork_offset, subnetwork_offset + subnetwork_size - 1);

    loop {
        /**
         * INVARIANT:
         * node w from left hand side can always be routed
         * to the right-hand side using the upper network.
         */
        /* route w to its target on RHS, wprime = pi[w], using upper network */
        let wprime = permutation.get(w);

        /* route (column_idx_start, w) forward via top subnetwork */
        routing[column_idx_start][w] =
            benes_get_switch_setting_from_subnetwork(dimension, column_idx_start, w, true);
        new_permutation.set(
            benes_lhs_packet_destination(dimension, column_idx_start, w, true),
            benes_rhs_packet_source(dimension, column_idx_end, wprime, true),
        );
        lhs_routed[w - subnetwork_offset] = true;

        /* route (column_idx_end, wprime) backward via top subnetwork */
        routing[column_idx_end - 1]
            [benes_rhs_packet_source(dimension, column_idx_end, wprime, true)] =
            benes_get_switch_setting_from_subnetwork(dimension, column_idx_end - 1, wprime, true);
        new_permutation_inv.set(
            benes_rhs_packet_source(dimension, column_idx_end, wprime, true),
            benes_lhs_packet_destination(dimension, column_idx_start, w, true),
        );

        /* now the other neighbor of wprime must be back-routed via the lower network, so get vprime, the neighbor on RHS and v, its target on LHS */
        let vprime = benes_packet_cross_source(dimension, column_idx_end, wprime);
        let v = permutation_inv.get(vprime);
        assert!(!lhs_routed[v - subnetwork_offset]);

        /* back-route (column_idx_end, vprime) using the lower subnetwork */
        routing[column_idx_end - 1]
            [benes_rhs_packet_source(dimension, column_idx_end, vprime, false)] =
            benes_get_switch_setting_from_subnetwork(dimension, column_idx_end - 1, vprime, false);
        new_permutation_inv.set(
            benes_rhs_packet_source(dimension, column_idx_end, vprime, false),
            benes_lhs_packet_destination(dimension, column_idx_start, v, false),
        );

        /* forward-route (column_idx_start, v) using the lower subnetwork */
        routing[column_idx_start][v] =
            benes_get_switch_setting_from_subnetwork(dimension, column_idx_start, v, false);
        new_permutation.set(
            benes_lhs_packet_destination(dimension, column_idx_start, v, false),
            benes_rhs_packet_source(dimension, column_idx_end, vprime, false),
        );
        lhs_routed[v - subnetwork_offset] = true;

        /* if the other neighbor of v is not routed, route it; otherwise, find the next unrouted node  */
        if !lhs_routed
            [benes_packet_cross_destination(dimension, column_idx_start, v) - subnetwork_offset]
        {
            w = benes_packet_cross_destination(dimension, column_idx_start, v);
        } else {
            while ((last_unrouted < subnetwork_offset + subnetwork_size)
                && lhs_routed[last_unrouted - subnetwork_offset])
            {
                last_unrouted += 1;
            }

            if last_unrouted == subnetwork_offset + subnetwork_size {
                break; /* all routed! */
            } else {
                w = last_unrouted;
            }
        }
    }

    let new_permutation_upper = new_permutation.slice(
        subnetwork_offset,
        subnetwork_offset + subnetwork_size / 2 - 1,
    );
    let new_permutation_lower = new_permutation.slice(
        subnetwork_offset + subnetwork_size / 2,
        subnetwork_offset + subnetwork_size - 1,
    );

    let new_permutation_inv_upper = new_permutation_inv.slice(
        subnetwork_offset,
        subnetwork_offset + subnetwork_size / 2 - 1,
    );
    let new_permutation_inv_lower = new_permutation_inv.slice(
        subnetwork_offset + subnetwork_size / 2,
        subnetwork_offset + subnetwork_size - 1,
    );

    /* route upper part */
    route_benes_inner(
        dimension,
        &new_permutation_upper,
        &new_permutation_inv_upper,
        column_idx_start + 1,
        column_idx_end - 1,
        subnetwork_offset,
        subnetwork_size / 2,
        routing,
    );

    /* route lower part */
    route_benes_inner(
        dimension,
        &new_permutation_lower,
        &new_permutation_inv_lower,
        column_idx_start + 1,
        column_idx_end - 1,
        subnetwork_offset + subnetwork_size / 2,
        subnetwork_size / 2,
        routing,
    );
}

pub fn get_benes_routing(permutation: &integer_permutation) -> benes_routing {
    let num_packets = permutation.size();
    let num_columns = benes_num_columns(num_packets);
    let dimension = log2(num_packets);

    let mut routing = vec![vec![false; num_packets]; num_columns];

    route_benes_inner(
        dimension,
        permutation,
        &permutation.inverse(),
        0,
        num_columns,
        0,
        num_packets,
        &mut routing,
    );

    return routing;
}

/* auxiliary function that is used in valid_benes_routing below */

pub fn route_by_benes<T: Default + Clone>(routing: &benes_routing, start: &Vec<T>) -> Vec<Vec<T>> {
    let num_packets = start.len();
    let num_columns = benes_num_columns(num_packets);
    let dimension = log2(num_packets);

    let mut res = vec![vec![T::default(); num_packets]; num_columns + 1];
    res[0] = start.clone();

    for column_idx in 0..num_columns {
        let mask = benes_cross_edge_mask(dimension, column_idx);

        for packet_idx in 0..num_packets {
            let next_packet_idx = if (routing[column_idx][packet_idx] == false) {
                packet_idx
            } else {
                packet_idx ^ mask
            };
            res[column_idx + 1][next_packet_idx] = res[column_idx][packet_idx].clone();
        }
    }

    return res;
}

pub fn valid_benes_routing(permutation: &integer_permutation, routing: &benes_routing) -> bool {
    let num_packets = permutation.size();
    let num_columns = benes_num_columns(num_packets);

    let mut input_packets = vec![0; num_packets];
    for packet_idx in 0..num_packets {
        input_packets[packet_idx] = packet_idx;
    }

    let routed_packets = route_by_benes(routing, &input_packets);

    for packet_idx in 0..num_packets {
        if routed_packets[num_columns][permutation.get(packet_idx)] != input_packets[packet_idx] {
            return false;
        }
    }

    return true;
}
