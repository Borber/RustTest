use crate::errors::DaMieError;

pub fn get_blake3_hash(data: &Vec<u8>) -> Result<blake3::Hash, DaMieError> {
    let hash: blake3::Hash = if data.len() < 128000 {
        blake3::hash(&data)
    } else {
        let input: &[u8] = &data;
        let mut hasher = blake3::Hasher::new();
        hasher.update_rayon(input);
        hasher.finalize()
    };
    Ok(hash)
}