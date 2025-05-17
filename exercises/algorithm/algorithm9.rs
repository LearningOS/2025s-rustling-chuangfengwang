/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;

        self.heapify_up();
    }

    fn heapify_up(&mut self){
        let mut cur_index = self.count - 1;
        while cur_index > 0 {
            let parent_index = self.parent_idx(cur_index);
            if (self.comparator)(&self.items[cur_index], &self.items[parent_index]) {
                self.items.swap(cur_index, parent_index);
                cur_index = parent_index;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let mut cur_index = idx;
        while cur_index < self.count {
            cur_index = self.left_child_idx(cur_index);
        }
        cur_index
    }

    fn heapify_down(&mut self, mut index: usize) {        
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            
            let mut chosen = index;
            
            if left_child < self.count && (self.comparator)(&self.items[left_child], &self.items[chosen]) {
                chosen = left_child;
            } else if right_child < self.count && (self.comparator)(&self.items[right_child], &self.items[chosen]) {
                chosen = right_child;
            } else {
                break;
            }
            
            self.items.swap(index, chosen);
            index = chosen;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }
        self.items.swap(0, self.count - 1);
        let result = self.items.pop();
        self.count -= 1;

        self.heapify_down(0usize);
        return result;
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}

fn main() {
    let mut heap = MinHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
}