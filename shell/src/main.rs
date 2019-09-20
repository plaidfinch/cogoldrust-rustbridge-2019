use std::io;
use std::io::prelude::*;

mod command_output;
mod commands;
mod error;
mod shell_command;

use crate::command_output::*;
use shell_command::ShellCommand;

fn main() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    for line in stdin.lock().lines() {
        let user_input = line.unwrap();
        match ShellCommand::create_shell_command(&user_input) {
            Ok(command) => match command.execute_shell_command() {
                Ok(output) => output.print_command(),
                Err(e) => println!("Error: {:?}", e),
            },
            Err(e) => {
                println!("Unable to parse command: {:?}", e);
            }
        }

        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
    }
}
