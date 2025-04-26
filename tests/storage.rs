mod common;
use common::*;

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;

    #[test]
    fn filesystem_initialize() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("some").join("dirs").join("key-bindings.toml");

        assert!(!tempdir.path().join("some").exists());
        assert!(!tempdir.path().join("some").join("dirs").exists());
        assert!(!tempdir.path().join("some").join("dirs").join("key-bindings.toml").exists());

        let storage = Storage::Filesystem { path };

        storage.initialize()?;

        assert!(tempdir.path().join("some").exists());
        assert!(tempdir.path().join("some").join("dirs").exists());
        assert!(!tempdir.path().join("some").join("dirs").join("key-bindings.toml").exists());

        Ok(())
    }

    #[test]
    fn filesystem_occupied() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");
        let storage = Storage::Filesystem { path: path.clone() };

        assert!(!path.exists());
        assert!(!storage.occupied());

        std::fs::write(&path, "".as_bytes()).unwrap();

        assert!(path.exists());
        assert!(storage.occupied());

        Ok(())
    }

    #[test]
    fn filesystem_display() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");
        let storage = Storage::Filesystem { path: path.clone() };

        assert_eq!(format!("{storage}"), format!("{}", path.to_str().unwrap()));

        Ok(())
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
    fn local_storage_initialize() -> anyhow::Result<()> {
        let key = "key-bindings.toml";
        let storage = Storage::LocalStorage { key: key.to_owned() };

        assert!(storage.initialize().is_ok());

        Ok(())
    }

    #[wasm_bindgen_test]
    fn local_storage_occupied() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";
        let storage = Storage::LocalStorage { key: key.to_owned() };

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());
        assert!(!storage.occupied());

        LocalStorage::set(key, "bevy-persistent".as_bytes())?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());
        assert!(storage.occupied());

        Ok(())
    }

    #[wasm_bindgen_test]
    fn local_storage_display() -> anyhow::Result<()> {
        let key = "key-bindings.toml";
        let storage = Storage::LocalStorage { key: key.to_owned() };

        let separator = std::path::MAIN_SEPARATOR;
        assert_eq!(format!("{}", storage), format!("{}local{}{}", separator, separator, key));

        Ok(())
    }


    #[wasm_bindgen_test]
    fn session_storage_initialize() -> anyhow::Result<()> {
        let key = "key-bindings.toml";
        let storage = Storage::SessionStorage { key: key.to_owned() };

        assert!(storage.initialize().is_ok());

        Ok(())
    }

    #[wasm_bindgen_test]
    fn session_storage_occupied() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";
        let storage = Storage::SessionStorage { key: key.to_owned() };

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());
        assert!(!storage.occupied());

        SessionStorage::set(key, "bevy-persistent".as_bytes())?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());
        assert!(storage.occupied());

        Ok(())
    }

    #[wasm_bindgen_test]
    fn session_storage_display() -> anyhow::Result<()> {
        let key = "key-bindings.toml";
        let storage = Storage::SessionStorage { key: key.to_owned() };

        let separator = std::path::MAIN_SEPARATOR;
        assert_eq!(format!("{}", storage), format!("{}session{}{}", separator, separator, key));

        Ok(())
    }


    wasm_bindgen_test_configure!(run_in_browser);
}
