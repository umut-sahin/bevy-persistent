//! A persistent resource.

use crate::prelude::*;

/// A persistent resource.
///
/// Persistent resources are Bevy resources which are automatically synchronized with the disk.
///
/// They require a name for logging, a path to be saved to and loaded from, a storage format,
/// and a default resource in case the persistent resource is created for the first time.
///
/// They are synchronized with the disk from the moment of their creation.
#[derive(Debug, Reflect, Resource)]
pub struct Persistent<R: Resource + Serialize + DeserializeOwned> {
    pub(crate) name: String,
    pub(crate) format: StorageFormat,
    pub(crate) storage: Storage,
    pub(crate) resource: R,
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Creates a persistent resource builder.
    pub fn builder() -> PersistentBuilder<R> {
        PersistentBuilder { name: None, format: None, path: None, default: None }
    }

    /// Creates a persistent resource.
    pub fn new(
        name: impl ToString,
        format: StorageFormat,
        storage: Storage,
        default: R,
    ) -> Result<Persistent<R>, PersistenceError> {
        let name = name.to_string();

        if !storage.occupied() {
            let resource = default;

            storage.initialize().map_err(|error| {
                // initialize can only return error for filesystem storage
                log::warn!(
                    "failed to create the parent directory for {} at {}: {}",
                    name,
                    storage,
                    error,
                );
                error
            })?;

            storage
                .write(&name, format, &resource)
                .map(|_| {
                    log::info!("saved default {} to {}", name, storage);
                })
                .map_err(|error| {
                    // serialization errors are already logged
                    if !error.is_serde() {
                        log::warn!("failed to save default {} to {}: {}", name, storage, error);
                    } else {
                        log::warn!(
                            "failed to save default {} to {} due to a serialization error",
                            name,
                            storage,
                        );
                    }
                    error
                })?;

            return Ok(Persistent { name, format, storage, resource });
        }

        let resource = storage.read(&name, format).map_err(|error| {
            // serialization errors are already logged
            if !error.is_serde() {
                log::warn!("failed to load {} from {}: {}", name, storage, error);
            } else {
                log::warn!(
                    "failed to load {} from {} due to a deserialization error",
                    name,
                    storage,
                );
            }
            error
        })?;

        log::info!("loaded {} from {}", name, storage);

        Ok(Persistent { name, format, storage, resource })
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Gets the name of the resource.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the storage format of the resource.
    pub fn format(&self) -> StorageFormat {
        self.format
    }

    /// Gets the storage of the resource.
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    /// Gets the resource.
    pub fn get(&self) -> &R {
        &self.resource
    }

    /// Gets the resource mutably.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.resource
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Sets the resource.
    ///
    /// Changes are synchronized with the underlying storage immediately.
    pub fn set(&mut self, new_resource: R) -> Result<(), PersistenceError> {
        self.resource = new_resource;
        self.persist()
    }

    /// Updates the resource.
    ///
    /// Changes are synchronized with the underlying storage immediately.
    pub fn update(&mut self, updater: impl Fn(&mut R)) -> Result<(), PersistenceError> {
        updater(&mut self.resource);
        self.persist()
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Writes the resource to the underlying storage.
    pub fn persist(&self) -> Result<(), PersistenceError> {
        self.storage
            .write(&self.name, self.format, &self.resource)
            .map(|_| {
                log::info!("saved new {} to {}", self.name, self.storage);
            })
            .map_err(|error| {
                // serialization errors are logged in format module
                if !error.is_serde() {
                    log::warn!("failed to save new {} to {}: {}", self.name, self.storage, error);
                } else {
                    log::warn!(
                        "failed to save new {} to {} due to a serialization error",
                        self.name,
                        self.storage,
                    );
                }
                error
            })
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Deref for Persistent<R> {
    type Target = R;

    fn deref(&self) -> &R {
        self.get()
    }
}

impl<R: Resource + Serialize + DeserializeOwned> DerefMut for Persistent<R> {
    fn deref_mut(&mut self) -> &mut R {
        self.get_mut()
    }
}
