use std::io;
use std::path::Path;
use rust_test::util::file;

#[test]
fn file_test() -> Result<(), io::Error>{
    let data = file::metadata(Path::new("bin/mlar"));
    println!("{:?}", data.unwrap().len());
    Ok(())
}

#[test]
fn test_save_file_to_fake_gz() -> Result<(), io::Error>{
    let path = Path::new(r"data/test.txt");
    println!("{:?}", file::read_file_header(path).unwrap());
    file::to_fake_gz(path);
    println!("{:?}", file::read_file_header(path).unwrap());
    Ok(())
}