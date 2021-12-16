mod util;
mod errors;

use std::env;
use walkdir::WalkDir;
use crate::util::{clp, zip, file, sys};
use rand::rngs::OsRng;

fn main()  -> Result<(), std::io::Error> {
    println!("当前运行路径: {}", sys::get_current_dir().unwrap() );
    //}
    // let mut csprng = OsRng {};
    // let keypair = generate_keypair(&mut csprng).unwrap();
    // println!("{:?}", keypair.public_der);
    // println!("{:?}", keypair.private_der);
    // let mut v:Vec<String> = vec![];
    // let count = clp::get_copied_files(&mut v);
    // match count {
    //     0 => println!("你还没复制呢!"),
    //     _ => {
    //         for i in v {
    //             for entry in WalkDir::new(i).into_iter().filter_map(|e| e.ok()) {
    //                 println!("{}", entry.path().display());
    //             }
    //         }
    //     }
    // }
    // let path = v.get(0).unwrap();
    // println!("{}", path);
    // zip::pack("backup.tar.gz", "data", path);
    // zip::unpack("backup.tar.gz", r"C:\Users\BORBER\Desktop");
    Ok(())
}

