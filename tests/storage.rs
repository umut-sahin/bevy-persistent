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

        assert_eq!(format!("{}", storage), format!("{}", path.to_str().unwrap()));

        Ok(())
    }
}
