use std::io;
use std::io::Write;
use std::process::Command;
use std::process::ExitStatus;
use std::iter::FromIterator;

#[macro_use]
extern crate lazy_static;
extern crate readline;

mod utilities;

fn main() {
    loop {
        let mut input = readline::readline(&get_ps1()).unwrap();
        readline::add_history(&input);
        let trimmed_input = input.trim_right();
        run(&trimmed_input);
    }
}

fn get_ps1() -> String {
    match std::env::var("PS1") {
        Ok(ps1) => ps1,
        Err(_) => String::from("$")
    }
}

pub fn run(command : &str) -> Option<ExitStatus> {
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
        match command.unwrap().status() {
            Ok(status) => Some(status),
            Err(_) => None
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
    fn run_ls_should_be_some() {
        assert!(run("ls").is_some());
    }

    #[test]
    fn run_ls_with_args_should_be_some() {
        assert!(run("ls -a").is_some());
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
