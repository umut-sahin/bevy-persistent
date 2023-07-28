//! A persistence error.

use crate::prelude::*;

/// A persistence error.
#[derive(Debug, Error)]
pub enum PersistenceError {
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

    #[cfg(any(
        feature = "ini",
        feature = "json",
        feature = "ron",
        feature = "toml",
        feature = "yaml"
    ))]
    #[error("{0}")]
    Encoding(
        #[from]
        #[source]
        std::str::Utf8Error,
    ),

    #[cfg(feature = "bincode")]
    #[error("{0}")]
    BincodeDeserialization(#[source] bincode::Error),
    #[cfg(feature = "bincode")]
    #[error("{0}")]
    BincodeSerialization(#[source] bincode::Error),

    #[cfg(feature = "ini")]
    #[error("{0}")]
    IniDeserialization(#[source] serde_ini::de::Error),
    #[cfg(feature = "ini")]
    #[error("{0}")]
    IniSerialization(#[source] serde_ini::ser::Error),

    #[cfg(feature = "json")]
    #[error("{0}")]
    JsonDeserialization(#[source] serde_json::Error),
    #[cfg(feature = "json")]
    #[error("{0}")]
    JsonSerialization(#[source] serde_json::Error),

    #[cfg(feature = "ron")]
    #[error("{0}")]
    RonDeserialization(#[source] ron::Error),
    #[cfg(feature = "ron")]
    #[error("{0}")]
    RonSerialization(#[source] ron::Error),

    #[cfg(feature = "toml")]
    #[error("{0}")]
    TomlDeserialization(#[source] toml::de::Error),
    #[cfg(feature = "toml")]
    #[error("{0}")]
    TomlSerialization(#[source] toml::ser::Error),

    #[cfg(feature = "yaml")]
    #[error("{0}")]
    YamlDeserialization(#[source] serde_yaml::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    YamlSerialization(#[source] serde_yaml::Error),
}

impl PersistenceError {
    pub fn is_serde(&self) -> bool {
        match self {
            #[cfg(not(target_family = "wasm"))]
            PersistenceError::Filesystem(_) => false,
            #[cfg(target_family = "wasm")]
            PersistenceError::Browser(_) => false,

            _ => true,
        }
    }
}
