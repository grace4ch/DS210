// algorithm.rs
use std::collections::{VecDeque, HashMap};
use crate::graph::Graph;
use rand::prelude::SliceRandom;

// bfs on the graph
pub fn bfs(graph: &Graph, start_node: i32) -> HashMap<i32, i32> {
    let mut distances = HashMap::new();  
    let mut queue = VecDeque::new();    

    // the start node distance and queue
    distances.insert(start_node, 0);
    queue.push_back(start_node);

    // iterate through the queue until it's empty
    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        // look at the neighbors of the current node
        if let Some(neighbors) = graph.adjacency_list.get(&current) {
            for &neighbor in neighbors {
                // add unvisited neighbors to the queue and distances
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }

    // return the map of distances from the start node
    distances
}

// check if two nodes are within a given number of degrees of separation
pub fn are_nodes_within_degrees_of_separation(graph: &Graph, start: i32, end: i32, max_degrees: i32) -> bool {
    let distances = bfs(graph, start);
    // check if the end node is reachable within the max degrees of separation
    distances.get(&end).map_or(false, |&d| d <= max_degrees)
}

// the average degree of separation between node pairs
pub fn calculate_six_degrees(graph: &Graph, num_pairs: usize, test_mode: bool) -> f64 {
    let mut rng = rand::thread_rng();
    let nodes: Vec<i32> = graph.adjacency_list.keys().cloned().collect();
    let mut total_distance = 0.0;
    let mut pairs_counted = 0;

    // iterate over a specified number of node pairs
    for _ in 0..num_pairs {
        let start_node = nodes.choose(&mut rng).unwrap();
        let end_node = if test_mode {
            start_node
        } else {
            nodes.choose(&mut rng).unwrap()
        };

        // calculate distances using BFS
        let distances = bfs(graph, *start_node);
        // sum the distances for reachable node pairs
        if let Some(&distance) = distances.get(end_node) {
            total_distance += distance as f64;
            pairs_counted += 1;
        }
    }

    // return the average degree of separation
    if pairs_counted == 0 { 0.0 } else { total_distance / pairs_counted as f64 }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_dataset;
    use std::path::PathBuf;

    // Function to get the path for the test dataset
    fn get_test_dataset_path() -> PathBuf {
        PathBuf::from("/Users/gracechong/Downloads/DSproject/finalproject/src/twitter_combined.txt")
    }

    // Test from the dataset
    #[test]
    fn test_graph_creation() {
        let test_dataset = get_test_dataset_path();
        let graph_result = parse_dataset(test_dataset.to_str().unwrap());

        assert!(graph_result.is_ok());
        let _graph = graph_result.unwrap();
    }

    // Test for known dataset
    #[test]
    fn test_bfs_on_test_dataset() {
        let test_dataset = get_test_dataset_path();
        let graph = parse_dataset(test_dataset.to_str().unwrap()).unwrap();

        let start_node = 12345; // a sample node
        let distances = bfs(&graph, start_node);

        assert_eq!(distances[&12345], 0); 
    }
}
