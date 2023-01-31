use crate::{
    terminal::{Action, TerminalError, UserInterface},
    todo::TodoStorage,
};

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
    fn add(&mut self) -> Result<(), TerminalError> {
        let item = self.user_interface.add_todo()?;
        if let Some(todo) = self.todo_storage.add(item) {
            self.user_interface
                .show_sucess(&todo, "adicionado com sucesso")?;
        } else {
            self.user_interface
                .show_error("Não foi possível adicionar o TODO")?;
        }
        self.user_interface.press_key()?;
        Ok(())
    }

    fn done(&mut self, id: u32) -> Result<(), TerminalError> {
        if let Some(todo) = self.todo_storage.done(id) {
            self.user_interface
                .show_sucess(todo, "marcado como feito")?;
        } else {
            self.user_interface
                .show_error("Não foi possível marcar o TODO como feito")?;
        }
        Ok(())
    }

    fn delete(&mut self, id: u32) -> Result<(), TerminalError> {
        if let Some(todo) = self.todo_storage.delete(id) {
            self.user_interface
                .show_sucess(&todo, "deletado com sucesso")?;
        } else {
            self.user_interface
                .show_error("Não foi possível deletar o TODO")?;
        }

        Ok(())
    }

    fn update(&mut self, id: u32, message: String) -> Result<(), TerminalError> {
        if let Some(todo) = self.todo_storage.update(id, message) {
            self.user_interface
                .show_sucess(todo, "atualizado com sucesso!")?;
        } else {
            self.user_interface
                .show_error("Não foi possível atualizar o TODO")?;
        }
        Ok(())
    }

    fn edit(&mut self) -> Result<(), TerminalError> {
        self.user_interface.list_todo(self.todo_storage.list())?;
        if let Some(id) = self.user_interface.select_todo()? {
            if !self.todo_storage.exist(id) {
                self.user_interface
                    .show_error("Não existe um TODO com esse ID")?;
            } else {
                let action = self.user_interface.ask_for_todo_action(id)?;
                match action {
                    Action::Done(id) => self.done(id)?,
                    Action::Delete(id) => self.delete(id)?,
                    Action::Update(id, message) => self.update(id, message)?,
                    _ => (),
                };
            }
        } else {
            self.user_interface
                .show_error("O ID informado é inválido")?;
        }

        self.user_interface.press_key()?;

        Ok(())
    }

    fn list(&mut self) -> Result<(), TerminalError> {
        self.user_interface.list_todo(self.todo_storage.list())?;
        self.user_interface.press_key()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), TerminalError> {
        self.user_interface.welcome()?;
        loop {
            let action = self.user_interface.ask_for_action()?;
            match action {
                Action::Add => self.add()?,
                Action::List => self.list()?,
                Action::Edit => self.edit()?,
                Action::Exit => return self.user_interface.exit(),
                _ => (),
            }
        }
    }
}
