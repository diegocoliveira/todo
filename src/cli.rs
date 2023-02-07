use crate::{
    terminal::{Action, TerminalError, UserInterface},
    todo::{StorageError, TodoStorage},
};

pub enum CliError {
    Storage(StorageError),
    Terminal(TerminalError),
}
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Storage(err) => write!(f, "Erro no armazenamento: {}", err),
            CliError::Terminal(err) => write!(f, "Erro no terminal: {}", err),
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
    async fn add(&mut self) -> Result<(), CliError> {
        let item = self
            .user_interface
            .add_todo()
            .await
            .map_err(CliError::Terminal)?;
        if let Some(todo) = self
            .todo_storage
            .add(item)
            .await
            .map_err(CliError::Storage)?
        {
            self.user_interface
                .show_sucess(&todo, "adicionado com sucesso")
                .await
                .map_err(CliError::Terminal)?;
        } else {
            self.user_interface
                .show_error("Não foi possível adicionar o TODO")
                .await
                .map_err(CliError::Terminal)?;
        }
        self.user_interface
            .press_key()
            .await
            .map_err(CliError::Terminal)?;
        Ok(())
    }

    async fn done(&mut self, id: u32) -> Result<(), CliError> {
        if let Some(todo) = self
            .todo_storage
            .done(id)
            .await
            .map_err(CliError::Storage)?
        {
            self.user_interface
                .show_sucess(todo, "marcado como feito")
                .await
                .map_err(CliError::Terminal)?;
        } else {
            self.user_interface
                .show_error("Não foi possível marcar o TODO como feito")
                .await
                .map_err(CliError::Terminal)?;
        }
        Ok(())
    }

    async fn delete(&mut self, id: u32) -> Result<(), CliError> {
        if let Some(todo) = self
            .todo_storage
            .delete(id)
            .await
            .map_err(CliError::Storage)?
        {
            self.user_interface
                .show_sucess(&todo, "deletado com sucesso")
                .await
                .map_err(CliError::Terminal)?;
        } else {
            self.user_interface
                .show_error("Não foi possível deletar o TODO")
                .await
                .map_err(CliError::Terminal)?;
        }

        Ok(())
    }

    async fn update(&mut self, id: u32, message: String) -> Result<(), CliError> {
        if let Some(todo) = self
            .todo_storage
            .update(id, message)
            .await
            .map_err(CliError::Storage)?
        {
            self.user_interface
                .show_sucess(todo, "atualizado com sucesso!")
                .await
                .map_err(CliError::Terminal)?;
        } else {
            self.user_interface
                .show_error("Não foi possível atualizar o TODO")
                .await
                .map_err(CliError::Terminal)?;
        }
        Ok(())
    }

    async fn edit(&mut self) -> Result<(), CliError> {
        self.user_interface
            .list_todo(self.todo_storage.list().await.map_err(CliError::Storage)?)
            .await
            .map_err(CliError::Terminal)?;
        if let Some(id) = self
            .user_interface
            .select_todo()
            .await
            .map_err(CliError::Terminal)?
        {
            if !self
                .todo_storage
                .exist(id)
                .await
                .map_err(CliError::Storage)?
            {
                self.user_interface
                    .show_error("Não existe um TODO com esse ID")
                    .await
                    .map_err(CliError::Terminal)?;
            } else {
                let action = self
                    .user_interface
                    .ask_for_todo_action(id)
                    .await
                    .map_err(CliError::Terminal)?;
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
                .await
                .map_err(CliError::Terminal)?;
        }

        self.user_interface
            .press_key()
            .await
            .map_err(CliError::Terminal)?;

        Ok(())
    }

    async fn list(&mut self) -> Result<(), CliError> {
        self.user_interface
            .list_todo(self.todo_storage.list().await.map_err(CliError::Storage)?)
            .await
            .map_err(CliError::Terminal)?;
        self.user_interface
            .press_key()
            .await
            .map_err(CliError::Terminal)?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), CliError> {
        self.user_interface
            .welcome()
            .await
            .map_err(CliError::Terminal)?;
        loop {
            let action = self
                .user_interface
                .ask_for_action()
                .await
                .map_err(CliError::Terminal)?;
            match action {
                Action::Add => self.add().await?,
                Action::List => self.list().await?,
                Action::Edit => self.edit().await?,
                Action::Exit => {
                    return Ok(self
                        .user_interface
                        .exit()
                        .await
                        .map_err(CliError::Terminal)?)
                }
                _ => (),
            }
        }
    }
}
