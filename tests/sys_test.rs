use std::io;
use rust_test::util::sys;


#[test]
fn test() -> Result<(), io::Error>{
    println!(
        "当前运行目录: {:?}",
        sys::get_current_dir().unwrap()
    );
    Ok(())
}


#[test]
fn test_run_command() -> Result<(), io::Error>{
    let args = vec!["-h"];
    let program = r"C:\Users\BORBER\CLionProjects\RustTest\bin\mlar";
    print!(
        "运行结果: \n{}",
        sys::run_command(program, &args).unwrap()
    );
    Ok(())
}
