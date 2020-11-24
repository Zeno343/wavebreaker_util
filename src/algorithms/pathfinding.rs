use std::collections::HashMap;

use crate::data_structures::{
    Graph,
    MinPriorityQueue,
};

pub fn find_path<N>(
    start: (usize, usize), 
    end: (usize, usize), 
    map: &dyn Graph<(usize, usize), N>
) -> Vec<(usize, usize)> {
    let mut frontier = MinPriorityQueue::new(std::usize::MAX);
    let mut reached = HashMap::new();
    let mut costs = HashMap::new();
    
    frontier.push(start, 0.0f64);
    reached.insert(start, None);
    costs.insert(start, 0.0f64);

    while frontier.len() > 0 {
        let frontier_item = frontier.pop().unwrap();
        
        let current_tile = frontier_item;
        if current_tile == end {
            break;
        }

        for next_tile in map.neighbors(&current_tile).into_iter() {
            let cost = 
                costs.get(&current_tile).unwrap() 
                + map.cost(&current_tile, &next_tile);
            if !costs.contains_key(&next_tile) 
                || cost < *costs.get(&next_tile).unwrap() 
            {
                costs.insert(next_tile, cost);
                frontier.push(next_tile, cost);
                reached.insert(next_tile, Some(current_tile));
            }
            
        }
    }

    let mut current_tile = end;
    let mut path = Vec::new();

    while current_tile != start {
        path.push(current_tile);

        current_tile = reached.get(&current_tile).unwrap().unwrap();
    }

    path
}
