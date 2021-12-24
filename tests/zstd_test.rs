use std::io;
use rust_test::util::zip;
#[test]
fn test_compress() -> io::Result<()>{
    zip::compress(r"C:\Users\BORBER\VirtualBox VMs\vm_default_1631520211017_54809\packer-virtualbox.vmdk")
}