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
    file::to_fake_gz(path, "c1c2cc9fcef387715aab3b3b7a253e4c68721291a7739a60faf8dd467347c9e4");
    println!("{:?}", file::read_file_header(path).unwrap());
    Ok(())
}