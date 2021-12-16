use std::{env, error, io};
use crate::errors::DaMeiError;

pub fn get_current_dir() -> Result<String , io::Error> {
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


#[test]
fn test() -> Result<(), io::Error>{
    println!(
        "当前运行目录: {:?}",
        get_current_dir_3().unwrap()
    );
    Ok(())
}
