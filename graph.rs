use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use crate::bfs::Graph;

pub fn read_file(file_path: &str, max_nodes: usize) -> io::Result<Graph> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut edges = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        if index >= max_nodes {
            break; // stop reading file after 4000 nodes to prevent excessive run time
        }

        let line = line?;
        let mut iter = line.split_whitespace();

        if let (Some(a_str), Some(b_str)) = (iter.next(), iter.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse(), b_str.parse()) {
                edges.push((a, b));
            } else {
                eprintln!("Error parsing edge: {}", line);
            }
        }
    }

    let num_vertices = edges.iter().map(|&(a, b)| usize::max(a, b)).max().map_or(0, |x| x + 1);
    let mut graph = Graph::new(num_vertices);

    for &(a, b) in &edges {
        graph.add_undirected_edge(a, b);
    }

    Ok(graph)
}