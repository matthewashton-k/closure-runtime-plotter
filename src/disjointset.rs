use std::collections::HashMap;

/// A DisjointSetHashMap implementation of the DisjointSet trait.
/// This implementation uses a HashMap for storing element relationships
/// and performs path compression to optimize the operations.
#[derive(Clone)]
pub struct DisjointSetHashMap<E> {
    /// HashMap for storing the elements and their corresponding parent elements
    backing_map: HashMap<E, E>,
}

impl<E: Eq + std::hash::Hash + Clone> DisjointSetHashMap<E> {
    /// Constructs an empty DisjointSetHashMap.
    pub fn new() -> DisjointSetHashMap<E> {
        DisjointSetHashMap { backing_map: HashMap::new() }
    }

    /// Creates a new set containing the given element.
    /// If the element is already in the disjoint set, this method does nothing.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to create a set for.
    pub fn make_set(&mut self, element: E) {
        if self.backing_map.contains_key(&element) {
            return;
        }
        // Store the element as its own parent
        self.backing_map.insert(element.clone(), element.clone());
    }

    /// Returns the representative element of the set containing the given element.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to find the representative for.
    ///
    /// # Returns
    ///
    /// The representative element of the set.
    pub fn get_representative(&mut self, element: E) -> E {
        return self.recursive_get_representative(element.clone());
    }

    /// Unions the sets containing the given elements.
    ///
    /// # Arguments
    ///
    /// * `e1` - The first element.
    /// * `e2` - The second element.
    ///
    /// # Returns
    ///
    /// `true` if the union was successful, `false` otherwise.
    pub fn union(&mut self, e1: E, e2: E) -> bool {
        let n1 = self.backing_map.get(&e1).cloned(); // o1
        let n2 = self.backing_map.get(&e2).cloned(); // o1
        if n1.is_none() || n2.is_none() {
            return false;
        }
        // Get the representatives of the two elements
        let rep1 = self.get_representative(n1.unwrap());
        let rep2 = self.get_representative(n2.unwrap());

        // Update the parent of rep1 to rep2, effectively joining the sets
        self.backing_map.insert(rep1.clone(), rep2.clone());

        return true;
    }
}

impl<E: Eq + std::hash::Hash + Clone> DisjointSetHashMap<E> {
    /// Recursively finds the representative element of the given element.
    /// Performs path compression to optimize future calls.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to find the representative for.
    ///
    /// # Returns
    ///
    /// The representative element of the given element.
    fn recursive_get_representative(&mut self, element: E) -> E {
        if element == *self.backing_map.get(&element).unwrap() {
            return element.clone();
        }

        // Recursively find the representative of the parent element
        let parent = self.recursive_get_representative(self.backing_map.get(&element).unwrap().clone());
        // Path compression: update the parent of the current element to its representative
        self.backing_map.insert(element.clone(), parent.clone()); // o1
        return parent;
    }
}


