use std::{collections::BTreeMap, fmt::Display};

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
            self.done.then(|| "feito").unwrap_or("pendente")
        )
    }
}

pub trait TodoStorage {
    fn add(&mut self, message: String) -> Option<&Todo>;
    fn list(&self) -> Vec<&Todo>;
    fn exist(&self, id: u32) -> bool;
    fn update(&mut self, id: u32, message: String) -> Option<&Todo>;
    fn done(&mut self, id: u32) -> Option<&Todo>;
    fn delete(&mut self, id: u32) -> Option<Todo>;
}

pub struct Todos {
    sequence: u32,
    todos: BTreeMap<u32, Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            todos: BTreeMap::new(),
        }
    }

    fn next_id(&mut self) -> u32 {
        self.sequence += 1;
        self.sequence
    }
}

impl TodoStorage for Todos {
    fn add(&mut self, message: String) -> Option<&Todo> {
        let id = self.next_id();
        let todo = Todo::new(id, message);
        self.todos.insert(id, todo);
        self.todos.get(&id)
    }

    fn list(&self) -> Vec<&Todo> {
        self.todos.values().collect()
    }

    fn exist(&self, id: u32) -> bool {
        self.todos.contains_key(&id)
    }

    fn done(&mut self, id: u32) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id)?;
        todo.done = true;
        Some(todo)
    }

    fn delete(&mut self, id: u32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    fn update(&mut self, id: u32, message: String) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id)?;
        todo.message = message;
        Some(todo)
    }
}
