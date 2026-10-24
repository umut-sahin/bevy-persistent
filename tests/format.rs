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
        let expected_serialized_resource =
            bincode::serde::encode_to_vec(&resource, bincode::config::standard())?;

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let (expected_deserialized_resource, _) =
            bincode::serde::decode_from_slice::<KeyBindings, _>(
                &expected_serialized_resource,
                bincode::config::standard(),
            )?;

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[test]
    #[cfg(feature = "bincode")]
    fn test_bincode_v1_compatibility() {
        // Bincode 1 encodes fixed-width integers and string lengths in little-endian order.
        let legacy_bytes = [4, 3, 2, 1, 3, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];
        let resource = (0x01020304_u32, String::from("abc"));
        let format = StorageFormat::BincodeLegacy;

        assert_eq!(format.serialize("legacy save", &resource).unwrap(), legacy_bytes);
        assert_eq!(
            format.deserialize::<(u32, String)>("legacy save", &legacy_bytes).unwrap(),
            resource,
        );
        let standard_bytes = StorageFormat::Bincode.serialize("new save", &resource).unwrap();
        assert_ne!(standard_bytes, legacy_bytes);
        assert_eq!(
            StorageFormat::Bincode
                .deserialize::<(u32, String)>("new save", &standard_bytes)
                .unwrap(),
            resource,
        );
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
        let expected_serialized_resource =
            bincode::serde::encode_to_vec(&resource, bincode::config::standard())?;

        assert_eq!(actual_serialized_resource, expected_serialized_resource);

        let actual_deserialized_resource =
            format.deserialize::<KeyBindings>("key bindings", &actual_serialized_resource).unwrap();
        let (expected_deserialized_resource, _) =
            bincode::serde::decode_from_slice::<KeyBindings, _>(
                &expected_serialized_resource,
                bincode::config::standard(),
            )?;

        assert_eq!(expected_deserialized_resource, actual_deserialized_resource);

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(feature = "bincode")]
    fn test_bincode_legacy() -> anyhow::Result<()> {
        let resource = KeyBindings::default();
        let format = StorageFormat::BincodeLegacy;
        let bytes = format.serialize("key bindings", &resource)?;
        let expected = bincode::serde::encode_to_vec(&resource, bincode::config::legacy())?;

        assert_eq!(bytes, expected);
        assert_eq!(format.deserialize::<KeyBindings>("key bindings", &bytes)?, resource);

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
