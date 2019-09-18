use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::fmt::Debug;

/// Variants that take a String failed during parsing.
/// Variants with a ShellCommand failed during evaluation.
#[derive(Debug)]
enum ShellError {
    IoError(io::Error),
    EmptyCommandInPipe(String),
    KnownCommandWrongArgs(String),
    /// During parsing the command was no recognized.
    UnknownCommand(String),
    ExpectedPipeInput(ShellCommand),
    UnexpectedPipeInput(ShellCommand),
    PipeInputNotAccepted(ShellCommand),
    // Command produces no input for pipe.
    NoInputForPipe(ShellCommand),
}

#[derive(Clone, Debug)]
enum CommandOutput {
    Single(String),
    List(Vec<String>),
    None,
}

impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::IoError(error)
    }
}

#[derive(Debug, Clone)]
enum ShellCommand {
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
    Pipe(Box<ShellCommand>, Box<ShellCommand>)
}

fn main() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    for line in stdin.lock().lines() {

        let user_input = line.unwrap();
        match create_shell_command(&user_input) {
            Ok(command) => {
                let output = execute_shell_command(&command);
                output_results(output);
            }
            Err(e) => {
                println!("Unable to parse command: {:?}", e);
            }
        }


        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
    }
}

fn output_results(r: Result<CommandOutput, ShellError>) {
    match r {
        Ok(CommandOutput::Single(s)) => println!("{}", s),
        Ok(CommandOutput::List(ss)) => {
            for s in ss {
                println!("{}", s);
            }
        }
        Ok(CommandOutput::None) => {}
        Err(e) => println!("Error: {:?}", e),
    }
}

fn create_shell_command(cli_input: &str) ->  Result<ShellCommand, ShellError> {
    let commands: Vec<&str> = cli_input.split('|').collect();

    for c in &commands {
        if c.is_empty() {
            return Err(ShellError::EmptyCommandInPipe(c.to_string()))
        }
    }

    fn make_pipe(list: &[&str]) -> Result<ShellCommand, ShellError> {
        match list {
            // base case: single element left.
            &[last] => {
                parse_single_command(last)
            }
            list => {
                // Indexing guarnteed to work due to pattern matching.
                let first = Box::new(parse_single_command(list[0])?);
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
        ["more", path] => {
            Ok(ShellCommand::More(PathBuf::from(path)))
        }
        ["ls"] => {
            Ok(ShellCommand::Ls(PathBuf::from(".")))
        }
        ["ls", path] => {
            Ok(ShellCommand::Ls(PathBuf::from(path)))
        }
        ["cd", path] => {
            Ok(ShellCommand::Cd(PathBuf::from(path)))
        }
        ["find-file", path, dir] => {
            Ok(ShellCommand::FindFile(PathBuf::from(path), PathBuf::from(dir)))
        }
        // Find string with 2 arguments.
        ["find-string", content, string] => {
            Ok(ShellCommand::FindString(Some(content.to_string()), string.to_string()))
        }
        // Find string with 1 argument, used for a pipe.
        ["find-string", string] => {
            Ok(ShellCommand::FindString(None, string.to_string()))
        }
        ["subs-string", content, from, to] => {
            Ok(ShellCommand::SubsString(Some(content.to_string()),
                                        from.to_string(),
                                        to.to_string()))
        }
        ["subs-string", from, to] => {
            Ok(ShellCommand::SubsString(None, from.to_string(), to.to_string()))
        }

        parts => {
            // Hard cases, Rust's pattern matching is unable to
            // handle their matching. Specifically:
            // cat since it takes multiple files.
            if parts[0] == "cat" {
                let args = &parts[1..];
                if args.is_empty() {
                  return Err(ShellError::KnownCommandWrongArgs(command.to_string()));
                } else {
                    let args: Vec<PathBuf> = args.
                        into_iter().
                        map(|s| PathBuf::from(s)).
                        collect();
                    return Ok(ShellCommand::Cat(args));
                }
            }

            // Look to see if this is a known command with the wrong # of arguments.
            let known_commands =
                ["more", "cat", "ls", "cd", "find-string", "subs-string", "find-file"];

            // known command, but wrong number of arguments passed to it...
            if known_commands.contains(&parts[0]) {
                Err(ShellError::KnownCommandWrongArgs(command.to_string()))
            } else {
                Err(ShellError::UnknownCommand(command.to_string()))
            }

        }
    }
}

fn execute_shell_command(c: &ShellCommand) -> Result<CommandOutput, ShellError> {
    match c {
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
            Err(ShellError::ExpectedPipeInput(c.clone()))
        }
        ShellCommand::SubsString(Some(content), from, to) => {
            Ok(subs_string(content, from, to))
        }
        ShellCommand::SubsString(None, _, _) => {
            Err(ShellError::ExpectedPipeInput(c.clone()))
        }
        // Pipe
        ShellCommand::Pipe(c1, c2) => {
            let results = execute_shell_command(c1)?;
            match &**c2 {
                ShellCommand::FindString(None, search_str) => {
                    let input = to_single_string(results, &**c1)?;
                    Ok(find_string(&input, &search_str))
                }
                ShellCommand::FindString(Some(_), _) => {
                    Err(ShellError::UnexpectedPipeInput(c.clone()))
                }
                ShellCommand::SubsString(None, from, to) => {
                    let input = to_single_string(results, &**c1)?;
                    Ok(subs_string(&input, from, to))
                }
                _ => Err(ShellError::PipeInputNotAccepted(c.clone()))
            }
        }
    }
}


fn to_single_string(c: CommandOutput, command: &ShellCommand)
                    -> Result<String, ShellError> {
    match c {
        CommandOutput::List(results) => {
            Ok(results.join(" "))
        }
        CommandOutput::Single(result) => {
            Ok(result)
        }
        CommandOutput::None => {
            Err(ShellError::NoInputForPipe(command.clone()))
        }
    }
}

fn find_string<'a>(content: &String, search_str: &String) -> CommandOutput {
    let mut matches: Vec<String> = vec![];

    for line in content.split("\n") {
        if line.contains(search_str) {
            matches.push(line.to_string());
        }
    }
    CommandOutput::List(matches)
}

fn subs_string(original: &String, from: &String, to: &String) -> CommandOutput {
    CommandOutput::Single(original.replace(from, to))
}

// Returs string so that it can be reused by cat_file.
fn more_file(path: impl AsRef<Path>) -> Result<String, ShellError> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn cat_files(paths: &Vec<PathBuf>) ->  Result<CommandOutput, ShellError> {
    let mut cat = String::new();
    for p in paths {
        cat.push_str(&more_file(p)?);
    }
    Ok(CommandOutput::Single(cat))
}

fn ls_dir(path: &impl AsRef<Path>) ->  Result<CommandOutput, ShellError> {
    let mut entries = vec![];
    let err_msg = "File path is not valid UTF-8 String";

    for dir_entry in std::fs::read_dir(path)? {
        let dir_entry = dir_entry?;
        let s = dir_entry.path().into_os_string().into_string().
            map_err(|_| io::Error::new(ErrorKind::InvalidData, err_msg))?;

        entries.push(s);
    }
    Ok(CommandOutput::List(entries))
}

// This one was hard to implement. Even for me. So this should probably
// be one of those bonus 3-star excercises.
fn find_file<P: AsRef<Path>, Q: AsRef<Path>>(starting_dir: P,
                                search_name: Q)
             ->  Result<CommandOutput, ShellError> {

    fn find_file_rec<P: AsRef<Path>, Q: AsRef<Path>>(current_dir: P,
                     search_name: &Q,
                     mut entries: Vec<PathBuf>) ->  Result<Vec<PathBuf>, ShellError> {
        for entry in std::fs::read_dir(&current_dir)? {
            let entry = entry?;
            let ft = entry.file_type()?;

            // Recurse!
            if ft.is_dir() {
                let next_dir = current_dir.as_ref().join(entry.file_name());
                entries = find_file_rec(next_dir, search_name, entries)?;
            }

            if & entry.file_name() == search_name.as_ref().as_os_str() {
                entries.push(current_dir.as_ref().join(entry.file_name()));
            }

        }
        Ok(entries)
    }

    // if starting_dir.
    let entries = vec![];
    let entries = find_file_rec(&starting_dir, &search_name, entries)?;
    let entries: Vec<String> = entries.into_iter().map(|path| path.
                                           into_os_string().
                                           into_string().
                                           expect("Path is invalid UTF-8")).collect();
    Ok(CommandOutput::List(entries))
}

fn cd(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    std::env::set_current_dir(path)?;
    Ok(CommandOutput::None)
}


#[test]
fn find_file_test() {
    assert_eq!(find_file("./resources", "dummy.txt").unwrap(),
               vec![PathBuf::from("./resources/test_dir/dummy.txt"),
                    PathBuf::from("./resources/test_dir/test_dir2/dummy.txt")]);
}

#[test]
fn find_file_test_empty() {
    let empty: Vec<PathBuf> = vec![];
    assert_eq!(find_file("./resources", "does_not_exist.txt").unwrap(),
               empty);
}

#[test]
fn more_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    assert_eq!(more_file(test_file).unwrap(),
               include_str!("../resources/cat_file_test.txt"));
}

#[test]
fn cat_file_test_one_file() {
    let test_file = "./resources/cat_file_test.txt";
    let paths = vec![PathBuf::from(test_file)];
    assert_eq!(more_file(test_file).unwrap(),
               execute_shell_command(&ShellCommand::Cat(paths)).unwrap());
}

#[test]
fn cat_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    let test_file2 = "./resources/cat_file_test.txt";
    let paths = vec![PathBuf::from(test_file), PathBuf::from(test_file2)];

    let mut manual = more_file(test_file).unwrap();
    manual.push_str(&more_file(test_file2).unwrap());
    assert_eq!(manual,
               execute_shell_command(&ShellCommand::Cat(paths)).unwrap());
}

#[test]
fn test_create_shell_command() {
    let command = "ls |";
    create_shell_command(command);
}
