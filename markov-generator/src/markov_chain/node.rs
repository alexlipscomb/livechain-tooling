use crate::markov_chain::action::Action;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Node {
    pub id: u32,
    pub actions: Vec<Action>,
}

impl Node {
    pub fn new(id: u32, actions: Option<Vec<Action>>) -> Node {
        let _actions = match actions {
            Some(actions) => actions,
            None => Vec::new(),
        };

        Node {
            id,
            actions: _actions,
        }
    }
}
