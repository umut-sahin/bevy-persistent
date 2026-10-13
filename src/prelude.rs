//! Preludes of the crate.

pub(crate) use crate::{
    builder::PersistentBuilder,
    error::PersistenceError,
    storage::Storage,
};
pub(crate) use bevy_ecs::prelude::*;
pub(crate) use bevy_reflect::prelude::*;
pub(crate) use bevy_log as log;
pub(crate) use serde::{
    Serialize,
    de::DeserializeOwned,
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
