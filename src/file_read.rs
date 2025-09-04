use std::{fs, io};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ErrMessage {
    #[error("error reading file")]
    ReadIOError(#[from] io::Error),
}
#[allow(unused)]
pub fn read_file(file_path: &str) -> Result<Vec<u8>, ErrMessage> {
    let data = fs::read(file_path)?;
    Ok(data)
}
