mod common;
use common::*;

#[test]
#[cfg(feature = "toml")]
fn test_builder_build() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;

    let name = "key bindings";
    let format = StorageFormat::Toml;
    let path = tempdir.path().join("key-bindings.toml");
    let default = KeyBindings::default();

    assert!(!path.exists());

    let resource = Persistent::<KeyBindings>::builder()
        .name(name)
        .format(format)
        .path(&path)
        .default(default.clone())
        .build();

    assert!(path.exists());

    assert_eq!(resource.name(), name);
    assert_eq!(resource.path(), path);
    assert_eq!(resource.format(), format);
    assert_eq!(resource.get(), &default);

    Ok(())
}

#[test]
#[should_panic(expected = "persistent resource name is not set")]
fn test_builder_no_name() {
    Persistent::<KeyBindings>::builder().build();
}

#[test]
#[should_panic(expected = "persistent resource format is not set")]
fn test_builder_no_format() {
    Persistent::<KeyBindings>::builder().name("key bindings").build();
}

#[test]
#[should_panic(expected = "persistent resource path is not set")]
#[cfg(feature = "toml")]
fn test_builder_no_path() {
    Persistent::<KeyBindings>::builder().name("key bindings").format(StorageFormat::Toml).build();
}

#[test]
#[should_panic(expected = "persistent resource default is not set")]
#[cfg(feature = "toml")]
fn test_builder_no_default() {
    Persistent::<KeyBindings>::builder()
        .name("key bindings")
        .format(StorageFormat::Toml)
        .path("")
        .build();
}
