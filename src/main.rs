/*
   no caso da variável answer é mais interessando usar ela mutável ou sombreamento?
*/

fn main() {
    let mut answer:String; // sem essa linha
    println!("😃_>> Olá, gostaria de adicionar um novo TODO? (s/n) ");
    loop {
        answer = input(); //essa linha no lugar: let answer = input();
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
    let todo = input();
    println!("\n😃_>> O TODO foi adicionado com sucesso! \n");
    println!("📝 - {} \n", todo);
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
