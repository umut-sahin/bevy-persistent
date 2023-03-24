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
    #[cfg(feature = "ron")]
    Ron,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}

#[cfg(any(
    feature = "bincode",
    feature = "ini",
    feature = "json",
    feature = "ron",
    feature = "toml",
    feature = "yaml"
))]
impl StorageFormat {
    /// Serializes a resource into bytes.
    pub fn serialize<R: Resource + Serialize + DeserializeOwned>(
        self,
        name: &str,
        resource: &R,
    ) -> Option<Vec<u8>> {
        match self {
            #[cfg(feature = "bincode")]
            StorageFormat::Bincode => {
                match bincode::serialize(resource) {
                    Ok(serialized_resource) => Some(serialized_resource),
                    Err(error) => {
                        log::warn!("failed to serialize {} to binary\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "ini")]
            StorageFormat::Ini => {
                match serde_ini::to_string(resource) {
                    Ok(serialized_resource) => Some(serialized_resource.into_bytes()),
                    Err(error) => {
                        log::warn!("failed to serialize {} to INI\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "json")]
            StorageFormat::Json => {
                match serde_json::to_string(resource) {
                    Ok(serialized_resource) => Some(serialized_resource.into_bytes()),
                    Err(error) => {
                        log::warn!("failed to serialize {} to JSON\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "ron")]
            StorageFormat::Ron => {
                match ron::to_string(resource) {
                    Ok(serialized_resource) => Some(serialized_resource.into_bytes()),
                    Err(error) => {
                        log::warn!("failed to serialize {} to RON\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "toml")]
            StorageFormat::Toml => {
                match toml::to_string(resource) {
                    Ok(serialized_resource) => Some(serialized_resource.into_bytes()),
                    Err(error) => {
                        log::warn!("failed to serialize {} to TOML\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "yaml")]
            StorageFormat::Yaml => {
                match serde_yaml::to_string(resource) {
                    Ok(serialized_resource) => Some(serialized_resource.into_bytes()),
                    Err(error) => {
                        log::warn!("failed to serialize {} to YAML\n\n{}", name, error);
                        None
                    },
                }
            },
        }
    }

    /// Deserializes a resource from bytes.
    pub fn deserialize<R: Resource + Serialize + DeserializeOwned>(
        self,
        name: &str,
        serialized_resource: &[u8],
    ) -> Option<R> {
        #[cfg(feature = "bincode")]
        #[allow(irrefutable_let_patterns)]
        if let StorageFormat::Bincode = self {
            return bincode::deserialize::<R>(serialized_resource)
                .map_err(|error| {
                    log::warn!("failed to parse {} as binary\n\n{}", name, error);
                    error
                })
                .ok();
        }

        #[cfg(any(feature = "ini", feature = "json", feature = "toml", feature = "yaml"))]
        let serialized_resource_str = match std::str::from_utf8(serialized_resource) {
            Ok(serialized_resource_str) => serialized_resource_str,
            Err(error) => {
                log::warn!("failed to parse {} as utf8 string\n\n{}", name, error);
                return None;
            },
        };

        match self {
            #[cfg(feature = "bincode")]
            StorageFormat::Bincode => unreachable!(),
            #[cfg(feature = "ini")]
            StorageFormat::Ini => {
                match serde_ini::from_str::<R>(serialized_resource_str) {
                    Ok(resource) => Some(resource),
                    Err(error) => {
                        log::warn!("failed to parse {} as INI\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "json")]
            StorageFormat::Json => {
                match serde_json::from_str::<R>(serialized_resource_str) {
                    Ok(resource) => Some(resource),
                    Err(error) => {
                        log::warn!("failed to parse {} as JSON\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "ron")]
            StorageFormat::Ron => {
                match ron::from_str::<R>(serialized_resource_str) {
                    Ok(resource) => Some(resource),
                    Err(error) => {
                        log::warn!("failed to parse {} as RON\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "toml")]
            StorageFormat::Toml => {
                match toml::from_str::<R>(serialized_resource_str) {
                    Ok(resource) => Some(resource),
                    Err(error) => {
                        log::warn!("failed to parse {} as TOML\n\n{}", name, error);
                        None
                    },
                }
            },
            #[cfg(feature = "yaml")]
            StorageFormat::Yaml => {
                match serde_yaml::from_str::<R>(serialized_resource_str) {
                    Ok(resource) => Some(resource),
                    Err(error) => {
                        log::warn!("failed to parse {} as YAML\n\n{}", name, error);
                        None
                    },
                }
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
    feature = "yaml"
)))]
impl StorageFormat {
    /// Serializes a resource into bytes.
    pub fn serialize<R: Resource + Serialize + DeserializeOwned>(
        self,
        _name: &str,
        _resource: &R,
    ) -> Option<Vec<u8>> {
        unreachable!()
    }

    /// Deserializes a resource from bytes.
    pub fn deserialize<R: Resource + Serialize + DeserializeOwned>(
        self,
        _name: &str,
        _serialized_resource: &[u8],
    ) -> Option<R> {
        unreachable!()
    }
}
