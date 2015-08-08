use std::io;
use std::io::Write;
use std::process::Command;
use std::iter::FromIterator;

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
        for arg in iter {
            command.arg(arg);
        }
        Some(command)
    } else {
        None
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
