use std::fs::File;
use std::{fs, io};
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use tar::Archive;
// 7z.exe a -t7z -p123456 pack.7z -mhe=on -m0=Copy "C:\Users\BORBER\VirtualBox VMs\vm_default_1631520211017_54809\packer-virtualbox.vmdk"
const SUFFIX: &'static str = ".zst";

pub fn pack(output: &str,path: &str, src_path: &str) -> Result<(), std::io::Error> {
    // Create Gzip file
    let compressed_file = File::create(output)?;
    let mut encoder = GzEncoder::new(compressed_file, Compression::default());

    {
        // Create tar archive and compress files
        let mut archive = tar::Builder::new(&mut encoder);
        archive.append_dir_all(path, src_path)?;
    }
    // Finish Gzip file
    encoder.finish()?;
    Ok(())
}

pub fn unpack(path: &str, dst: &str) -> Result<(), std::io::Error>{
    let tar_gz = File::open(path)?;
    Archive::new(GzDecoder::new(tar_gz)).unpack(dst)?;
    Ok(())
}

pub fn compress(source: &str) -> Result<(), io::Error> {
    let mut file = fs::read_dir(source)?;
    let mut encoder = {
        let target = fs::File::create(source.to_string() + SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

pub fn decompress(source: &str) -> io::Result<()> {
    let mut decoder = {
        let file = fs::File::open(source)?;
        zstd::Decoder::new(file)?
    };

    let mut target = fs::File::create(source.trim_end_matches(SUFFIX))?;

    io::copy(&mut decoder, &mut target)?;

    Ok(())
}