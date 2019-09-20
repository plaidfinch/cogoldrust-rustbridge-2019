#[derive(Clone, Debug)]
pub enum CommandOutput {
    Single(String),
    List(Vec<String>),
    None,
}

impl CommandOutput {
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

    pub fn to_string(self) -> Option<String> {
        match self {
            CommandOutput::List(results) => Some(results.join(" ")),
            CommandOutput::Single(result) => Some(result),
            CommandOutput::None => None,
        }
    }
}
