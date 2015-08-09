use std::env;
use std::process;

pub fn cd(args : Vec<&str>) {
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
        env::set_current_dir(path);
        env::set_var("PWD", path);
        match original_pwd{
            Ok(old_pwd) => env::set_var("OLDPWD", old_pwd),
            Err(_) => () // No action needed if PWD was unset
        }
    }
}

pub fn exit() {
    process::exit(0);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
    use std::sync::Mutex;
    use std::thread;

    extern crate tempdir;

    lazy_static! {
        static ref ENV_VAR_MUTEX: Mutex<u8> = {
            let m = Mutex::new(0);
            m
        };
    }

    #[test]
    fn cd_root() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(vec!["/"]);
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_etc() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(vec!["/etc"]);
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/etc"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_no_arg() {
        let mutex = ENV_VAR_MUTEX.lock().unwrap();
        cd(vec![]);
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
        cd(vec![]);
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
        cd(vec![first]);
        cd(vec![second]);
        cd(vec!["-"]);
        assert_eq!(first, env::current_dir().ok().unwrap().to_str().unwrap());
        assert_eq!(second, env::var("OLDPWD").ok().unwrap());
        cd(vec!["-"]);
        assert_eq!(second, env::current_dir().ok().unwrap().to_str().unwrap());
        assert_eq!(first, env::var("OLDPWD").ok().unwrap());
    }

}
