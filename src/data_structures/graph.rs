use std::ops::Index;

pub trait Graph<I, N>: Index<I, Output = N> { 
    fn neighbors(&self, _: &I) -> Vec<I>;

    fn cost(&self, _: &I, _: &I) -> usize;

    fn contains(&self, _: &I) -> bool;
}
