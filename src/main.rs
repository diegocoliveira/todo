mod cli;
mod terminal;
mod todo;

use cli::TodoCli;
use console::style;

#[tokio::main]
async fn main() {
    let storage: Box<dyn todo::TodoStorage> =
    match todo::Todos::new().await {
        Ok(todos) => Box::new(todos),
        Err(err) => {
            println!(
                "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado.",
            );
            println!("\n🤨_>> Erro: {}", style(err).red());
            return;
        }
    };

    let mut todo_cli = TodoCli::new(Box::new(terminal::Terminal::new()), storage);
    if let Err(err) = todo_cli.run().await {
        println!(
            "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado.",
        );
        println!("\n🤨_>> Erro: {}", style(err).red());
    }
}
