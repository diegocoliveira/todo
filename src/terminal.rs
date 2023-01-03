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

pub struct Terminal {
    term: Term,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            //substituição do stdin e stdout por term
            term: Term::stdout(),
        }
    }

    fn input(&self) -> Result<String, TerminalError> {
        self.term.read_line().map_err(TerminalError::Stdin)
    }
    pub fn press_key(&self) -> Result<(), TerminalError> {
        println!("Pressione qualquer tecla para continuar ...");
        self.term.read_char().map_err(TerminalError::Stdin)?;
        Ok(())
    }

    pub fn ask_for_todo_action(&self, id: i32) -> Result<Action, TerminalError> {
        loop {
            println!("{} >> Digite 'f' para marcar como feito", Emoji("✅", ":)"));
            println!("{} >> Digite 'e' para editar", Emoji("📝", ":)"));
            println!("{} >> Digite 'd' para deletar", Emoji("🗑 ", ":)"));
            println!("{} >> Digite 'x' para voltar", Emoji("👈", ":)"));

            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                'f' => return Ok(Action::Done(id)),
                'e' => {
                    println!("{} >> Digite o novo texto do TODO", Emoji("😃", ":)"));
                    let text = self.input()?;
                    return Ok(Action::Update(id, text));
                }
                'd' => return Ok(Action::Delete(id)),
                'x' => return Ok(Action::Exit),
                _ => println!("{}_>> Desculpa eu não entendi.", Emoji("🤨", ":/")),
            }
        }
    }

    pub fn ask_for_action(&self) -> Result<Action, TerminalError> {
        println!("\nAguarde ...");
        thread::sleep(Duration::from_millis(2000));
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        println!(
            "################# {} ################# \n\n",
            "BEM VINDO AO TODO CLI"
        );
        println!("{}_>> Olá, como posso te ajudar?", Emoji("😃", ":)"));
        println!(
            "{} >> Digite 'a' para adicionar um novo TODO",
            Emoji("✅", ":)")
        );
        println!("{} >> Digite 'l' para listar os TODOs", Emoji("📝", ":)"));
        println!("{} >> Digite 'x' para sair", Emoji("👋", ":)"));
        loop {
            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                'a' => return Ok(Action::Add),
                'l' => return Ok(Action::List),
                'x' => return Ok(Action::Exit),
                'w' => return Err(TerminalError::Test("AlphaEdtech & TerraMagna".to_string())),
                _ =>  println!("{}_>> Desculpa eu não entendi. Digite 'a' para adicionar um novo TODO, 'l' para listar os TODOs ou 'x' para sair. ", Emoji("🤨",":/"))
            }
        }
    }

    pub fn add_todo(&self) -> Result<String, TerminalError> {
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        println!(
            "################# {} ################# \n\n",
            "ADICIONAR TODO"
        );

        println!(
            "{} >> Qual é o novo TODO que gostaria de adicionar?",
            Emoji("😃", ":)")
        );
        let message = self.input()?;
        Ok(message)
    }

    pub fn list_todos(&self, list: Vec<&Todo>) -> Result<Option<String>, TerminalError> {
        self.term.clear_screen().map_err(TerminalError::Stdout)?;
        println!(
            "################# {} ################# \n\n",
            "LISTAGEM DOS TODOS"
        );
        if !list.is_empty() {
            writeln!(
                &self.term,
                "{}_>> Você tem {} TODOs cadastrados",
                Emoji("😃", ":)"),
                style(list.len()).red()
            )
            .map_err(TerminalError::Stdout)?;

            for todo in list {
                let color = if todo.done {
                    Style::new().magenta()
                } else {
                    Style::new().blue()
                };

                writeln!(
                    &self.term,
                    "{} - [{}] {}",
                    Emoji("✅", ":)"),
                    color.apply_to(&todo.id),
                    color.apply_to(&todo.message),
                )
                .map_err(TerminalError::Stdout)?;
            }
            println!(
                "\n\n {} >> Digite 'x' para voltar ou informe a chave do Todo que deseja acessar: ",
                Emoji("😃", ":)")
            );
            let answer = self.input()?;
            if answer == "x" {
                return Ok(None);
            } else {
                return Ok(Some(answer));
            }
        } else {
            writeln!(
                &self.term,
                "{}_>> {}",
                Emoji("😃", ":)"),
                style("Você não tem TODOs cadastrados").red()
            )
            .map_err(TerminalError::Stdout)?;
        }
        Ok(None)
    }

    pub fn show_todo(&self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(
            &self.term,
            "\n{}_>> O TODO foi adicionado com sucesso! \n",
            Emoji("😃", ":)")
        )
        .map_err(TerminalError::Stdout)?;
        writeln!(
            &self.term,
            "{} - {} \n",
            Emoji("✅", ":)"),
            style(&todo.message).italic().magenta()
        )
        .map_err(TerminalError::Stdout)?;
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

    pub fn welcome(&mut self) -> Result<(), TerminalError> {
        self.term
            .set_title(&format!("{} - TODO-CLI ", Emoji("📝", "")));
        writeln!(
            self.term,
            "\n\n\n{}",
            style("TODO-CLI").bold().underlined().blue()
        )
        .map_err(TerminalError::Stdout)?;
        writeln!(
            self.term,
            "\nDesenvolvido por {}",
            style("TerraMagna & AlphaEdtech").red()
        )
        .map_err(TerminalError::Stdout)?;
        writeln!(self.term, "Versão: {}", style("0.5.0").bold().green())
            .map_err(TerminalError::Stdout)?;
        writeln!(self.term, "Author: {}\n\n", style("Diego Oliveira").green())
            .map_err(TerminalError::Stdout)?;

        self.progress_bar_fake()?;
        self.term.clear_screen().map_err(TerminalError::Stdout)?;

        writeln!(self.term, "{}_>> Bem vindo ao TODO-CLI!", Emoji("😃", ":)"))
            .map_err(TerminalError::Stdout)?;
        thread::sleep(Duration::from_millis(800));
        writeln!(
            self.term,
            "{}_>> Aqui você pode adicionar TODOs e ver a lista de TODOs.",
            Emoji("😃", ":)")
        )
        .map_err(TerminalError::Stdout)?;
        thread::sleep(Duration::from_millis(800));
        Ok(())
    }
}
