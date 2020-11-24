use std::{
    collections::BinaryHeap,
    cmp::{
        PartialEq,
        Eq,
        Ord,
        Ordering,
    },
    mem::transmute,
};

pub struct Queue<T> {
    queue: Vec<T>,
    pub max_size: usize,
}

impl<T> Queue<T> {
    pub fn new(max_size: usize) -> Queue<T> {
        Queue {
            queue: Vec::new(),
            max_size, 
        }
    }

    pub fn push(&mut self, item: T) {
        if self.queue.len() == self.max_size {
            //remove the first item to make room
            self.pop();
            self.queue.push(item);
        } else {
            self.queue.push(item);
        }
    }

    pub fn pop(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.queue.iter()
    }
}
    
#[derive(Eq, PartialEq)]
pub struct MinPriorityItem<T> {
    pub item: T,
    cost: Cost,
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Cost {
    mantissa: u64,
    exponent: i16,
    sign: i8,
}

impl From<f64> for Cost {
    fn from(val: f64) -> Cost {
        let bits: u64 = unsafe { transmute(val) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        exponent -= 1023 + 52;
        Cost{ mantissa, exponent, sign }
    }
}

impl<T: Ord> Ord for MinPriorityItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap()
    }
}

impl<T: PartialOrd> PartialOrd for MinPriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost).map(|part| part.reverse())
    }
}

pub struct MinPriorityQueue<T> {
    heap: BinaryHeap<MinPriorityItem<T>>,
    max_size: usize,
}

impl<T: Eq + Ord> MinPriorityQueue<T> {
    pub fn new(max_size: usize) -> MinPriorityQueue<T> {
        MinPriorityQueue {
            heap: BinaryHeap::new(),
            max_size, 
        }
    }

    pub fn push(&mut self, item: T, cost: f64) {
        let min_item = MinPriorityItem { item, cost: cost.into() };

        if self.heap.len() == self.max_size {
            //remove the first item to make room
            self.pop();
            self.heap.push(min_item);
        } else {
            self.heap.push(min_item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|min| min.item)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn iter(&self) -> std::collections::binary_heap::Iter<MinPriorityItem<T>> {
        self.heap.iter()
    }
}
