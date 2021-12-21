use std::env;
use std::process::Command;
use crate::errors::{DaMieError, rs_error};

pub fn get_current_dir() -> Result<String , DaMieError> {
    let current = env::current_exe()?;
    match current.parent() {
        None => {
            Ok(String::from("/"))
        }
        Some(path) => {
            Ok(path.to_str().unwrap().to_string())
        }
    }
}

pub fn run_command(program: &str, args: &Vec<&str>) -> Result<String, DaMieError> {
    let output = Command::new(program)
        .args(args)
        .output()?;
    if !output.status.success() {
        return Err(rs_error(format!("{}{}",program, args.join(" "))));
    };
    Ok(String::from_utf8_lossy(&*output.stdout).to_string())
}
