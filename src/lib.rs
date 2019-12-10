pub mod days;

use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

pub enum FileType {
    Input,
    Example,
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::Input => "in",
            FileType::Example => "ex",
        }
        .to_string()
    }
}

pub fn custom_err<S: ToString>(s: S) -> AocErr {
    AocErr::Custom(s.to_string())
}

pub fn data_path() -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("data");
    cwd
}

pub fn day_path(day: usize) -> PathBuf {
    let mut p = data_path();
    p.push(format!("{:02}", day));
    p
}

pub fn file_path(file_type: FileType, day: usize, task: usize) -> PathBuf {
    let mut p = day_path(day);
    p.push(format!("{}_{:02}.data", file_type.to_string(), task));
    p
}

pub fn parse_file<T: FromStr>(file_type: FileType, day: usize, task: usize) -> AocResult<T>
where
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let p = file_path(file_type, day, task);
    let mut f = fs::File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    s.parse().map_err(|err| AocErr::Other(Box::new(err)))
}

pub struct ParseLineVec<T>(pub Vec<T>);

impl<T: FromStr> FromStr for ParseLineVec<T>
where
    <T as FromStr>::Err: std::error::Error + 'static,
{
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Self> {
        let mut v = Vec::new();
        for line in s.lines() {
            v.push(T::from_str(line).map_err(|err| AocErr::Other(Box::new(err)))?)
        }

        Ok(ParseLineVec(v))
    }
}
//std::num::ParseIntError

#[derive(Error, Debug)]
pub enum AocErr {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Parse Int parse")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Custom: {0}")]
    Custom(String),
    #[error("Other: {0}")]
    Other(Box<dyn std::error::Error>),
}

pub type AocResult<T> = Result<T, AocErr>;
