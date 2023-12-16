//algorithm2.rs
use crate::algorithm::bfs;
use crate::graph::Graph;
use rand::seq::IteratorRandom;

// average distance between nodes in the graph
pub fn calculate_average_distance(graph: &Graph, sample_size: usize) -> Option<f64> {
    let node_count = graph.adjacency_list.len();
    // none for empty graphs to avoid division by zero
    if node_count == 0 {
        return None;
    }

    // adjust the sample size if it's larger than the node count
    let actual_sample_size = sample_size.min(node_count);
    let mut rng = rand::thread_rng();

    // random sample nodes without replacement
    let sampled_nodes = graph.adjacency_list.keys()
        .choose_multiple(&mut rng, actual_sample_size);

    let mut total_distance = 0;
    let mut reachable_pairs = 0;

    // iterate through the sampled nodes and calculate distances
    for &node in sampled_nodes {
        let distances = bfs(graph, node);
        for &dist in distances.values() {
            if dist > 0 { // not self-distance
                total_distance += dist;
                reachable_pairs += 1;
            }
        }
    }

    // the average distance
    if reachable_pairs > 0 {
        Some(total_distance as f64 / reachable_pairs as f64)
    } else {
        // Return 0 if there are no reachable pairs
        Some(0.0)
    }
}

// testing 
#[cfg(test)]
mod tests {
    
    use super::calculate_average_distance;
    use crate::graph::Graph;

    #[test]
    fn test_average_distance() {
        let mut graph = Graph::new();
        
        // construct a known graph 
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        
        let expected_avg_distance = 2.0; // placeholder value for expected average distance

        // the average distance from a sample of nodes in the test graph
        let sample_size = 4; 
        let avg_distance = calculate_average_distance(&graph, sample_size).unwrap();

        assert!((avg_distance - expected_avg_distance).abs() < f64::EPSILON);
    }
}
