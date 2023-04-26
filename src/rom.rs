use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;
use sha1::{Sha1, Digest};
use anyhow::Result;

#[derive(Debug)]
pub struct Rom {
    path: PathBuf,
}

impl Rom {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn file(&self) -> std::io::Result<File> {
        File::open(self.path())
    }

    pub fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut file = self.file()?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Ok(buffer)
    }

    pub fn sha1_string(&self) -> Result<String> {
        let mut bytes = self.read_bytes()?;
        let generic_arr = Sha1::digest(&mut bytes);
        let mut hex = String::new();
        for byte in generic_arr.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        Ok(hex)
    }
}

impl From<PathBuf> for Rom {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl fmt::Display for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path().display())?;
        if let Ok(sha1) = self.sha1_string() {
            write!(f, " (SHA1: {})", sha1)?;
        }
        Ok(())
    }
}
