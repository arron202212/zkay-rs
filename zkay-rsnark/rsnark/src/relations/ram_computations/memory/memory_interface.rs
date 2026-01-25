
// Declaration of interfaces for a memory interface.

use crate::relations::ram_computations::rams::ram_params::ram_input_tape;
use crate::common::data_structures::merkle_tree::merkle_authentication_path_type;

/**
 * A function from addresses to values that represents a memory's contents.
 */
pub type memory_contents = std::collections::BTreeMap<usize, usize>;

/**
 * A memory interface is a virtual pub struct for specifying and maintaining a memory.
 *
 * A memory is parameterized by two quantities:
 * - num_addresses (which specifies the number of addresses); and
 * - value_size (which specifies the number of bits stored at each address).
 *
 * The methods get_val and set_val can be used to load and store values.
 */
#[derive(Default,Clone)]
pub struct memory_base<T:Default+Clone> {
    pub num_addresses: usize,
    pub value_size: usize,
    pub t: T,
}
impl<T:Default+Clone> memory_base<T> {
    pub fn new(num_addresses: usize, value_size: usize, t: T) -> Self {
        Self {
            num_addresses,
            value_size,
            t,
        }
    }
    // memory_interface(num_addresses:usize , value_size:usize,  contents_as_vector:Vec<usize>);
    // memory_interface(num_addresses:usize, value_size:usize, contents: memory_contents);
}
pub trait memory_interface:Default+Clone {
   
    fn num_addresses(&self)-> usize{
    0}
    fn  value_size(&self) ->usize{0}
    fn get_path(&self, address: usize) -> merkle_authentication_path_type{
    vec![]
    }
    fn get_value(&self, address: usize) -> usize;
    fn set_value(&mut self, address: usize, value: usize);
}


