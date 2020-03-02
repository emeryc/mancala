use mancala::{ayoayo::Ayoayo, GameState, MancalaError};

use rustyline::error::ReadlineError;
use rustyline::Editor;

enum Command {
    Quit,
    UnknownCommand(String),
    Play(usize),
}

fn main() {
    let mut game = Ayoayo::new();
    println!("{}", game);
    let mut rl = Editor::<()>::new();
    loop {
        let readline = match game.state {
            GameState::Draw => {
                println!("Nobody Won?");
                break;
            }
            GameState::Won(player) => {
                println!("{} Won!", player);
                break;
            }
            GameState::InProgress(player) => rl.readline(format!("{}'s Turn: ", player).as_ref()),
        };
        match readline.map(string_to_command) {
            Ok(Command::Play(size)) if size > 0 => match game.play(size - 1) {
                Ok(()) => {
                    println!("{}", game);
                }
                Err(MancalaError::MustFeedError) => println!("You must feed your oponent seeds"),
                Err(MancalaError::NoSeedsToSow) => println!("The cup you chose has no seeds"),
                Err(MancalaError::NoSuchCup) => println!("The cup you chose doesn't exist"),
            },
            Ok(Command::Play(_)) => println!("The cup you chose doesn't exist"),
            Ok(Command::Quit) => break,
            Ok(Command::UnknownCommand(command)) => {
                println!("Command not found: {}", command);
                break;
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn string_to_command(string: String) -> Command {
    match string.trim() {
        "quit" => Command::Quit,
        c if c.parse::<usize>().is_ok() => {
            Command::Play(c.parse::<usize>().expect("already tested"))
        }
        line => Command::UnknownCommand(String::from(line)),
    }
}
