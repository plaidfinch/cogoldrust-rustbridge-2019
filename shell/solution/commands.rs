//! This module implements the functions that do the actual IO for each
//! of our commands.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use crate::error::ShellError;
use crate::CommandOutput;

/// "grep"-like function. Returns all the lines in `content` where `search_str`
/// is a substring.
pub fn find_string<'a>(content: &String, search_str: &String) -> CommandOutput {
    let mut matches: Vec<String> = vec![];

    for line in content.split("\n") {
        if line.contains(search_str) {
            matches.push(line.to_string());
        }
    }
    CommandOutput::List(matches)
}

/// Change all instances of `from` to `to` in string `original`
pub fn subs_string(original: &String, from: &String, to: &String) -> CommandOutput {
    CommandOutput::Single(original.replace(from, to))
}

/// Return contents of file `path` as one giant string.
/// Returs string so that it can be reused by `cat_files` function.
pub fn more_file(path: impl AsRef<Path>) -> Result<String, ShellError> {

    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Returns conents of all files in `paths` as a CommandOutput.
/// concats files based on their order in slice.
/// Fails if any of the files have any IO error.
pub fn cat_files(paths: &[PathBuf]) -> Result<CommandOutput, ShellError> {
    let mut cat = String::new();
    for p in paths {
        cat.push_str(&more_file(p)?);
    }
    Ok(CommandOutput::Single(cat))
}

/// Returns all entries in a directory.
/// Fails if `path` is not a directory.
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

/// Bonus command. Difficult to implement! Leave for last!
/// This one was hard to implement. Even for me. So this should probably
/// be one of those bonus 3-star excercises.

/// Find all instances of `search_name` by recusively going through all
/// dirs and subdirs in `starting_dir`.
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

/// Change current working directory.
pub fn cd(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    std::env::set_current_dir(path)?;
    Ok(CommandOutput::None)
}


/// Verify contents of file match our more command.
#[test]
fn more_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    assert_eq!(
        more_file(test_file).unwrap(),
        include_str!("../resources/cat_file_test.txt")
    );
}

/// More-ing a file should be the same as cat-ing a single file.
#[test]
fn cat_file_test_one_file() {
    let test_file = "./resources/cat_file_test.txt";
    assert_eq!(
        more_file(test_file).unwrap(),
        cat_files(&[PathBuf::from(test_file)]).
            unwrap().
            to_string().
            unwrap()
    );
}

/// Cat-ing two files should be the same as more-ing two files and appending
/// their results.
#[test]
fn cat_file_test() {
    let test_file = "./resources/cat_file_test.txt";
    let test_file2 = "./resources/cat_file_test.txt";
    let paths = &[PathBuf::from(test_file), PathBuf::from(test_file2)];

    let mut manual = more_file(test_file).unwrap();
    manual.push_str(&more_file(test_file2).unwrap());
    assert_eq!(
        manual,
        cat_files(paths).
            unwrap().
            to_string().
            unwrap()
    );
}
