#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;
use std::process::Command;
use std::iter::FromIterator;

mod utilities;

fn main() {
    loop {
        let mut input = String::new();
        show_prompt();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let trimmed_input = input.trim_right();
        match run(&trimmed_input) {
            Some(result) => print!("{}", result),
            None => ()
        }
    }
}

pub fn show_prompt() {
    print!("{}", get_ps1());
    io::stdout()
        .flush()
        .ok()
        .expect("Failed to display prompt");
}

fn get_ps1() -> String {
    match std::env::var("PS1") {
        Ok(ps1) => ps1,
        Err(_) => String::from("$")
    }
}

pub fn run(command : &str) -> Option<String> {
    let mut iter = command.split_whitespace();
    let program = match iter.next() {
        Some(program) => program,
        None => return None
    };
    let args = iter.collect::<Vec<_>>();

    if run_builtin_if_possible(program, args) {
        return None;
    }
    let command = build_command(command);
    if command.is_some() {
        match command.unwrap().output() {
            Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
            Err(_) => Some(String::from("Error running command\n"))
        }
    }
    else {
        None
    }
}

fn build_command(input : &str) -> Option<Command> {
    let mut iter = input.split_whitespace();
    let program = iter.next();
    if program.is_some() {
        let mut command = Command::new(program.unwrap());
        command.args(&iter.collect::<Vec<_>>());
        Some(command)
    } else {
        None
    }
}

fn run_builtin_if_possible(program: &str, args: Vec<&str>) -> bool {
    match program {
        "cd" => {
            utilities::cd(args);
            true
        },
        "exit" => {
            utilities::exit();
            true
        }
        ":" => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_prompt_should_not_panic() {
        show_prompt();
    }

    #[test]
    fn run_ls_should_be_some() {
        assert!(run("ls").is_some());
    }

    #[test]
    fn run_ls_with_args_should_be_some() {
        assert!(run("ls -al").is_some());
    }

    #[test]
    fn run_empty_string_should_be_none() {
        assert!(run("").is_none());
    }

    #[test]
    fn run_gibberish_should_not_panic() {
        run("asdf-");
    }

}
