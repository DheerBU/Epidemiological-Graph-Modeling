// src/sir_model.rs
use crate::person::Interaction;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum State {
    Susceptible,
    Infected,
    Recovered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonState {
    pub id: usize,
    pub state: State,
}

impl PersonState {
    pub fn new(id: usize, state: State) -> Self {
        Self { id, state }
    }
}

pub struct SIRModel {
    pub graph: Graph<PersonState, Interaction, Undirected>,
    pub time_steps: usize,
    pub beta: f64,
    pub gamma: f64,
}

impl SIRModel {
    pub fn new(graph: Graph<PersonState, Interaction, Undirected>, time_steps: usize, beta: f64, gamma: f64) -> Self {
        Self {
            graph,
            time_steps,
            beta,
            gamma,
        }
    }

    pub fn simulate(&mut self) {
        let mut rng = thread_rng();
        let mut state_map: HashMap<usize, PersonState> = self.graph.node_indices()
            .map(|idx| (idx.index(), self.graph.node_weight(idx).unwrap().clone()))
            .collect();

        for _ in 0..self.time_steps {
            let mut new_state_map = state_map.clone();
            for (node_index, current_state) in state_map.iter() {
                let node_idx = NodeIndex::new(*node_index);
                let neighbors = self.graph.neighbors(node_idx);
                let infected_count = neighbors
                    .filter(|&n| state_map[&n.index()].state == State::Infected)
                    .count();

                match current_state.state {
                    State::Susceptible => {
                        let infection_probability = 1.0 - (1.0 - self.beta).powi(infected_count as i32);
                        if rng.gen::<f64>() < infection_probability {
                            new_state_map.get_mut(node_index).unwrap().state = State::Infected;
                        }
                    },
                    State::Infected => {
                        if rng.gen::<f64>() < self.gamma {
                            new_state_map.get_mut(node_index).unwrap().state = State::Recovered;
                        }
                    },
                    _ => {}
                }
            }
            state_map = new_state_map;
        }

        // Assign the final state back to the graph
        for (id, state) in &state_map {
            if let Some(node) = self.graph.node_weight_mut(NodeIndex::new(*id)) {
                node.state = state.state;
            }
        }
    }

    pub fn calculate_degree_centrality(&self) -> HashMap<usize, usize> {
        self.graph.node_indices()
            .map(|node| (node.index(), self.graph.edges(node).count()))
            .collect()
    }

    pub fn calculate_betweenness_centrality(&self) -> HashMap<usize, f64> {
        let mut centrality = HashMap::new();
        let node_indices: Vec<_> = self.graph.node_indices().collect();
    
        // Iterate over all nodes to calculate shortest paths
        for node in node_indices.iter() {
            let shortest_paths = dijkstra(
                &self.graph, 
                *node, 
                None, 
                |e| (e.weight().strength * 100.0) as u32  // Properly handle float by scaling and converting to u32
            );
    
            // Iterate over all nodes again to check if they appear in the shortest path from the current node
            for (target, _) in shortest_paths {
                if node != &target {
                    *centrality.entry(target.index()).or_insert(0.0) += 1.0;
                }
            }
        }
    
        let n = self.graph.node_count() as f64;
        let normalization_factor = if n <= 2.0 { 1.0 } else { (n-1.0) * (n-2.0) / 2.0 };
    
        // Normalize the betweenness centrality values
        for value in centrality.values_mut() {
            *value /= normalization_factor;
        }
    
        centrality
    }
}    


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_graph() -> Graph<PersonState, Interaction, Undirected> {
        let mut graph = Graph::<PersonState, Interaction, Undirected>::new_undirected();
        let node1 = graph.add_node(PersonState::new(1, State::Susceptible));
        let node2 = graph.add_node(PersonState::new(2, State::Infected));
        graph.add_edge(node1, node2, Interaction { frequency: 5, strength: 0.5 });
        graph
    }

    #[test]
    fn test_new_person_state() {
        let ps = PersonState::new(1, State::Susceptible);
        assert_eq!(ps.id, 1);
        assert_eq!(ps.state, State::Susceptible);
    }

    #[test]
    fn test_sir_model_initialization() {
        let graph = setup_test_graph();
        let sir_model = SIRModel::new(graph, 10, 0.3, 0.1);
        assert_eq!(sir_model.time_steps, 10);
        assert_eq!(sir_model.beta, 0.3);
        assert_eq!(sir_model.gamma, 0.1);
    }

}
