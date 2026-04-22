// Declaration of interfaces for a permutation of the integers in {self.min_element,...,self.max_element}.

use std::collections::HashSet;
#[derive(PartialEq)]
pub struct integer_permutation {
    pub contents: Vec<usize>, //offset by self.min_element

    pub min_element: usize,
    pub max_element: usize,
    
}


impl integer_permutation {
    pub fn new(size: usize) -> Self {
        
        Self {
            contents: vec![0; size],
            min_element: 0,
            max_element: size - 1,
        }
    }

    pub fn new2(min_element: usize, max_element: usize) -> Self {
        assert!(min_element <= max_element);
        let size = max_element - min_element + 1;
      
        Self {
            contents: vec![0; size],
            min_element,
            max_element,
        }
    }

    pub fn size(&self) -> usize {
        self.max_element - self.min_element + 1
    }

    pub fn set(&mut self, position: usize, value: usize) {
        assert!(self.min_element <= position && position <= self.max_element);
        self.contents[position - self.min_element] = value;
    }

    pub fn get(&self, position: usize) -> usize {
        assert!(self.min_element <= position && position <= self.max_element);
        self.contents[position - self.min_element]
    }

    pub fn is_valid(&self) -> bool {
        let mut elems = HashSet::new();

        for &el in &self.contents {
            if el < self.min_element || el > self.max_element || elems.contains(&el) {
                return false;
            }

            elems.insert(el);
        }

        true
    }

    pub fn inverse(&self) -> Self {
        let mut result = Self::new2(self.min_element, self.max_element);

        for position in self.min_element..=self.max_element {
            result.contents[self.contents[position - self.min_element] - self.min_element] =
                position;
        }

        // #ifdef DEBUG
        // assert!(result.is_valid());

        return result;
    }

    pub fn slice(&self, slice_min_element: usize, slice_max_element: usize) -> Self {
        assert!(
            self.min_element <= slice_min_element
                && slice_min_element <= slice_max_element
                && slice_max_element <= self.max_element
        );
        let mut result = Self::new2(slice_min_element, slice_max_element);
       
        result
            .contents
            .insert(0, self.contents[slice_min_element - self.min_element]);
       

        return result;
    }

    pub fn next_permutation(&self) -> bool {
        false
    }

    pub fn random_shuffle(&self) {
    }
}
