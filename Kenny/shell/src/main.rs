use std::io::*;
use std::fs::*;
use std::sync::mpsc::*;
use std::thread::spawn;

fn main() -> Result<()> {
    // Setting up input/output
    let stdin = stdin();
    let mut input = stdin.lock();
    let stdout = stdout();
    let mut output = stdout.lock();

    // Construct and run "cat"
    let mut cat = Cat::call(vec![]);
    let mut ls = Ls::call(vec![]);
    ls.run(&mut input, &mut output)
    // let mut command = Pipe::new(&mut ls, &mut cat);
    // command.run(&mut input, &mut output)
}

pub trait Command {
    fn run(&mut self, input: &mut BufRead, output: &mut Write) -> Result<()>;
}

pub struct Cat {
    current: Option<Box<BufRead>>,
    files: Vec<String>,
}

pub trait Call {
    fn call(args: Vec<String>) -> Self;
}

impl Call for Cat {
    fn call(args: Vec<String>) -> Self {
        Cat {
            current: None,
            files: if args.is_empty() {
                vec!["-".to_string()]
            } else {
                args
            }
        }
    }
}

fn forward_line(input: &mut BufRead,
                output: &mut Write) -> Result<usize> {
    let mut buf = String::new();
    let size = input.read_line(&mut buf)?;
    output.write_all(buf.as_bytes())?;
    Ok(size)
}

impl Command for Cat {
    fn run(&mut self, input: &mut BufRead, output: &mut Write) -> Result<()> {
        match &mut self.current {

            // If there is a current open file:
            Some(handle) => {
                let size = forward_line(handle, output)?;
                if size == 0 {
                    self.current = None;
                }
                self.run(input, output)
            },

            // If there is no current open file:
            // FIXME: this grabs first lines sometimes
            None => match self.files.pop() {
                None => Ok(()),
                Some(filename) => {
                    if filename.as_str() == "-" {
                        let size = forward_line(input, output)?;  // fwd a line of input
                        if size != 0 {
                            self.files.push("-".to_string());  // persist reading from input
                        }
                        Ok(())
                    } else {
                        let file = File::open(filename)?;
                        self.current = Some(Box::new(BufReader::new(file)));
                        self.run(input, output)
                    }
                }
            }
        }
    }
}

pub struct Ls;

impl Call for Ls {
    fn call(_args: Vec<String>) -> Self {
        Ls
    }
}

impl Command for Ls {
    fn run(&mut self, _input: &mut BufRead, output: &mut Write) -> Result<()> {
        let dir = read_dir(".")?;
        for result in dir {
            match result {
                Err(e) => return Err(e),
                Ok(entry) => {
                    let path = entry.path().to_str().unwrap().to_string() + "\n";
                    let bytes = path.as_bytes();
                    output.write_all(bytes)?;
                },
            }
        }
        Ok(())
    }
}

impl<C, D> Command for (C, D) where C: Command + Send, D: Command + Send {
    fn run(&mut self, input: &mut BufRead, output: &mut Write) -> Result<()> {
        let (sender, receiver) = channel::<u8>();
        let mut into_pipe = SenderWriter { sender };
        let mut from_pipe = BufReader::new(ReceiverReader { receiver });
        let pre_thread = spawn(move || {
            self.0.run(input, &mut into_pipe)
        });
        let post_thread = spawn(move || {
            self.1.run(&mut from_pipe, output)
        });
        unimplemented!()
    }
}

struct SenderWriter {
    sender: Sender<u8>
}

impl Write for SenderWriter {

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let broken_pipe_error =
            Err(Error::new(ErrorKind::BrokenPipe,
                           "Broken pipe in SenderWriter"));

        for byte in buf {
            match self.sender.send(*byte) {
                Ok(()) => (),
                Err(_) => return broken_pipe_error,
            }
        }
        Ok(buf.len())
    }

    // Flush is a no-op because there is no buffering
    fn flush(&mut self) -> Result<()> { Ok(()) }

}

struct ReceiverReader {
    receiver: Receiver<u8>
}

impl Read for ReceiverReader {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let broken_pipe_error =
            Err(Error::new(ErrorKind::BrokenPipe,
                          "Broken pipe in ReceiverReader"));

        for bytes_read in 0..buf.len() - 1 {
            match self.receiver.try_recv() {
                Ok(byte) => buf[bytes_read] = byte,
                Err(TryRecvError::Empty) =>
                    if bytes_read > 0 {
                        // If we've already read > 0 bytes give up & return them
                        return Ok(bytes_read)
                    } else if let Ok(byte) = self.receiver.recv() {
                        buf[bytes_read] = byte;
                    } else {
                        return broken_pipe_error
                    },
                Err(TryRecvError::Disconnected) =>
                    return broken_pipe_error
            }
        }
        Ok(buf.len())
    }

}

// pub struct Pipe<'a> {
//     pre: &'a mut Command,
//     buffer: VecDeque<u8>,
//     post: &'a mut Command,
// }

// impl<'a> Pipe<'a> {
//     pub fn new(c: &'a mut Command, d: &'a mut Command) -> Self {
//         Pipe {
//             pre: c,
//             buffer: VecDeque::new(),
//             post: d,
//         }
//     }
// }


// impl<'a> Command for Pipe<'a> {
//     fn tick(&mut self, input: &mut BufRead, output: &mut Write) -> Result<usize> {
//         let mut pre_input =
//             BufReader::new(VecPipeReader {
//                 input,
//                 buffer:  &mut self.buffer,
//                 command: &mut *self.pre
//             });
//         self.post.tick(&mut pre_input, output)
//     }
// }

// struct VecPipeReader<'a> {
//     command: &'a mut Command,
//     input:   &'a mut BufRead,
//     buffer:  &'a mut VecDeque<u8>,
// }

// impl<'a> Read for VecPipeReader<'a> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let mut bytes_read = 0;
//         while bytes_read < buf.len() {
//             if let Some(byte) = self.buffer.pop_back() {
//                 buf[bytes_read] = byte;
//                 bytes_read += 1;
//             } else {
//                 let mut output = VecWriter { buffer: &mut self.buffer };
//                 let command_bytes_written =
//                     self.command.tick(&mut self.input, &mut output)?;
//                 if command_bytes_written == 0 {
//                     break;
//                 }
//             }
//         }
//         Ok(bytes_read)
//     }
// }

// struct VecWriter<'a> {
//     buffer: &'a mut VecDeque<u8>,
// }

// impl<'a> Write for VecWriter<'a> {
//     fn flush(&mut self) -> Result<()> { Ok(()) /* no-op */ }
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         for byte in buf {
//             self.buffer.push_front(*byte);
//         }
//         Ok(buf.len())
//     }
// }
