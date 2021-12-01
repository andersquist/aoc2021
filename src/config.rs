use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub fn path() -> PathBuf {
    dirs::config_dir()
        .expect("no home directory exists")
        .join("adventofcode")
        .join("2021.json")
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Session cookie
    pub session: String,
    /// Path to input files
    pub input_files: Option<PathBuf>,
}

impl Config {
    pub fn save(&self) -> Result<(), Error> {
        let path = path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let serialized = serde_json::to_string(self)?;
        std::fs::write(path, serialized.as_bytes()).map_err(Into::into)
    }

    pub fn load() -> Result<Self, Error> {
        let content = std::fs::read(path())?;
        serde_json::from_slice(&content).map_err(Into::into)
    }

    pub fn input_files(&self) -> PathBuf {
        match self.input_files {
            Some(ref input_files) => input_files.to_owned(),
            None => match std::env::current_dir() {
                Ok(current) => current.join("input"),
                Err(_) => dirs::config_dir()
                    .expect("no home directory exists")
                    .join("adventofcode")
                    .join("2021"),
            },
        }
    }

    pub fn input_for(&self, day: u8) -> PathBuf {
        self.input_files().join(format!("input-{:02}.txt", day))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CouldNotSave(#[from] std::io::Error),
    #[error(transparent)]
    CouldNotSerialize(#[from] serde_json::Error),
}
