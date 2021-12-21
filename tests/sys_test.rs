use std::io;
use rust_test::util::sys;


#[test]
fn test_current_dir() -> Result<(), io::Error>{
    println!(
        "当前运行目录: {:?}",
        sys::get_current_dir().unwrap()
    );
    Ok(())
}


#[test]
fn test_run_command() -> Result<(), io::Error>{
    let args = vec!["-h"];
    let dir = sys::get_current_dir().unwrap();
    let program = format!("{}{}", dir, "/bin/mlar");
    print!(
        "执行命令:\n{}\n运行结果: \n{}",
        program,
        sys::run_command(&program, &args).unwrap()
    );
    Ok(())
}
