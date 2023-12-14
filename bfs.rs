use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;
pub struct Graph {
    pub adjacency_list: Vec<Vec<usize>>,
}
    
impl Graph {

    pub fn new(num_vertices: usize) -> Self {
        let adjacency_list = vec![Vec::new(); num_vertices];
        Graph {adjacency_list}
    }

    // Both graphs are undirected (no one-sided connections)
    pub fn add_undirected_edge(&mut self, a: usize, b: usize) {
        self.adjacency_list[a].push(b);
        self.adjacency_list[b].push(a);
    }

    pub fn bfs(& self, start_vertex: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.adjacency_list.len()];
    
        queue.push_back((start_vertex, 0));
        visited[start_vertex] = true;
        distances.insert(start_vertex, 0);
    
        while let Some((current_vertex, distance)) = queue.pop_front() {
            for &neighbor in &self.adjacency_list[current_vertex] {
                if !visited[neighbor] {
                    queue.push_back((neighbor, distance + 1));
                    visited[neighbor] = true;
                    distances.insert(neighbor, distance + 1);
                }
            }
        }
    
        distances
    }  

    // Determines the average distance between any two nodes in the graph
    pub fn avg_path_length(&self) -> f64 {
        let mut total_length = 0.0;
        let mut num_paths = 0;

        for start_vertex in 0..self.adjacency_list.len() {
            let distances = self.bfs(start_vertex);

            for &distance in distances.values() {
                total_length += distance as f64;
                num_paths += 1;
            }
        }

        if num_paths > 0 {
            total_length / num_paths as f64
        } else {
            0.0 
        }
    }

    pub fn bfs_max_distance(&self, start_vertex: usize) -> usize {
        let mut max_distance = 0;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.adjacency_list.len()];
        let mut distances = vec![0; self.adjacency_list.len()];

        queue.push_back(start_vertex);
        visited[start_vertex] = true;

        while let Some(current_vertex) = queue.pop_front() {
            for &neighbor in &self.adjacency_list[current_vertex] {
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor] = true;
                    distances[neighbor] = distances[current_vertex] + 1;
                    max_distance = max_distance.max(distances[neighbor]);
                }
            }
        }

        max_distance
    }

    pub fn furthest_points(&self) -> usize {
        let mut dist = 0;

        for start_node in 0..self.adjacency_list.len() {
            let individual_dist = self.bfs_max_distance(start_node);
            dist = dist.max(individual_dist);
        }

        dist
    }

    pub fn local_clustering_coefficient(&self, node: usize) -> f64 {
        let neighbors: HashSet<usize> = self.adjacency_list[node].iter().cloned().collect();
        let num_neighbors = neighbors.len();

        if num_neighbors < 2 {
            return 0.0; // Clustering coefficient is undefined for nodes with fewer than 2 neighbors
        }

        let mut connected_neighbors = 0;

        for &neighbor1 in &neighbors {
            for &neighbor2 in &neighbors {
                if neighbor1 != neighbor2 && self.check_neighbors(neighbor1, neighbor2) {
                    connected_neighbors += 1;
                }
            }
        }

        let clustering_coefficient = connected_neighbors as f64 / (num_neighbors * (num_neighbors - 1)) as f64;

        clustering_coefficient
    }

    fn check_neighbors(&self, node1: usize, node2: usize) -> bool {
        self.adjacency_list[node1].contains(&node2) || self.adjacency_list[node2].contains(&node1)
    }

}