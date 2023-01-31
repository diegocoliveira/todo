mod cli;
mod terminal;
mod todo;

use cli::TodoCli;
use console::style;

fn main() {
    let mut todo_cli = TodoCli::new(
        Box::new(terminal::Terminal::new()),
        Box::new(todo::Todos::new()),
    );
    if let Err(err) = todo_cli.run() {
        println!(
            "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado.",
        );
        println!("\n🤨_>> Erro: {}", style(err).red());
    }
}
