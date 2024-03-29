
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
        // Step #3
        // Match on the difference variants of CommandOutput!
        match self {
            _ => { unimplemented!() }
        }
    }

    /// Converts variants into a single String.
    /// `None` returns None.
    /// List concats the strings separated by a single space.
    pub fn to_string(self) -> Option<String> {
        // Step #4
        // Match on the difference variants of CommandOutput
        unimplemented!()
    }
}
