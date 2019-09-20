use crate::commands::more_file;
use crate::error::ShellError;
use crate::CommandOutput;
use std::path::PathBuf;

use crate::commands::*;

#[derive(Debug, Clone)]
pub enum ShellCommand {
    More(PathBuf),
    Cat(Vec<PathBuf>),
    Ls(PathBuf),
    Cd(PathBuf),
    FindFile(PathBuf, PathBuf),
    // Can take input from  pipe.
    FindString(Option<String>, String),

    // Can take input from  pipe.
    // SubsString(contents, from, to)
    SubsString(Option<String>, String, String),
    Pipe(Box<ShellCommand>, Box<ShellCommand>),
}

impl ShellCommand {
    pub fn create_shell_command(cli_input: &str) -> Result<ShellCommand, ShellError> {
        let commands: Vec<&str> = cli_input.split('|').collect();

        for c in &commands {
            if c.is_empty() {
                return Err(ShellError::EmptyCommandInPipe(c.to_string()));
            }
        }

        fn make_pipe(list: &[&str]) -> Result<ShellCommand, ShellError> {
            match list {
                // base case: single element left.
                &[last] => ShellCommand::parse_single_command(last),
                list => {
                    // Indexing guarnteed to work due to pattern matching.
                    let first = Box::new(ShellCommand::parse_single_command(list[0])?);
                    let second = Box::new(make_pipe(&list[1..])?);
                    Ok(ShellCommand::Pipe(first, second))
                }
            }
        }

        make_pipe(&commands)
    }

    fn parse_single_command(command: &str) -> Result<ShellCommand, ShellError> {
        // Check for pipe, reject if pipe present.
        // Check for empty strings.
        let command = command.trim();
        let parts: Vec<&str> = command.split(" ").collect();

        match parts.as_slice() {
            ["more", path] => Ok(ShellCommand::More(PathBuf::from(path))),
            ["ls"] => Ok(ShellCommand::Ls(PathBuf::from("."))),
            ["ls", path] => Ok(ShellCommand::Ls(PathBuf::from(path))),
            ["cd", path] => Ok(ShellCommand::Cd(PathBuf::from(path))),
            ["find-file", path, dir] => Ok(ShellCommand::FindFile(
                PathBuf::from(path),
                PathBuf::from(dir),
            )),
            // Find string with 2 arguments.
            ["find-string", content, string] => Ok(ShellCommand::FindString(
                Some(content.to_string()),
                string.to_string(),
            )),
            // Find string with 1 argument, used for a pipe.
            ["find-string", string] => Ok(ShellCommand::FindString(None, string.to_string())),
            ["subs-string", content, from, to] => Ok(ShellCommand::SubsString(
                Some(content.to_string()),
                from.to_string(),
                to.to_string(),
            )),
            ["subs-string", from, to] => Ok(ShellCommand::SubsString(
                None,
                from.to_string(),
                to.to_string(),
            )),

            parts => {
                // Hard cases, Rust's pattern matching is unable to
                // handle their matching. Specifically:
                // cat since it takes multiple files.
                if parts[0] == "cat" {
                    let args = &parts[1..];
                    if args.is_empty() {
                        return Err(ShellError::KnownCommandWrongArgs(command.to_string()));
                    } else {
                        let args: Vec<PathBuf> =
                            args.into_iter().map(|s| PathBuf::from(s)).collect();
                        return Ok(ShellCommand::Cat(args));
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

                // known command, but wrong number of arguments passed to it...
                if known_commands.contains(&parts[0]) {
                    Err(ShellError::KnownCommandWrongArgs(command.to_string()))
                } else {
                    Err(ShellError::UnknownCommand(command.to_string()))
                }
            }
        }
    }

    pub fn execute_shell_command(&self) -> Result<CommandOutput, ShellError> {
        match self {
            ShellCommand::More(path) => Ok(CommandOutput::Single(more_file(path)?)),
            ShellCommand::Cat(paths) => cat_files(paths),
            ShellCommand::Ls(path) => ls_dir(path),
            ShellCommand::Cd(path) => cd(path),
            ShellCommand::FindFile(starting_dir, search_name) => {
                find_file(starting_dir, search_name)
            }
            ShellCommand::FindString(Some(content), search_str) => {
                Ok(find_string(content, search_str))
            }
            ShellCommand::FindString(None, _) => {
                // Return my own error.
                Err(ShellError::ExpectedPipeInput(self.clone()))
            }
            ShellCommand::SubsString(Some(content), from, to) => Ok(subs_string(content, from, to)),
            ShellCommand::SubsString(None, _, _) => {
                Err(ShellError::ExpectedPipeInput(self.clone()))
            }
            // Pipe
            ShellCommand::Pipe(c1, c2) => {
                let cmd_output = c1.execute_shell_command()?;
                match &**c2 {
                    ShellCommand::FindString(None, search_str) => {
                        let input = cmd_output
                            .to_string()
                            .ok_or(ShellError::NoInputForPipe(self.clone()))?;
                        Ok(find_string(&input, &search_str))
                    }
                    ShellCommand::FindString(Some(_), _) => {
                        Err(ShellError::UnexpectedPipeInput(self.clone()))
                    }
                    ShellCommand::SubsString(None, from, to) => {
                        let input = cmd_output
                            .to_string()
                            .ok_or(ShellError::NoInputForPipe(self.clone()))?;
                        Ok(subs_string(&input, from, to))
                    }
                    _ => Err(ShellError::PipeInputNotAccepted(self.clone())),
                }
            }
        }
    }
}
