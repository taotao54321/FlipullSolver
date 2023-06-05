use std::collections::BinaryHeap;

/// 最小の k 個のみを保持する max heap。
#[derive(Clone, Debug)]
pub struct BoundedHeap<T> {
    capacity: usize,
    heap: BinaryHeap<T>,
}

impl<T: Ord> BoundedHeap<T> {
    pub fn new(capacity: usize) -> Self {
        let heap = BinaryHeap::with_capacity(capacity + 1);

        Self { capacity, heap }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.heap.len() == self.capacity
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    /// `x` を追加する。
    /// 要素数が容量を超える場合、`x` も含めて最大の要素が削除される。
    pub fn insert(&mut self, x: T) {
        self.heap.push(x);

        if self.heap.len() > self.capacity {
            self.heap.pop().unwrap();
        }
    }

    pub fn into_inner(self) -> BinaryHeap<T> {
        self.heap
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bh = BoundedHeap::new(5);
        for x in [9, 3, 5, 1, 7, 0, 8, 6, 2, 4] {
            bh.insert(x);
        }
        assert_eq!(bh.into_sorted_vec(), [0, 1, 2, 3, 4]);
    }
}
