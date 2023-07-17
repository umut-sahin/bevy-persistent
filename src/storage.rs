//! A storage.

use crate::prelude::*;

/// A storage.
#[derive(Clone, Debug, Eq, PartialEq, Reflect)]
pub enum Storage {
    #[cfg(not(target_family = "wasm"))]
    Filesystem { path: PathBuf },
    #[cfg(target_family = "wasm")]
    LocalStorage { key: String },
    #[cfg(target_family = "wasm")]
    SessionStorage { key: String },
}

impl Storage {
    /// Initializes the storage.
    pub fn initialize(&self) -> Result<(), StorageError> {
        match self {
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            },
            #[cfg(target_family = "wasm")]
            Storage::LocalStorage { .. } => {},
            #[cfg(target_family = "wasm")]
            Storage::SessionStorage { .. } => {},
        }
        Ok(())
    }

    /// Gets if the storage is occupied.
    pub fn occupied(&self) -> bool {
        match self {
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => path.exists(),
            #[cfg(target_family = "wasm")]
            Storage::LocalStorage { key } => {
                use gloo_storage::{
                    LocalStorage,
                    Storage,
                };
                matches!(LocalStorage::raw().get_item(key), Ok(Some(_)))
            },
            #[cfg(target_family = "wasm")]
            Storage::SessionStorage { key } => {
                use gloo_storage::{
                    SessionStorage,
                    Storage,
                };
                matches!(SessionStorage::raw().get_item(key), Ok(Some(_)))
            },
        }
    }

    /// Reads a resource from the storage.
    pub fn read<R: Resource + Serialize + DeserializeOwned>(
        &self,
        name: &str,
        format: StorageFormat,
    ) -> Result<R, StorageError> {
        Ok(match self {
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => {
                let bytes = std::fs::read(path)?;
                if let Some(resource) = format.deserialize(name, &bytes) {
                    resource
                } else {
                    return Err(StorageError::Serde);
                }
            },
            #[cfg(target_family = "wasm")]
            Storage::LocalStorage { key } => {
                use gloo_storage::{
                    LocalStorage,
                    Storage,
                };

                #[cfg(feature = "json")]
                if format == StorageFormat::Json {
                    return Ok(LocalStorage::get::<R>(key)?);
                }
                #[cfg(all(feature = "json", feature = "pretty"))]
                if format == StorageFormat::JsonPretty {
                    return Ok(LocalStorage::get::<R>(key)?);
                }

                #[cfg(feature = "bincode")]
                if format == StorageFormat::Bincode {
                    let bytes = LocalStorage::get::<Vec<u8>>(key)?;
                    return match format.deserialize::<R>(name, &bytes) {
                        Some(resource) => Ok(resource),
                        None => Err(StorageError::Serde),
                    };
                }

                let content = LocalStorage::get::<String>(key)?;
                match format.deserialize::<R>(name, content.as_bytes()) {
                    Some(resource) => resource,
                    None => return Err(StorageError::Serde),
                }
            },
            #[cfg(target_family = "wasm")]
            Storage::SessionStorage { key } => {
                use gloo_storage::{
                    SessionStorage,
                    Storage,
                };

                #[cfg(feature = "json")]
                if format == StorageFormat::Json {
                    return Ok(SessionStorage::get::<R>(key)?);
                }
                #[cfg(all(feature = "json", feature = "pretty"))]
                if format == StorageFormat::JsonPretty {
                    return Ok(SessionStorage::get::<R>(key)?);
                }

                #[cfg(feature = "bincode")]
                if format == StorageFormat::Bincode {
                    let bytes = SessionStorage::get::<Vec<u8>>(key)?;
                    return match format.deserialize::<R>(name, &bytes) {
                        Some(resource) => Ok(resource),
                        None => Err(StorageError::Serde),
                    };
                }

                let content = SessionStorage::get::<String>(key)?;
                match format.deserialize::<R>(name, content.as_bytes()) {
                    Some(resource) => resource,
                    None => return Err(StorageError::Serde),
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
            #[cfg(not(target_family = "wasm"))]
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
            #[cfg(target_family = "wasm")]
            Storage::LocalStorage { key } => {
                use gloo_storage::{
                    LocalStorage,
                    Storage,
                };

                #[cfg(feature = "json")]
                if format == StorageFormat::Json {
                    LocalStorage::set::<&R>(key, resource)?;
                    return Ok(());
                }
                #[cfg(all(feature = "json", feature = "pretty"))]
                if format == StorageFormat::JsonPretty {
                    LocalStorage::set::<&R>(key, resource)?;
                    return Ok(());
                }

                #[cfg(feature = "bincode")]
                if format == StorageFormat::Bincode {
                    if let Some(bytes) = format.serialize(name, resource) {
                        LocalStorage::set::<&[u8]>(key, &bytes)?;
                    } else {
                        return Err(StorageError::Serde);
                    }
                    return Ok(());
                }

                if let Some(bytes) = format.serialize(name, resource) {
                    // unwrapping is okay in this case because
                    // remaining storage formats all return a string
                    // and that string is converted to bytes
                    let string = std::str::from_utf8(&bytes).unwrap();
                    LocalStorage::set::<&str>(key, string)?;
                } else {
                    return Err(StorageError::Serde);
                }
            },
            #[cfg(target_family = "wasm")]
            Storage::SessionStorage { key } => {
                use gloo_storage::{
                    SessionStorage,
                    Storage,
                };

                #[cfg(feature = "json")]
                if format == StorageFormat::Json {
                    SessionStorage::set::<&R>(key, resource)?;
                    return Ok(());
                }
                #[cfg(all(feature = "json", feature = "pretty"))]
                if format == StorageFormat::JsonPretty {
                    SessionStorage::set::<&R>(key, resource)?;
                    return Ok(());
                }

                #[cfg(feature = "bincode")]
                if format == StorageFormat::Bincode {
                    if let Some(bytes) = format.serialize(name, resource) {
                        SessionStorage::set::<&[u8]>(key, &bytes)?;
                    } else {
                        return Err(StorageError::Serde);
                    }
                    return Ok(());
                }

                if let Some(bytes) = format.serialize(name, resource) {
                    // unwrapping is okay in this case because
                    // remaining storage formats all return a string
                    // and that string is converted to bytes
                    let string = std::str::from_utf8(&bytes).unwrap();
                    SessionStorage::set::<&str>(key, string)?;
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
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => {
                if let Some(path) = path.to_str() {
                    write!(f, "{}", path)
                } else {
                    write!(f, "{:?}", path)
                }
            },
            #[cfg(target_family = "wasm")]
            Storage::LocalStorage { key } => {
                let separator = std::path::MAIN_SEPARATOR;
                write!(f, "{}local{}{}", separator, separator, key)
            },
            #[cfg(target_family = "wasm")]
            Storage::SessionStorage { key } => {
                let separator = std::path::MAIN_SEPARATOR;
                write!(f, "{}session{}{}", separator, separator, key)
            },
        }
    }
}

/// A storage error.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("(de)serialization failed")]
    Serde,
    #[cfg(not(target_family = "wasm"))]
    #[error("{0}")]
    Filesystem(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[cfg(target_family = "wasm")]
    #[error("{0}")]
    Browser(
        #[from]
        #[source]
        gloo_storage::errors::StorageError,
    ),
}
