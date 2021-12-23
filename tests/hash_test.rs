use std::path::Path;
use anyhow::Error;
use rust_test::util::hash;
#[test]
fn test_hash() -> Result<(), Error>{
    let code = hash::hash_one(Path::new(r"C:\Users\BORBER\VirtualBox VMs\vm_default_1631520211017_54809\packer-virtualbox.vmdk"))?;
    println!("{}", code);
    Ok(())
}