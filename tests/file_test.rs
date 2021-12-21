use std::io;
use rust_test::util::file;

#[test]
fn file_test() -> Result<(), io::Error>{
    let data = file::metadata("bin/mlar");
    println!("{:?}", data.unwrap().len());
    Ok(())
}