use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub weight: f32,
}

impl Edge {
    pub fn new(from: u32, to: u32, weight: f32) -> Edge {
        Edge { from, to, weight }
    }
}
