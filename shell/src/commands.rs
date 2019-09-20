use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use crate::error::ShellError;
use crate::CommandOutput;

pub fn find_string<'a>(content: &String, search_str: &String) -> CommandOutput {
    let mut matches: Vec<String> = vec![];

    for line in content.split("\n") {
        if line.contains(search_str) {
            matches.push(line.to_string());
        }
    }
    CommandOutput::List(matches)
}

pub fn subs_string(original: &String, from: &String, to: &String) -> CommandOutput {
    CommandOutput::Single(original.replace(from, to))
}

// Returs string so that it can be reused by cat_file.
pub fn more_file(path: impl AsRef<Path>) -> Result<String, ShellError> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn cat_files(paths: &Vec<PathBuf>) -> Result<CommandOutput, ShellError> {
    let mut cat = String::new();
    for p in paths {
        cat.push_str(&more_file(p)?);
    }
    Ok(CommandOutput::Single(cat))
}

pub fn ls_dir(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    let mut entries = vec![];
    let err_msg = "File path is not valid UTF-8 String";

    for dir_entry in std::fs::read_dir(path)? {
        let dir_entry = dir_entry?;
        let s = dir_entry
            .path()
            .into_os_string()
            .into_string()
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, err_msg))?;

        entries.push(s);
    }
    Ok(CommandOutput::List(entries))
}

// This one was hard to implement. Even for me. So this should probably
// be one of those bonus 3-star excercises.
pub fn find_file<P: AsRef<Path>, Q: AsRef<Path>>(
    starting_dir: P,
    search_name: Q,
) -> Result<CommandOutput, ShellError> {
    fn find_file_rec<P: AsRef<Path>, Q: AsRef<Path>>(
        current_dir: P,
        search_name: &Q,
        mut entries: Vec<PathBuf>,
    ) -> Result<Vec<PathBuf>, ShellError> {
        for entry in std::fs::read_dir(&current_dir)? {
            let entry = entry?;
            let ft = entry.file_type()?;

            // Recurse!
            if ft.is_dir() {
                let next_dir = current_dir.as_ref().join(entry.file_name());
                entries = find_file_rec(next_dir, search_name, entries)?;
            }

            if &entry.file_name() == search_name.as_ref().as_os_str() {
                entries.push(current_dir.as_ref().join(entry.file_name()));
            }
        }
        Ok(entries)
    }

    // if starting_dir.
    let entries = vec![];
    let entries = find_file_rec(&starting_dir, &search_name, entries)?;
    let entries: Vec<String> = entries
        .into_iter()
        .map(|path| {
            path.into_os_string()
                .into_string()
                .expect("Path is invalid UTF-8")
        })
        .collect();
    Ok(CommandOutput::List(entries))
}

pub fn cd(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    std::env::set_current_dir(path)?;
    Ok(CommandOutput::None)
}

#[test]
fn find_file_test() {
    assert_eq!(
        find_file("./resources", "dummy.txt").unwrap(),
        vec![
            PathBuf::from("./resources/test_dir/dummy.txt"),
            PathBuf::from("./resources/test_dir/test_dir2/dummy.txt")
        ]
    );
}

#[test]
fn find_file_test_empty() {
    let empty: Vec<PathBuf> = vec![];
    assert_eq!(
        find_file("./resources", "does_not_exist.txt").unwrap(),
        empty
    );
}

#[test]
fn more_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    assert_eq!(
        more_file(test_file).unwrap(),
        include_str!("../resources/cat_file_test.txt")
    );
}

#[test]
fn cat_file_test_one_file() {
    let test_file = "./resources/cat_file_test.txt";
    let paths = vec![PathBuf::from(test_file)];
    assert_eq!(
        more_file(test_file).unwrap(),
        execute_shell_command(&ShellCommand::Cat(paths)).unwrap()
    );
}

#[test]
fn cat_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    let test_file2 = "./resources/cat_file_test.txt";
    let paths = vec![PathBuf::from(test_file), PathBuf::from(test_file2)];

    let mut manual = more_file(test_file).unwrap();
    manual.push_str(&more_file(test_file2).unwrap());
    assert_eq!(
        manual,
        execute_shell_command(&ShellCommand::Cat(paths)).unwrap()
    );
}

#[test]
fn test_create_shell_command() {
    let command = "ls |";
    create_shell_command(command);
}
