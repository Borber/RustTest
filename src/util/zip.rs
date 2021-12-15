use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use tar::Archive;

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