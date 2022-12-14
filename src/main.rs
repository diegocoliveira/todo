mod terminal;
mod todo;

use console::{self, style, Emoji};
use terminal::Action;
use todo::Todos;

use crate::terminal::{Terminal, TerminalError};

struct TodoCli {
    terminal: Terminal,
    todos: Todos,
}

impl TodoCli {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(),
            todos: Todos::new(),
        }
    }
    fn add(&mut self) -> Result<(), TerminalError> {
        let item = self.terminal.add_todo()?;
        if let Some(todo) = self.todos.add(item) {
            self.terminal.show_todo(&todo)?;
        } else {
            self.terminal
                .write_line("Não foi possível adicionar o TODO")?;
        }

        Ok(())
    }

    fn done(&mut self, id: i32) -> Result<(), TerminalError> {
        if let Some(todo) = self.todos.done(id) {
            self.terminal.write_line(&format!(
                "[{:?}] {}",
                style(todo).magenta(),
                style(" - marcado como feito!").green(),
            ))?;
        } else {
            self.terminal
                .write_line("Não foi possível marcar o TODO como feito")?;
        }
        Ok(())
    }

    fn delete(&mut self, id: i32) -> Result<(), TerminalError> {
        if let Some(todo) = self.todos.delete(id) {
            self.terminal.write_line(&format!(
                "[{:?}] {}",
                style(todo).magenta(),
                style(" - deletado com sucesso!").green()
            ))?;
        } else {
            self.terminal
                .write_line("Não foi possível deletar o TODO")?;
        }

        Ok(())
    }

    fn update(&mut self, id: i32, message: String) -> Result<(), TerminalError> {
        if let Some(todo) = self.todos.update(id, message) {
            self.terminal.write_line(&format!(
                "[{:?}] {}",
                style(todo).magenta(),
                style(" - atualizado com sucesso!").green(),
            ))?;
        } else {
            self.terminal
                .write_line("Não foi possível atualizar o TODO")?;
        }
        Ok(())
    }

    fn edit(&mut self) -> Result<(), TerminalError> {
        self.terminal.list_todos(self.todos.list())?;
        let id = self.terminal.select_todo()?;
        let id = id.parse::<i32>().unwrap_or(0);
        if !self.todos.exist(id) {
            self.terminal.write_line(&format!(
                "{} Não existe um TODO com esse ID",
                Emoji("😕", ":/")
            ))?;
        } else {
            let action = self.terminal.ask_for_todo_action(id)?;
            match action {
                Action::Done(id) => self.done(id)?,
                Action::Delete(id) => self.delete(id)?,
                Action::Update(id, message) => self.update(id, message)?,
                _ => (),
            };
        }

        self.terminal.press_key()?;

        Ok(())
    }

    fn list(&mut self) -> Result<(), TerminalError> {
        self.terminal.list_todos(self.todos.list())?;
        self.terminal.press_key()?;
        Ok(())
    }

    fn exit(&mut self) -> Result<(), TerminalError> {
        self.terminal.write_line(&format!(
            "\n{}_>> {} Obrigado por usar o TODO-CLI! ",
            Emoji("😃", ":)"),
            Emoji("👋", "Tchau."),
        ))?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), TerminalError> {
        self.terminal.welcome()?;
        loop {
            let action = self.terminal.ask_for_action()?;
            match action {
                Action::Add => self.add()?,
                Action::List => self.list()?,
                Action::Edit => self.edit()?,
                Action::Exit => return self.exit(),
                _ => (),
            }
        }
    }
}

fn main() {
    let mut todo_cli = TodoCli::new();
    if let Err(err) = todo_cli.run() {
        println!(
            "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado.",
        );
        println!("\n🤨_>> Erro: {}", style(err).red());
    }
}
