use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Result;
use crate::errors::{DaMieError, rs_error};

pub fn get_exe_path() -> Result<PathBuf> {
    let current = env::current_exe()?;
    match current.parent() {
        None => {
            Ok(current)
        }
        Some(path) => {
            Ok(path.to_path_buf())
        }
    }
}

pub fn get_path() -> Result<PathBuf> {
    Ok(env::current_dir()?)
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
