use std::{
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration,
};

use super::todo::Todo;

use console::{style, Emoji, Term};

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

struct Terminal {
    term: Term,
}

impl Terminal {
    fn new() -> Self {
        Self {
            //substituição do stdin e stdout por term
            term: Term::stdout(),
        }
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        self.term.read_line().map_err(TerminalError::Stdin)
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        println!(
            "{}_>> Olá, gostaria de adicionar um novo TODO? (s/n)",
            Emoji("😃", ":)")
        );
        loop {
            let answer = self.term.read_char().map_err(TerminalError::Stdin)?;
            match answer {
                's' => return Ok(Some(self.add_todo()?)),
                'n' => return Ok(None),
                'x' => return Err(TerminalError::Test("AlphaEdtech & TerraMagna".to_string())),
                _ =>  println!("{}_>> Desculpa eu não entendi. Digite 's' se deseja adicionar um novo TODO ou 'n' se deseja sair. ", Emoji("🤨",":/"))
            }
        }
    }

    fn add_todo(&mut self) -> Result<Todo, TerminalError> {
        println!("{} >> Qual é o TODO?", Emoji("😃", ":)"));
        let message = self.input()?;
        Ok(Todo::new(message))
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(
            self.term,
            "\n{}_>> O TODO foi adicionado com sucesso! \n",
            Emoji("😃", ":)")
        )
        .map_err(TerminalError::Stdout)?;
        writeln!(
            self.term,
            "{} - {:?} \n",
            Emoji("✅", ":)"),
            style(todo).italic().magenta()
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

    fn welcome(&mut self) -> Result<(), TerminalError> {
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
        writeln!(self.term, "Versão: {}", style("0.4.0").bold().green())
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

pub fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    terminal.welcome()?;
    loop {
        if let Some(todo) = terminal.ask_for_new_todo()? {
            terminal.show_todo(&todo)?;
        } else {
            println!(
                "\n{}_>> {} Obrigado por usar o TODO-CLI! ",
                Emoji("😃", ":)"),
                Emoji("👋", "Tchau.")
            );
            return Ok(());
        }
    }
}
