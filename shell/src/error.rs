use std::io;

use crate::shell_command::ShellCommand;

/// Variants that take a String failed during parsing.
/// Variants with a ShellCommand failed during evaluation.
#[derive(Debug)]
pub enum ShellError {
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

impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::IoError(error)
    }
}
