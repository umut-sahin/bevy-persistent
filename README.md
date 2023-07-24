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
            .build()
            .expect("failed to initialize key bindings")
    )
}
```

If it's the first run, the resource will have the specified default value and that default value will be saved to the specified path in the specified format. Otherwise, key bindings will be loaded from the specified path using the specified format.

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
        key_bindings.crouch = KeyCode::ControlLeft;
    }).expect("failed to update key bindings");
}
```

[Persistent\<R>](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html) has [set](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.set) and [update](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.update) methods to modify the underlying resource. Both of those methods write the updated resource to the disk before returning.

If any failures happen at any point (e.g., no permission to read/write to the specified path), the error will be returned, but the underlying object would be updated, and new value would be visible for the rest of the game. However, it won't persist to the next game session!

## Manual Persistence

Some resources are updated frequently and persisting on each small update might not be desirable. Or persistence could have to be triggered manually (e.g., auto saves on certain points in the game).

For such cases, you can avoid using [set](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.set) and [update](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.update) methods and update the resource directly.

```rust
fn modify_key_bindings(mut key_bindings: ResMut<Persistent<KeyBindings>>) {
    key_bindings.crouch = KeyCode::ControlLeft;
}
```

When you want the resource to persist with its current value, you can use [persist](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.persist) method.

```rust
fn persist_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    key_bindings.persist().expect("failed to save new key bindings");
}
```

### Reverting

It might make sense for some persistent resources to be reverted to default. Imagine having a settings page, it's a good idea to put revert to default button to this page because if players mess up, it'd be much easier if they can revert everything to its default state compared to manually adjusting keys.

Such persistent resources need to be created revertibly.

```rust
fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    commands.insert_resource(
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path(config_dir.join("key-bindings.toml"))
            .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
            .revertible(true)
            //^^^^^^^^^^^^^^^ using this
            .build()
            .expect("failed to initialize key bindings")
    )
}
```

Revertible persistent resources can use [revert_to_default](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.revert_to_default) method.

```rust
fn revert_key_bindings_to_default(key_bindings: Res<Persistent<KeyBindings>>) {
    key_bindings.revert_to_default().expect("failed to revert key bindings to default");
}
```

### Unloading/Reloading

Persistent resources are kept in memory by default. This might lead to unnecessarily high memory usage. To overcome this, you can use [unload](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.unload) method.

```rust
fn unload_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    key_bindings.unload().expect("failed to persist key bindings before unloading");
}
```

Once this system is run, latest version of the underlying resource will be written to the underlying storage, and underlying resource will be removed from the memory.

If you don't want to persist the latest version of the resource before unloading, or if you know the latest version is already persisted, can use [unload_without_persisting](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.unload_without_persisting) method.

```rust
fn unload_key_bindings_without_persisting(key_bindings: Res<Persistent<KeyBindings>>) {
    key_bindings.unload_without_persisting();
}
```

If immediate loading to memory upon creation is not desired, the persistent resource can be created in the unloaded state.

```rust
fn setup(mut commands: Commands) {
    let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    commands.insert_resource(
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path(config_dir.join("key-bindings.toml"))
            .default(KeyBindings { jump: KeyCode::Space, crouch: KeyCode::C })
            .loaded(false)
            //^^^^^^^^^^^^ using this or ".unloaded(true)"
            .build()
            .expect("failed to initialize key bindings")
    )
}
```

When the persistent resource is unloaded, the underlying resource will not be accessible until it's loaded back to memory using [reload](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.reload) method.

```rust
fn reload_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    key_bindings.reload().expect("failed to reload key bindings");
}
```

Reload can also be used on already loaded persistent resources to synchronize with the external changes. This can be very useful in certain situations, such as a save editor. The save editor would update the save files, and calling reload would bring those updates to the game without restarting the game!

#### You should be careful when using unloaded persistent resources!

```rust
// all of these will panic
fn incorrect_usage_of_unloaded_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    // can't access the fields since it's not in memory
    println!("{}", key_bindings.crouch);
    key_bindings.crouch = KeyCode::ControlLeft;

    // can't get the underlying resource since it's not in memory
    key_bindings.get();
    key_bindings.get_mut();

    // can't update, persist, or unload the underlying resource since it's not in memory
    key_bindings.update(...);
    key_bindings.persist();
    key_bindings.unload(); // this will call persist before unloading so it'll panic as well
}

// none of these will panic
fn correct_usage_of_unloaded_key_bindings(key_bindings: Res<Persistent<KeyBindings>>) {
    // can get properties of the persistent resource
    key_bindings.name();
    key_bindings.format();
    key_bindings.storage();
    key_bindings.is_loaded();
    key_bindings.is_unloaded();

    // can try to get the underlying resource
    key_bindings.try_get();
    key_bindings.try_get_mut();

    // can set the resource to a new value
    key_bindings.set(...);

    // can unload without persisting as it's a no-op
    key_bindings.unload_without_persisting(...);

    // can reload the persistent resource
    key_bindings.reload(...);

    // can revert the persistent resource to its default
    key_bindings.revert_to_default(...);
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
            .build()
            .expect("failed to initialize key bindings")
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
            .build()
            .expect("failed to initialize key bindings")
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
    let revertible = false;
    let loaded = true;

    commands.insert_resource(
        Persistent::new(name, format, storage, default, revertible, loaded)
            .expect("failed to initialize key bindings")
    );
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

## Relation to

### bevy_pkv

[bevy_pkv](https://github.com/johanhelsing/bevy_pkv) is a generic key value store for [Bevy](https://bevyengine.org/). It is an excellent library, and it can be used as an alternative to [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/). Here are some of the differences between the two libraries!

- **bevy_pkv is a lot more flexible**

Let's say you want to have two different `Settings` for two different users.

For the time being, this is not straightforward to do with [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) because `Persistent<R>` is a resource, there can only be a single instance of a resource. So you can't have two `Persistent<Settings>` in the same app. You can work around this by defining a custom struct (e.g., `Persistent<AllSettings>`), using a tuple (e.g., `Persistent<(Settings, Settings)>`), using a vector (e.g., `Persistent<Vec<Settings>>`), or using a hashmap (e.g., `Persistent<HashMap<Settings>>`).

This is very easy to do with [bevy_pkv](https://github.com/johanhelsing/bevy_pkv)!

```rust
fn setup(mut pkv: ResMut<PkvStore>) {
    // ...
    let blue_settings: Settings = ...;
    let red_settings: Settings = ...;
    // ...
    pkv.set("blue-settings", &blue_settings).unwrap();
    pkv.set("red-settings", &red_settings).unwrap();
    // ...
}

fn utilize_settings(pkv: Res<PkvStore>) {
    // ...
    let blue_settings: Settings = pkv.get("blue-settings").unwrap();
    let red_settings: Settings = pkv.get("red-settings").unwrap();
    // ...
}
```

Maybe [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) can provide a solution to this problem at some point if it's requested by the community! I really like the idea of providing `PersistentVec<R>` and `PersistentMap<K, R> where K: impl AsRef<str>`.

```rust
fn setup(mut settings: ResMut<PersistentMap<&'static str, Settings>>) {
    // ...
    let blue_settings: Settings = ...;
    let red_settings: Settings = ...;
    // ...
    settings.insert("blue", blue_settings)?;
    settings.insert("red", red_settings)?;
    // ...
}

fn utilize_settings(settings: Res<PersistentMap<&'static str, Settings>>) {
    // ...
    let blue_settings: &Settings = settings["blue"];
    let red_settings: &Settings = settings["red"];
    // ...
}
```

- **bevy_pkv is using extremely optimized key-value storage engines (in native apps)**

As far as I can see, the current version of [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) has 2 storage options, [sled](https://sled.rs/) and [RocksDB](https://rocksdb.org/), which are extremely well optimized. This means if the persistent objects are updated frequently, [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) performance will be outstanding!

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) on the other hand stores each persistent object as a separate file, which means persisting objects trigger direct disk writes! This is okay for most use cases (e.g., settings or manual saves), but if your application requires very frequent updates, [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) is the way to go!

Mandatory note on this, please please please profile before making decisions for performance reasons!

- **bevy_pkv parses the objects on each read**

Writes being extremely optimized, reads can be slow in [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) because reading a persistent object requires parsing, on each read!

In [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/), reads are basically free as `Persistent<R>` is just a wrapper around the actual object. That is unless you unload them using [unload](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.unload) or [unload_without_persisting](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.unload_without_persisting) methods to save memory. When you load them back using [reload](https://docs.rs/bevy-persistent/latest/bevy_persistent/persistent/struct.Persistent.html#method.reload) method, parsing will happen once and reads will be free again.

Again, always profile before making decisions for performance reasons!

- **bevy_pkv doesn't do automatic management**

With [bevy_pkv](https://github.com/johanhelsing/bevy_pkv), modifying the object and making the changes persistent is always two different steps.

```rust
fn modify_key_bindings(mut pkv: ResMut<PkvStore>) {
    let mut key_bindings = pkv.get::<KeyBindings>("key-bindings");
    key_bindings.crouch = KeyCode::ControlLeft;
    pkv.set("key-bindings", &key_bindings)
}
```

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) on the other hand provides APIs to automate this process (see [Modification](#modification)).

- **bevy_pkv makes external edits very hard (in native apps)**

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) stores each persistent object as a separate file in the specified format. If a textual format is used, the object becomes extremely easy to edit! The example in [Creation](#creation) will create `key-bindings.toml` in the specified location with the following contents:

```toml
jump = "Space"
crouch = "C"
```

You can easily change this file to:

```toml
jump = "Space"
crouch = "ControlLeft"
```

And it'd work perfectly in the next run!

[bevy_pkv](https://github.com/johanhelsing/bevy_pkv) on the other hand stores the objects in a single database in a binary format, which is more efficient but not suitable external modification.

- **bevy_pkv makes storing objects in different locations a bit inconvenient (in native apps)**

This one is kinda like the one above, but it's more about how things are structured to satisfy your needs! For example, you may want to synchronize user saves through [Steam](https://store.steampowered.com/) but not user settings as they need to be different across machines.

This scenario is doable with [bevy_pkv](https://github.com/johanhelsing/bevy_pkv), but it requires creating new types that wrap `PkvStore` within them and using those instead of a single `PkvStore`, which is a little inconvenient. It's very easy with [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) however as you can structure your persistent objects however you see fit in the filesystem using a single type!

- **bevy_pkv only supports JSON and Binary storage formats**

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) supports a wide variety of storage formats! If you're porting a game from another engine to [Bevy](https://bevyengine.org/) you might be able to just define the structure of some objects (e.g., settings, saves) and let [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) handle the rest!

With [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) however, you need to create a transformation layer as it's not directly compatible with any of the widely used formats.

- **bevy_pkv is not type safe**

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) integrates with type system of [Bevy](https://bevyengine.org/). Types of persistent resources are specified in system definitions and everything is handled by [Bevy](https://bevyengine.org/).

[bevy_pkv](https://github.com/johanhelsing/bevy_pkv) on the other hand provides a single resource which you can use to query a persistent key value database with any type in the runtime. This is very flexible, but you need to specify types on each access, so it's error-prone. Also, you can't see what the system is doing easily without looking to the function body.

- **bevy_pkv can obstruct parallelism**

Each persistent resource in [bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) is a separate resource. Which means systems can be scheduled to access/modify different persistent resources at the same time.

This cannot be the case with [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) as it has a single resource type (`pkv: Res<PkvStore>` or `mut pkv: ResMut<PkvStore>`) for all systems, which prevents concurrent reads/writes on different persistent objects.

## License

[bevy-persistent](https://github.com/umut-sahin/bevy-persistent/) is free, open source and permissively licensed, just like [Bevy](https://bevyengine.org/)!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/bevy-persistent/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!
