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
#[derive(Component, Debug, Resource)]
pub struct Persistent<R: Resource + Serialize + DeserializeOwned> {
    pub(crate) name: String,
    pub(crate) format: StorageFormat,
    pub(crate) storage: Storage,
    pub(crate) resource: Option<R>,
    pub(crate) default: Option<Box<R>>,
    pub(crate) revert_to_default_on_deserialization_errors: bool,
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Creates a persistent resource builder.
    pub fn builder() -> PersistentBuilder<R> {
        PersistentBuilder {
            name: None,
            format: None,
            path: None,
            loaded: true,
            default: None,
            revertible: false,
            revert_to_default_on_deserialization_errors: false,
        }
    }

    /// Creates a persistent resource.
    ///
    /// # Panics
    ///
    /// Panics if `revert_to_default_on_deserialization_errors`
    /// is set to `true` but `revertible` is set to `false`.
    pub fn new(
        name: impl ToString,
        format: StorageFormat,
        storage: Storage,
        loaded: bool,
        default: R,
        revertible: bool,
        revert_to_default_on_deserialization_errors: bool,
    ) -> Result<Persistent<R>, PersistenceError> {
        if revert_to_default_on_deserialization_errors && !revertible {
            panic!(
                "revert to default on deserialization errors \
                is set for a non-revertible persistent resource"
            );
        }

        let name = name.to_string();

        if !storage.occupied() {
            // first run

            storage.initialize().map_err(|error| {
                // initialize can only return error for filesystem storage
                log::error!(
                    "failed to create the parent directory for {} at {}: {}",
                    name,
                    storage,
                    error,
                );
                error
            })?;

            storage
                .write(&name, format, &default)
                .map(|_| {
                    log::info!("saved default {} to {}", name, storage);
                })
                .map_err(|error| {
                    // serialization errors are already logged
                    if !error.is_serde() {
                        log::error!("failed to save default {} to {}: {}", name, storage, error);
                    } else {
                        log::error!(
                            "failed to save default {} to {} due to a serialization error",
                            name,
                            storage,
                        );
                    }
                    error
                })?;

            let resource = if loaded {
                // we need to make a copy of the default resource without using clone
                // this is because cloning can have special semantics
                // e.g., cloning Persistent<Arc<RwLock<R>>> and changing it
                // would change the default object, which is not desired
                let serialized = format.serialize(&name, &default).map_err(|error| {
                    log::error!("failed to clone default {} due to a serialization error", name);
                    error
                })?;
                let reconstructed = format.deserialize(&name, &serialized).map_err(|error| {
                    log::error!("failed to clone default {} due to a deserialization error", name);
                    error
                })?;

                Some(reconstructed)
            } else {
                None
            };
            let default = if revertible { Some(Box::new(default)) } else { None };

            return Ok(Persistent {
                name,
                format,
                storage,
                resource,
                default,
                revert_to_default_on_deserialization_errors,
            });
        }

        let default = if revertible { Some(Box::new(default)) } else { None };

        if !loaded {
            return Ok(Persistent {
                name,
                format,
                storage,
                resource: None,
                default,
                revert_to_default_on_deserialization_errors,
            });
        }

        let resource = match storage.read(&name, format) {
            Ok(resource) => resource,
            Err(error) => {
                if !error.is_serde() {
                    log::error!("failed to load {} from {}: {}", name, storage, error);
                } else {
                    log::error!(
                        "failed to load {} from {} due to a deserialization error",
                        name,
                        storage,
                    );

                    if revert_to_default_on_deserialization_errors {
                        log::info!(
                            "attempting to revert {} to default in {} automatically",
                            name,
                            storage,
                        );

                        let mut result = Persistent {
                            name,
                            format,
                            storage,
                            resource: None,
                            default,
                            revert_to_default_on_deserialization_errors,
                        };
                        if result.revert_to_default().is_err() {
                            // return the original deserialization error
                            return Err(error);
                        }
                        if loaded && result.revert_to_default_in_memory().is_err() {
                            // return the original deserialization error
                            return Err(error);
                        }

                        return Ok(result);
                    }
                }
                return Err(error);
            },
        };

        log::info!("loaded {} from {}", name, storage);

        Ok(Persistent {
            name,
            format,
            storage,
            resource: Some(resource),
            default,
            revert_to_default_on_deserialization_errors,
        })
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

    /// Gets if the resource is revertible.
    pub fn is_revertible(&self) -> bool {
        self.default.is_some()
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
                log::error!(
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
        match self.storage.read(&self.name, self.format) {
            Ok(resource) => self.resource = Some(resource),
            Err(error) => {
                if !error.is_serde() {
                    log::error!("failed to reload {} from {}: {}", self.name, self.storage, error);
                } else {
                    log::error!(
                        "failed to reload {} from {} due to a deserialization error",
                        self.storage,
                        self.name,
                    );

                    if self.revert_to_default_on_deserialization_errors {
                        log::info!(
                            "attempting to revert {} to default in {} automatically",
                            self.name,
                            self.storage,
                        );
                        if self.revert_to_default().is_err() {
                            // return the original deserialization error
                            return Err(error);
                        }
                        return Ok(());
                    }
                }
                return Err(error);
            },
        }
        log::info!("reloaded {} from {}", self.name, self.storage);
        Ok(())
    }

    /// Reverts the resource to it's default value.
    ///
    /// Loaded status is kept upon reloading.
    ///
    /// # Panics
    ///
    /// Panics if the resource is not revertible.
    pub fn revert_to_default(&mut self) -> Result<(), PersistenceError> {
        if !self.is_revertible() {
            panic!("tried to revert non-revertible {}", self.name);
        }

        self.storage
            .write(&self.name, self.format, self.default.as_ref().unwrap())
            .map(|_| {
                log::info!("reverted {} to default in {}", self.name, self.storage);
            })
            .map_err(|error| {
                // serialization errors are logged in format module
                if !error.is_serde() {
                    log::error!(
                        "failed to revert {} to default in {}: {}",
                        self.name,
                        self.storage,
                        error,
                    );
                } else {
                    log::error!(
                        "failed to revert {} to default in {} due to a serialization error",
                        self.name,
                        self.storage,
                    );
                }
                error
            })?;

        if self.is_loaded() {
            self.revert_to_default_in_memory()?;
        }

        Ok(())
    }

    /// Reverts the resource to it's default value only in memory, not persistent storage.
    ///
    /// # Panics
    ///
    /// Panics if the resource is not revertible.
    pub fn revert_to_default_in_memory(&mut self) -> Result<(), PersistenceError> {
        if !self.is_revertible() {
            panic!("tried to revert non-revertible {}", self.name);
        }

        // we need to make a copy of the default resource without using clone
        // this is because cloning can have special semantics
        // e.g., cloning Persistent<Arc<RwLock<R>>> and changing it
        // would change the default object, which is not desired
        let serialized =
            self.format.serialize(&self.name, self.default.as_ref().unwrap()).map_err(|error| {
                log::error!(
                    "failed to revert {} to default in memory due to a serialization error",
                    self.name,
                );
                error
            })?;
        let reconstructed = self.format.deserialize(&self.name, &serialized).map_err(|error| {
            log::error!(
                "failed to revert {} to default in memory due to a deserialization error",
                self.name,
            );
            error
        })?;

        self.resource = Some(reconstructed);
        log::info!("reverted {} to default in memory", self.name);
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
                        log::error!(
                            "failed to save new {} to {}: {}",
                            self.name,
                            self.storage,
                            error,
                        );
                    } else {
                        log::error!(
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
