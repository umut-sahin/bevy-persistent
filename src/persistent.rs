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
#[derive(Debug, Resource)]
pub struct Persistent<R: Resource + Serialize + DeserializeOwned> {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) storage: StorageFormat,
    pub(crate) resource: R,
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Creates a persistent resource builder.
    pub fn builder() -> PersistentBuilder<R> {
        PersistentBuilder { name: None, path: None, storage: None, default: None }
    }

    /// Creates a persistent resource.
    pub fn new(
        name: impl ToString,
        storage: StorageFormat,
        path: impl Into<PathBuf>,
        default: R,
    ) -> Persistent<R> {
        let name = name.to_string();
        let path = path.into();

        let path = path.canonicalize().unwrap_or(path);
        if !path.exists() {
            let resource = default;
            let serialized_resource =
                if let Some(serialized_resource) = storage.serialize(&name, &resource) {
                    serialized_resource
                } else {
                    // serialization in the condition will log errors
                    return Persistent { name, path, storage, resource };
                };

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|error| {
                        log::warn!(
                            "failed to create the parent directory for default {} at {:?}: {}",
                            name,
                            path,
                            error,
                        );
                    })
                    .ok();
            }
            std::fs::write(&path, serialized_resource)
                .map(|_| {
                    log::info!("saved default {} to {:?}", name, path);
                })
                .map_err(|error| {
                    log::warn!("failed to save default {} to {:?}: {}", name, path, error);
                })
                .ok();

            return Persistent { name, path, storage, resource };
        }

        log::info!("loading {} from {:?}", name, path);

        let resource = match std::fs::read(&path) {
            Ok(content) => storage.deserialize(&name, &content).unwrap_or(default),
            Err(error) => {
                log::warn!("failed to read {}: {}", name, error);
                return Persistent { name, path, storage, resource: default };
            },
        };

        Persistent { name, path, storage, resource }
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Gets the name of the resource.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the storage format of the resource.
    pub fn format(&self) -> StorageFormat {
        self.storage
    }

    /// Gets the path of the resource.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the resource.
    pub fn get(&self) -> &R {
        &self.resource
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    /// Sets the resource.
    ///
    /// Changes are synchronized with the disk immediately.
    pub fn set(&mut self, new_resource: R) {
        self.resource = new_resource;
        self.sync();
    }

    /// Updates the resource.
    ///
    /// Changes are synchronized with the disk immediately.
    pub fn update(&mut self, updater: impl Fn(&mut R)) {
        updater(&mut self.resource);
        self.sync();
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Persistent<R> {
    fn sync(&mut self) {
        if let Some(serialized_resource) = self.storage.serialize(self.name(), &self.resource) {
            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .and_then(|mut file| file.write_all(&serialized_resource))
                .map(|_| {
                    log::info!("saved new {} to {:?}", self.name, self.path);
                })
                .map_err(|error| {
                    log::warn!("failed to save new {} to {:?}: {}", self.name, self.path, error);
                })
                .ok();
        }
    }
}

impl<R: Resource + Serialize + DeserializeOwned> Deref for Persistent<R> {
    type Target = R;

    fn deref(&self) -> &R {
        self.get()
    }
}
