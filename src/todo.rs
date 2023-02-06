use std::{collections::BTreeMap, fmt::Display};

use serde::{Deserialize, Serialize};
use tokio::io;

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
            self.done.then(|| "feito").unwrap_or("pendente")
        )
    }
}
#[derive(Debug)]
pub enum StorageError {
    Write(io::Error),
    Read(io::Error),
    Parse(serde_json::Error),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Write(err) => write!(f, "Não foi possível escrever no arquivo: {}", err),
            StorageError::Read(err) => write!(f, "Não foi possível ler o arquivo: {}", err),
            StorageError::Parse(err) => write!(f, "Não foi possível parsear o arquivo: {}", err),
        }
    }
}

#[async_trait::async_trait]
pub trait TodoStorage {
    async fn add(&mut self, message: String) -> Result<Option<&Todo>, StorageError>;
    async fn list(&self) -> Result<Vec<&Todo>, StorageError>;
    async fn exist(&self, id: u32) -> Result<bool, StorageError>;
    async fn update(&mut self, id: u32, message: String) -> Result<Option<&Todo>, StorageError>;
    async fn done(&mut self, id: u32) -> Result<Option<&Todo>, StorageError>;
    async fn delete(&mut self, id: u32) -> Result<Option<Todo>, StorageError>;
}

pub struct Todos {
    sequence: u32,
    todo_list: BTreeMap<u32, Todo>,
    path: String,
}

impl Todos {
    pub async fn new() -> Result<Self, StorageError> {
        let _path: String = "todo_storage.json".to_string();
        let contents = tokio::fs::read_to_string(&_path)
            .await
            .map_err(StorageError::Read)?;
        let (_sequence, _todo_list) =
            serde_json::from_str(&contents).unwrap_or((0 as u32, BTreeMap::<u32, Todo>::new()));

        Ok(Self {
            sequence: _sequence,
            todo_list: _todo_list,
            path: _path,
        })
    }

    fn next_id(&mut self) -> u32 {
        self.sequence += 1;
        self.sequence
    }

    async fn save(&self) -> Result<(), StorageError> {
        let contents = serde_json::to_string(&(self.sequence, &self.todo_list))
            .map_err(StorageError::Parse)?;
        tokio::fs::write(&self.path, contents)
            .await
            .map_err(StorageError::Write)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl TodoStorage for Todos {
    async fn add(&mut self, message: String) -> Result<Option<&Todo>, StorageError> {
        let id = self.next_id();
        let todo = Todo::new(id, message);
        self.todo_list.insert(id, todo);
        self.save().await?;
        Ok(self.todo_list.get(&id))
    }

    async fn list(&self) -> Result<Vec<&Todo>, StorageError> {
        Ok(self.todo_list.values().collect())
    }

    async fn exist(&self, id: u32) -> Result<bool, StorageError> {
        Ok(self.todo_list.contains_key(&id))
    }

    async fn done(&mut self, id: u32) -> Result<Option<&Todo>, StorageError> {
        if let Some(todo) = self.todo_list.get_mut(&id) {
            todo.done = true;
            self.save().await?;
        }
        Ok(self.todo_list.get(&id)) //realizando uma nova busca para retornar o todo sem a mutabilidade
    }

    async fn delete(&mut self, id: u32) -> Result<Option<Todo>, StorageError> {
        if let Some(todo) = self.todo_list.remove(&id) {
            self.save().await?;
            return Ok(Some(todo));
        }
        Ok(None)
    }

    async fn update(&mut self, id: u32, message: String) -> Result<Option<&Todo>, StorageError> {
        if let Some(todo) = self.todo_list.get_mut(&id) {
            todo.message = message;
            self.save().await?;
            //return Ok(Some(todo));
        }
        //Ok(None)
        Ok(self.todo_list.get(&id)) //realizando uma nova busca para retornar o todo sem a mutabilidade
    }
}
