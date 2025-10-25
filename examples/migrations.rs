use bevy::{
    app::AppExit,
    prelude::*,
};
use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::path::Path;

// Imagine that the old version of the project had this definition:
//
// pub struct Statistics {
//     n_kills: usize,
//     xp_gained: f32,
// }
//
// But to improve clarity and precision, it's updated to this definition:

#[derive(Debug, Resource)]
pub struct Statistics {
    number_of_enemies_killed: usize,
    xp_gained: f64,
}

// To make the new version compatible with the old version, we will create a proxy type
// that can be (de)serialized to all previous versions like so:

#[derive(Serialize, Deserialize)]
enum StatisticsSerde {
    V1 { n_kills: usize, xp_gained: f32 },
    V2 { number_of_enemies_killed: usize, xp_gained: f64 },
}

// Then we'll implement serialization and deserialization for `Statistics` manually
// which will allow us to write a custom migration for all previous version:

impl Serialize for Statistics {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let &Self { number_of_enemies_killed, xp_gained } = self;
        StatisticsSerde::V2 { number_of_enemies_killed, xp_gained }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Statistics {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match StatisticsSerde::deserialize(deserializer)? {
            StatisticsSerde::V1 { n_kills, xp_gained } => {
                Statistics { number_of_enemies_killed: n_kills, xp_gained: xp_gained as f64 }
            },
            StatisticsSerde::V2 { number_of_enemies_killed, xp_gained } => {
                Statistics { number_of_enemies_killed, xp_gained }
            },
        })
    }
}

// With these traits implemented, we can work with `Persistent<Statistics>` directly.
//
// When you first run this example, you'll see something like:
// ```
// INFO bevy_persistent::persistent: loaded statistics from /path/to/bevy-persistent/examples/migration/statistics.toml
// INFO migrations: Statistics { number_of_enemies_killed: 0, xp_gained: 0.0 }
// ```
//
// If you go ahead an open the file, it'll contain:
// ```
// [V2]
// number_of_enemies_killed = 0
// xp_gained = 0.0
// ```
//
// Feel free to edit it to be:
// ```
// [V1]
// n_kills = 3
// xp_gained = 10.0
// ```
//
// And run the example again. It'll handle the migration
// on its own and create the correct `Statistics` object:
// ```
// INFO migrations: Statistics { number_of_enemies_killed: 3, xp_gained: 10.0 }
// ```
//
// Upon modifications, it'll update the persistent
// storage using the latest version of the object.
// This means the older versions might not work after
// running a more recent version (as they don't have the newest definition).
//
// Note that this approach is not bullet-proof. If the underlying fields
// are not properly versioned, things can still break.
//
// As an example, when Bevy v0.13 is released, it changed the names of `KeyCode`
// enum variants (e.g., `KeyCode::W` -> `KeyCode::KeyW`), details can be found at
// https://bevy.org/learn/migration-guides/0-12-to-0-13/#keycode-changes.
// This meant deserialization of structs with a `KeyCode` property would
// fail after the upgrade.
//
// Technically, this can be mitigated by using standard (e.g., `String`, `HashMap`)
// or manually defined (i.e., like `Statistics` above) structures in the custom
// serializer in this example, but it requires care, so be careful.

fn main() {
    App::new()
        .add_plugins(
            // We don't need a window for this example.
            DefaultPlugins.set(WindowPlugin { primary_window: None, ..default() }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (show_statistics, exit).chain())
        .run();
}

fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("bevy-persistent"))
        .unwrap_or(Path::new("session").join("configuration"))
        .join("examples")
        .join("migration");

    commands.insert_resource(
        Persistent::<Statistics>::builder()
            .name("statistics")
            .format(StorageFormat::Toml)
            .path(config_dir.join("statistics.toml"))
            .default(Statistics { number_of_enemies_killed: 0, xp_gained: 0.0 })
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize statistics"),
    )
}

fn show_statistics(statistics: Res<Persistent<Statistics>>) {
    info!("{:?}", statistics.get());
}

fn exit(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
