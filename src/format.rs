//! A storage format.

use crate::prelude::*;

/// A storage format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageFormat {
    #[cfg(feature = "bincode")]
    Bincode,
    #[cfg(feature = "ini")]
    Ini,
    #[cfg(feature = "json")]
    Json,
    #[cfg(all(feature = "json", feature = "pretty"))]
    JsonPretty,
    #[cfg(feature = "ron")]
    Ron,
    #[cfg(all(feature = "ron", feature = "pretty"))]
    RonPretty,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(all(feature = "toml", feature = "pretty"))]
    TomlPretty,
    #[cfg(feature = "yaml")]
    Yaml,
}

#[cfg(any(
    feature = "bincode",
    feature = "ini",
    feature = "json",
    feature = "ron",
    feature = "toml",
    feature = "yaml",
))]
impl StorageFormat {
    /// Serializes a resource into bytes.
    pub fn serialize<R: Serialize + DeserializeOwned>(
        self,
        name: &str,
        resource: &R,
    ) -> Result<Vec<u8>, PersistenceError> {
        match self {
            #[cfg(feature = "bincode")]
            StorageFormat::Bincode => {
                bincode::serialize(resource).map_err(|error| {
                    log::warn!("failed to serialize {} to Bincode\n\n{}", name, error);
                    PersistenceError::BincodeSerialization(error)
                })
            },
            #[cfg(feature = "ini")]
            StorageFormat::Ini => {
                serde_ini::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to INI\n\n{}", name, error);
                        PersistenceError::IniSerialization(error)
                    })
            },
            #[cfg(feature = "json")]
            StorageFormat::Json => {
                serde_json::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to JSON\n\n{}", name, error);
                        PersistenceError::JsonSerialization(error)
                    })
            },
            #[cfg(all(feature = "json", feature = "pretty"))]
            StorageFormat::JsonPretty => {
                serde_json::to_string_pretty(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to pretty JSON\n\n{}", name, error);
                        PersistenceError::JsonSerialization(error)
                    })
            },
            #[cfg(feature = "ron")]
            StorageFormat::Ron => {
                ron::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to RON\n\n{}", name, error);
                        PersistenceError::RonSerialization(error)
                    })
            },
            #[cfg(all(feature = "ron", feature = "pretty"))]
            StorageFormat::RonPretty => {
                ron::ser::to_string_pretty(resource, Default::default())
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to pretty RON\n\n{}", name, error);
                        PersistenceError::RonSerialization(error)
                    })
            },
            #[cfg(feature = "toml")]
            StorageFormat::Toml => {
                toml::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to TOML\n\n{}", name, error);
                        PersistenceError::TomlSerialization(error)
                    })
            },
            #[cfg(all(feature = "toml", feature = "pretty"))]
            StorageFormat::TomlPretty => {
                toml::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to pretty TOML\n\n{}", name, error);
                        PersistenceError::TomlSerialization(error)
                    })
            },
            #[cfg(feature = "yaml")]
            StorageFormat::Yaml => {
                serde_yaml::to_string(resource)
                    .map(|serialized_resource| serialized_resource.into_bytes())
                    .map_err(|error| {
                        log::warn!("failed to serialize {} to YAML\n\n{}", name, error);
                        PersistenceError::YamlSerialization(error)
                    })
            },
        }
    }

    /// Deserializes a resource from bytes.
    pub fn deserialize<R: Serialize + DeserializeOwned>(
        self,
        name: &str,
        serialized_resource: &[u8],
    ) -> Result<R, PersistenceError> {
        #[cfg(feature = "bincode")]
        #[allow(irrefutable_let_patterns)]
        if let StorageFormat::Bincode = self {
            return bincode::deserialize::<R>(serialized_resource).map_err(|error| {
                log::warn!("failed to parse {} as Bincode\n\n{}", name, error);
                PersistenceError::BincodeDeserialization(error)
            });
        }

        #[cfg(any(feature = "ini", feature = "json", feature = "toml", feature = "yaml"))]
        let serialized_resource_str =
            std::str::from_utf8(serialized_resource).map_err(|error| {
                log::warn!("failed to decode {} as UTF-8\n\n{}", name, error);
                PersistenceError::Encoding(error)
            })?;

        match self {
            #[cfg(feature = "bincode")]
            StorageFormat::Bincode => unreachable!(),
            #[cfg(feature = "ini")]
            StorageFormat::Ini => {
                serde_ini::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as INI\n\n{}", name, error);
                    PersistenceError::IniDeserialization(error)
                })
            },
            #[cfg(feature = "json")]
            StorageFormat::Json => {
                serde_json::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as JSON\n\n{}", name, error);
                    PersistenceError::JsonDeserialization(error)
                })
            },
            #[cfg(all(feature = "json", feature = "pretty"))]
            StorageFormat::JsonPretty => {
                serde_json::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as pretty JSON\n\n{}", name, error);
                    PersistenceError::JsonDeserialization(error)
                })
            },
            #[cfg(feature = "ron")]
            StorageFormat::Ron => {
                ron::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as RON\n\n{}", name, error);
                    PersistenceError::RonDeserialization(error.into())
                })
            },
            #[cfg(all(feature = "ron", feature = "pretty"))]
            StorageFormat::RonPretty => {
                ron::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as pretty RON\n\n{}", name, error);
                    PersistenceError::RonDeserialization(error.into())
                })
            },
            #[cfg(feature = "toml")]
            StorageFormat::Toml => {
                toml::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as TOML\n\n{}", name, error);
                    PersistenceError::TomlDeserialization(error)
                })
            },
            #[cfg(all(feature = "toml", feature = "pretty"))]
            StorageFormat::TomlPretty => {
                toml::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as pretty TOML\n\n{}", name, error);
                    PersistenceError::TomlDeserialization(error)
                })
            },
            #[cfg(feature = "yaml")]
            StorageFormat::Yaml => {
                serde_yaml::from_str::<R>(serialized_resource_str).map_err(|error| {
                    log::warn!("failed to parse {} as YAML\n\n{}", name, error);
                    PersistenceError::YamlDeserialization(error)
                })
            },
        }
    }
}

#[cfg(not(any(
    feature = "bincode",
    feature = "ini",
    feature = "json",
    feature = "ron",
    feature = "toml",
    feature = "yaml",
)))]
impl StorageFormat {
    /// Serializes a resource into bytes.
    pub fn serialize<R: Serialize + DeserializeOwned>(
        self,
        _name: &str,
        _resource: &R,
    ) -> Result<Vec<u8>, PersistenceError> {
        unreachable!()
    }

    /// Deserializes a resource from bytes.
    pub fn deserialize<R: Serialize + DeserializeOwned>(
        self,
        _name: &str,
        _serialized_resource: &[u8],
    ) -> Result<R, PersistenceError> {
        unreachable!()
    }
}
