//! A builder for a persistent resource.

use crate::prelude::*;

/// A builder for a persistent resource.
pub struct PersistentBuilder<R: Resource + Serialize + DeserializeOwned> {
    pub(crate) name: Option<String>,
    pub(crate) format: Option<StorageFormat>,
    pub(crate) path: Option<PathBuf>,
    pub(crate) default: Option<R>,
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
}

impl<R: Resource + Serialize + DeserializeOwned> PersistentBuilder<R> {
    /// Builds the persistent resource.
    ///
    /// # Panics
    ///
    /// Panics if `name`, `path`, `format` or `default` is not set.
    pub fn build(self) -> Persistent<R> {
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

        #[allow(unreachable_code)]
        Persistent::new(
            self.name.unwrap(),
            self.format.unwrap(),
            self.path.unwrap(),
            self.default.unwrap(),
        )
    }
}
