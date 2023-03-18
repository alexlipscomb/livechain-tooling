mod markov_chain;
use crate::markov_chain::{action::Action, edge::Edge, node::Node, MarkovChain};
use serde_json;
use serde_json::json;

fn main() {
    let mut mc = MarkovChain::new(None, None);
    let n1 = Node::new(1, None);
    let n2 = Node::new(2, None);
    let n3 = Node::new(3, None);
    mc.add_nodes(&[n1, n2, n3]);

    let a1 = Action::new(100, Some(json!(100.5)));
    let a2 = Action::new(200, Some(json!(200.5)));
    let a3 = Action::new(300, Some(json!(300.5)));

    mc.add_node_actions(1, &[a1]);
    mc.add_node_actions(2, &[a2, a3]);

    mc.add_edge(Edge::new(1, 2, 0.2));
    mc.add_edge(Edge::new(2, 3, 0.4));
    mc.add_edge(Edge::new(3, 1, 0.8));
    mc.add_edge(Edge::new(3, 2, 0.4));

    let serialized = serde_json::to_string(&mc).unwrap();

    println!("{}", serialized);
}
