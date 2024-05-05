// src/main.rs
mod sir_model;
mod person;

use sir_model::{SIRModel, PersonState, State};
use person::Interaction;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use rand::Rng;  // Import the Rng trait

fn main() {
    let num_nodes = 1000;
    let average_degree = 15;
    let mut rng = rand::thread_rng();

    let mut graph = Graph::<PersonState, Interaction, petgraph::Undirected>::default();

    for i in 0..num_nodes {
        graph.add_node(PersonState::new(i, if rng.gen_bool(0.1) { State::Infected } else { State::Susceptible }));
    }

    for idx in graph.node_indices() {
        let mut connections = 0;
        while connections < average_degree {
            let target_idx = NodeIndex::new(rng.gen_range(0..num_nodes));
            if idx != target_idx && graph.find_edge(idx, target_idx).is_none() {
                graph.add_edge(
                    idx, 
                    target_idx, 
                    Interaction { 
                        frequency: rng.gen_range(1..10),
                        strength: rng.gen_range(0.1..1.0),
                    }
                );
                connections += 1;
            }
        }
    }

    let time_steps = 100;
    let beta = 0.3;
    let gamma = 0.1;

    let mut sir_model = SIRModel::new(graph, time_steps, beta, gamma);
    sir_model.simulate();

    let degree_centrality = sir_model.calculate_degree_centrality();
    let betweenness_centrality = sir_model.calculate_betweenness_centrality();

    println!("Degree Centrality:");
    for (node, degree) in degree_centrality {
        println!("Node {}: Degree {}", node, degree);
    }

    println!("Betweenness Centrality:");
    for (node, centrality) in betweenness_centrality {
        println!("Node {}: Betweenness {:.2}", node, centrality);
    }
}
