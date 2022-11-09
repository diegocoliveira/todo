/* tentei usar print! para que o cursor ficasse ao lado da frase, mas não funcionou quando eu chamo a função input
por isso usei println!. Não entendi o motivo deve ser a forma que ler a linha do stdin*/

fn main() {
    println!("😃_>> Olá, gostaria de adicionar um novo TODO? (s/n) ");
    let mut answer = input();
    loop {
        if answer == "s" {
            add_todo();
            println!("😃_>> Gostaria de adicionar outro TODO? (s/n) ");
            answer = input();
        } else if answer == "n" {
            println!("😃_>> Obrigado por usar o TODO-CLI! 👋");
            break;
        } else {
            println!("🤨_>> Desculpa eu não entendi. Digite 's' se deseja adicionar um novo TODO ou 'n' se deseja sair. ");
            answer = input();
        }
    }
}

fn add_todo() {
    println!("😃 >> Qual é o TODO?");
    let todo = input();
    println!("\n😃_>> O TODO foi adicionado com sucesso! \n");
    println!("📝 - {} \n", todo);
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
