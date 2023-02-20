use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub message: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: u32, message: String) -> Self {
        Self {
            id,
            message,
            done: false,
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}-{}, status: {})",
            self.id,
            self.message,
            if self.done { "feito" } else { "pendente" }
        )
    }
}
