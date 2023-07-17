//! A storage.

use crate::prelude::*;

/// A storage.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn initialize(&self) -> Result<(), PersistenceError> {
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
    ) -> Result<R, PersistenceError> {
        match self {
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => {
                let bytes = std::fs::read(path)?;
                format.deserialize::<R>(name, &bytes)
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
                    return format.deserialize::<R>(name, &bytes);
                }

                let content = LocalStorage::get::<String>(key)?;
                format.deserialize::<R>(name, content.as_bytes())
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
                    return format.deserialize::<R>(name, &bytes);
                }

                let content = SessionStorage::get::<String>(key)?;
                format.deserialize::<R>(name, content.as_bytes())
            },
        }
    }

    /// Writes a resource to the storage.
    pub fn write<R: Resource + Serialize + DeserializeOwned>(
        &self,
        name: &str,
        format: StorageFormat,
        resource: &R,
    ) -> Result<(), PersistenceError> {
        match self {
            #[cfg(not(target_family = "wasm"))]
            Storage::Filesystem { path } => {
                let bytes = format.serialize(name, resource)?;

                use std::io::Write;
                std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(path)
                    .and_then(|mut file| file.write_all(&bytes))?;
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
                    let bytes = format.serialize(name, resource)?;
                    LocalStorage::set::<&[u8]>(key, &bytes)?;
                    return Ok(());
                }

                let bytes = format.serialize(name, resource)?;

                // unwrapping is okay in this case because
                // remaining storage formats all return a string
                // and that string is converted to bytes
                let string = std::str::from_utf8(&bytes).unwrap();
                LocalStorage::set::<&str>(key, string)?;
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
                    let bytes = format.serialize(name, resource)?;
                    SessionStorage::set::<&[u8]>(key, &bytes)?;
                    return Ok(());
                }

                let bytes = format.serialize(name, resource)?;

                // unwrapping is okay in this case because
                // remaining storage formats all return a string
                // and that string is converted to bytes
                let string = std::str::from_utf8(&bytes).unwrap();
                SessionStorage::set::<&str>(key, string)?;
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
