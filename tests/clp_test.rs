use std::io;
use rust_test::util::clp;

#[test]
fn get_copied_files_test() -> Result<(), io::Error>{
    let mut v:Vec<String> = vec![];
    clp::get_copied_files(&mut v);
    println!(
        "复制文件: {:?}",
        v
    );
    Ok(())
}