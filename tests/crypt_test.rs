use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use rust_test::util::crypt::{encrypt_chacha, decrypt_chacha};
use uuid::Uuid;
use rust_test::errors::DaMieError;
use rust_test::util::{crypt, file, sys};
use rust_test::util::hash::get_blake3_hash;

// [31, 139, 8, 0, 54, 182, 186, 97, 0, 0, 236, 189, 123, 92, 84, 101, 254, 56, 126, 6,
// [31, 139, 8, 0, 117, 218, 186, 97, 0, 0, 1, 185, 1, 70, 254, 77, 76, 65, 1, 0, 0, 0, 3, 1,
// [31, 139, 8, 0, 208, 217, 186, 97, 0, 0, 1, 48, 0, 207, 255, 48,

#[test]
fn test_encrypt() -> Result<(), Box<dyn std::error::Error>>{
    // let path = Path::new(r"C:\Users\BORBER\Downloads\Lark-win32_ia32-4.10.16-signed.exe");
    // let mut data = file::read_file(path)?;
    let data = b"67890";
    let data2 = b"12456";
    let hash = get_blake3_hash(&data.to_vec())?;
    let uuid = Uuid::new_v4();
    // let encrypt_data = encrypt_chacha(data.to_vec(), hash.to_string().as_str().split_at(32).0)?;
    let encrypt_data = encrypt_chacha(data.to_vec(), "41649a37ade3a3c0bf2a12dfe4f1c8ba");
    let encrypt_data2 = encrypt_chacha(data2.to_vec(), "41649a37ade3a3c0bf2a12dfe4f1c8ba");
    println!("{}", hash.to_string().as_str().split_at(32).0);
    let mut file = File::create(Path::new("data").join("encrypt"))?;
    file.write_all(&encrypt_data)?;
    file.write_all(&encrypt_data2)?;
    Ok(())
}

#[test]
fn test_decrypt() -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new(r"data/encrypt");
    let mut data = file::read_file(path)?;
    let hash = "41649a37ade3a3c0bf2a12dfe4f1c8baa517701c08cc635f405016d3234b8c0b";
    let uuid = Uuid::new_v4();
    let encrypt_data = decrypt_chacha(data, "41649a37ade3a3c0bf2a12dfe4f1c8ba")?;
    println!("{}", hash.to_string());
    let mut file = File::create(Path::new("data").join("decrypt"))?;
    file.write_all(&encrypt_data)?;
    Ok(())
}

