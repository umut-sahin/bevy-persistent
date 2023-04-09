//! A storage.

use crate::prelude::*;

/// A storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Storage {
    Filesystem { path: PathBuf },
}

impl Storage {
    /// Initializes the storage.
    pub fn initialize(&self) -> Result<(), StorageError> {
        match self {
            Storage::Filesystem { path } => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            },
        }
        Ok(())
    }

    /// Gets if the storage is occupied.
    pub fn occupied(&self) -> bool {
        match self {
            Storage::Filesystem { path } => path.exists(),
        }
    }

    /// Reads a resource from the storage.
    pub fn read<R: Resource + Serialize + DeserializeOwned>(
        &self,
        name: &str,
        format: StorageFormat,
    ) -> Result<R, StorageError> {
        Ok(match self {
            Storage::Filesystem { path } => {
                let bytes = std::fs::read(path)?;
                if let Some(resource) = format.deserialize(name, &bytes) {
                    resource
                } else {
                    return Err(StorageError::Serde);
                }
            },
        })
    }

    /// Writes a resource to the storage.
    pub fn write<R: Resource + Serialize + DeserializeOwned>(
        &self,
        name: &str,
        format: StorageFormat,
        resource: &R,
    ) -> Result<(), StorageError> {
        match self {
            Storage::Filesystem { path } => {
                if let Some(bytes) = format.serialize(name, resource) {
                    use std::io::Write;
                    std::fs::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(path)
                        .and_then(|mut file| file.write_all(&bytes))?;
                } else {
                    return Err(StorageError::Serde);
                }
            },
        }
        Ok(())
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Storage::Filesystem { path } => {
                if let Some(path) = path.to_str() {
                    write!(f, "{}", path)
                } else {
                    write!(f, "{:?}", path)
                }
            },
        }
    }
}

/// A storage error.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("(de)serialization failed")]
    Serde,
    #[error("{0}")]
    Filesystem(
        #[from]
        #[source]
        std::io::Error,
    ),
}
