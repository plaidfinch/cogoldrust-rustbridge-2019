//! Command line shell which accepts common shell commands.
//! Easily extendible to do more commands.
//! Does not "fork" work to underlying shell. Does all commands
//! using IO.

use std::io;
use std::io::prelude::*;

mod command_output;
mod commands;
mod error;
mod shell_command;

use crate::command_output::*;
use shell_command::ShellCommand;

/// Main read eval loop for shell.
/// Keeps accepting commands, parses them into a ShellCommand,
/// and executes the command. Either prints CommandOutput, or prints
/// error.
fn main() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    for line in stdin.lock().lines() {
        let user_input = line.unwrap();

        // Step #20 ShellCommand functions and CommandOutput
        // function to parse the user input, execute the command.
        // and print it!
        unimplemented!();

        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
    }
}
