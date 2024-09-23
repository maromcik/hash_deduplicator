use md5::Digest;

pub struct File {
    pub filename: String,
    pub hash: Digest
}