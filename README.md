# bevy-persistent

A [Bevy](https://bevyengine.org/) helper to easily manage resources that need to persist across game sessions.

## Background

In games, there are a lot of resources that need to persist across game sessions:
- Statistics (e.g., high scores, number of deaths, play time)
- Settings (e.g., key bindings, game difficulty, audio settings)
- States (e.g., last window position and size, saves)
- and many more...

This crate aims to simplify management of such resources!

## Installation

With all supported storage formats:

```shell
cargo add bevy-persistent --features all
```

Or explicitly:

```shell
cargo add bevy-persistent --features bincode,ini,json,toml,yaml
```

And of course, you can just pick the storage formats you're planning to use:

```shell
cargo add bevy-persistent --features bincode,toml
```

## Usage

### Prelude

You only need [Persistent\<R>](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html) and [StorageFormat](https://docs.rs/bevy-persistent/latest/bevy_persistent/storage/enum.StorageFormat.html) types to use the library, and they are exported from the prelude module.

```rust
use bevy_persistent::prelude::*;
```

### Definition

You need to define the [Resource](https://docs.rs/bevy/latest/bevy/ecs/system/trait.Resource.html) you want to persist, and it needs to implement [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits  from [serde](https://github.com/serde-rs/serde).

```rust
#[derive(Resource, Serialize, Deserialize)]
struct KeyBindings {
  jump: KeyCode,
  crouch: KeyCode,
}
```

### Creation

In your setup system, you can create the persistent resource and insert it to your game.

```rust
fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    commands.insert_resource(
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path(config_dir.join("key-bindings.toml"))
            .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
            .build(),
    )
}
```

If it's the first run, the resource will have the specified default value and that default value will be saved to the specified path in the specified format. Otherwise, key bindings will be loaded from the specified path using the specified format.

If any failures happen at any point (e.g., no permission to read/write to the specified path), the error will be logged, and the specified default value will be used for the resource.

### Access

To access the resource, you can have a parameter of type `Res<Persistent<R>>`.

```rust
fn access_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    log::info!("you can crouch using {:?}", key_bindings.crouch);
}
```

`Persistent<R>` implements `Deref<Target = R>` so you can access public fields/methods of your resource easily.

### Modification

To modify the resource, you can have a parameter of type `ResMut<Persistent<R>>`.

```rust
fn modify_key_bindings(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
  key_bindings.update(|key_bindings| {
    key_bindings.crouch = KeyCode::LControl;
  });
}
```

[Persistent\<R>](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html) has [set](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.set) and [update](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.update) methods to modify the underlying resource. Both of those methods write the updated resource to the disk before returning.

## Examples

There are a few examples that you can run directly and play around with in the [examples](https://github.com/umut-sahin/bevy-persistent/tree/main/examples) folder.

```shell
cargo run --release --features all --example name-of-the-example
```

## License

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) is free, open source and permissively licensed, just like [Bevy](https://bevyengine.org/)!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!
