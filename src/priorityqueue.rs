/// Simple Max Priority Queue
/// struct which only holds onto
/// a 32-bit integer heap.
pub struct MaxPQ {
    heap: Vec<i32>,
}

impl MaxPQ {
    /// Returns an empty MaxPQ. Note that
    /// this implies the first element
    /// of the heap is initialized to 0.
    pub fn empty() -> MaxPQ {
        MaxPQ { heap: vec![0] }
    }

    /// Accepts a vector of integers to be
    /// used to form a heap and returns a
    /// new MaxPQ. 0 is prepended to the
    /// input vector before it's heapified.
    pub fn from(mut values: Vec<i32>) -> MaxPQ {
        values.insert(0, 0);
        let mut mpq = MaxPQ { heap: values };
        mpq.heapify();
        mpq
    }

    /// Internal swim method for moving an element
    /// of the heap up to its proper position.
    fn swim(&mut self, i: usize) {
        let mut ptr = i;

        while ptr > 1 {
            if self.heap[ptr] > self.heap[ptr / 2] {
                self.heap.swap(ptr, ptr / 2);
                ptr = ptr / 2;
            } else {
                break;
            }
        }
    }

    /// Internal sink method for moving an element
    /// of the heap down to its proper position.
    fn sink(&mut self, i: usize) {
        let mut ptr = i;

        while ptr * 2 < self.heap.len() {
            // Pick the greater of the two children to swap with
            let greater =
                if 2 * ptr + 1 == self.heap.len() || self.heap[2 * ptr + 1] < self.heap[2 * ptr] {
                    2 * ptr
                } else {
                    2 * ptr + 1
                };

            if self.heap[ptr] < self.heap[greater] {
                self.heap.swap(ptr, greater);
                ptr = greater;
            } else {
                break;
            }
        }
    }

    /// Insert a value into the MaxPQ.
    ///
    /// # Examples
    ///
    /// ```
    /// use rds::priorityqueue::*;
    ///
    /// let mut mpq = MaxPQ::from(vec![1, 2, 3, 4, 5]);
    /// mpq.insert(6);
    ///
    /// assert_eq!(6, mpq.del_max());
    /// ```
    pub fn insert(&mut self, i: i32) {
        self.heap.push(i);
        self.swim(self.heap.len() - 1)
    }

    /// Remove the current maximum value
    /// from the MaxPQ. The value is
    /// returned after it is removed from
    /// the PriorityQueue and after
    /// the remaining values are re-adjusted.
    /// # Examples
    ///
    /// ```
    /// use rds::priorityqueue::*;
    ///
    /// let mut mpq = MaxPQ::from(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(5, mpq.del_max());
    /// assert_eq!(4, mpq.del_max());
    /// ```
    pub fn del_max(&mut self) -> i32 {
        let max = self.heap[1];
        let last = self.heap.len() - 1;
        self.heap.swap(1, last);
        self.heap.remove(self.heap.len() - 1);
        self.sink(1);
        max
    }

    /// Internal method for heapifying
    /// the currently stored Vec<i32>.
    fn heapify(&mut self) {
        let mut ptr = (self.heap.len() - 1) / 2;

        while ptr >= 1 {
            self.sink(ptr);
            ptr -= 1;
        }
    }

    /// Retrieves and removes the k largest
    /// elements of the MaxPQ.
    ///
    /// # Examples
    ///
    /// ```
    /// use rds::priorityqueue::*;
    ///
    /// let mut mpq = MaxPQ::from(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(vec![5, 4, 3], mpq.top_k(3));
    /// ```
    pub fn top_k(&mut self, k: usize) -> Vec<i32> {
        let mut values = vec![0; k];

        for i in 0..k {
            values[i] = self.del_max();
        }

        values
    }

    /// Uses the internally stored heap to
    /// produce a vector containing all inserted
    /// values in sorted ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rds::priorityqueue::*;
    ///
    /// let values = vec![5, 3, 4, 1, 2];
    /// let sorted_values = MaxPQ::from(values).heapsort();
    ///
    /// assert_eq!(sorted_values, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn heapsort(&mut self) -> Vec<i32> {
        let mut values = vec![0; self.heap.len() - 1];

        for i in (0..self.heap.len() - 1).rev() {
            values[i] = self.del_max();
        }

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heapsort() {
        let mut mpq = MaxPQ::from(vec![5, 4, 3, 2, 1]);

        assert_eq!(vec![1, 2, 3, 4, 5], mpq.heapsort());
    }

    #[test]
    fn top_5() {
        let mut mpq = MaxPQ::from(vec![5, 4, 3, 2, 1]);

        assert_eq!(vec![5, 4, 3, 2, 1], mpq.top_k(5));
    }
}
