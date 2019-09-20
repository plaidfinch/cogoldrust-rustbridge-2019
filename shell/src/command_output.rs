
/// Represents the output of a ShellCommand.
/// Some commands output a single line, others multiple lines,
/// and some no output at all. This handles all these types of output.
#[derive(Clone, Debug)]
pub enum CommandOutput {
    Single(String),
    List(Vec<String>),
    None,
}

impl CommandOutput {
    /// Prints different variants for CommandOutput.
    /// `Single` prints the sole line with a newline.
    /// `List` prints one entry per line.
    /// `None` prints nothing.
    pub fn print_command(&self) {
        match self {
            CommandOutput::Single(s) => println!("{}", s),
            CommandOutput::List(ss) => {
                for s in ss {
                    println!("{}", s);
                }
            }
            CommandOutput::None => {}
        }
    }

    /// Convers variants into a single String.
    /// `None` returns None.
    /// List concats the strings separated by a single space.
    pub fn to_string(self) -> Option<String> {
        match self {
            CommandOutput::List(results) => Some(results.join(" ")),
            CommandOutput::Single(result) => Some(result),
            CommandOutput::None => None,
        }
    }
}
