mod database;

use std::{
    io::{stdin, Result},
    process,
};

use database::{Database, Entity};
use sdb::{Command, Lexer, Token};

pub struct App {
    database: Database,
}

impl App {
    fn read_input() -> Result<String> {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }
    fn process_cmd(&mut self, command: Command, lexer: &mut Lexer) {
        let valid_commands = vec![Command::SELECT, Command::INSERT];
        assert!(valid_commands.contains(&command));
        match command {
            Command::SELECT => {
                self.database.select();
            }
            Command::INSERT => {
                let id_opt = lexer.next();
                let user_name_opt = lexer.next();
                let email_opt = lexer.next();
                match (id_opt, user_name_opt, email_opt) {
                    (
                        Some(Token::Number(id)),
                        Some(Token::AlphaNumeric(user_name)),
                        Some(Token::AlphaNumeric(email)),
                    ) => {
                        self.database.insert(Entity::new(id, user_name, email));
                    }
                    _ => {
                        eprintln!("Invalid entity")
                    }
                }
            }
            _ => {
                unreachable!("Should never reach here!!")
            }
        }
    }
    fn evaluate_input(&mut self, buffer: String) -> Result<()> {
        let mut lexer = Lexer::new(buffer);
        match lexer.next() {
            Some(Token::Command(command)) => {
                self.process_cmd(command, &mut lexer);
            }
            Some(Token::SpecialChar('.')) => {
                if let Some(Token::Command(cli_cmd)) = lexer.next() {
                    if cli_cmd == Command::EXIT {
                        process::exit(1);
                    }
                }
            }
            Some(_) | None => {
                eprintln!("[ERROR]: Invalid command")
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        App {
            database: Database::new(),
        }
    }

    pub fn repl(&mut self) -> Result<()> {
        loop {
            let buf = Self::read_input()
                .inspect_err(|err| eprintln!("Error reading input str {err:?}"))?;
            self.evaluate_input(buf)?;
        }
    }
}
