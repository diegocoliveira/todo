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

fn main() {
    println!("😃_>> Olá, gostaria de adicionar um novo TODO? (s/n) ");
    loop {
        let answer = input();
        if answer == "s" {
            add_todo();
            println!("😃_>> Gostaria de adicionar outro TODO? (s/n) ");
        } else if answer == "n" {
            println!("😃_>> Obrigado por usar o TODO-CLI! 👋");
            break;
        } else {
            println!("🤨_>> Desculpa eu não entendi. Digite 's' se deseja adicionar um novo TODO ou 'n' se deseja sair. ");
        }
    }
}

fn add_todo() {
    println!("😃 >> Qual é o TODO?");
    let message = input();
    let todo = Todo::new(message);
    println!("\n😃_>> O TODO foi adicionado com sucesso! \n");
    println!("📝 - {:?} \n", todo);
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
