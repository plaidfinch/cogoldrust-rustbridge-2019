use pancurses::{initscr, endwin, Input, noecho};

use Operation::*;
use CalcError::*;

////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//   When you've solved this exercise (or while you're figuring it out),      //
//   you can run the calculator * play with it by running `cargo run`!        //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/*
    Back in the day, HP used to manufacture a weird kind of calculator. Unlike
    the normal calculators most of us use, you entered operations on it in
    "reverse Polish notation". This means that instead of writing `1 + 2 =`, you
    would write `1 2 +`, and the result would emerge as soon as you pressed `+`.
    There's no need for parentheses on this calculator: instead of writing `(1 +
    2) * 3`, you write `1 2 + 3 *`. Today, let's build such a calculator.
 */

/// First, let's define the operations as an enum: addition, subtraction,
/// multiplication, and division.

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}


/// At any given time, the calculator has an internal state: a *stack* of
/// numbers which have yet to be processed. This stack builds up the numbers
/// which have been entered, before operations are applied to them. We'll
/// represent this stack as a `Vec<i64>`. To push and pop from the stack, we
/// will use `Vec::push()` and Vec::pop()`.

#[derive(Debug, Clone)]
pub struct Calculator {
    stack: Vec<i64>,
}

/// An operation on the calculator can fail in one of two ways: 1) there aren't
/// enough numbers on its stack for an operation (all the operations need at
/// least 2 numbers on the stack), or 2) the user tried to divide by zero.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CalcError {
    StackTooSmall,
    DivideByZero,
}

// We've defined a few methods for the `Calculator` type below. Your task is to
// fill in the implementation of the final method, `press_operator()`.

impl Calculator {

    /// Makes a new calculator, with an empty stack.
    pub fn new() -> Calculator {
        Calculator {
            stack: Vec::new()
        }
    }

    /// Resets the calculator to have an empty stack.
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Pushes a number onto the calculator's stack.
    pub fn press_number(&mut self, number: i64) {
        self.stack.push(number);
    }

    /// Updates the internal state of the calculator to correspond to the given
    /// button being pushed. Returns the result of the operation, or an error if
    /// the operation was invalid. If the operation was invalid, the internal
    /// state of the calculator should remain unchanged.
    pub fn press_operator(&mut self, operation: &Operation) -> Result<i64, CalcError> {
        unimplemented!()  // delete this line and implement here :)
    }
}


////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//    Here are a bunch of unit tests that will help you make sure your        //
//    implementation works! You can run the tests with `cargo test`.          //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Operation::*;
    use super::CalcError::*;
    use super::Calculator;

    /// When any operator is called on a calculator with the empty stack, it
    /// should be a `StackTooSmall` error.
    #[test]
    fn calc_op_on_empty() {
        for op in &[Add, Mul, Div, Sub] {
            let mut c = Calculator::new();
            let result = c.press_operator(op);
            assert_eq!(c.stack, vec![],
                       "Operator {:?} changed the empty stack to the stack {:?} when it should have left it unchanged",
                       op, c.stack);
            assert_eq!(result, Err(StackTooSmall),
                       "Operator {:?} did not return 'Err(StackTooSmall)' when called on empty stack", op);
        }
    }

    /// When any operator is called on a calculator with a stack of size one, it
    /// should be a `StackTooSmall` error.
    #[test]
    fn calc_op_on_one() {
        for op in &[Add, Mul, Div, Sub] {
            let mut c = Calculator::new();
            c.press_number(1);
            let result = c.press_operator(op);
            assert_eq!(result, Err(StackTooSmall),
                       "Operator {:?} did not return 'Err(StackTooSmall)' when called on stack {:?}",
                       op, c.stack);
            assert_eq!(c.stack, vec![1],
                       "Operator {:?} changed the stack [1] to the stack {:?} when it should have left it unchanged",
                       c.stack, op);
        }
    }

    /// When any operator is called on a stack of size 2, it should be a
    /// success, and should reduce the size of the stack by one.
    #[test]
    fn calc_successful_op() {
        for op in &[Add, Mul, Div, Sub] {
            let mut c = Calculator::new();
            c.press_number(1);
            c.press_number(2);
            let result = c.press_operator(op);
            assert!(result.is_ok(),
                    "Operation {:?} returned {:?} on the stack {:?} when it should have been successful",
                    op, result, c.stack);
            assert_eq!(c.stack, vec![result.unwrap()],
                       "Stack {:?} is not one element after operation starting from two elements", c.stack);
        }
    }

    /// '2 1 -' should give the result 1, not -1.
    #[test]
    fn calc_sub() {
        let mut c = Calculator::new();
        c.press_number(2);
        c.press_number(1);
        let result = c.press_operator(&Sub);
        assert_eq!(result, Ok(1),
                   "Subtraction might be backwards (what should '2 1 -' return?)");
    }

    /// '4 2 /' should give the result 2, not 0.
    #[test]
    fn calc_div() {
        let mut c = Calculator::new();
        c.press_number(4);
        c.press_number(2);
        let result = c.press_operator(&Div);
        assert_eq!(result, Ok(2),
                   "Division might be backwards (what should '4 2 /' return?)");
    }

    /// Division by zero should be properly caught and returned as
    /// `Err(DivideByZero)`.
    #[test]
    fn calc_div_zero() {
        let mut c = Calculator::new();
        c.press_number(2);
        c.press_number(0);
        let result = c.press_operator(&Div);
        assert_eq!(c.stack, vec![2,0],
                   "Division by zero changes the stack when it should leave it alone");
        assert_eq!(result, Err(DivideByZero),
                   "Division by zero returns {:?} when it should return Err(DivideByZero)", result);
    }

}


////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//    Main program implementation below -- no need to look, unless you're     //
//    just curious to see how it works! But you don't need to modify it.      //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

/// A whitelist of valid characters: anything not in this list will not be
/// printed to the screen when you type it (effectively, all other keys are just
/// disabled within the program).
const VALID_CHARS: &[char]
    = &[' ', '\n', '.',
        '+', '*', '-', '/',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'c', 'q'];

pub fn main() {

    // Set up terminal handling (screen clearing, key listening, etc.)
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    // Welcome message and initial prompt
    window.addstr("Welcome to the stack calculator!");
    window.addstr("\n\rEnter numbers separated by <space> or <enter>");
    window.addstr("\n\rPress operators to do math: +, -, *, /");
    window.addstr("\n\rTo clear, press 'c'");
    window.addstr("\n\rTo quit, press 'q'");
    window.addstr("\n\n\r> ");

    // Make a new calculator
    let mut calc = Calculator::new();

    // Keep track of current number being entered, if any
    let mut current_number = None;

    // Loop over the input characters from the user's keyboard
    loop {
        // Wait for the user to press one of VALID_CHARS
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

        // What result should we return at the completion of this iteration of
        // the loop? Results are returned when an operator is used.
        let mut result_string = None;

        // When we're amidst entering a number and we press another digit,
        // append that digit to the currently-being-entered number.
        if let Some(number) = number_from_digit(input_char) {
            current_number = match current_number {
                None => Some(number),
                Some(previous) => Some(previous * 10 + number),
            }

        // If the user cleared the calculator, do that.
        } else if input_char == 'c' {
            calc.clear();
            result_string = Some(String::new());

        // If the user wanted to quit, do that.
        } else if input_char == 'q' {
            break;

        // Otherwise they're done inputting a number, and maybe typed an operand
        } else {
            // Input the current number they previously were typing, if any
            // (this is to make sure operators work correctly even in the case
            // where the user presses an operator before pressing <space> or
            // <enter>).
            if let Some(n) = current_number {
                calc.press_number(n);
                current_number = None;
            }

            // Maybe they pressed an operator? If so, process it and store the
            // result so we can print it later.
            if let Some(operation) = Operation::from_char(input_char) {

                // Push the operation button and examine the result.
                let result = calc.press_operator(&operation);
                result_string =
                    Some(match result {
                        Ok(result) => format!("= {}", result),
                        Err(StackTooSmall) => "[Error: stack too small]".to_string(),
                        Err(DivideByZero)  => "[Error: divide by zero]".to_string(),
                    });
            }
        }

        // If the user's actions mean we need to refresh the screen in some way,
        // do that.
        if input_char == '\n' || result_string.is_some() {
            // Write the prompt
            window.addstr("\r> ");
            for i in &calc.stack {
                window.addstr(format!("{} ", i));
            }
        }

        // If there is a result to be printed, print it out below the prompt.
        if let Some(result) = result_string {
            // Clear remainder of text on the screen
            window.clrtoeol();
            window.clrtobot();
            // Save the current window position and move down one
            let (y, x) = window.get_cur_yx();
            window.mv(y + 1, 0);
            window.refresh();
            // Write out the result here
            window.addstr(result);
            // Go back to where the cursor was before we moved it
            window.mv(y, x);
        }
    }

    // Shut down the terminal handling (this restores the previous contents of
    // the terminal to what they were before you typed `cargo run` or whatnot)
    endwin();
}

impl Operation {
    /// Convert a character to its corresponding operation, or return `None` if
    /// there is no such corresponding operation.
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

/// Convert a character representing a digit into a number.
/// For example: `number_from_digit('1') == 1`.
fn number_from_digit(c: char) -> Option<i64> {
    let number = c as i64 - 48;
    if 0 <= number && number < 10 {
        Some(number)
    } else {
        None
    }
}
