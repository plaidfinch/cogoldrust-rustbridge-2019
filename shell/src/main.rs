use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::fmt::Display;
use std::fmt::Debug;

enum ShellCommand {
    More(PathBuf),
    Cat(Vec<PathBuf>),
    Ls(PathBuf),
    Cd(PathBuf),
    FindString(String, String),
}

fn main() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    for line in stdin.lock().lines() {

        let user_input = line.unwrap();
        let command: Vec<&str> = user_input.split(" ").collect();
        match command.as_slice() {
            ["ls"] => {
                output_results(ls_dir(&"."));
            }
            ["more", path] => {
                output_results(more_file(path));
            }
            _ => {
                println!("Unkwon Command. Please try again.");
            }
        };

        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
    }
}

fn output_results<T: Debug>(r: io::Result<T>) {
    match r {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn execute_shell_command(c: &ShellCommand) -> io::Result<String> {
    match c {
        ShellCommand::More(path) => more_file(path),
        ShellCommand::Cat(paths) => cat_files(paths),
        ShellCommand::Ls(path) => unimplemented!(),
        ShellCommand::Cd(path) => unimplemented!(),
        ShellCommand::FindString(content, search_str) => unimplemented!(),
    }
}

fn find_string<'a>(content: &'a String, search_str: &String) -> Vec<&'a str> {
    let mut matches: Vec<&'a str> = vec![];
    for line in content.split("\n") {
        if line.contains(search_str) {
            matches.push(line);
        }
    }
    matches
}

fn subs_string(original: &String, from: &String, to: &String) -> String {
    original.replace(from, to)
}

fn more_file(path: impl AsRef<Path>) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn cat_files(paths: &Vec<PathBuf>) ->  io::Result<String> {
    let mut cat = String::new();
    for p in paths {
        cat.push_str(&more_file(p)?);
    }
    Ok(cat)
}

fn ls_dir(path: &impl AsRef<Path>) ->  io::Result<Vec<String>> {
    let mut entries = vec![];
    let err_msg = "File path is not valid UTF-8 String";

    for dir_entry in std::fs::read_dir(path)? {
        let dir_entry = dir_entry?;
        let s = dir_entry.path().into_os_string().into_string().
            map_err(|_| io::Error::new(ErrorKind::InvalidData, err_msg))?;

        entries.push(s);
    }
    Ok(entries)
}

// This one was hard to implement. Even for me. So this should probably
// be one of those bonus 3-star excercises.
fn find_file<P: AsRef<Path>, Q: AsRef<Path>>(starting_dir: P,
                                search_name: Q)
             ->  io::Result<Vec<PathBuf>> {

    fn find_file_rec<P: AsRef<Path>, Q: AsRef<Path>>(current_dir: P,
                     search_name: &Q,
                     mut entries: Vec<PathBuf>) ->  io::Result<Vec<PathBuf>> {
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
    let mut entries = vec![];
    find_file_rec(&starting_dir, &search_name, entries)
}

fn cd(path: &impl AsRef<Path>) -> io::Result<()> {
    std::env::set_current_dir(path)
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
