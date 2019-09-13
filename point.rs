use self::Day::*;
use std::io;
use std::io::Write;
use std::fs::*;

struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn day_after(today: String) -> String {
    if today == "Monday" {
        "Tuesday"
    } else if today == "Tuesday" {
        "Wednesday"
    } else if today == "Wednesday" {
        "Thursday"
    } else if today == "Thursday" {
        "Friday"
    } else if today == "Friday" {
        "Saturday"
    } else if today == "Saturday" {
        "Sunday"
    } else if today == "Sunday" {
        "Monday"
    } else {
        unimplemented!("What should go here?")
    }.to_string()
}

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn day_after2(today: &Day) -> Day {
    match today {
        Monday    => Tuesday,
        Tuesday   => Wednesday,
        Wednesday => Thursday,
        Thursday  => Friday,
        Friday    => Saturday,
        Saturday  => Sunday,
        Sunday    => Monday,
    }
}

fn write_hello2(filename: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all("Hello, world!".as_bytes())?;
    println!("Wrote to file!");
    Ok(())
}

fn write_hello(filename: &str) -> Result<(), io::Error> {
    match File::create(filename) {
        Err(e) => Err(e),
        Ok(mut file) => {
            match file.write_all("Hello, world!".as_bytes()) {
                Err(e) => Err(e),
                Ok(()) => {
                    println!("Wrote to file!");
                    Ok(())
                },
            }
        }
    }
}
