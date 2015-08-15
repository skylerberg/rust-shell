#![feature(drain)]

use std::process::Command;
use std::process::ExitStatus;

#[macro_use]
extern crate lazy_static;
extern crate readline;

mod utilities;
mod lexer;
mod shell;
mod expansion;

pub fn main() {
    let mut shell = shell::Shell::new();
    loop {
        let mut input = readline::readline(&get_ps1()).unwrap();
        readline::add_history(&input);
        input.push('\n');
        run(&mut shell, &input);
    }
}

fn get_ps1() -> String {
    match std::env::var("PS1") {
        Ok(ps1) => ps1,
        Err(_) => String::from("$")
    }
}

pub fn run(shell : &mut shell::Shell, command : &str) -> Option<ExitStatus> {
    let mut lexed = lexer::lex(command);
    let mut iter = expansion::expand_aliases(&shell.aliases, &mut lexed).into_iter();
    let program = match iter.next() {
        Some(program) => program,
        None => return None
    };
    let args = iter.collect::<Vec<_>>();

    if run_builtin_if_possible(shell, &program, &args) {
        return None;
    }
    let mut command = build_command(&program, &args);
    match command.status() {
        Ok(status) => Some(status),
        Err(_) => None
    }
}

fn build_command(program : &str, args : &Vec<String>) -> Command {
    let mut command = Command::new(program);
    command.args(&args.into_iter().collect::<Vec<_>>());
    command
}

fn run_builtin_if_possible(shell : &mut shell::Shell, program: &str, args: &Vec<String>) -> bool {
    match program {
        "alias" => {
            utilities::alias(shell, args);
            true
        }
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
    use super::shell;

    #[test]
    fn run_ls_should_be_some() {
        let mut shell = shell::Shell::new();
        assert!(run(&mut shell, "ls\n").is_some());
    }

    #[test]
    fn run_ls_with_args_should_be_some() {
        let mut shell = shell::Shell::new();
        assert!(run(&mut shell, "ls -a").is_some());
    }

    #[test]
    fn run_empty_string_should_be_none() {
        let mut shell = shell::Shell::new();
        assert!(run(&mut shell, "").is_none());
    }

    #[test]
    fn run_gibberish_should_not_panic() {
        let mut shell = shell::Shell::new();
        run(&mut shell, "asdf-");
    }

}
