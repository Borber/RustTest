use std::io;
use rust_test::util::config;

#[test]
fn get_config() -> Result<(), io::Error>{
    print!("{}", config::get_config(&String::from("log/settings.toml")).unwrap());
    Ok(())
}