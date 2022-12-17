#[derive(Debug, Clone)]
pub struct Todo {
    message: String,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
