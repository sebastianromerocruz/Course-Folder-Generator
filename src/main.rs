mod get_user_input;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use crate::get_user_input::get_user_input;

const CD_COMMAND: &str = "cd";
const DIR_CHAR: &str = "/";

fn main(){
    loop {
        // Prepare prompt by:
        //     1. Using the `>` character as the prompt
        //     2. Flushes standard output to ensure `>` prints before `read_line()`
        print!("> ");
        io::stdout().flush().expect("FLUSH: Thread panicked!");

        // Read user input into an empty string
        let mut user_input = String::new();
        user_input = get_user_input(user_input);

        // Isolates command and arguments by:
        //     1. Trimming and splitting the input by the whitespace (`" "`)
        //     2. Grabs the zero-th element from the `SplitWhitespace` object (`next()`) as the
        //        command
        //     3. Leaves the rest as the arguments
        let mut argument_vector = user_input.trim().split_whitespace();
        let command = argument_vector.next().unwrap();
        let arguments = argument_vector;

        match command {
            CD_COMMAND => {
                // Default to '/' as new directory if one was not provided
                let new_directory = arguments
                    .peekable()
                    .peek()
                    .map_or(DIR_CHAR, |x| *x);
                let root = Path::new(new_directory);

                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            command => {
                // Initiates a new child process:
                //     1. Using `command` as the command...
                //     2. ...and `arguments` as the arguments
                let mut child_process = Command::new(command)
                    .args(arguments)
                    .spawn()
                    .unwrap();

                // Waits so as to not accept another command until this one completes
                child_process.wait().expect("WAIT: Thread panicked");
            }
        }
    }
}