/** @file
 *****************************************************************************

 Declaration of interfaces for the memory load gadget.
 The gadget can be used to verify a memory load from a "delegated memory".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MEMORY_LOAD_GADGET_HPP_
// #define MEMORY_LOAD_GADGET_HPP_

use libsnark/gadgetlib1/gadgets/merkle_tree/merkle_tree_check_read_gadget;



template<typename FieldT, typename HashT>
using memory_load_gadget = merkle_tree_check_read_gadget<FieldT, HashT>;



//#endif // MEMORY_LOAD_GADGET_HPP_
