use std::io;
use crate::shell_command::ShellCommand;

/// Our own error type representing the different ways our
/// shell can fail.
/// Variants that take a String failed during parsing.
/// Variants with a ShellCommand failed during evaluation.
#[derive(Debug)]
pub enum ShellError {
    /// An underlying io failure.
    IoError(io::Error),
    /// A command is expected to be commands separated by pipes.
    /// but the command between some set of pipes was missing.
    /// e.g. cat file.txt | |
    EmptyCommandInPipe(String),
    /// The command passed was recognized but the wrong number
    /// or argumetns were passed to it!
    KnownCommandWrongArgs(String),
    /// During parsing the command was not recognized.
    UnknownCommand(String),
    /// Some command expected pipe input, but the previous command
    /// returned no input.
    ExpectedPipeInput(ShellCommand),
    /// The command did not expect pipe input, but input was piped
    /// to it.
    UnexpectedPipeInput(ShellCommand),
    /// This command does not accept pipe input.
    PipeInputNotAccepted(ShellCommand),
    /// Command produces no input for pipe.
    NoInputForPipe(ShellCommand),
}

/// Allow for automatic conversion between io::Error to ShellError
/// when using '?'.
impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::IoError(error)
    }
}
