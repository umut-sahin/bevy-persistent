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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(!path.exists());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
        let existing_content = toml::to_string(&existing_resource)?;

        std::fs::write(&path, &existing_content)?;
        assert!(path.exists());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(!path.exists());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(!path.exists());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(!path.exists());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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

    #[test]
    #[cfg(feature = "toml")]
    fn unload_reload() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(!path.exists());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(path.exists());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

        resource.unload_without_persisting();

        assert!(!resource.is_loaded());
        assert!(resource.is_unloaded());

        assert!(resource.try_get().is_none());
        assert!(resource.try_get_mut().is_none());

        let mut new_resource = expected_initial_resource;
        new_resource.crouch = KeyCode::ControlLeft;

        std::fs::write(&path, format.serialize(name, &new_resource)?)?;
        resource.reload()?;

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

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
    fn revert_on_error() -> anyhow::Result<()> {
        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("key-bindings.toml");

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::Filesystem { path: path.clone() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = true;
        let revert_to_default_on_deserialization_errors = true;

        std::fs::write(&path, "invalid keybindings")?;

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(path.exists());

        let expected_resource = KeyBindings::default();
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = toml::to_string(&expected_resource)?;
        let actual_content = std::fs::read_to_string(&path)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }
}

#[cfg(target_family = "wasm")]
mod wasm {
    use super::*;
    use gloo_storage::{LocalStorage, SessionStorage, Storage as _};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn create_non_existing_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
        let existing_content = toml::to_string(&existing_resource)?;

        LocalStorage::set(key, existing_content.as_str())?;
        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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
    fn unload_reload_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(LocalStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = LocalStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

        resource.unload_without_persisting();

        assert!(!resource.is_loaded());
        assert!(resource.is_unloaded());

        assert!(resource.try_get().is_none());
        assert!(resource.try_get_mut().is_none());

        let mut new_resource = expected_initial_resource;
        new_resource.crouch = KeyCode::ControlLeft;

        LocalStorage::set::<&str>(
            key,
            std::str::from_utf8(&format.serialize(name, &new_resource)?)?,
        )?;
        resource.reload()?;

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

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
    fn revert_on_error_local_storage() -> anyhow::Result<()> {
        LocalStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::LocalStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = true;
        let revert_to_default_on_deserialization_errors = true;

        LocalStorage::raw().set_item(key, "invalid keybindings").unwrap();

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
    fn create_non_existing_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        let existing_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
        let existing_content = toml::to_string(&existing_resource)?;

        SessionStorage::set(key, existing_content.as_str())?;
        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        let new_resource = KeyBindings { jump: KeyCode::Space, crouch: KeyCode::ControlLeft };
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        fn updater(key_bindings: &mut KeyBindings) {
            key_bindings.crouch = KeyCode::ControlLeft;
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

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn reload_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = false;
        let revert_to_default_on_deserialization_errors = false;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_none());

        let mut resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_initial_resource = KeyBindings::default();
        let actual_initial_resource = resource.get();

        assert_eq!(actual_initial_resource, &expected_initial_resource);

        let expected_initial_content = toml::to_string(&expected_initial_resource)?;
        let actual_initial_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_initial_content.trim(), actual_initial_content.trim());

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

        resource.unload_without_persisting();

        assert!(!resource.is_loaded());
        assert!(resource.is_unloaded());

        assert!(resource.try_get().is_none());
        assert!(resource.try_get_mut().is_none());

        let mut new_resource = expected_initial_resource;
        new_resource.crouch = KeyCode::ControlLeft;

        SessionStorage::set::<&str>(
            key,
            std::str::from_utf8(&format.serialize(name, &new_resource)?)?,
        )?;
        resource.reload()?;

        assert!(resource.is_loaded());
        assert!(!resource.is_unloaded());

        assert!(resource.try_get().is_some());
        assert!(resource.try_get_mut().is_some());

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
    fn revert_on_error_session_storage() -> anyhow::Result<()> {
        SessionStorage::clear();

        let key = "key-bindings.toml";

        let name = "key bindings";
        let format = StorageFormat::Toml;
        let storage = Storage::SessionStorage { key: key.to_owned() };
        let loaded = true;
        let default = KeyBindings::default();
        let revertible = true;
        let revert_to_default_on_deserialization_errors = true;

        SessionStorage::raw().set_item(key, "invalid keybindings").unwrap();

        let resource = Persistent::new(
            name,
            format,
            storage,
            loaded,
            default,
            revertible,
            revert_to_default_on_deserialization_errors,
        )?;

        assert!(SessionStorage::raw().get_item(key).unwrap().is_some());

        let expected_resource = KeyBindings::default();
        let actual_resource = resource.get();

        assert_eq!(actual_resource, &expected_resource);

        let expected_content = toml::to_string(&expected_resource)?;
        let actual_content = SessionStorage::get::<String>(key)?;

        assert_eq!(expected_content.trim(), actual_content.trim());

        Ok(())
    }

    wasm_bindgen_test_configure!(run_in_browser);
}
