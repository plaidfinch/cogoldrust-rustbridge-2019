//! This module implements the functions that do the actual IO for each
//! of our commands.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::fs::DirEntry;
// Step #1 Check out src/error.rs
use crate::error::ShellError;
// Step #2 Check out src/command_output.
use crate::CommandOutput;

/// "grep"-like function. Returns all the lines in `content` where `search_str`
/// is a substring.
pub fn find_string<'a>(content: &String, search_str: &String) -> CommandOutput {
    // Step #5 (see command_output.rs for #3 and #4)
    // Now that you're familar with CommandOutput you are ready
    // to implement this function.

    // Have a look at the str::split() function in the standar library documentation
    // (std docs)
    // Also look at str::contains()
    let mut matches: Vec<String> = vec![];
    unimplemented!();
    CommandOutput::List(matches)
}

/// Change all instances of `from` to `to` in string `original`
pub fn subs_string(original: &String, from: &String, to: &String) -> CommandOutput {
    // Step #6
    // Check out str::replace() in std docs.
    unimplemented!()
}

/// Return contents of file `path` as one giant string.
/// Returs string so that it can be reused by `cat_files` function.
pub fn more_file(path: impl AsRef<Path>) -> Result<String, ShellError> {
    // Step #7
    // See File::open().
    // See std::fs::read_to_string()
    // First try implementing using match expressions.
    // then use '?' to get rid of all match expressions.
    // Ask for help if needed! This can be tricky the first time!
    unimplemented!()
}

/// Returns conents of all files in `paths` as a CommandOutput.
/// concats files based on their order in slice.
/// Fails if any of the files have any IO error.
pub fn cat_files(paths: &[PathBuf]) -> Result<CommandOutput, ShellError> {
    // Step #8
    // See string::push_str()
    unimplemented!()
}

/// Returns all entries in a directory.
/// Fails if `path` is not a directory.
pub fn ls_dir(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    // Step #9
    // Use std::fs::read_dir()
    // as well as the get_path function defined below to turn a DirEntry
    // into a String.
    unimplemented!()
}

fn get_path(dir: DirEntry) -> Result<String, ShellError> {
    let err_msg = "File path is not valid UTF-8 String";
    Ok(dir
        .path()
        .into_os_string()
        .into_string()
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, err_msg))?)
}

/// This one was hard to implement. Even for me. So this should probably
/// be one of those bonus 3-star excercises.

/// Find all instances of `search_name` by recusively going through all
/// dirs and subdirs in `starting_dir`.
pub fn find_file<P: AsRef<Path>, Q: AsRef<Path>>(
    starting_dir: P,
    search_name: Q,
) -> Result<CommandOutput, ShellError> {
    // Bonus command. Very difficult to implement! Leave for last!

    // Feel free to look at the solution implementation and then
    // try to implement it yourself.
    unimplemented!()
}

/// Change current working directory.
pub fn cd(path: &impl AsRef<Path>) -> Result<CommandOutput, ShellError> {
    // Step #10
    // See std::env::set_current_dir
    unimplemented!()
}

// Step #11
// These are sample tests. Feel free to write your own to gain confidence
// in your implementation!


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
