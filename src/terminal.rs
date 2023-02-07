use std::{fmt::Display, thread, time::Duration};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use super::todo::Todo;

use console::{style, Emoji, Style, Term};

pub enum Action {
    Add,
    List,
    Edit,
    Exit,
    Done(u32),
    Delete(u32),
    Update(u32, String),
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
#[async_trait::async_trait]
pub trait UserInterface {
    async fn input(&mut self) -> Result<String, TerminalError>;
    async fn press_key(&mut self) -> Result<(), TerminalError>;
    async fn welcome(&mut self) -> Result<(), TerminalError>;
    async fn exit(&mut self) -> Result<(), TerminalError>;
    async fn ask_for_action(&mut self) -> Result<Action, TerminalError>;
    async fn ask_for_todo_action(&mut self, id: u32) -> Result<Action, TerminalError>;
    async fn add_todo(&mut self) -> Result<String, TerminalError>;
    async fn select_todo(&mut self) -> Result<Option<u32>, TerminalError>;
    async fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), TerminalError>;
    async fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), TerminalError>;
    async fn show_error(&mut self, msg: &str) -> Result<(), TerminalError>;
}

pub struct Terminal {
    stdin: BufReader<io::Stdin>,
    stdout: io::Stdout,
    term: Term,
    version: String,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            stdin: BufReader::new(tokio::io::stdin()),
            stdout: io::stdout(),
            term: Term::stdout(),
            version: "0.8.0".to_string(),
        }
    }

    async fn write_line(&mut self, text: &str) -> Result<(), TerminalError> {
        let text = format!("{}\n", text);
        self.stdout
            .write(text.as_bytes())
            .await
            .map_err(TerminalError::Stdout)?;
        Ok(())
    }

    async fn clean_screen(&mut self) -> Result<(), TerminalError> {
        self.stdout
            .write("\x1Bc\x1B[0K".as_bytes())
            .await
            .map_err(TerminalError::Stdout)?;
        Ok(())
    }

    async fn title(&mut self, text: &str) -> Result<(), TerminalError> {
        self.clean_screen().await?;
        self.write_line(&format!(
            "################# {} ################# \n\n",
            style(text).bold().green(),
        ))
        .await?;
        Ok(())
    }

    /*essa fn irá continuando o term */
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

#[async_trait::async_trait]
impl UserInterface for Terminal {
    async fn input(&mut self) -> Result<String, TerminalError> {
        //self.term.read_line().map_err(TerminalError::Stdin)
        let mut buffer = String::new();
        self.stdin
            .read_line(&mut buffer)
            .await
            .map_err(TerminalError::Stdin)?;

        Ok(buffer.trim().to_string())
    }

    async fn press_key(&mut self) -> Result<(), TerminalError> {
        self.write_line("\n\n Pressione qualquer tecla para continuar ...")
            .await?;
        self.term.read_char().map_err(TerminalError::Stdin)?;
        Ok(())
    }

    async fn welcome(&mut self) -> Result<(), TerminalError> {
        self.term
            .set_title(&format!("{} - TODO-CLI ", Emoji("📝", "")));
        self.write_line(&format!(
            "\n\n\n{}",
            style("TODO-CLI").bold().underlined().blue(),
        ))
        .await?;
        self.write_line(&format!(
            "\nDesenvolvido por {}",
            style("TerraMagna & AlphaEdtech").red(),
        ))
        .await?;
        self.write_line(&format!("Versão: {}", style(&self.version).bold().green()))
            .await?;
        self.write_line(&format!("Author: {}\n\n", style("Diego Oliveira").green()))
            .await?;

        self.progress_bar_fake()?;
        //self.term.clear_screen().map_err(TerminalError::Stdout)?;
        self.clean_screen().await?;

        self.write_line(&format!("{}_>> Bem vindo ao TODO-CLI!", Emoji("😃", ":)")))
            .await?;
        thread::sleep(Duration::from_millis(800));
        self.write_line(&format!(
            "{}_>> Aqui você pode adicionar TODOs e ver a lista de TODOs.",
            Emoji("😃", ":)"),
        ))
        .await?;
        thread::sleep(Duration::from_millis(800));
        Ok(())
    }

    async fn exit(&mut self) -> Result<(), TerminalError> {
        self.write_line(&format!(
            "\n{}_>> {} Obrigado por usar o TODO-CLI! ",
            Emoji("😃", ":)"),
            Emoji("👋", "Tchau.")
        ))
        .await?;
        Ok(())
    }

    async fn ask_for_action(&mut self) -> Result<Action, TerminalError> {
        self.write_line("\nAguarde ...").await?;
        thread::sleep(Duration::from_millis(2000));
        self.title("BEM VINDO AO TODO CLI").await?;
        self.write_line(&format!(
            "{}_>> Olá, como posso te ajudar?",
            Emoji("😃", ":)")
        ))
        .await?;
        self.write_line(&format!(
            "{} >> Digite '{}' para adicionar um novo TODO",
            Emoji("✅", ":)"),
            style("a").bold().green()
        ))
        .await?;
        self.write_line(&format!(
            "{} >> Digite '{}' para listar os TODOs",
            Emoji("🧾", ":)"),
            style("l").bold().green()
        ))
        .await?;
        self.write_line(&format!(
            "{} >> Digite '{}' para selecionar/editar um TODO",
            Emoji("📝", ":)"),
            style("e").bold().green()
        ))
        .await?;
        self.write_line(&format!(
            "{} >> Digite '{}' para sair",
            Emoji("👋", ":)"),
            style("x").bold().red()
        ))
        .await?;
        loop {
            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            self.write_line("").await?; //para quebrar a linha após a resposta
            match answer {
                'a' => return Ok(Action::Add),
                'l' => return Ok(Action::List),
                'e' => return Ok(Action::Edit),
                'x' => return Ok(Action::Exit),
                'w' => return Err(TerminalError::Test("AlphaEdtech & TerraMagna".to_string())),
                _ => {
                    self.write_line(&format!(
                        "{}_>> Desculpa eu não entendi.",
                        Emoji("🤨", ":/")
                    ))
                    .await?
                }
            }
        }
    }

    async fn ask_for_todo_action(&mut self, id: u32) -> Result<Action, TerminalError> {
        loop {
            self.write_line(&format!(
                "{} >> Digite '{}' para marcar como feito",
                Emoji("✅", ":)"),
                style("f").bold().green()
            ))
            .await?;
            self.write_line(&format!(
                "{} >> Digite '{}' para editar",
                Emoji("📝", ":)"),
                style("e").bold().green()
            ))
            .await?;
            self.write_line(&format!(
                "{} >> Digite '{}' para deletar",
                Emoji("🗑 ", ":)"),
                style("d").bold().green()
            ))
            .await?;
            self.write_line(&format!(
                "{} >> Digite '{}' para voltar",
                Emoji("👈", ":)"),
                style("x").bold().red()
            ))
            .await?;

            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                'f' => return Ok(Action::Done(id)),
                'e' => {
                    self.write_line(&format!(
                        "{} >> Digite o novo texto do TODO",
                        Emoji("😃", ":)")
                    ))
                    .await?;
                    let text = self.input().await?;
                    return Ok(Action::Update(id, text));
                }
                'd' => return Ok(Action::Delete(id)),
                'x' => return Ok(Action::Exit),
                _ => {
                    self.write_line(&format!(
                        "{}_>> Desculpa eu não entendi.",
                        Emoji("🤨", ":/")
                    ))
                    .await?
                }
            }
        }
    }

    async fn add_todo(&mut self) -> Result<String, TerminalError> {
        self.title("ADICIONAR TODO").await?;

        self.write_line(&format!(
            "{} >> Qual é o novo TODO que gostaria de adicionar?",
            Emoji("😃", ":)")
        ))
        .await?;
        let message = self.input().await?;
        Ok(message)
    }

    async fn select_todo(&mut self) -> Result<Option<u32>, TerminalError> {
        self.write_line(&format!(
            "\n\n {} >> Informe a chave do Todo que deseja acessar: ",
            Emoji("😃", ":)")
        ))
        .await?;
        let input = self.input().await?;
        if let Ok(id) = input.parse::<u32>() {
            return Ok(Some(id));
        } else {
            return Ok(None);
        }
    }

    async fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), TerminalError> {
        self.title("LISTAGEM DOS TODOS").await?;
        if !list.is_empty() {
            self.write_line(&format!(
                "{}_>> Você tem {} TODOs cadastrados",
                Emoji("😃", ":)"),
                style(list.len()).red()
            ))
            .await?;

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
                ))
                .await?;
            }
        } else {
            self.write_line(&format!(
                "{}_>> {}",
                Emoji("😃", ":)"),
                style("Você não tem TODOs cadastrados").red()
            ))
            .await?;
        }
        Ok(())
    }

    async fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), TerminalError> {
        self.write_line(&format!("\n{}_>> O TODO: \n", Emoji("😃", ":)")))
            .await?;

        self.write_line(&format!(
            "{} - {}! \n",
            style(&todo).italic().magenta(),
            style(msg).green()
        ))
        .await?;
        Ok(())
    }

    async fn show_error(&mut self, msg: &str) -> Result<(), TerminalError> {
        self.write_line(&format!("{}_>> {}", Emoji("😕", ":/"), style(msg).red()))
            .await?;
        Ok(())
    }
}
