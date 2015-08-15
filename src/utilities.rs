use std::env;
use std::process;
use std::collections::HashMap;

use super::shell;

pub fn alias(shell : &mut shell::Shell, args : &Vec<String>) {
    if args.len() == 0 {
        for alias in shell.aliases.keys() {
            print_alias(&shell.aliases, alias);
        }
    }
    for arg in args.iter() {
        let mut split = arg.splitn(2, "=");
        let alias_name = split.next().unwrap();
        let alias_value = split.next();
        if alias_value.is_some() {
            shell.aliases.insert(alias_name.to_string(), alias_value.unwrap().to_string());
        }
        else {
            print_alias(&shell.aliases, &alias_name.to_string());
        }
    }
}

fn print_alias(aliases : &HashMap<String, String>, alias : &String) {
    match aliases.get(alias) {
        Some(alias_value) => println!("alias {}='{}'", alias, alias_value),
        None => println!("nosh: alias: {}: not found", alias)
    }
}

pub fn cd(args : &Vec<String>) {
    match args.first() {
        Some(path) => change_directory(path),
        None => {
            match env::var("HOME") {
                Ok(home) => change_directory(&home),
                Err(_) => ()
            }
        }
    }
}

fn change_directory(path : &str) {
    let original_pwd = env::var("PWD");
    if path == "-" {
        match env::var("OLDPWD") {
            Ok(old_pwd) => change_directory(&old_pwd),
            Err(_) => panic!("Should give warning if OLDPWD is not set")
        };
        let output = process::Command::new("pwd")
               .output()
               .ok()
               .unwrap();
        print!("{}", String::from_utf8(output.stdout).unwrap());
    }
    else {
        match env::set_current_dir(path) {
            Err(e) => println!("{}", e),
            _ => ()
        }
        env::set_var("PWD", path);
        match original_pwd {
            Ok(old_pwd) => env::set_var("OLDPWD", old_pwd),
            Err(_) => ()  // No action needed if PWD was unset
        }
    }
}

pub fn exit() {
    process::exit(0);
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::shell;
    use std::env;
    use std::sync::Mutex;

    extern crate tempdir;

    lazy_static! {
        static ref ENV_VAR_MUTEX: Mutex<u8> = {
            let m = Mutex::new(0);
            m
        };
    }

    fn string_vec(vec : Vec<&str>) -> Vec<String> {
        vec.into_iter().map(String::from).collect()
    }

    #[test]
    fn cd_root() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(&string_vec(vec!["/"]));
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_etc() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(&string_vec(vec!["/etc"]));
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/etc"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_no_arg() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(&string_vec(vec![]));
        match env::var("PWD") {
            Ok(pwd) => match env::var("HOME") {
                Ok(home) => assert!(pwd == home),
                Err(_) => panic!("HOME should be set.")
            },
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_no_arg_home_unset() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        let old_home = env::var("HOME");
        let current_pwd = env::current_dir();
        env::remove_var("HOME");
        cd(&string_vec(vec![]));
        match (current_pwd, env::current_dir()) {
            (Ok(expected), Ok(actual)) => assert_eq!(expected, actual),
            (Err(expected), Err(actual)) => (),
            _ => panic!("Once call to current_dir failed")
        };
        match old_home { // cleanup
            Ok(home) => env::set_var("HOME", home),
            Err(_) => ()
        };
    }

    #[test]
    fn cd_hyphen() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        let first_dir = tempdir::TempDir::new("first").unwrap();
        let second_dir = tempdir::TempDir::new("second").unwrap();
        let first = first_dir
            .path()
            .to_str()
            .unwrap();
        let second = second_dir
            .path()
            .to_str()
            .unwrap();
        cd(&string_vec(vec![first]));
        cd(&string_vec(vec![second]));
        cd(&string_vec(vec!["-"]));
        assert_eq!(first, env::current_dir().ok().unwrap().to_str().unwrap());
        assert_eq!(second, env::var("OLDPWD").ok().unwrap());
        cd(&string_vec(vec!["-"]));
        assert_eq!(second, env::current_dir().ok().unwrap().to_str().unwrap());
        assert_eq!(first, env::var("OLDPWD").ok().unwrap());
    }

    #[test]
    fn cd_relative() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        let parent = tempdir::TempDir::new("parent").unwrap();
        let child = tempdir::TempDir::new_in(parent.path(), "child").unwrap();
        let parent_path = parent
            .path()
            .to_str()
            .unwrap();
        let child_path = child
            .path()
            .to_str()
            .unwrap();
        let child_dir_name = child_path
            .split('/')
            .last()
            .unwrap();
        cd(&string_vec(vec![parent_path]));
        cd(&string_vec(vec![child_dir_name]));
        assert_eq!(child_path, env::current_dir().ok().unwrap().to_str().unwrap());
    }

    #[test]
    fn add_alias() {
        let mut shell = shell::Shell::new();
        let args : &Vec<String> = &string_vec(vec!["ll=ls -l"]);
        alias(&mut shell, args);
        assert_eq!(shell.aliases.get(&"ll".to_string()).unwrap(), &"ls -l".to_string());
    }

}
