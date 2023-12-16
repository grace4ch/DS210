//main.rs
mod graph;
mod algorithm;
mod algorithm2;

use crate::algorithm::{are_nodes_within_degrees_of_separation, calculate_six_degrees};
use crate::algorithm2::calculate_average_distance;
use crate::graph::parse_dataset;
use std::io;

fn main() -> io::Result<()> {
    // parsing the dataset
    let graph_result = parse_dataset("/Users/gracechong/Downloads/DSproject/finalproject/src/twitter_combined.txt"); 

    // the result of the parsing
    match graph_result {
        Ok(graph) => {
            // printing nodes and edges
            graph.print_summary();

            // calculating the average distance between nodes in the graph
            if let Some(average_distance) = calculate_average_distance(&graph, 100) {
                println!("Average Distance: {:.2}", average_distance);
            } else {
                println!("Average Distance: Not applicable for an empty graph or isolated nodes.");
            }

            // checking for degree of separation with a sample from my dataset
            let start_node = 222261763; 
            let end_node = 88323281; 
            let max_degrees = 6;

            // checking if the two nodes are within degrees of separation 
            let are_connected = are_nodes_within_degrees_of_separation(&graph, start_node, end_node, max_degrees);
            println!(
                "Nodes {} and {} {} within {} degrees of separation.",
                start_node,
                end_node,
                if are_connected { "are" } else { "are not" },
                max_degrees
            );

            // calculating the average six degrees of separation (not my test case but it is for testing)
            let average_six_degrees = calculate_six_degrees(&graph, 100, true);
            println!("Average six degrees separation (test = true): {}", average_six_degrees);
            // calculating the average six degrees of separation for random nodes 
            let random_six_degrees = calculate_six_degrees(&graph, 100, false);
            println!("Average six degrees separation (test = false): {}", random_six_degrees);

        },
        Err(e) => {
            // error if the graph fails to create
            eprintln!("Failed to create graph: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
