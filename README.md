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

You need to define the [Resource](https://docs.rs/bevy/latest/bevy/ecs/system/trait.Resource.html) you want to persist, and it needs to implement [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits from [serde](https://github.com/serde-rs/serde).

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

## Manual Persistence

Some resources are updated frequently and persisting on each small update might not be desirable. Or persistence could have to be triggered manually (e.g., auto saves on certain points in the game).

For such cases, you can avoid using [set](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.set) and [update](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.update) methods and update the resource directly.

```rust
fn modify_key_bindings(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
  key_bindings.crouch = KeyCode::LControl;
}
```

When you want the resource to persist with its current value, you can use [persist](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.persist) method.

```rust
fn persist_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
  key_bindings.persist();
}
```

## Prettifying

It's a good idea to store some resources with a prettified format during development to easily observe/modify them.

You can use `pretty` feature to enable prettified textual formats:

```toml
[features]
debug = ["bevy-persistent/pretty"]
```

And in your game:

```rust
fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    commands.insert_resource(
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format({
                #[cfg(feature = "debug")]
                {
                    StorageFormat::JsonPretty
                }
                #[cfg(not(feature = "debug"))]
                {
                    StorageFormat::Json
                }
            })
            .path(config_dir.join("key-bindings.toml"))
            .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
            .build(),
    )
}
```

Then you can develop your game using:

```shell
cargo run --features debug
```

And to release your game, you can compile using:

```shell
cargo build --release
```

## WebAssembly

### ...is supported!

When building persistent resources, you need to specify a path. Normally, this path is used to specify a location in the filesystem, but there is no filesystem in WebAssembly. Instead, it has [local storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage) and [session storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage).

Changing the API of the library, or creating a separate API for WebAssembly would make the library complicated to use. Instead, the library uses the fact that the selection of storage can be done using a path.

- `/local/settings/key-bindings.toml` would store the persistent resource in local storage with the key `settings/key-bindings.toml`
- `/session/settings/key-bindings.toml` would store the persistent resource in session storage with the key `settings/key-bindings.toml`

It might seem complicated at first, but makes it really easy to support both Native and WebAssembly application.

```rust
use std::path::Path;

fn setup(mut commands: Commands) {
  let config_dir = dirs::config_dir()
    .map(|native_config_dir| native_config_dir.join("your-amazing-game"))
    .unwrap_or(Path::new("local").join("configuration"));

  commands.insert_resource(
    Persistent::<KeyBindings>::builder()
      .name("key bindings")
      .format(StorageFormat::Json)
      .path(config_dir.join("key-bindings.toml"))
      .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
      .build(),
  )
}
```

With the code above, you don't need to have any conditional compilation to support both Native and WebAssembly application.

- In Native applications, it'll determine the configuration directory of the platform (e.g., `~/.config`) and join the name of your game to it (e.g., `~/.config/your-amazing-game`), and use it as the base directory in the filesystem.
- In WebAssembly applications, it'll use local storage with the base key of `configuration`, once you join it with `key-binding.toml`, the resource will be stored using the key `configuration/key-bindings.toml`.

If the first element of the specified path is not `"local"` or `"session"`, the library will panic!

If you don't like this approach, and want to be strict with types, you can use the [new](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.new) method of [Persistent\<R>](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html) instead.

```rust
fn setup(mut commands: Commands) {
    use bevy_persistent::Storage;

    let name = "key bindings";
    let format = StorageFormat::Toml;
    let storage = Storage::LocalStorage { key: "key bindings".to_owned() };
    let default = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C };

    commands.insert_resource(Persistent::new(name, format, storage, default));
}
```

## Examples

There are a few examples that you can run directly and play around with in the [examples](https://github.com/umut-sahin/bevy-persistent/tree/main/examples) folder.

```shell
cargo run --release --features all --example name-of-the-example
```

To run the examples in a browser using WebAssembly, you can use [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner).

```shell
cargo run --release --features all --target wasm32-unknown-unknown --example name-of-the-example
```

## License

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) is free, open source and permissively licensed, just like [Bevy](https://bevyengine.org/)!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!
