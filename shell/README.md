# Shell Project

In this project, we will build a shell! This is a command line terminal program
which takes commands entered by the user and makes changes to the filesystem
or reads from the filesystem to display some data.

Example:
```bash
> ls
./solution
./Cargo.lock
./src
./.#README.md
./#README.md#
./resources
./target
./Cargo.toml
./.gitignore
> cd src
> ls
./main.rs
./.#main.rs
./error.rs
./command_output.rs
./shell_command.rs
./#main.rs#
./commands.rs
>
```

## Implementing Commands
We will start by implementing IO functions in src/commands.rs.
Follow the steps to implement the functions! Write your own unit tests!
See the bottom of commands.rs for some unit tests that have already been written for you!

Come back to this file once you're done!

## Shell Commands

Check out shell_commands.rs

## Putting it all together
Now go to main.rs

## What's Next?
If you're done with these parts. You can now try working on getting piping working.
Feel free to ask if you're no familiar with pipes!
Take a second to look at the types and see if you can make sense of how pipes should
work and how they fit with the types.
