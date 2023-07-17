mod common;
use common::*;

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;

    #[test]
    #[cfg(feature = "toml")]
    fn create_non_existing() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let default = KeyBindings::default();

        assert!(!path.exists());

        let resource = Persistent::new(name, format, storage, default)?;

        assert!(path.exists());

        let expected_resource = KeyBindings::default();
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = toml::to_string(&expected_resource)?;
        let actual_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[test]
    #[cfg(feature = "toml")]
    fn create_existing() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let default = KeyBindings::default();

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        let existing_content = toml::to_string(&existing_resource)?;

        std::fs::write(&path, &existing_content)?;
        assert!(path.exists());

        let resource = Persistent::new(name, format, storage, default)?;

        let expected_resource = existing_resource;
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = existing_content;
        let actual_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[test]
    #[cfg(feature = "toml")]
    fn set() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let default = KeyBindings::default();

        assert!(!path.exists());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        resource.set(new_resource.clone())?;

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[test]
    #[cfg(feature = "toml")]
    fn update() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let default = KeyBindings::default();

        assert!(!path.exists());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        resource.update(updater)?;

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[test]
    #[cfg(feature = "toml")]
    fn persist() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let default = KeyBindings::default();

        assert!(!path.exists());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        updater(resource.get_mut());

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = expected_initial_content;
        let actual_new_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        resource.persist()?;

        let expected_final_content = toml::to_string(&expected_new_resource)?;
        let actual_final_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_final_content.trim(), actual_final_content.trim());

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
    #[cfg(feature = "toml")]
    fn create_non_existing_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let resource = Persistent::new(name, format, storage, default)?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_resource = KeyBindings::default();
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = toml::to_string(&expected_resource)?;
        let actual_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn create_existing_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        let existing_content = toml::to_string(&existing_resource)?;

        LocalStorage::set(key, existing_content.as_str())?;
        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let resource = Persistent::new(name, format, storage, default)?;

        let expected_resource = existing_resource;
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = existing_content;
        let actual_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn set_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        resource.set(new_resource.clone())?;

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn update_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        resource.update(updater)?;

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn persist_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        updater(resource.get_mut());

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = expected_initial_content;
        let actual_new_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        resource.persist()?;

        let expected_final_content = toml::to_string(&expected_new_resource)?;
        let actual_final_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_final_content.trim(), actual_final_content.trim());

        Ok(())
    }


    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn create_non_existing_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let resource = Persistent::new(name, format, storage, default)?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_resource = KeyBindings::default();
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = toml::to_string(&expected_resource)?;
        let actual_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn create_existing_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        let existing_content = toml::to_string(&existing_resource)?;

        SessionStorage::set(key, existing_content.as_str())?;
        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let resource = Persistent::new(name, format, storage, default)?;

        let expected_resource = existing_resource;
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = existing_content;
        let actual_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn set_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::LControl };
        resource.set(new_resource.clone())?;

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn update_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        resource.update(updater)?;

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = toml::to_string(&expected_new_resource)?;
        let actual_new_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn persist_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let default = KeyBindings::default();

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(name, format, storage, default)?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::LControl;
        }
        updater(resource.get_mut());

        let mut new_resource = expected_initial_resource;
        updater(&mut new_resource);

        let expected_new_resource = new_resource;
        let actual_new_resource = resource.get();

        assert_eq!(actual_new_resource, &expected_new_resource);

        let expected_new_content = expected_initial_content;
        let actual_new_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_new_content.trim(), actual_new_content.trim());

        resource.persist()?;

        let expected_final_content = toml::to_string(&expected_new_resource)?;
        let actual_final_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_final_content.trim(), actual_final_content.trim());

        Ok(())
    }


    wasm_bindgen_test_configure!(run_in_browser);
}
