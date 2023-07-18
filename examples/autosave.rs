use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    path::Path,
    time::Duration,
};

const PLAYER_SIZE: f32 = 50.00;
const PLAYER_SPEED: f32 = 500.00;

const AUTOSAVE_INTERVAL_SECONDS: f32 = 3.0;

#[derive(Component)]
struct Player;

#[derive(Default, Resource, Serialize, Deserialize)]
struct GameState {
    player_position: Vec3,
}

#[derive(Resource)]
struct AutosaveTimer {
    timer: Timer,
}

fn main() {
    let state_dir = dirs::state_dir()
        .map(|native_state_dir| native_state_dir.join("bevy-persistent"))
        .unwrap_or(Path::new("local").join("state"))
        .join("examples")
        .join("autosave");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(
            Persistent::<GameState>::builder()
                .name("game state")
                .format(StorageFormat::Bincode)
                .path(state_dir.join("game-state.bin"))
                .default(GameState::default())
                .build()
                .expect("failed to initialize game state"),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, autosave.after(player_movement))
        .run();
}

fn setup(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    game_state: Res<Persistent<GameState>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Player).insert(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(game_state.player_position),
        ..default()
    });

    commands.insert_resource(AutosaveTimer {
        timer: Timer::new(Duration::from_secs_f32(AUTOSAVE_INTERVAL_SECONDS), TimerMode::Repeating),
    });
}

fn player_movement(
    time: Res<Time>,

    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,

    mut game_state: ResMut<Persistent<GameState>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() == 0.0 {
            return;
        }

        direction = direction.normalize();
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

        game_state.player_position = transform.translation;
    }
}

fn autosave(
    time: Res<Time>,
    mut autosave: ResMut<AutosaveTimer>,
    game_state: Res<Persistent<GameState>>,
) {
    autosave.timer.tick(time.delta());
    if autosave.timer.finished() {
        game_state.persist().ok();
    }
}
