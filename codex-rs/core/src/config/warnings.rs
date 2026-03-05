use crate::config_loader::ConfigLayerStack;
use crate::features::is_known_feature_key;
use std::collections::BTreeSet;
use toml::Value as TomlValue;

pub(crate) fn uses_deprecated_instructions_file(config_layer_stack: &ConfigLayerStack) -> bool {
    config_layer_stack
        .layers_high_to_low()
        .into_iter()
        .any(|layer| toml_uses_deprecated_instructions_file(&layer.config))
}

pub(crate) fn uses_deprecated_tools_web_search(config_layer_stack: &ConfigLayerStack) -> bool {
    config_layer_stack
        .layers_high_to_low()
        .into_iter()
        .any(|layer| toml_uses_deprecated_tools_web_search(&layer.config))
}

pub(crate) fn uses_deprecated_features_web_search(config_layer_stack: &ConfigLayerStack) -> bool {
    config_layer_stack
        .layers_high_to_low()
        .into_iter()
        .any(|layer| toml_uses_deprecated_features_web_search(&layer.config))
}

pub(crate) fn unknown_feature_keys(config_layer_stack: &ConfigLayerStack) -> Vec<String> {
    let mut keys = BTreeSet::new();
    for layer in config_layer_stack.layers_high_to_low() {
        collect_unknown_feature_keys(&layer.config, &mut keys);
    }
    keys.into_iter().collect()
}

fn toml_uses_deprecated_instructions_file(value: &TomlValue) -> bool {
    let Some(table) = value.as_table() else {
        return false;
    };
    if table.contains_key("experimental_instructions_file") {
        return true;
    }
    let Some(profiles) = table.get("profiles").and_then(TomlValue::as_table) else {
        return false;
    };
    profiles.values().any(|profile| {
        profile.as_table().is_some_and(|profile_table| {
            profile_table.contains_key("experimental_instructions_file")
        })
    })
}

fn toml_uses_deprecated_tools_web_search(value: &TomlValue) -> bool {
    let Some(table) = value.as_table() else {
        return false;
    };
    if tools_table_has_web_search(table.get("tools")) {
        return true;
    }
    let Some(profiles) = table.get("profiles").and_then(TomlValue::as_table) else {
        return false;
    };
    profiles.values().any(|profile| {
        profile
            .as_table()
            .is_some_and(|profile_table| tools_table_has_web_search(profile_table.get("tools")))
    })
}

fn toml_uses_deprecated_features_web_search(value: &TomlValue) -> bool {
    let Some(table) = value.as_table() else {
        return false;
    };
    if features_table_has_web_search(table.get("features")) {
        return true;
    }
    let Some(profiles) = table.get("profiles").and_then(TomlValue::as_table) else {
        return false;
    };
    profiles.values().any(|profile| {
        profile.as_table().is_some_and(|profile_table| {
            features_table_has_web_search(profile_table.get("features"))
        })
    })
}

fn tools_table_has_web_search(value: Option<&TomlValue>) -> bool {
    value
        .and_then(TomlValue::as_table)
        .is_some_and(|tools| tools.contains_key("web_search"))
}

fn features_table_has_web_search(value: Option<&TomlValue>) -> bool {
    value
        .and_then(TomlValue::as_table)
        .is_some_and(|features| features.contains_key("web_search"))
}

fn collect_unknown_feature_keys(value: &TomlValue, keys: &mut BTreeSet<String>) {
    let Some(table) = value.as_table() else {
        return;
    };
    if let Some(features) = table.get("features").and_then(TomlValue::as_table) {
        for key in features.keys() {
            if key == "web_search" {
                continue;
            }
            if !is_known_feature_key(key) {
                keys.insert(key.to_string());
            }
        }
    }
    let Some(profiles) = table.get("profiles").and_then(TomlValue::as_table) else {
        return;
    };
    for profile in profiles.values() {
        if let Some(profile_table) = profile.as_table() {
            if let Some(features) = profile_table.get("features").and_then(TomlValue::as_table) {
                for key in features.keys() {
                    if key == "web_search" {
                        continue;
                    }
                    if !is_known_feature_key(key) {
                        keys.insert(key.to_string());
                    }
                }
            }
        }
    }
}
