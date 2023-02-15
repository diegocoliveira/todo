use std::fmt::Display;

use crate::{
    terminal::{Action, UserInterface},
    todo::TodoStorage,
};

use tokio::io;

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
            Self::Stdout(err) => write!(f, "Erro ao escrever no terminal: {}", err),
            Self::Stdin(err) => write!(f, "Erro ao ler do terminal: {}", err),
            Self::Write(err) => write!(f, "Não foi possível escrever no arquivo: {}", err),
            Self::Read(err) => write!(f, "Não foi possível ler o arquivo: {}", err),
            Self::Parse(err) => write!(f, "Não foi possível parsear o arquivo: {}", err),
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
                .show_sucess(&todo, "adicionado com sucesso")
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
                Action::Exit => return Ok(self.user_interface.exit().await?),
                _ => (),
            }
        }
    }
}
