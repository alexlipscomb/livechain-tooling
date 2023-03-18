use crate::markov_chain::{action::Action, edge::Edge, node::Node};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum MarkovChainError {
    StuckError,
    NodeDoesNotExistError,
    EdgeDoesNotExistError,
    ActionDoesNotExistError,
    NodeHasNoEdgesError,
    TransitionFailedError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovChain {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    current_node: Option<u32>,
}

impl MarkovChain {
    pub fn new(nodes: Option<Vec<Node>>, edges: Option<Vec<Edge>>) -> MarkovChain {
        MarkovChain {
            nodes: nodes.unwrap_or_default(),
            edges: edges.unwrap_or_default(),
            current_node: None,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_nodes(&mut self, nodes: &[Node]) {
        self.nodes.extend_from_slice(nodes);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn remove_node(&mut self, node_id: u32) {
        self.nodes.retain(|node| node.id != node_id);
    }

    pub fn remove_edge(&mut self, from_node_id: u32, to_node_id: u32) {
        self.edges
            .retain(|edge| !(edge.from == from_node_id && edge.to == to_node_id));
    }

    pub fn get_node(&self, node_id: u32) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == node_id)
    }

    pub fn get_edge(&self, from_node_id: u32, to_node_id: u32) -> Option<&Edge> {
        self.edges
            .iter()
            .find(|edge| edge.from == from_node_id && edge.to == to_node_id)
    }

    pub fn add_node_actions(&mut self, node_id: u32, actions: &[Action]) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id == node_id) {
            node.actions.extend_from_slice(actions);
        }
    }

    pub fn get_node_actions(&self, node_id: u32) -> Option<&Vec<Action>> {
        self.nodes
            .iter()
            .find(|node| node.id == node_id)
            .map(|node| &node.actions)
    }

    pub fn get_node_action(&self, node_id: u32, action_id: u32) -> Option<&Action> {
        self.nodes
            .iter()
            .find(|node| node.id == node_id)
            .and_then(|node| node.actions.iter().find(|action| action.id == action_id))
    }

    pub fn get_edge_from(&self, node_id: u32) -> Option<&u32> {
        self.edges
            .iter()
            .find(|edge| edge.from == node_id)
            .map(|edge| &edge.from)
    }

    pub fn get_edge_to(&self, node_id: u32) -> Option<&u32> {
        self.edges
            .iter()
            .find(|edge| edge.to == node_id)
            .map(|edge| &edge.to)
    }

    pub fn get_node_edges(&self, node_id: u32) -> Result<Vec<Edge>, MarkovChainError> {
        if !self.node_exists(node_id) {
            return Err(MarkovChainError::NodeDoesNotExistError);
        }

        let edges: Vec<Edge> = self
            .edges
            .iter()
            .filter(|edge| edge.from == node_id)
            .cloned()
            .collect();

        Ok(edges)
    }

    pub fn node_exists(&self, node_id: u32) -> bool {
        self.nodes.iter().any(|node| node.id == node_id)
    }

    pub fn edge_exists(&self, from_node_id: u32, to_node_id: u32) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.from == from_node_id && edge.to == to_node_id)
    }

    pub fn set_current_node(&mut self, node_id: u32) -> Result<(), MarkovChainError> {
        if let Some(_) = self.nodes.iter().find(|node| node.id == node_id) {
            self.current_node = Some(node_id);
            Ok(())
        } else {
            Err(MarkovChainError::NodeDoesNotExistError)
        }
    }

    pub fn get_current_node(&self) -> Option<u32> {
        self.current_node
    }

    pub fn next<'a>(&'a mut self) -> Result<(), MarkovChainError> {
        let mut rng = rand::thread_rng();
        if self.current_node.is_none() {
            return Err(MarkovChainError::NodeDoesNotExistError);
        }

        let current_node_id = self.current_node.unwrap();

        let edges = match self.get_node_edges(current_node_id) {
            Ok(edges) => edges,
            Err(_) => return Err(MarkovChainError::NodeHasNoEdgesError),
        };

        if edges.is_empty() {
            return Err(MarkovChainError::NodeHasNoEdgesError);
        }

        let mut total_weight = 0.0;
        for edge in edges.iter() {
            total_weight += edge.weight;
        }

        let mut random_weight = rng.gen_range(0.0..total_weight);
        for edge in edges.iter() {
            if random_weight < edge.weight {
                self.current_node = Some(edge.to);
                return Ok(());
            }
            random_weight -= edge.weight;
        }

        Err(MarkovChainError::TransitionFailedError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_nodes() -> Vec<Node> {
        vec![
            Node {
                id: 1,
                actions: vec![],
            },
            Node {
                id: 2,
                actions: vec![],
            },
            Node {
                id: 3,
                actions: vec![],
            },
        ]
    }

    fn create_test_edges() -> Vec<Edge> {
        vec![
            Edge {
                from: 1,
                to: 2,
                weight: 1.0,
            },
            Edge {
                from: 1,
                to: 3,
                weight: 1.0,
            },
            Edge {
                from: 2,
                to: 3,
                weight: 1.0,
            },
        ]
    }

    #[test]
    fn test_add_node() {
        let mut mc = MarkovChain::new(None, None);
        let node = Node {
            id: 1,
            actions: vec![],
        };
        mc.add_node(node.clone());
        assert_eq!(mc.get_node(1), Some(&node));
    }

    #[test]
    fn test_add_nodes() {
        let mut mc = MarkovChain::new(None, None);
        let nodes = create_test_nodes();
        mc.add_nodes(&nodes);
        assert_eq!(mc.get_node(1), Some(&nodes[0]));
        assert_eq!(mc.get_node(2), Some(&nodes[1]));
        assert_eq!(mc.get_node(3), Some(&nodes[2]));
    }

    #[test]
    fn test_remove_node() {
        let mut mc = MarkovChain::new(Some(create_test_nodes()), None);
        mc.remove_node(1);
        assert_eq!(mc.get_node(1), None);
    }

    #[test]
    fn test_add_edge() {
        let mut mc = MarkovChain::new(None, None);
        let edge = Edge {
            from: 1,
            to: 2,
            weight: 1.0,
        };
        mc.add_edge(edge.clone());
        assert_eq!(mc.get_edge(1, 2), Some(&edge));
    }

    #[test]
    fn test_remove_edge() {
        let mut mc = MarkovChain::new(None, Some(create_test_edges()));
        mc.remove_edge(1, 2);
        assert_eq!(mc.get_edge(1, 2), None);
    }

    #[test]
    fn test_set_current_node() {
        let mut mc = MarkovChain::new(Some(create_test_nodes()), None);

        assert_eq!(mc.set_current_node(1), Ok(()));
        assert_eq!(
            mc.set_current_node(4),
            Err(MarkovChainError::NodeDoesNotExistError)
        );

        match mc.get_current_node() {
            Some(id) => assert_eq!(id, 1),
            None => assert!(false),
        }
    }

    #[test]
    fn test_next() {
        let mut mc = MarkovChain::new(Some(create_test_nodes()), Some(create_test_edges()));
        assert_eq!(mc.next(), Err(MarkovChainError::NodeDoesNotExistError));

        mc.set_current_node(1).unwrap();
        assert!(mc.next().is_ok());
    }

    #[test]
    fn test_get_node_edges() {
        let mc = MarkovChain::new(Some(create_test_nodes()), Some(create_test_edges()));
        let edges = mc.get_node_edges(1).unwrap();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&Edge {
            from: 1,
            to: 2,
            weight: 1.0
        }));
        assert!(edges.contains(&Edge {
            from: 1,
            to: 3,
            weight: 1.0
        }));
    }

    #[test]
    fn test_next_edge_distribution() {
        let mut mc = MarkovChain::new(Some(create_test_nodes()), Some(create_test_edges()));
        mc.set_current_node(1).unwrap();

        let mut count_to_2 = 0;
        let mut count_to_3 = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            mc.next().unwrap();
            match mc.current_node {
                Some(2) => count_to_2 += 1,
                Some(3) => count_to_3 += 1,
                _ => panic!("Unexpected node"),
            }
            mc.set_current_node(1).unwrap();
        }

        let ratio_to_2 = count_to_2 as f64 / iterations as f64;
        let ratio_to_3 = count_to_3 as f64 / iterations as f64;
        let expected_ratio = 0.5;

        assert!((ratio_to_2 - expected_ratio).abs() < 0.05);
        assert!((ratio_to_3 - expected_ratio).abs() < 0.05);
    }
}
