use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::io;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashError{
    pub reason: String
}
impl HashError{
    fn new(reason:String)->Self{
        return HashError{
            reason
        }
    }
}
impl Display for HashError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash Error:{}", self.reason)
    }
}
impl Error for HashError{}
fn sha256sum(src: PathBuf) -> Result<String, Box<dyn Error>>{
    let mut file=File::open(src)?;
    let mut hasher=Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash=hasher.finalize();
    let hash=hex::encode(hash);
    Ok(hash)
}

pub fn check_file(src:PathBuf, dst: PathBuf)->Result<(), Box<dyn Error>>{
    let src_sum=sha256sum(src)?;
    let dst_sum=sha256sum(dst)?;
    if src_sum == dst_sum{
        return Ok(())
    }
    else{
        return Err(Box::new(HashError::new("Mismatch hash code".to_string())));
    }
}