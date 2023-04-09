use bevy::{
    app::AppExit,
    log,
    prelude::*,
};
use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use std::path::Path;

#[derive(Debug, Deserialize, Resource, Serialize)]
struct KeyBindings {
    jump: KeyCode,
    crouch: KeyCode,
}

fn main() {
    App::new()
        .add_plugins(
            // We don't need a window for this example.
            DefaultPlugins.set(WindowPlugin { primary_window: None, ..Default::default() }),
        )
        .add_startup_system(setup)
        .add_systems(
            (show_initial_key_bindings, switch_crouch_key, show_final_key_bindings, exit).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("bevy-persistent"))
        .unwrap_or(Path::new("session").join("configuration"))
        .join("examples")
        .join("alternating-crouch-key");

    commands.insert_resource(
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path(config_dir.join("key-bindings.toml"))
            .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
            .build(),
    )
}

fn show_initial_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    log::info!("initial key bindings: {:?}", key_bindings.get());
}

fn switch_crouch_key(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
    key_bindings.update(|key_bindings| {
        key_bindings.crouch = match key_bindings.crouch {
            KeyCode::C => KeyCode::LControl,
            KeyCode::LControl => KeyCode::C,
            _ => unimplemented!(),
        }
    });
}

fn show_final_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    log::info!("final key bindings: {:?}", key_bindings.get());
}

fn exit(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
