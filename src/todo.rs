use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
pub mod mocks {
    use super::Todo;

    factori::factori!(Todo, {
        default {
            id =1,
            message = "todo 1".to_string(),
            done = false
        }

        mixin done {
            done = true
        }

        mixin updated {
            message = "todo 1 updated".to_string()
        }
    });
}

#[cfg(test)]
mod tests {
    use super::mocks::*;

    #[test]
    fn test_todo_new() {
        let todo = factori::create!(Todo);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.message, "todo 1");
        assert!(!todo.done);
    }

    #[test]
    fn test_todo_new_done() {
        let todo = factori::create!(Todo, :done);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.message, "todo 1");
        assert!(todo.done);
    }

    #[test]
    fn test_todo_new_updated() {
        let todo = factori::create!(Todo, :updated);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.message, "todo 1 updated");
        assert!(!todo.done);
    }
}
