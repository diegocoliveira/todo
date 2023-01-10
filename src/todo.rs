use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub message: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: i32, message: String) -> Self {
        Self {
            id,
            message,
            done: false,
        }
    }
}

pub struct Todos {
    sequence: i32,
    todos: BTreeMap<i32, Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            todos: BTreeMap::new(),
        }
    }

    fn next_id(&mut self) -> i32 {
        self.sequence += 1;
        self.sequence
    }

    pub fn add(&mut self, message: String) -> Option<&Todo> {
        let id = self.next_id();
        let todo = Todo::new(id, message);
        self.todos.insert(id, todo);
        self.todos.get(&id)
    }

    pub fn list(&self) -> Vec<&Todo> {
        self.todos.values().collect()
    }

    pub fn exist(&self, id: i32) -> bool {
        self.todos.contains_key(&id)
    }

    pub fn done(&mut self, id: i32) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id)?;
        todo.done = true;
        Some(todo)
    }

    pub fn delete(&mut self, id: i32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    pub fn update(&mut self, id: i32, message: String) -> Option<&Todo> {
        let todo = self.todos.get_mut(&id)?;
        todo.message = message;
        Some(todo)
    }
}
