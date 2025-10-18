#![allow(unused_imports)]

pub use bevy::prelude::*;
pub use bevy_persistent::{
    prelude::*,
    storage::Storage,
};
pub use serde::{
    Deserialize,
    Serialize,
};
pub use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Resource, Serialize)]
pub struct KeyBindings {
    pub jump: KeyCode,
    pub crouch: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> KeyBindings {
        KeyBindings { jump: KeyCode::Space, crouch: KeyCode::KeyC }
    }
}
