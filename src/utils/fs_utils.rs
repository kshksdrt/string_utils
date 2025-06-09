use crc32fast::Hasher;

pub(crate) fn hash_file_path(input: &str) -> u32 {
    let mut hasher = Hasher::new();

    hasher.update(input.as_bytes());
    let hash_value = hasher.finalize();
    hash_value
}
