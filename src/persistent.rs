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
    pub(crate) resource: Option<R>,
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Creates a persistent resource builder.
    pub fn builder() -> PersistentBuilder<R> {
        PersistentBuilder { name: None, format: None, path: None, default: None, loaded: true }
    }

    /// Creates a persistent resource.
    pub fn new(
        name: impl ToString,
        format: StorageFormat,
        storage: Storage,
        default: R,
        loaded: bool,
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

            let resource = if loaded { Some(resource) } else { None };
            return Ok(Persistent { name, format, storage, resource });
        }

        if !loaded {
            return Ok(Persistent { name, format, storage, resource: None });
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

        Ok(Persistent { name, format, storage, resource: Some(resource) })
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

    /// Gets if the resource is loaded.
    pub fn is_loaded(&self) -> bool {
        self.resource.is_some()
    }

    /// Gets if the resource is unloaded.
    pub fn is_unloaded(&self) -> bool {
        self.resource.is_none()
    }

    /// Gets the resource.
    ///
    /// # Panics
    ///
    /// Panics if the resource is unloaded.
    pub fn get(&self) -> &R {
        if let Some(resource) = &self.resource {
            resource
        } else {
            panic!("tried to get unloaded {}", self.name);
        }
    }

    /// Gets the resource mutably.
    ///
    /// # Panics
    ///
    /// Panics if the resource is unloaded.
    pub fn get_mut(&mut self) -> &mut R {
        if let Some(resource) = &mut self.resource {
            resource
        } else {
            panic!("tried to get unloaded {} mutably", self.name);
        }
    }

    /// Tries to get the resource.
    pub fn try_get(&self) -> Option<&R> {
        self.resource.as_ref()
    }

    /// Tries to get the resource mutably.
    pub fn try_get_mut(&mut self) -> Option<&mut R> {
        self.resource.as_mut()
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Sets the resource.
    ///
    /// Changes are synchronized with the underlying storage immediately.
    pub fn set(&mut self, new_resource: R) -> Result<(), PersistenceError> {
        self.resource = Some(new_resource);
        self.persist()
    }

    /// Updates the resource.
    ///
    /// Changes are synchronized with the underlying storage immediately.
    ///
    /// # Panics
    ///
    /// Panics if the resource is unloaded.
    pub fn update(&mut self, updater: impl Fn(&mut R)) -> Result<(), PersistenceError> {
        if let Some(resource) = self.resource.as_mut() {
            updater(resource);
            self.persist()
        } else {
            panic!("tried to update unloaded {}", self.name);
        }
    }

    /// Unloads the resource from memory.
    ///
    /// Changes are synchronized with the underlying storage before unloading.
    ///
    /// # Panics
    ///
    /// Panics if the resource is unloaded.
    pub fn unload(&mut self) -> Result<(), PersistenceError> {
        if self.resource.is_some() {
            self.persist().map_err(|error| {
                log::warn!(
                    "failed to unload {} due to not being able to persist it before unloading",
                    self.name,
                );
                error
            })?;
            self.resource = None;
            log::info!("unloaded {}", self.name);
        }
        Ok(())
    }

    /// Unloads the resource from memory immediately.
    ///
    /// Changes are **not** synchronized with the underlying storage before unloading.
    pub fn unload_without_persisting(&mut self) {
        if self.resource.is_some() {
            self.resource = None;
            log::info!("unloaded {} without persisting", self.name);
        }
    }

    /// Reloads the resource from the underlying storage.
    ///
    /// If reloading fails, the underlying resource is kept untouched.
    pub fn reload(&mut self) -> Result<(), PersistenceError> {
        self.resource = Some(self.storage.read(&self.name, self.format).map_err(|error| {
            // serialization errors are already logged
            if !error.is_serde() {
                log::warn!("failed to reload {} from {}: {}", self.name, self.storage, error);
            } else {
                log::warn!(
                    "failed to reload {} from {} due to a deserialization error",
                    self.storage,
                    self.name,
                );
            }
            error
        })?);
        log::info!("reloaded {} from {}", self.name, self.storage);
        Ok(())
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Writes the resource to the underlying storage.
    ///
    /// # Panics
    ///
    /// Panics if the resource is unloaded.
    pub fn persist(&self) -> Result<(), PersistenceError> {
        if let Some(resource) = &self.resource {
            self.storage
                .write(&self.name, self.format, resource)
                .map(|_| {
                    log::info!("saved new {} to {}", self.name, self.storage);
                })
                .map_err(|error| {
                    // serialization errors are logged in format module
                    if !error.is_serde() {
                        log::warn!(
                            "failed to save new {} to {}: {}",
                            self.name,
                            self.storage,
                            error,
                        );
                    } else {
                        log::warn!(
                            "failed to save new {} to {} due to a serialization error",
                            self.name,
                            self.storage,
                        );
                    }
                    error
                })
        } else {
            panic!("tried to save unloaded {}", self.name);
        }
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
