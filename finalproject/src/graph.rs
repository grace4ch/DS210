// graph.rs
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::io;
use std::io::BufRead;

// defining the Graph struct with adjacency list and in-degree list
pub struct Graph {
    pub adjacency_list: HashMap<i32, HashSet<i32>>,
    pub in_degree_list: HashMap<i32, usize>,
}

impl Graph {
    // a new Graph object
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            in_degree_list: HashMap::new(),
        }
    }

    // method to add an edge from src to dest
    pub fn add_edge(&mut self, src: i32, dest: i32) {
        // destination in the adjacency list of the source
        self.adjacency_list.entry(src).or_insert_with(HashSet::new).insert(dest);
        // the in-degree count for the destination node
        *self.in_degree_list.entry(dest).or_insert(0) += 1;
    }

    // method for the total number of edges in the graph
    pub fn number_of_edges(&self) -> usize {
        self.adjacency_list.values().map(|edges| edges.len()).sum()
    }

    // method for the total number of unique nodes in the graph
    pub fn number_of_nodes(&self) -> usize {
        let mut unique_nodes = HashSet::new();
        for (src, dests) in self.adjacency_list.iter() {
            unique_nodes.insert(src);
            for dest in dests {
                unique_nodes.insert(dest);
            }
        }
        unique_nodes.len()
    }

    // summary of the graph
    pub fn print_summary(&self) {
        println!("Number of nodes: {}", self.number_of_nodes());
        println!("Number of edges: {}", self.number_of_edges());
        for (node, edges) in self.adjacency_list.iter().take(5) {
            println!("Node {} has connections to: {:?}", node, edges);
        }
    }
}

// parsing a dataset from a file and create a Graph
pub fn parse_dataset(file_path: &str) -> io::Result<Graph> {
    let path = Path::new(file_path);
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut graph = Graph::new();
    // for all the unique nodes
    let mut all_nodes = HashSet::new(); 

    // iterate over each line in the file
    for line in reader.lines() {
        let line = line?;
        let nodes: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // representing an edge knowing that each line has two nodes. 
        if nodes.len() == 2 {
            let src = nodes[0];
            let dest = nodes[1];
            graph.add_edge(src, dest);
            all_nodes.insert(src);
            all_nodes.insert(dest);
        }
    }

    // ensure all nodes are in the adjacency list, even if they have no edges
    for &node in all_nodes.iter() {
        graph.adjacency_list.entry(node).or_insert_with(HashSet::new);
    }

    Ok(graph)
}
