pub use bevy::prelude::*;
pub use bevy_persistent::{
    prelude::*,
    storage::Storage,
};
pub use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Resource, Serialize)]
pub struct KeyBindings {
    pub jump: KeyCode,
    pub crouch: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> KeyBindings {
        KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C }
    }
}
