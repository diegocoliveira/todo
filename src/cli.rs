use std::fmt::Display;

use crate::{
    storage::TodoStorage,
    terminal::{Action, UserInterface},
};

use tokio::io;

#[derive(Debug)]
pub enum AppError {
    Stdout(io::Error),
    Stdin(io::Error),
    Write(io::Error),
    Read(io::Error),
    Parse(serde_json::Error),
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdout(err) => write!(f, "Erro ao escrever no terminal: {err}"),
            Self::Stdin(err) => write!(f, "Erro ao ler do terminal: {err}"),
            Self::Write(err) => write!(f, "Não foi possível escrever no arquivo: {err}"),
            Self::Read(err) => write!(f, "Não foi possível ler o arquivo: {err}"),
            Self::Parse(err) => write!(f, "Não foi possível parsear o arquivo: {err}"),
        }
    }
}

pub struct TodoCli {
    user_interface: Box<dyn UserInterface>,
    todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn new(user_interface: Box<dyn UserInterface>, todo_storage: Box<dyn TodoStorage>) -> Self {
        Self {
            user_interface,
            todo_storage,
        }
    }
    async fn add(&mut self) -> Result<(), AppError> {
        let item = self.user_interface.add_todo().await?;

        if let Some(todo) = self.todo_storage.add(item).await? {
            self.user_interface
                .show_sucess(todo, "adicionado com sucesso")
                .await?;
        } else {
            self.user_interface
                .show_error("Não foi possível adicionar o TODO")
                .await?;
        }
        self.user_interface.press_key().await?;
        Ok(())
    }

    async fn done(&mut self, id: u32) -> Result<(), AppError> {
        if let Some(todo) = self.todo_storage.done(id).await? {
            self.user_interface
                .show_sucess(todo, "marcado como feito")
                .await?;
        } else {
            self.user_interface
                .show_error("Não foi possível marcar o TODO como feito")
                .await?;
        }
        Ok(())
    }

    async fn delete(&mut self, id: u32) -> Result<(), AppError> {
        if let Some(todo) = self.todo_storage.delete(id).await? {
            self.user_interface
                .show_sucess(&todo, "deletado com sucesso")
                .await?;
        } else {
            self.user_interface
                .show_error("Não foi possível deletar o TODO")
                .await?;
        }

        Ok(())
    }

    async fn update(&mut self, id: u32, message: String) -> Result<(), AppError> {
        if let Some(todo) = self.todo_storage.update(id, message).await? {
            self.user_interface
                .show_sucess(todo, "atualizado com sucesso!")
                .await?;
        } else {
            self.user_interface
                .show_error("Não foi possível atualizar o TODO")
                .await?;
        }
        Ok(())
    }

    async fn edit(&mut self) -> Result<(), AppError> {
        self.user_interface
            .list_todo(self.todo_storage.list().await?)
            .await?;
        if let Some(id) = self.user_interface.select_todo().await? {
            if !self.todo_storage.exist(id).await? {
                self.user_interface
                    .show_error("Não existe um TODO com esse ID")
                    .await?;
            } else {
                let action = self.user_interface.ask_for_todo_action(id).await?;
                match action {
                    Action::Done(id) => self.done(id).await?,
                    Action::Delete(id) => self.delete(id).await?,
                    Action::Update(id, message) => self.update(id, message).await?,
                    _ => (),
                };
            }
        } else {
            self.user_interface
                .show_error("O ID informado é inválido")
                .await?;
        }

        self.user_interface.press_key().await?;

        Ok(())
    }

    async fn list(&mut self) -> Result<(), AppError> {
        self.user_interface
            .list_todo(self.todo_storage.list().await?)
            .await?;
        self.user_interface.press_key().await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        self.user_interface.welcome().await?;
        loop {
            let action = self.user_interface.ask_for_action().await?;
            match action {
                Action::Add => self.add().await?,
                Action::List => self.list().await?,
                Action::Edit => self.edit().await?,
                Action::Exit => return self.user_interface.exit().await,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use mockall::lazy_static;

    use super::*;
    use crate::storage::*;
    use crate::terminal::*;
    use crate::todo::mocks::*;
    use crate::todo::Todo;

    #[tokio::test]
    async fn exit() {
        let mut mock_user_interface = MockUserInterface::default();

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mock_todo_storage = MockTodoStorage::default();

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn add() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Add));

        mock_user_interface
            .expect_add_todo()
            .times(1)
            .returning(|| Ok(factori::create!(Todo).message));

        mock_user_interface
            .expect_show_sucess()
            .times(1)
            .withf(|todo, msg| todo.message == TODO.message && msg == "adicionado com sucesso")
            .returning(|_, _| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Exit));

        mock_user_interface.expect_exit().returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_add()
            .withf(|message| message == &TODO.message)
            .times(1)
            .returning(move |_| Ok(Some(&TODO)));

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn list() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::List));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .withf(|todos| !todos.is_empty())
            .returning(|_| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));
        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn edit_id_invalid() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Edit));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_select_todo()
            .returning(|| Ok(None));

        mock_user_interface
            .expect_show_error()
            .times(1)
            .withf(|msg| msg == "O ID informado é inválido")
            .returning(|_| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));
        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn edit_todo_not_exist() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Edit));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_select_todo()
            .returning(|| Ok(Some(10)));

        mock_user_interface
            .expect_show_error()
            .times(1)
            .withf(|msg| msg.contains("Não existe"))
            .returning(|_| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));

        mock_todo_storage
            .expect_exist()
            .times(1)
            .returning(|_| Ok(false));

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn done() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo, :done);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Edit));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_select_todo()
            .returning(|| Ok(Some(1)));

        mock_user_interface
            .expect_ask_for_todo_action()
            .times(1)
            .returning(|_| Ok(Action::Done(1)));

        mock_user_interface
            .expect_show_sucess()
            .times(1)
            .withf(|todo, msg| todo.done && msg.contains("marcado como feito"))
            .returning(|_, _| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));

        mock_todo_storage
            .expect_exist()
            .times(1)
            .returning(|_| Ok(true));

        mock_todo_storage
            .expect_done()
            .times(1)
            .returning(|_| Ok(Some(&TODO)));

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn update() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo, :updated);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Edit));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_select_todo()
            .returning(|| Ok(Some(1)));

        mock_user_interface
            .expect_ask_for_todo_action()
            .times(1)
            .returning(|_| Ok(Action::Update(1, "updated".to_string())));

        mock_user_interface
            .expect_show_sucess()
            .times(1)
            .withf(|todo, msg| {
                todo.message.contains("updated") && msg.contains("atualizado com sucesso")
            })
            .returning(|_, _| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));

        mock_todo_storage
            .expect_exist()
            .times(1)
            .returning(|_| Ok(true));

        mock_todo_storage
            .expect_update()
            .times(1)
            .returning(|_, _| Ok(Some(&TODO)));

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }

    #[tokio::test]
    async fn delete() {
        let mut mock_user_interface = MockUserInterface::default();

        let mut seq = mockall::Sequence::new();

        lazy_static! {
            static ref TODO: Todo = factori::create!(Todo);
        };

        mock_user_interface
            .expect_welcome()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(Action::Edit));

        mock_user_interface
            .expect_list_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_select_todo()
            .returning(|| Ok(Some(1)));

        mock_user_interface
            .expect_ask_for_todo_action()
            .times(1)
            .returning(|_| Ok(Action::Delete(1)));

        mock_user_interface
            .expect_show_sucess()
            .times(1)
            .withf(|todo, msg| todo.id == 1 && msg.contains("deletado com sucesso"))
            .returning(|_, _| Ok(()));

        mock_user_interface.expect_press_key().returning(|| Ok(()));

        mock_user_interface
            .expect_ask_for_action()
            .times(1)
            .returning(|| Ok(Action::Exit));

        mock_user_interface
            .expect_exit()
            .times(1)
            .returning(|| Ok(()));

        let mut mock_todo_storage = MockTodoStorage::default();

        mock_todo_storage
            .expect_list()
            .times(1)
            .returning(|| Ok(vec![&TODO]));

        mock_todo_storage
            .expect_exist()
            .times(1)
            .returning(|_| Ok(true));

        mock_todo_storage
            .expect_delete()
            .times(1)
            .returning(|_| Ok(Some(factori::create!(Todo))));

        let mut todo_cli = TodoCli::new(Box::new(mock_user_interface), Box::new(mock_todo_storage));
        todo_cli.run().await.unwrap();
    }
}
