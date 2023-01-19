use std::{
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration,
};

use super::todo::Todo;

use console::{style, Emoji, Style, Term};

pub enum Action {
    Add,
    List,
    Edit,
    Exit,
    Done(i32),
    Delete(i32),
    Update(i32, String),
}

pub enum TerminalError {
    Stdout(io::Error),
    Stdin(io::Error),
    Test(String), // usado para simular um erro
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdout(err) => write!(f, "Erro ao escrever no terminal: {}", err),
            Self::Stdin(err) => write!(f, "Erro ao ler do terminal: {}", err),
            Self::Test(err) => write!(f, "Simulação de erro: {}", err),
        }
    }
}

pub trait UserInterface {
    fn input(&self) -> Result<String, TerminalError>;
    fn press_key(&mut self) -> Result<(), TerminalError>;
    fn welcome(&mut self) -> Result<(), TerminalError>;
    fn exit(&mut self) -> Result<(), TerminalError>;
    fn ask_for_action(&mut self) -> Result<Action, TerminalError>;
    fn ask_for_todo_action(&mut self, id: i32) -> Result<Action, TerminalError>;
    fn add_todo(&mut self) -> Result<String, TerminalError>;
    fn select_todo(&mut self) -> Result<String, TerminalError>;
    fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), TerminalError>;
    fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), TerminalError>;
    fn show_error(&mut self, msg: &str) -> Result<(), TerminalError>;
}

pub struct Terminal {
    version: String,
    term: Term,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            //substituição do stdin e stdout por term
            term: Term::stdout(),
            version: "0.6.0".to_string(),
        }
    }

    fn write_line(&mut self, text: &str) -> Result<(), TerminalError> {
        writeln!(self.term, "{}", text).map_err(TerminalError::Stdout)?;
        Ok(())
    }

    fn title(&mut self, text: &str) -> Result<(), TerminalError> {
        self.write_line(&format!(
            "################# {} ################# \n\n",
            style(text).bold().green(),
        ))?;
        Ok(())
    }

    fn progress_bar_fake(&mut self) -> Result<(), TerminalError> {
        let mut progess_bar = String::new();
        let mut progess_bar_ok = String::new();
        self.term.hide_cursor().map_err(TerminalError::Stdout)?;
        for _i in 0..25 {
            progess_bar.push(' ');
        }
        for i in 0..=25 {
            for j in 0..=3 {
                let x = match j {
                    0 => "|",
                    1 => "/",
                    2 => "-",
                    3 => "\\",
                    _ => " ",
                };
                self.term
                    .clear_last_lines(1)
                    .map_err(TerminalError::Stdout)?;
                self.term
                    .write_line(&format!(
                        "Carregando ... {}% -[{}] - [{}{}]",
                        style(i * 4).red(),
                        style(x).cyan(),
                        style(&progess_bar_ok).on_green(),
                        progess_bar
                    ))
                    .map_err(TerminalError::Stdout)?;
                thread::sleep(Duration::from_millis(100));
            }
            progess_bar_ok.push(' ');
            progess_bar.pop();
        }
        thread::sleep(Duration::from_millis(1500));

        self.term.show_cursor().map_err(TerminalError::Stdout)?;
        self.term
            .clear_last_lines(1)
            .map_err(TerminalError::Stdout)?;
        self.term
            .write_line("Pressione qualquer tecla para iniciar...")
            .map_err(TerminalError::Stdout)?;
        self.term.read_key().map_err(TerminalError::Stdin)?;
        Ok(())
    }
}

impl UserInterface for Terminal {
    fn input(&self) -> Result<String, TerminalError> {
        self.term.read_line().map_err(TerminalError::Stdin)
    }

    fn press_key(&mut self) -> Result<(), TerminalError> {
        self.write_line("\n\n Pressione qualquer tecla para continuar ...")?;
        self.term.read_char().map_err(TerminalError::Stdin)?;
        Ok(())
    }

    fn welcome(&mut self) -> Result<(), TerminalError> {
        self.term
            .set_title(&format!("{} - TODO-CLI ", Emoji("📝", "")));
        self.write_line(&format!(
            "\n\n\n{}",
            style("TODO-CLI").bold().underlined().blue(),
        ))?;
        self.write_line(&format!(
            "\nDesenvolvido por {}",
            style("TerraMagna & AlphaEdtech").red(),
        ))?;
        self.write_line(&format!("Versão: {}", style(&self.version).bold().green()))?;
        self.write_line(&format!("Author: {}\n\n", style("Diego Oliveira").green()))?;

        self.progress_bar_fake()?;
        self.term.clear_screen().map_err(TerminalError::Stdout)?;

        self.write_line(&format!("{}_>> Bem vindo ao TODO-CLI!", Emoji("😃", ":)")))?;
        thread::sleep(Duration::from_millis(800));
        self.write_line(&format!(
            "{}_>> Aqui você pode adicionar TODOs e ver a lista de TODOs.",
            Emoji("😃", ":)"),
        ))?;
        thread::sleep(Duration::from_millis(800));
        Ok(())
    }

    fn exit(&mut self) -> Result<(), TerminalError> {
        self.write_line(&format!(
            "\n{}_>> {} Obrigado por usar o TODO-CLI! ",
            Emoji("😃", ":)"),
            Emoji("👋", "Tchau.")
        ))?;
        Ok(())
    }

    fn ask_for_action(&mut self) -> Result<Action, TerminalError> {
        self.write_line("\nAguarde ...")?;
        thread::sleep(Duration::from_millis(2000));
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        self.title("BEM VINDO AO TODO CLI")?;
        self.write_line(&format!(
            "{}_>> Olá, como posso te ajudar?",
            Emoji("😃", ":)")
        ))?;
        self.write_line(&format!(
            "{} >> Digite '{}' para adicionar um novo TODO",
            Emoji("✅", ":)"),
            style("a").bold().green()
        ))?;
        self.write_line(&format!(
            "{} >> Digite '{}' para listar os TODOs",
            Emoji("🧾", ":)"),
            style("l").bold().green()
        ))?;
        self.write_line(&format!(
            "{} >> Digite '{}' para selecionar/editar um TODO",
            Emoji("📝", ":)"),
            style("e").bold().green()
        ))?;
        self.write_line(&format!(
            "{} >> Digite '{}' para sair",
            Emoji("👋", ":)"),
            style("x").bold().red()
        ))?;
        loop {
            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                'a' => return Ok(Action::Add),
                'l' => return Ok(Action::List),
                'e' => return Ok(Action::Edit),
                'x' => return Ok(Action::Exit),
                'w' => return Err(TerminalError::Test("AlphaEdtech & TerraMagna".to_string())),
                _ => self.write_line(&format!(
                    "{}_>> Desculpa eu não entendi.",
                    Emoji("🤨", ":/")
                ))?,
            }
        }
    }

    fn ask_for_todo_action(&mut self, id: i32) -> Result<Action, TerminalError> {
        loop {
            self.write_line(&format!(
                "{} >> Digite '{}' para marcar como feito",
                Emoji("✅", ":)"),
                style("f").bold().green()
            ))?;
            self.write_line(&format!(
                "{} >> Digite '{}' para editar",
                Emoji("📝", ":)"),
                style("e").bold().green()
            ))?;
            self.write_line(&format!(
                "{} >> Digite '{}' para deletar",
                Emoji("🗑 ", ":)"),
                style("d").bold().green()
            ))?;
            self.write_line(&format!(
                "{} >> Digite '{}' para voltar",
                Emoji("👈", ":)"),
                style("x").bold().red()
            ))?;

            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                'f' => return Ok(Action::Done(id)),
                'e' => {
                    self.write_line(&format!(
                        "{} >> Digite o novo texto do TODO",
                        Emoji("😃", ":)")
                    ))?;
                    let text = self.input()?;
                    return Ok(Action::Update(id, text));
                }
                'd' => return Ok(Action::Delete(id)),
                'x' => return Ok(Action::Exit),
                _ => self.write_line(&format!(
                    "{}_>> Desculpa eu não entendi.",
                    Emoji("🤨", ":/")
                ))?,
            }
        }
    }

    fn add_todo(&mut self) -> Result<String, TerminalError> {
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        self.title("ADICIONAR TODO")?;

        self.write_line(&format!(
            "{} >> Qual é o novo TODO que gostaria de adicionar?",
            Emoji("😃", ":)")
        ))?;
        let message = self.input()?;
        Ok(message)
    }

    fn select_todo(&mut self) -> Result<String, TerminalError> {
        self.write_line(&format!(
            "\n\n {} >> Informe a chave do Todo que deseja acessar: ",
            Emoji("😃", ":)")
        ))?;
        Ok(self.input()?)
    }

    fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), TerminalError> {
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        self.title("LISTAGEM DOS TODOS")?;
        if !list.is_empty() {
            self.write_line(&format!(
                "{}_>> Você tem {} TODOs cadastrados",
                Emoji("😃", ":)"),
                style(list.len()).red()
            ))?;

            for todo in list {
                let color = if todo.done {
                    Style::new().magenta()
                } else {
                    Style::new().blue()
                };

                self.write_line(&format!(
                    "{} - [{}] {}",
                    Emoji("✅", ":)"),
                    color.apply_to(&todo.id),
                    color.apply_to(&todo.message)
                ))?;
            }
        } else {
            self.write_line(&format!(
                "{}_>> {}",
                Emoji("😃", ":)"),
                style("Você não tem TODOs cadastrados").red()
            ))?;
        }
        Ok(())
    }

    fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), TerminalError> {
        self.write_line(&format!("\n{}_>> O TODO: \n", Emoji("😃", ":)")))?;

        self.write_line(&format!(
            "{} - {}! \n",
            style(&todo).italic().magenta(),
            style(msg).green()
        ))?;
        Ok(())
    }

    fn show_error(&mut self, msg: &str) -> Result<(), TerminalError> {
        self.write_line(&format!("{}_>> {}", Emoji("😕", ":/"), style(msg).red()))?;
        Ok(())
    }
}
