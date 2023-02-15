use super::todo::Todo;
use crate::cli::AppError;
use std::{thread, time::Duration};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

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

#[async_trait::async_trait]
pub trait UserInterface {
    async fn input(&mut self) -> Result<String, AppError>;
    async fn press_key(&mut self) -> Result<(), AppError>;
    async fn welcome(&mut self) -> Result<(), AppError>;
    async fn exit(&mut self) -> Result<(), AppError>;
    async fn ask_for_action(&mut self) -> Result<Action, AppError>;
    async fn ask_for_todo_action(&mut self, id: u32) -> Result<Action, AppError>;
    async fn add_todo(&mut self) -> Result<String, AppError>;
    async fn select_todo(&mut self) -> Result<Option<u32>, AppError>;
    async fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), AppError>;
    async fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), AppError>;
    async fn show_error(&mut self, msg: &str) -> Result<(), AppError>;
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
            version: String::from("0.9.0"),
        }
    }

    async fn write_line(&mut self, text: &str) -> Result<(), AppError> {
        let text = format!("{text}\n");
        self.stdout
            .write(text.as_bytes())
            .await
            .map_err(AppError::Stdout)?;
        Ok(())
    }

    async fn clean_screen(&mut self) -> Result<(), AppError> {
        self.stdout
            .write("\x1Bc\x1B[0K".as_bytes())
            .await
            .map_err(AppError::Stdout)?;
        Ok(())
    }

    async fn title(&mut self, text: &str) -> Result<(), AppError> {
        self.clean_screen().await?;
        self.write_line(&format!(
            "################# {} ################# \n\n",
            style(text).bold().green(),
        ))
        .await?;
        Ok(())
    }

    /*essa fn irá continuando o term */
    fn progress_bar_fake(&mut self) -> Result<(), AppError> {
        let mut progess_bar = String::new();
        let mut progess_bar_ok = String::new();
        self.term.hide_cursor().map_err(AppError::Stdout)?;
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
                self.term.clear_last_lines(1).map_err(AppError::Stdout)?;
                self.term
                    .write_line(&format!(
                        "Carregando ... {}% -[{}] - [{}{}]",
                        style(i * 4).red(),
                        style(x).cyan(),
                        style(&progess_bar_ok).on_green(),
                        progess_bar
                    ))
                    .map_err(AppError::Stdout)?;
                thread::sleep(Duration::from_millis(100));
            }
            progess_bar_ok.push(' ');
            progess_bar.pop();
        }
        thread::sleep(Duration::from_millis(1500));

        self.term.show_cursor().map_err(AppError::Stdout)?;
        self.term.clear_last_lines(1).map_err(AppError::Stdout)?;
        self.term
            .write_line("Pressione qualquer tecla para iniciar...")
            .map_err(AppError::Stdout)?;
        self.term.read_key().map_err(AppError::Stdin)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserInterface for Terminal {
    async fn input(&mut self) -> Result<String, AppError> {
        //self.term.read_line().map_err(AppError::Stdin)
        let mut buffer = String::new();
        self.stdin
            .read_line(&mut buffer)
            .await
            .map_err(AppError::Stdin)?;

        Ok(buffer.trim().to_string())
    }

    async fn press_key(&mut self) -> Result<(), AppError> {
        self.write_line("\n\n Pressione qualquer tecla para continuar ...")
            .await?;
        self.term.read_char().map_err(AppError::Stdin)?;
        Ok(())
    }

    async fn welcome(&mut self) -> Result<(), AppError> {
        self.term
            .set_title(format!("{} - TODO-CLI ", Emoji("📝", "")));
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
        //self.term.clear_screen().map_err(AppError::Stdout)?;
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

    async fn exit(&mut self) -> Result<(), AppError> {
        self.write_line(&format!(
            "\n{}_>> {} Obrigado por usar o TODO-CLI! ",
            Emoji("😃", ":)"),
            Emoji("👋", "Tchau.")
        ))
        .await?;
        Ok(())
    }

    async fn ask_for_action(&mut self) -> Result<Action, AppError> {
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
            let answer = self.term.read_char().map_err(AppError::Stdin)?;
            self.write_line("").await?; //para quebrar a linha após a resposta
            match answer {
                'a' => return Ok(Action::Add),
                'l' => return Ok(Action::List),
                'e' => return Ok(Action::Edit),
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

    async fn ask_for_todo_action(&mut self, id: u32) -> Result<Action, AppError> {
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

            let answer = self.term.read_char().map_err(AppError::Stdin)?;
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

    async fn add_todo(&mut self) -> Result<String, AppError> {
        self.title("ADICIONAR TODO").await?;

        self.write_line(&format!(
            "{} >> Qual é o novo TODO que gostaria de adicionar?",
            Emoji("😃", ":)")
        ))
        .await?;
        let message = self.input().await?;
        Ok(message)
    }

    async fn select_todo(&mut self) -> Result<Option<u32>, AppError> {
        self.write_line(&format!(
            "\n\n {} >> Informe a chave do Todo que deseja acessar: ",
            Emoji("😃", ":)")
        ))
        .await?;
        let input = self.input().await?;
        if let Ok(id) = input.parse::<u32>() {
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }

    async fn list_todo(&mut self, list: Vec<&Todo>) -> Result<(), AppError> {
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

    async fn show_sucess(&mut self, todo: &Todo, msg: &str) -> Result<(), AppError> {
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

    async fn show_error(&mut self, msg: &str) -> Result<(), AppError> {
        self.write_line(&format!("{}_>> {}", Emoji("😕", ":/"), style(msg).red()))
            .await?;
        Ok(())
    }
}
