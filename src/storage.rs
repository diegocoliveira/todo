pub mod file;

use std::collections::BTreeMap;

use crate::{cli::AppError, todo::Todo};

#[async_trait::async_trait(?Send)]
pub trait TodoStorage {
    async fn add(&mut self, message: String) -> Result<Option<&Todo>, AppError>;
    async fn list(&self) -> Result<Vec<&Todo>, AppError>;
    async fn exist(&self, id: u32) -> Result<bool, AppError>;
    async fn update(&mut self, id: u32, message: String) -> Result<Option<&Todo>, AppError>;
    async fn done(&mut self, id: u32) -> Result<Option<&Todo>, AppError>;
    async fn delete(&mut self, id: u32) -> Result<Option<Todo>, AppError>;
}

pub struct Todos {
    sequence: u32,
    todo_list: BTreeMap<u32, Todo>,
    file: Box<dyn file::TodoFile>,
}

impl Todos {
    pub async fn new(file: Box<dyn file::TodoFile>) -> Result<Self, AppError> {
        let (sequence, todo_list) = file.load().await?;
        Ok(Self {
            sequence,
            todo_list,
            file,
        })
    }

    fn next_id(&mut self) -> u32 {
        self.sequence += 1;
        self.sequence
    }

    async fn save(&mut self) -> Result<(), AppError> {
        self.file.save(self.sequence, &self.todo_list).await
    }
}

#[async_trait::async_trait(?Send)]
impl TodoStorage for Todos {
    async fn add(&mut self, message: String) -> Result<Option<&Todo>, AppError> {
        let id = self.next_id();
        let todo = Todo::new(id, message);
        self.todo_list.insert(id, todo);
        self.save().await?;
        Ok(self.todo_list.get(&id))
    }

    async fn list(&self) -> Result<Vec<&Todo>, AppError> {
        Ok(self.todo_list.values().collect())
    }

    async fn exist(&self, id: u32) -> Result<bool, AppError> {
        Ok(self.todo_list.contains_key(&id))
    }

    async fn done(&mut self, id: u32) -> Result<Option<&Todo>, AppError> {
        if let Some(todo) = self.todo_list.get_mut(&id) {
            todo.done = true;
            self.save().await?;
        }
        Ok(self.todo_list.get(&id)) //realizando uma nova busca para retornar o todo sem a mutabilidade
    }

    async fn delete(&mut self, id: u32) -> Result<Option<Todo>, AppError> {
        if let Some(todo) = self.todo_list.remove(&id) {
            self.save().await?;
            Ok(Some(todo))
        } else {
            Ok(None)
        }
    }

    async fn update(&mut self, id: u32, message: String) -> Result<Option<&Todo>, AppError> {
        if let Some(todo) = self.todo_list.get_mut(&id) {
            todo.message = message;
            self.save().await?;
            //return Ok(Some(todo));
        }
        //Ok(None)
        Ok(self.todo_list.get(&id)) //realizando uma nova busca para retornar o todo sem a mutabilidade
    }
}
