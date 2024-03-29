mod common;
use common::*;

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;

    #[test]
    #[cfg(feature = "bincode")]
    fn test_bincode() -> anyhow::Result<()> {
        let format = StorageFormat::Bincode;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = bincode::serialize(&resource)?;

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            bincode::deserialize::<KeyBindings>(&expected_serialized_resource)?;

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "ini")]
    fn test_ini() -> anyhow::Result<()> {
        let format = StorageFormat::Ini;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_ini::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            serde_ini::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json() -> anyhow::Result<()> {
        let format = StorageFormat::Json;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_json::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_json::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "json", feature = "pretty"))]
    fn test_json_pretty() -> anyhow::Result<()> {
        let format = StorageFormat::JsonPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            serde_json::to_string_pretty(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_json::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "ron")]
    fn test_ron() -> anyhow::Result<()> {
        let format = StorageFormat::Ron;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = ron::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "ron", feature = "pretty"))]
    fn test_ron_pretty() -> anyhow::Result<()> {
        use ron::ser::PrettyConfig;
        let pretty_config = PrettyConfig::new().struct_names(false);

        let format = StorageFormat::RonPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            ron::ser::to_string_pretty(&resource, pretty_config).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "ron", feature = "pretty"))]
    fn test_ron_pretty_with_struct_names() -> anyhow::Result<()> {
        use ron::ser::PrettyConfig;
        let pretty_config = PrettyConfig::new().struct_names(true);

        let format = StorageFormat::RonPrettyWithStructNames;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            ron::ser::to_string_pretty(&resource, pretty_config).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml() -> anyhow::Result<()> {
        let format = StorageFormat::Toml;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = toml::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            toml::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "toml", feature = "pretty"))]
    fn test_toml_pretty() -> anyhow::Result<()> {
        let format = StorageFormat::TomlPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = toml::to_string_pretty(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            toml::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml() -> anyhow::Result<()> {
        let format = StorageFormat::Yaml;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_yaml::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_yaml::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }
}

#[cfg(target_family = "wasm")]
mod wasm {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[cfg(feature = "bincode")]
    fn test_bincode() -> anyhow::Result<()> {
        let format = StorageFormat::Bincode;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = bincode::serialize(&resource)?;

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            bincode::deserialize::<KeyBindings>(&expected_serialized_resource)?;

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "ini")]
    fn test_ini() -> anyhow::Result<()> {
        let format = StorageFormat::Ini;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_ini::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            serde_ini::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "json")]
    fn test_json() -> anyhow::Result<()> {
        let format = StorageFormat::Json;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_json::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_json::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(all(feature = "json", feature = "pretty"))]
    fn test_json_pretty() -> anyhow::Result<()> {
        let format = StorageFormat::JsonPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            serde_json::to_string_pretty(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_json::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "ron")]
    fn test_ron() -> anyhow::Result<()> {
        let format = StorageFormat::Ron;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = ron::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(all(feature = "ron", feature = "pretty"))]
    fn test_ron_pretty() -> anyhow::Result<()> {
        use ron::ser::PrettyConfig;
        let pretty_config = PrettyConfig::new().struct_names(false);

        let format = StorageFormat::RonPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            ron::ser::to_string_pretty(&resource, pretty_config).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(all(feature = "ron", feature = "pretty"))]
    fn test_ron_pretty_with_struct_names() -> anyhow::Result<()> {
        use ron::ser::PrettyConfig;
        let pretty_config = PrettyConfig::new().struct_names(true);

        let format = StorageFormat::RonPrettyWithStructNames;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource =
            ron::ser::to_string_pretty(&resource, pretty_config).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            ron::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "toml")]
    fn test_toml() -> anyhow::Result<()> {
        let format = StorageFormat::Toml;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = toml::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            toml::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(all(feature = "toml", feature = "pretty"))]
    fn test_toml_pretty() -> anyhow::Result<()> {
        let format = StorageFormat::TomlPretty;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = toml::to_string_pretty(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource =
            toml::from_str::<KeyBindings>(std::str::from_utf8(&expected_serialized_resource)?)
                .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "yaml")]
    fn test_yaml() -> anyhow::Result<()> {
        let format = StorageFormat::Yaml;
        let resource = KeyBindings::default();

        let actual_serialized_resource = format.serialize("key bindings", &resource).unwrap();
        let expected_serialized_resource = serde_yaml::to_string(&resource).unwrap().into_bytes();

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let expected_deserialized_resource = serde_yaml::from_str::<KeyBindings>(
            std::str::from_utf8(&expected_serialized_resource)?,
        )
        .unwrap();

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    wasm_bindgen_test_configure!(run_in_browser);
}
