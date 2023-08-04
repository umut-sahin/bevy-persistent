use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{
        WindowResized,
        WindowResolution,
    },
};
use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};
use std::path::Path;

#[derive(Resource, Serialize, Deserialize)]
struct WindowState {
    position: (i32, i32),
    size: (u32, u32),
}

fn main() {
    let mut app = App::new();
    app.add_plugins(LogPlugin::default());

    let state_dir = dirs::state_dir()
        .map(|native_state_dir| native_state_dir.join("bevy-persistent"))
        .unwrap_or(Path::new("local").join("state"))
        .join("examples")
        .join("smart-windows");

    let persistent_window_state = Persistent::<WindowState>::builder()
        .name("window state")
        .format(StorageFormat::Toml)
        .path(state_dir.join("window-state.toml"))
        .default(WindowState { position: (0, 0), size: (800, 600) })
        .build()
        .expect("failed to initialize window state");

    let title = "I am a smart window!".to_owned();
    let position = WindowPosition::At(IVec2::from(persistent_window_state.position));
    let resolution = WindowResolution::new(
        persistent_window_state.size.0 as f32,
        persistent_window_state.size.1 as f32,
    );

    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>().set(WindowPlugin {
        primary_window: Some(Window { title, position, resolution, ..Default::default() }),
        ..Default::default()
    }));

    app.insert_resource(persistent_window_state)
        .add_systems(Update, on_window_moved)
        .add_systems(Update, on_window_resized);

    app.run();
}

fn on_window_moved(
    events: EventReader<WindowMoved>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

fn on_window_resized(
    events: EventReader<WindowResized>,
    windows: Query<&Window>,
    window_state: ResMut<Persistent<WindowState>>,
) {
    if !events.is_empty() {
        update_window_state(window_state, windows.single());
    }
}

fn update_window_state(mut window_state: ResMut<Persistent<WindowState>>, window: &Window) {
    let position = match &window.position {
        WindowPosition::At(position) => (position.x, position.y),
        _ => unreachable!(),
    };
    let size = (window.resolution.physical_width(), window.resolution.physical_height());

    if window_state.position != position || window_state.size != size {
        window_state.set(WindowState { position, size }).ok();
    }
}
