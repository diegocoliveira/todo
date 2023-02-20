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

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::mocks;

    #[tokio::test]
    async fn save_file_from_map() {
        let mut file = TodoFileImpl::new("storage_test.json");
        let (sequence, map) = mocks::sequence_map_todo();
        file.save(sequence, &map).await.unwrap();
        let contents = tokio::fs::read_to_string("storage_test.json")
            .await
            .unwrap();
        tokio::fs::remove_file("storage_test.json").await.unwrap();
        assert_eq!(
            contents,
            r#"[5,{"1":{"id":1,"message":"todo 1","done":true},"2":{"id":2,"message":"todo 2","done":false},"5":{"id":5,"message":"todo 5","done":false}}]"#
        );
    }
    #[tokio::test]
    async fn load_map_from_file() {
        tokio::fs::write(
            "storage_test.jon",
            r#"[1,{"1":{"id":1,"message":"todo 1","done":true}}]"#,
        )
        .await
        .unwrap();
        let file = TodoFileImpl::new("storage_test.jon");
        let (sequence, map) = file.load().await.unwrap();
        tokio::fs::remove_file("storage_test.jon").await.unwrap();
        assert_eq!(sequence, 1);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1).unwrap().message, "todo 1");
    }
}
