use std::io;
use rust_test::util::zip;
#[test]
fn test_compress() -> io::Result<()>{
    zip::compress(r"C:\Users\BORBER\Downloads\7z")
}