//! Preludes of the crate.

pub(crate) use crate::{
    builder::PersistentBuilder,
    storage::{
        Storage,
        StorageError,
    },
};
pub(crate) use bevy::{
    log,
    prelude::*,
};
pub(crate) use serde::{
    de::DeserializeOwned,
    Serialize,
};
pub(crate) use std::{
    fmt::{
        self,
        Display,
    },
    ops::{
        Deref,
        DerefMut,
    },
    path::PathBuf,
};
pub(crate) use thiserror::Error;

pub use crate::{
    format::StorageFormat,
    persistent::Persistent,
};
