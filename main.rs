mod bfs;
mod graph;
use crate::graph::read_file;
fn main() {
    
    let max_nodes: usize = 15000;

    let fb_file_path = "C:\\Users\\mamar\\DS210 HW\\210 Final Project\\facebook_combined.txt";
    match read_file(fb_file_path, max_nodes) {
        Ok(graph1) => {
            let avgdist_fb = graph1.avg_path_length();
            println!("Average distance between two nodes for Facebook: {:.3}", avgdist_fb);

            let max_distance_fb = graph1.furthest_points();
            println!("Minimum distance between the two furthest nodes: {}", max_distance_fb);

            let mut cluster_coef_fb = 0.0;
            for node in 0..graph1.adjacency_list.len() {
                cluster_coef_fb += graph1.local_clustering_coefficient(node);
            }
            println!("Average local clustering coefficient: {:.3}", cluster_coef_fb / graph1.adjacency_list.len() as f64);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    println!();

    let gh_file_path = "C:\\Users\\mamar\\DS210 HW\\210 Final Project\\musae_git_edges.txt";
    match read_file(gh_file_path, max_nodes) {
        Ok(graph2) => {
            let avgdist_gh = graph2.avg_path_length();
            println!("Average distance between two nodes for GitHub: {:.3}", avgdist_gh);

            let max_distance_gh = graph2.furthest_points();
            println!("Minimum distance between the two furthest nodes: {}", max_distance_gh);

            let mut cluster_coef_gh = 0.0;
            for node in 0..graph2.adjacency_list.len() {
                cluster_coef_gh += graph2.local_clustering_coefficient(node);
            }
            println!("Average local clustering coefficient: {:.3}", cluster_coef_gh / graph2.adjacency_list.len() as f64);
        }
        Err(err) => eprintln!("Error: {}", err),
    }

}

#[allow(unused_imports)]
use crate::bfs::Graph;

#[test]
    fn test_bfs() {
        let graph = Graph {
            adjacency_list: vec![
                vec![1, 2], // interpretation: node 0 is connected to nodes 1 and 2
                vec![0, 2, 5],
                vec![0, 1, 3, 4],
                vec![2, 4],
                vec![2, 3],
                vec![1],
            ],
        };

        // check distances from node 0
        let distances = graph.bfs(0);
        assert_eq!(distances[&0], 0);
        assert_eq!(distances[&1], 1);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 2);
        assert_eq!(distances[&4], 2);
        assert_eq!(distances[&5], 2);
    }

    #[test]
    fn test_avg_path_length() {
        let graph = Graph {
            adjacency_list: vec![
                vec![1, 2],
                vec![0, 2, 5],
                vec![0, 1, 3, 4],
                vec![2, 4],
                vec![2, 3],
                vec![1],
            ],
        };

        let avg_length = graph.avg_path_length();
        assert_eq!(avg_length, (50.0 / 36.0)); // Sum of the distances between each pair of nodes / the total number of paths 
    }

    #[test]
    fn test_furthest_points() {
        let graph = Graph {
            adjacency_list: vec![
                vec![1, 2],
                vec![0, 2, 5],
                vec![0, 1, 3, 4],
                vec![2, 4],
                vec![2, 3],
                vec![1]
            ],
        };

        let max_distance = graph.furthest_points();
        assert_eq!(max_distance, 3);
    }
