// Leetcode problem 703
// https://leetcode.com/problems/kth-largest-element-in-a-stream/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    size: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let size = k as usize;
        let mut heap = BinaryHeap::with_capacity(size + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > size {
                heap.pop();
            }
        }
        Self { size, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.size {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[test]
fn test() {
    let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(k.add(3), 4);
    assert_eq!(k.add(5), 5);
    assert_eq!(k.add(10), 5);
    assert_eq!(k.add(9), 8);
    assert_eq!(k.add(4), 8);
}

