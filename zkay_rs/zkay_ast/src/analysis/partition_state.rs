// from __future__ import annotations
// from typing import Set, Dict, Optional, Generic, TypeVar

// T = TypeVar('T')
// use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PartitionState<T: Ord> {
    pub _partitions: BTreeMap<i32, BTreeSet<T>>,
    pub _next_unused: i32,
}
// class PartitionState(Generic[T]):
// """
// Supports operations on partitions

// * insert: create a new partition with a single element
// * merge: merge partitions
// * ...
// """
impl<
    T: std::fmt::Display
        + std::clone::Clone
        + std::hash::Hash
        + std::cmp::Eq
        + std::cmp::Ord
        + crate::ast::Immutable
        + std::fmt::Debug,
> PartitionState<T>
{
    pub fn new() -> Self {
        Self {
            _partitions: BTreeMap::new(),
            _next_unused: 0,
        }
    }

    pub fn insert(&mut self, x: T) {
        self._insert_partition(BTreeSet::from([x]));
    }

    fn _insert_partition(&mut self, p: BTreeSet<T>) {
        self._partitions.insert(self._next_unused, p);
        self._next_unused += 1;
    }
    // """
    // Return index for element x.

    // :param x:
    // :return: the index of the partition containing x
    // """
    pub fn get_index(&self, x: &T) -> Option<i32> {
        // println!("===get_index====x======{:?}",x);
        for (k, p) in &self._partitions {
            // println!("===get_index===={:?}======{:?}",p.iter().map(|t|t.to_string()).collect::<Vec<_>>(),x.to_string());
            //  println!("===get_index===={:?}=====",p);
            let pp = p.iter().map(|t| t.to_string()).collect::<BTreeSet<_>>();
            if pp.contains(&x.to_string()) {
                return Some(*k);
            }
        }
        println!("===get_index======len no===={:?}", self._partitions.len());
        None
    }

    pub fn has(&self, x: &T) -> bool {
        self.get_index(x).is_some()
    }

    pub fn same_partition(&self, x: &T, y: &T) -> bool {
        if x == y {
            return true;
        }

        // get x
        let xp = self.get_index(x);
        if xp.is_none() {
            return false;
        }
        // get y
        let yp = self.get_index(y);
        if yp.is_none() {
            return false;
        }
        // compare
        xp == yp
    }

    pub fn merge(&mut self, x: &T, y: &T) {
        // locate
        let xp_key = self.get_index(x).unwrap();
        let yp_key = self.get_index(y).unwrap();

        if xp_key == yp_key {
            // merging not necessary
            return;
        }

        // remove y
        let yp = self._partitions.remove(&yp_key).unwrap();

        // insert y
        self._partitions.entry(xp_key).and_modify(|v| {
            *v = (*v).union(&yp).cloned().collect();
        });
    }
    // """
    // Removes x from its partition

    // :param x:
    // :return:
    // """
    pub fn remove(&mut self, x: &T) {
        // locate
        let xp_key = self.get_index(x);
        assert!(xp_key.is_some(), "element {x} not found");
        let xp_key = xp_key.unwrap();
        // remove x
        self._partitions.entry(xp_key).and_modify(|v| {
            v.remove(x);
        });

        // potentially remove whole partition
        if self._partitions[&xp_key].is_empty() {
            self._partitions.remove(&xp_key);
        }
    }

    // """
    // Moves x to the partition of y

    // :param x:
    // :param y:
    // """
    pub fn move_to(&mut self, x: &T, y: &T) {
        if self.same_partition(x, y) {
            // no action necessary
            return;
        }

        // remove
        self.remove(x);

        // locate y
        let yp_key = self.get_index(y).unwrap();

        // insert x
        self._partitions.entry(yp_key).and_modify(|v| {
            v.insert(x.clone());
        });
    }

    // """
    // Moves x to a fresh partition

    // :param x:
    // """
    pub fn move_to_separate(&mut self, x: &T) {
        // remove
        self.remove(x);

        // insert
        self.insert(x.clone());
    }

    pub fn separate_all(&self) -> PartitionState<T> {
        let mut s = PartitionState::<T>::new();
        for p in self._partitions.values() {
            // Side effects do not affect the aliasing of final values
            let mut immutable_vals = BTreeSet::new();
            for x in p {
                if x.is_immutable() {
                    immutable_vals.insert(x.clone());
                } else {
                    s.insert(x.clone());
                }
            }
            if !immutable_vals.is_empty() {
                s._insert_partition(immutable_vals);
            }
        }
        s
    }

    // """
    // Combine two states.
    // Overlaps in partitions between self and other will be preserved.
    // e.g. if self contains (a, b, c), (x) and other contains (a, b), (c, x), new state will contain (a, b), (c), (x)

    // :param other: other state, must contain the same values as self (partitions can be different)
    // :return: joined state
    // """
    pub fn join(&self, other: &PartitionState<T>) -> PartitionState<T> {
        let mut s = PartitionState::<T>::new();

        // Collect all values
        let my_vals = self
            ._partitions
            .values()
            .flat_map(|subset| subset.iter().cloned().collect::<Vec<_>>())
            .collect::<BTreeSet<_>>();
        let other_vals = other
            ._partitions
            .values()
            .flat_map(|subset| subset.iter().cloned().collect::<Vec<_>>())
            .collect::<BTreeSet<_>>();
        // println!("===my_vals.len(),other_vals========================={:?},{:?}",my_vals.iter().map(|x|x.to_string()).collect::<Vec<_>>(),other_vals.iter().map(|x|x.to_string()).collect::<Vec<_>>());
        assert!(
            my_vals.symmetric_difference(&other_vals).count() == 0,
            "joined branches do not contain the same values"
        );
        let values_in_both: BTreeSet<_> = my_vals.intersection(&other_vals).collect();

        let mut new_parts = BTreeSet::new();
        for val in values_in_both {
            let my_part = self._partitions[&self.get_index(val).unwrap()].clone();
            let other_part = other._partitions[&other.get_index(val).unwrap()].clone();

            let shared_elems: BTreeSet<_> = my_part.intersection(&other_part).cloned().collect();
            new_parts = new_parts.union(&shared_elems).cloned().collect();
        }

        s._insert_partition(new_parts);
        s
    }
    pub fn codes(&self) -> Vec<String> {
        self._partitions
            .values()
            .flat_map(|subset| subset.iter().map(|x| x.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
    // """
    // Create a shallow copy of the partition state.

    // :param project: (iterator) if not None, only keep entries that are in project
    // :return:
    // """
    pub fn copy(&self, project: Option<BTreeSet<T>>) -> PartitionState<T> {
        let mut c = PartitionState::<T>::new();
        c._next_unused = self._next_unused;
        for (k, p) in &self._partitions {
            // shallow copy
            let kept: BTreeSet<T> = p
                .iter()
                .filter(|x| project.is_none() || project.as_ref().unwrap().contains(x))
                .cloned()
                .collect();
            if !kept.is_empty() {
                c._partitions.insert(*k, kept);
            }
        }
        c
    }
}

use std::fmt;
impl<T: std::fmt::Display + std::cmp::Ord> fmt::Display for PartitionState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ps: Vec<_> = self
            ._partitions
            .values()
            .flat_map(|p| {
                let mut s: Vec<_> = p.iter().map(|e| e.to_string()).collect();
                s.sort();
                s
            })
            .collect();
        ps.sort();
        ps.concat();
        write!(f, "{:?}", ps)
    }
}
