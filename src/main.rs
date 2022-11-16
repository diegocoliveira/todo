use std::io::{Stdin, Stdout, Write};

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Self { message }
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

    fn input(&mut self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        println!("😃_>> Olá, gostaria de adicionar um novo TODO? (s/n) ");
        loop {
            let answer = self.input();
            if answer == "s" {
                println!("😃 >> Qual é o TODO?");
                let message = self.input();
                let todo = Todo::new(message);
                return todo;
            } else if answer == "n" {
                println!("😃_>> Obrigado por usar o TODO-CLI! 👋");
                std::process::exit(0);
            } else {
                println!("🤨_>> Desculpa eu não entendi. Digite 's' se deseja adicionar um novo TODO ou 'n' se deseja sair. ");
            }
        }
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "\n😃_>> O TODO foi adicionado com sucesso! \n").unwrap();
        writeln!(self.stdout, "📝 - {:?} \n", todo).unwrap();
    }
}

fn main() {
    let mut terminal = Terminal::new();
    loop {
        let todo = terminal.ask_for_new_todo();
        terminal.show_todo(&todo);
    }
}
