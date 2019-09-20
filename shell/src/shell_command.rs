use crate::commands::more_file;
use crate::error::ShellError;
use crate::CommandOutput;
use std::path::PathBuf;

use crate::commands::*;

// Step #12
// Study the ShellCommand enum.
// Notice the variants correspond to the command you implemented.

/// Represents all the possible commands that our shell can execute.
/// Each one of these variants has a corresponding function in
/// `commands` which does the actual work!
#[derive(Debug, Clone)]
pub enum ShellCommand {
    /// Print the contents of file to screen.
    More(PathBuf),
    /// Takes 0 or more files and concats their contents.
    Cat(Vec<PathBuf>),
    /// Print all entries in a directory.
    Ls(PathBuf),
    /// Change current working directory.
    Cd(PathBuf),
    /// Look for file name recursively under directory.
    FindFile(String, PathBuf),
    /// Returns all lines that match the second string, in the
    /// first string.
    /// If Option is None, this command is expecting pipe input.
    /// Can take input from  pipe.
    /// FindString(content, matcher)
    FindString(Option<String>, String),
    /// Replaces all instances of `from` with `to` in
    /// contents.
    /// Can take input from  pipe.
    /// SubsString(contents, from, to)
    SubsString(Option<String>, String, String),
    /// Allows output of first command to be piped into second command.
    Pipe(Box<ShellCommand>, Box<ShellCommand>),
}

impl ShellCommand {
    /// Creates the new shell command. Returns ShellError if unable to parse command.
    /// Possible errors:
    pub fn create_shell_command(cli_input: &str) -> Result<ShellCommand, ShellError> {
        // Pipe command. Come back only after you have implemented
        // the full shell for single commands!!!

        // Split commands by pipes.
        let commands: Vec<&str> = unimplemented!();

        for c in &commands {
            if c.is_empty() {
                return Err(ShellError::EmptyCommandInPipe(c.to_string()));
            }
        }

        // Write the recursive helper function that creates a "tree"
        // of commands using ShellCommand::Pipe.
        // This is pretty tricky.
        // Use the function parse_single_command below.
        // We recommend copying the solution for this code.
        // Unless you really wanna try it.
        // Either way, study how the solution works.
        fn make_pipe(list: &[&str]) -> Result<ShellCommand, ShellError> {
            match list {
                // base case: single element left.
                &[last] => unimplemented!(),
                list => {
                    // Indexing guaranteed to work due to pattern matching.
                    unimplemented!()
                }
            }
        }

        make_pipe(&commands)
    }

    /// Parses a single command that does not contain any pipes.
    fn parse_single_command(command: &str) -> Result<ShellCommand, ShellError> {
        // Remove whitespace from command.
        let command = command.trim();

        // #Step 13
        // Split command by space.
        let parts: Vec<&str> = unimplemented!();

        // Step #14
        // Implement the code for most basic commands:
        // ls, more, cd, subs-string, find-string and cat.
        match parts.as_slice() {
            // Handle case for ls with no arguments.
            ["ls"] => Ok(ShellCommand::Ls(PathBuf::from("."))),
            ["ls", path] => unimplemented!(),
            ["more", path] => unimplemented!(),
            ["cd", path] => unimplemented!(),
            ["find-file", path, dir] => Ok(ShellCommand::FindFile(
                path.to_string(),
                PathBuf::from(dir),
            )),
            ["find-string", content, string] => unimplemented!(),

            // Find string with 1 argument, used for a pipe.
            // Requires pipes! Do later once you have finished shell
            // with no pipes!
            ["find-string", string] => unimplemented!(),
            ["subs-string", content, from, to] => unimplemented!(),

            // Requires pipes! Do later once you have finished shell
            // with no pipes!
            ["subs-string", from, to] => unimplemented!(),

            parts => {
                // Hard cases, Rust's pattern matching is unable to handle their
                // matching. Specifically `cat` since it takes multiple files.
                if parts[0] == "cat" {
                    let args = &parts[1..];
                    if args.is_empty() {
                        // Step #15
                        // return KnownCommandWrongArgs error!
                    } else {
                        // Step #16
                        // Create Cat command!
                    }
                }

                // Look to see if this is a known command with the wrong # of arguments.
                let known_commands = [
                    "more",
                    "cat",
                    "ls",
                    "cd",
                    "find-string",
                    "subs-string",
                    "find-file",
                ];
                // Step #17
                // Check if this is a known command, if so return
                // KnownCommandWrongArgs

                // Step #18
                // Else return UnknownCommand
                unimplemented!()
                }
        }
    }


    /// Take the command, and call the corresponding commands::function
    /// for the command.
    /// Handles piped commands by piping their input together.
    pub fn execute_shell_command(&self) -> Result<CommandOutput, ShellError> {
        // Match on the ShellCommand calling the right function from command.
        // Ignore pipes from now. Save them until you have single commands working!
        // Step #19
        unimplemented!()
    }
}
