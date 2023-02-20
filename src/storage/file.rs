use std::collections::BTreeMap;

use crate::{cli::AppError, todo::Todo};

pub struct TodoFileImpl {
    path: String,
}

#[async_trait::async_trait(?Send)]
pub trait TodoFile {
    async fn load(&self) -> Result<(u32, BTreeMap<u32, Todo>), AppError>;
    async fn save(
        &mut self,
        sequence: u32,
        todo_list: &BTreeMap<u32, Todo>,
    ) -> Result<(), AppError>;
}

impl TodoFileImpl {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl TodoFile for TodoFileImpl {
    async fn load(&self) -> Result<(u32, BTreeMap<u32, Todo>), AppError> {
        let contents = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(AppError::Read)?;
        let (sequence, todo_list) =
            serde_json::from_str(&contents).unwrap_or((0_u32, BTreeMap::<u32, Todo>::new()));

        Ok((sequence, todo_list))
    }

    async fn save(
        &mut self,
        sequence: u32,
        todo_list: &BTreeMap<u32, Todo>,
    ) -> Result<(), AppError> {
        let contents = serde_json::to_string(&(sequence, todo_list)).map_err(AppError::Parse)?;
        tokio::fs::write(&self.path, contents)
            .await
            .map_err(AppError::Write)?;
        Ok(())
    }
}
