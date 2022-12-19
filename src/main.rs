mod terminal;
mod todo;

/** steps
1) create mod: Todo, Terminal
2) split into files
3) refactory cli

3.1 replace stdout and stdin for term (write_line, read_char, read_line_initial_text, read_line, clear_screen, set_title)
3.2 use struct Emoji
3.3 use style
3.4 use thread:sleep

*/
use console::{self, style};

fn main() {
    if let Err(err) = terminal::run() {
        println!(
            "\n🤨_>> Desculpa aconteceu um erro no sistema e o sistema teve que ser encerrado."
        );
        println!("\n🤨_>> Erro: {}", style(err).red());
    }
}
