use std::env;
use std::process;

pub fn cd(path : Option<&str>) {
    match path {
        Some(path) => {
            env::set_var("PWD", path);
            env::set_current_dir(path);
        },
        None => {
            env::set_var("PWD", env::var("HOME").unwrap());
            env::set_current_dir(env::var("HOME").unwrap());
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

    #[test]
    fn cd_root() {
        cd(Some("/"));
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_etc() {
        cd(Some("/etc"));
        match env::var("PWD") {
            Ok(pwd) => assert!(pwd == "/etc"),
            Err(_) => panic!("PWD should be set.")
        }
    }

    #[test]
    fn cd_no_arg() {
        cd(None);
        match env::var("PWD") {
            Ok(pwd) => match env::var("HOME") {
                Ok(home) => assert!(pwd == home),
                Err(_) => panic!("HOME should be set.")
            },
            Err(_) => panic!("PWD should be set.")
        }
    }

}
