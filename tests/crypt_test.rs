use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use enc_file::{decrypt_chacha, encrypt_chacha, encrypt_file};
use uuid::Uuid;
use rust_test::errors::DaMieError;
use rust_test::util::{crypt, file, sys};
use rust_test::util::crypt::get_blake3_hash;

// [31, 139, 8, 0, 54, 182, 186, 97, 0, 0, 236, 189, 123, 92, 84, 101, 254, 56, 126, 6,
// [31, 139, 8, 0, 117, 218, 186, 97, 0, 0, 1, 185, 1, 70, 254, 77, 76, 65, 1, 0, 0, 0, 3, 1,
// [31, 139, 8, 0, 208, 217, 186, 97, 0, 0, 1, 48, 0, 207, 255, 48,

#[test]
fn test_encrypt() -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new(r"C:\Users\BORBER\Downloads\Lark-win32_ia32-4.10.16-signed.exe");
    let mut data = file::read_file(path)?;
    let hash = get_blake3_hash(&data)?;
    let uuid = Uuid::new_v4();
    let encrypt_data = encrypt_chacha(data, hash.to_string().as_str().split_at(32).0)?;
    println!("{}", hash.to_string());
    let mut file = File::create(path.parent().unwrap().join(uuid.to_simple().to_string().as_str()))?;
    file.write_all(&encrypt_data)?;
    Ok(())
}

#[test]
fn test_decrypt() -> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new(r"data/cacec6229041419dac9ca45d26fce40c");
    let mut data = file::read_file(path)?;
    let hash = "c1c2cc9fcef387715aab3b3b7a253e4c68721291a7739a60faf8dd467347c9e4";
    let uuid = Uuid::new_v4();
    let encrypt_data = decrypt_chacha(data, &hash[0..32])?;
    println!("{}", hash.to_string());
    let mut file = File::create(path.parent().unwrap().join(uuid.to_simple().to_string().as_str()))?;
    file.write_all(&encrypt_data)?;
    Ok(())
}

