use std::io;
use rust_test::util::uuid;

#[test]
fn uuid_test() -> Result<(), io::Error>{
    uuid::new();
    Ok(())
}