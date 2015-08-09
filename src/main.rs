use std::io;
use std::io::Write;
use std::process::Command;
use std::process::ExitStatus;
use std::iter::FromIterator;

#[macro_use]
extern crate lazy_static;
extern crate readline;

mod utilities;
mod lexer;

fn main() {
    loop {
        let mut input = readline::readline(&get_ps1()).unwrap();
        readline::add_history(&input);
        input.push('\n');
        run(&input);
    }
}

fn get_ps1() -> String {
    match std::env::var("PS1") {
        Ok(ps1) => ps1,
        Err(_) => String::from("$")
    }
}

pub fn run(command : &str) -> Option<ExitStatus> {
    let mut iter = lexer::lex(command).into_iter();
    let program = match iter.next() {
        Some(program) => program,
        None => return None
    };
    let args = iter.collect::<Vec<_>>();

    if run_builtin_if_possible(&program, &args) {
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

fn run_builtin_if_possible(program: &str, args: &Vec<String>) -> bool {
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
        assert!(run("ls\n").is_some());
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
