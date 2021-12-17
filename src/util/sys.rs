use std::env;
use std::process::Command;
use crate::errors::DaMeiError;

pub fn get_current_dir() -> Result<String , DaMeiError> {
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

pub fn run_command(program: &str, args: &Vec<&str>) -> Result<String, DaMeiError> {
    let output = Command::new(program)
        .args(args)
        .output()?;
    if !output.status.success() {
        return Err(DaMeiError::RunCommandError(program.to_owned() + &*args.join(" ")));
    };
    Ok(String::from_utf8_lossy(&*output.stdout).to_string())
}
