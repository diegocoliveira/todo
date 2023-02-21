pub mod file;

use std::collections::BTreeMap;

use crate::{cli::AppError, todo::Todo};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait(?Send)]
pub trait TodoStorage {
    async fn add<'a>(&'a mut self, message: String) -> Result<Option<&'a Todo>, AppError>;
    async fn list<'a>(&'a self) -> Result<Vec<&'a Todo>, AppError>;
    async fn exist(&self, id: u32) -> Result<bool, AppError>;
    async fn update<'a>(
        &'a mut self,
        id: u32,
        message: String,
    ) -> Result<Option<&'a Todo>, AppError>;
    async fn done<'a>(&'a mut self, id: u32) -> Result<Option<&'a Todo>, AppError>;
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

#[cfg(test)]
pub mod mocks {
    use std::collections::BTreeMap;

    use super::Todo;

    pub fn sequence_map_todo() -> (u32, BTreeMap<u32, Todo>) {
        (
            5_u32,
            BTreeMap::from([
                (
                    1,
                    Todo {
                        id: 1,
                        message: "todo 1".to_string(),
                        done: true,
                    },
                ),
                (
                    2,
                    Todo {
                        id: 2,
                        message: "todo 2".to_string(),
                        done: false,
                    },
                ),
                (
                    5,
                    Todo {
                        id: 5,
                        message: "todo 5".to_string(),
                        done: false,
                    },
                ),
            ]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn exist() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        let todos = Todos::new(Box::new(mock)).await.unwrap();
        assert!(todos.exist(1).await.unwrap()); //true
        assert!(todos.exist(2).await.unwrap()); //true
        assert!(!todos.exist(3).await.unwrap()); //false
        assert!(!todos.exist(4).await.unwrap()); //false
        assert!(todos.exist(5).await.unwrap()); //true
    }

    #[tokio::test]
    async fn list() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        let todos = Todos::new(Box::new(mock)).await.unwrap();
        let list = todos.list().await.unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(
            *list[0],
            Todo {
                id: 1,
                message: "todo 1".to_string(),
                done: true
            }
        );
        assert_eq!(
            *list[1],
            Todo {
                id: 2,
                message: "todo 2".to_string(),
                done: false
            }
        );
        assert_eq!(
            *list[2],
            Todo {
                id: 5,
                message: "todo 5".to_string(),
                done: false
            }
        );
    }

    #[tokio::test]
    async fn add() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        mock.expect_save()
            .withf(|sequence, map| *sequence == 6 && map.len() == 4)
            .times(1)
            .returning(|_, _| Ok(()));

        let mut todos = Todos::new(Box::new(mock)).await.unwrap();
        todos.add("todo 6".to_string()).await.unwrap();

        assert_eq!(
            *todos.todo_list.get(&6).unwrap(),
            Todo {
                id: 6,
                message: "todo 6".to_string(),
                done: false
            }
        );
    }

    #[tokio::test]
    async fn update() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        mock.expect_save()
            .withf(|sequence, map| *sequence == 5 && map.len() == 3)
            .times(1)
            .returning(|_, _| Ok(()));

        let mut todos = Todos::new(Box::new(mock)).await.unwrap();

        assert_eq!(
            todos.update(3, "não existe".to_string()).await.unwrap(),
            None
        );

        todos.update(5, "todo alterado".to_string()).await.unwrap();

        assert_eq!(
            todos.todo_list.get(&5).unwrap().message,
            "todo alterado".to_string()
        );
    }

    #[tokio::test]
    async fn done() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        mock.expect_save()
            .withf(|sequence, map| *sequence == 5 && map.len() == 3)
            .times(1)
            .returning(|_, _| Ok(()));

        let mut todos = Todos::new(Box::new(mock)).await.unwrap();

        assert_eq!(todos.done(3).await.unwrap(), None);

        todos.done(5).await.unwrap();

        assert!(todos.todo_list.get(&5).unwrap().done);
    }

    #[tokio::test]
    async fn delete() {
        let mut mock = file::MockTodoFile::default();
        mock.expect_load()
            .return_once(|| Ok(mocks::sequence_map_todo()));

        mock.expect_save()
            .withf(|sequence, map| *sequence == 5 && map.len() == 2)
            .times(1)
            .returning(|_, _| Ok(()));

        let mut todos = Todos::new(Box::new(mock)).await.unwrap();

        assert_eq!(todos.delete(3).await.unwrap(), None);

        todos.delete(5).await.unwrap();

        assert!(!todos.exist(5).await.unwrap());
    }
}
