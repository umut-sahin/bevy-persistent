//! A builder for a persistent resource.

use crate::prelude::*;

/// A builder for a persistent resource.
pub struct PersistentBuilder<R: Resource + Serialize + DeserializeOwned> {
    pub(crate) name: Option<String>,
    pub(crate) format: Option<StorageFormat>,
    pub(crate) path: Option<PathBuf>,
    pub(crate) default: Option<R>,
    pub(crate) loaded: bool,
}

impl<R: Resource + Serialize + DeserializeOwned> PersistentBuilder<R> {
    /// Sets the name of the resource.
    pub fn name(mut self, name: impl ToString) -> PersistentBuilder<R> {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the storage format of the resource.
    pub fn format(mut self, format: StorageFormat) -> PersistentBuilder<R> {
        self.format = Some(format);
        self
    }

    /// Sets the path of the resource.
    pub fn path(mut self, path: impl Into<PathBuf>) -> PersistentBuilder<R> {
        self.path = Some(path.into());
        self
    }

    /// Sets the default value of the resource.
    pub fn default(mut self, resource: R) -> PersistentBuilder<R> {
        self.default = Some(resource);
        self
    }

    /// Sets the initial loaded status of the resource.
    pub fn loaded(mut self, loaded: bool) -> PersistentBuilder<R> {
        self.loaded = loaded;
        self
    }

    /// Sets the initial unloaded status of the resource.
    pub fn unloaded(mut self, unloaded: bool) -> PersistentBuilder<R> {
        self.loaded = !unloaded;
        self
    }
}

impl<R: Resource + Serialize + DeserializeOwned> PersistentBuilder<R> {
    /// Builds the persistent resource.
    ///
    /// # Panics
    ///
    /// Panics if `name`, `path`, `format` or `default` is not set.
    pub fn build(self) -> Result<Persistent<R>, PersistenceError> {
        if self.name.is_none() {
            panic!("persistent resource name is not set");
        }
        if self.format.is_none() {
            panic!("persistent resource format is not set");
        }
        if self.path.is_none() {
            panic!("persistent resource path is not set");
        }
        if self.default.is_none() {
            panic!("persistent resource default is not set");
        }

        let name = self.name.unwrap();
        let format = self.format.unwrap();
        let path = self.path.unwrap();
        let default = self.default.unwrap();
        let loaded = self.loaded;

        let storage = {
            #[cfg(not(target_family = "wasm"))]
            {
                Storage::Filesystem { path: path.canonicalize().unwrap_or(path) }
            }
            #[cfg(target_family = "wasm")]
            {
                let separator = std::path::MAIN_SEPARATOR_STR;
                let path = path.strip_prefix(separator).unwrap_or(&path);

                if let Ok(Some(key)) = path.strip_prefix("local").map(|p| p.to_str()) {
                    Storage::LocalStorage { key: key.to_owned() }
                } else if let Ok(Some(key)) = path.strip_prefix("session").map(|p| p.to_str()) {
                    Storage::SessionStorage { key: key.to_owned() }
                } else {
                    panic!(
                        "persistent resource path should start with \
                        \"local\" or \"session\" and be UTF-8 encoded in WebAssembly",
                    );
                }
            }
        };

        Persistent::new(name, format, storage, default, loaded)
    }
}
