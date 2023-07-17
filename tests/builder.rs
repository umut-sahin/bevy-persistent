mod common;
use common::*;

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;

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
            .build()?;

        assert!(path.exists());

        assert_eq!(resource.name(), name);
        assert_eq!(resource.format(), format);
        assert_eq!(resource.storage(), &Storage::Filesystem { path });
        assert_eq!(resource.get(), &default);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "persistent resource name is not set")]
    fn test_builder_no_name() {
        Persistent::<KeyBindings>::builder().build().ok();
    }

    #[test]
    #[should_panic(expected = "persistent resource format is not set")]
    fn test_builder_no_format() {
        Persistent::<KeyBindings>::builder().name("key bindings").build().ok();
    }

    #[test]
    #[should_panic(expected = "persistent resource path is not set")]
    #[cfg(feature = "toml")]
    fn test_builder_no_path() {
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .build()
            .ok();
    }

    #[test]
    #[should_panic(expected = "persistent resource default is not set")]
    #[cfg(feature = "toml")]
    fn test_builder_no_default() {
        Persistent::<KeyBindings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path("")
            .build()
            .ok();
    }
}


#[cfg(target_family = "wasm")]
mod wasm {
    use super::*;
    use gloo_storage::{
        LocalStorage,
        SessionStorage,
        Storage as _,
    };
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn test_builder_build_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let path = PathBuf::from("local").join("key-bindings.toml");
        let default = KeyBindings::default();

        assert!(LocalStorage::raw().get_item("key-bindings.toml").unwrap().is_none());

        let resource = Persistent::<KeyBindings>::builder()
            .name(name)
            .format(format)
            .path(path)
            .default(default.clone())
            .build()?;

        assert!(LocalStorage::raw().get_item("key-bindings.toml").unwrap().is_some());

        assert_eq!(resource.name(), name);
        assert_eq!(resource.format(), format);
        assert_eq!(
            resource.storage(),
            &Storage::LocalStorage { key: "key-bindings.toml".to_owned() },
        );
        assert_eq!(resource.get(), &default);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn test_builder_build_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let path = PathBuf::from("session").join("key-bindings.toml");
        let default = KeyBindings::default();

        assert!(SessionStorage::raw().get_item("key-bindings.toml").unwrap().is_none());

        let resource = Persistent::<KeyBindings>::builder()
            .name(name)
            .format(format)
            .path(path)
            .default(default.clone())
            .build()?;

        assert!(SessionStorage::raw().get_item("key-bindings.toml").unwrap().is_some());

        assert_eq!(resource.name(), name);
        assert_eq!(resource.format(), format);
        assert_eq!(
            resource.storage(),
            &Storage::SessionStorage { key: "key-bindings.toml".to_owned() },
        );
        assert_eq!(resource.get(), &default);

        Ok(())
    }

    wasm_bindgen_test_configure!(run_in_browser);
}
