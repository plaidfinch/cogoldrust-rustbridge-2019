use std::io::{self, Read, Write};
use std::process::exit;
use pancurses::{initscr, endwin, Input, noecho, start_color};
use ctrlc;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn from_char(c: char) -> Option<Operation> {
        use Operation::*;
        match c {
            '+' => Some(Add),
            '-' => Some(Sub),
            '*' => Some(Mul),
            '/' => Some(Div),
            _ => None,
        }
    }
}

pub enum Button {
    Op(Operation),
    Number(f64),
}

pub struct Calculator {
    stack: Vec<f64>,
}

impl Calculator {

    /// Makes a new calculator, with an empty state (no buttons have yet been
    /// pressed).
    pub fn new() -> Calculator {
        Calculator {
            stack: Vec::new()
        }
    }

    /// Resets the calculator to have an empty stack.
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Updates the internal state of the calculator to correspond to the given
    /// button being pushed. Returns the current value of the calculator, or
    /// `None` if the button was invalid.
    pub fn push_button(&mut self, button: &Button) -> Option<f64> {
        use Operation::*;
        use Button::*;
        match button {
            Number(n) => {
                self.stack.push(*n);
                Some(*n)
            }
            Op(operation) => {
                let x = self.stack.pop()?;
                if let Some(y) = self.stack.pop() {
                    let result = match operation {
                        Add => y + x,
                        Mul => y * x,
                        Sub => y - x,
                        Div => y / x,
                    };
                    self.stack.push(result);
                    Some(result)
                } else {
                    // If second arg isn't there, restore first arg
                    self.stack.push(x);
                    None
                }
            },
        }
        // unimplemented!("push_button() not yet implemented")
    }

}

/* Ways things can go wrong:
- wrong order for non-commutative ops
- forget to restore first arg when second doesn't pop
*/

fn number_from_digit(c: char) -> Option<i64> {
    let number = c as i64 - 48;
    if 0 <= number && number < 10 {
        Some(number)
    } else {
        None
    }
}

const VALID_CHARS: &[char]
    = &[' ', '\n',
        '+', '*', '-', '/',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'c', 'q'];


pub fn main() -> Result<(), io::Error> {
    use Button::*;

    // Init ncurses
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    ctrlc::set_handler(move || { endwin(); exit(1) })
        .expect("Error setting Ctrl-C handler");

    // Welcome message
    window.addstr("Welcome to the stack calculator!");
    window.addstr("\n\rAvailable operators: +, -, *, /");
    window.addstr("\n\rTo clear, press 'c'");
    window.addstr("\n\rTo quit, press 'q'");

    // Make a new calculator
    let mut calc = Calculator::new();

    // Keep track of current number being entered, if any
    let mut current_number = None;

    // Write the prompt
    window.addstr("\n\n\r>>> ");

    // All the valid characters:

    // Loop over the input
    loop {
        let input_char = loop {
            match window.getch() {
                Some(Input::Character(c)) => {
                    if c == '\n' {
                        break c;
                    } else if VALID_CHARS.contains(&c) {
                        window.addch(c);
                        break c;
                    }
                }
                _ => { }
            }
        };

        // Should we output a prompt again once we finish this loop?
        let mut newline = false;

        // When we're amidst entering a number, update the current number
        if let Some(number) = number_from_digit(input_char) {
            current_number = match current_number {
                None => Some(number as f64),
                Some(previous) => Some(previous * 10_f64 + number as f64),
            }

        // If the user cleared the calculator, do that
        } else if input_char == 'c' {
            calc.clear();
            newline = true;

        // If the user wanted to quit, do that
        } else if input_char == 'q' {
            break;

        // Otherwise they're done inputting a number, and maybe typed an operand
        } else {
            // Input the current number
            current_number.and_then(|n| calc.push_button(&Number(n)));
            current_number = None;

            // Maybe they pressed an operator?
            if let Some(operation) = Operation::from_char(input_char) {
                // Push the operation button
                match calc.push_button(&Op(operation)) {
                    Some(result) => {
                        window.insdelln(1);
                        window.addstr(format!("\r= {}", result));
                    },
                    None => {
                        window.addstr(" [Error: stack too small]");
                    },
                }
            }

            newline = true;
        }

        // If the user's actions led to a new prompt, print that
        if newline == true || input_char == '\n' {
            // Write the prompt
            window.insdelln(1);
            window.addstr("\r>>> ");
            for i in &calc.stack {
                window.addstr(format!("{} ", i));
            }
        }
    }

    endwin();
    Ok(())
}
