use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Action {
    pub id: u32,
    pub value: Value,
}

impl FromStr for Action {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Action {
    pub fn new(id: u32, value: Option<Value>) -> Action {
        let _value = match value {
            Some(value) => value,
            None => Value::Null,
        };

        Action { id, value: _value }
    }
}
