use std::{
    fmt::Display,
    io::{self, Stdin, Stdout, Write},
};

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Self { message }
    }
}

enum TerminalError {
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
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)?;
        Ok(buf.trim().to_string())
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        println!("😃_>> Olá, gostaria de adicionar um novo TODO? (s/n) ");
        loop {
            let answer = self.input()?;
            match answer.as_str() {
                "s" => return Ok(Some(self.add_todo()?)),
                "n" => return Ok(None),
                "xyz" => return Err(TerminalError::Test("AlphaEdtech & TerraMagna".to_string())),
                _ =>  println!("🤨_>> Desculpa eu não entendi. Digite 's' se deseja adicionar um novo TODO ou 'n' se deseja sair. ")
            }
        }
    }

    fn add_todo(&mut self) -> Result<Todo, TerminalError> {
        println!("😃 >> Qual é o TODO?");
        let message = self.input()?;
        Ok(Todo::new(message))
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "\n😃_>> O TODO foi adicionado com sucesso! \n")
            .map_err(TerminalError::Stdout)?;
        writeln!(self.stdout, "📝 - {:?} \n", todo).map_err(TerminalError::Stdout)?;
        Ok(())
    }
}

fn new_todo() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    loop {
        if let Some(todo) = terminal.ask_for_new_todo()? {
            terminal.show_todo(&todo)?;
        } else {
            println!("\n😃_>> Obrigado por usar o TODO-CLI! 👋");
            return Ok(());
        }
    }
}

fn main() {
    if let Err(err) = new_todo() {
        println!(
            "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado."
        );
        println!("\n🤨_>> Erro: {}", err);
    }
}
