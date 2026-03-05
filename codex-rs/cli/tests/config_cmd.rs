use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::Value as JsonValue;
use tempfile::TempDir;

mod common;

fn codex_command(codex_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(common::codex_bin()?);
    cmd.env("CODEX_HOME", codex_home);
    Ok(cmd)
}

fn write_deprecated_config(codex_home: &Path) -> Result<()> {
    let config = r#"
model = "gpt-5"
experimental_instructions_file = "instructions.md"

[tools]
web_search = true

[features]
web_search = true
mystery_flag = true
"#;
    fs::create_dir_all(codex_home)?;
    fs::write(codex_home.join("config.toml"), config)?;
    Ok(())
}

#[test]
fn warnings_surface_deprecated_and_unknown_features() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_deprecated_config(codex_home.path())?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd.args(["config", "warnings"]).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Config warnings:"));
    assert!(stdout.contains("Profile: (default)"));
    assert!(stdout.contains("Deprecated: experimental_instructions_file"));
    assert!(stdout.contains("Deprecated: tools.web_search"));
    assert!(stdout.contains("Deprecated: features.web_search"));
    assert!(stdout.contains("Unknown [features] keys ignored: mystery_flag"));

    Ok(())
}

#[test]
fn layers_include_deprecated_keys_per_layer() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_deprecated_config(codex_home.path())?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd.args(["config", "layers"]).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let expected_path = codex_home.path().join("config.toml");
    let expected_source = format!("user:{}", expected_path.display());
    assert!(stdout.contains("Config layers"));
    assert!(stdout.contains(&expected_source));
    assert!(stdout.contains(
        "deprecated=experimental_instructions_file, tools.web_search, features.web_search"
    ));

    Ok(())
}

#[test]
fn warnings_json_reports_sources() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_deprecated_config(codex_home.path())?;

    let mut cmd = codex_command(codex_home.path())?;
    let cwd = codex_home.path().join("cwd");
    std::fs::create_dir_all(&cwd)?;
    let output = cmd
        .args([
            "config",
            "warnings",
            "--json",
            "--cwd",
            cwd.to_string_lossy().as_ref(),
        ])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    let expected_path = codex_home.path().join("config.toml");
    let expected_source = format!("user:{}", expected_path.display());

    let deprecated = parsed
        .get("deprecated")
        .and_then(JsonValue::as_object)
        .expect("deprecated object");
    assert_eq!(parsed.get("profile").and_then(JsonValue::as_str), None);
    assert_eq!(
        parsed.get("cwd").and_then(JsonValue::as_str),
        Some(cwd.to_string_lossy().as_ref())
    );
    assert_eq!(
        parsed.get("has_warnings").and_then(JsonValue::as_bool),
        Some(true)
    );
    assert_eq!(
        parsed.get("deprecated_count").and_then(JsonValue::as_u64),
        Some(3)
    );
    assert_eq!(
        parsed.get("warnings_count").and_then(JsonValue::as_u64),
        Some(4)
    );
    let instructions_sources = deprecated
        .get("experimental_instructions_file")
        .and_then(JsonValue::as_array)
        .expect("instructions sources");
    assert!(
        instructions_sources
            .iter()
            .any(|val| val.as_str() == Some(expected_source.as_str()))
    );

    let tools_sources = deprecated
        .get("tools.web_search")
        .and_then(JsonValue::as_array)
        .expect("tools sources");
    assert!(
        tools_sources
            .iter()
            .any(|val| val.as_str() == Some(expected_source.as_str()))
    );

    let features_sources = deprecated
        .get("features.web_search")
        .and_then(JsonValue::as_array)
        .expect("features sources");
    assert!(
        features_sources
            .iter()
            .any(|val| val.as_str() == Some(expected_source.as_str()))
    );

    let unknown = parsed
        .get("unknown_features")
        .and_then(JsonValue::as_array)
        .expect("unknown features");
    assert!(
        unknown
            .iter()
            .any(|val| val.as_str() == Some("mystery_flag"))
    );

    let counts = parsed
        .get("counts")
        .and_then(JsonValue::as_object)
        .expect("counts");
    assert_eq!(
        counts
            .get("experimental_instructions_file")
            .and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(
        counts.get("tools.web_search").and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(
        counts
            .get("features.web_search")
            .and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(
        counts.get("unknown_features").and_then(JsonValue::as_u64),
        Some(1)
    );

    Ok(())
}

#[test]
fn layers_json_reports_deprecated_keys() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_deprecated_config(codex_home.path())?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd.args(["config", "layers", "--json"]).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    let expected_path = codex_home.path().join("config.toml");
    let expected_source = format!("user:{}", expected_path.display());

    let layers = parsed
        .get("layers")
        .and_then(JsonValue::as_array)
        .expect("layers array");
    let layer = layers
        .iter()
        .find(|entry| entry.get("source").and_then(JsonValue::as_str) == Some(&expected_source))
        .expect("user layer");
    assert_eq!(layer.get("precedence").and_then(JsonValue::as_u64), Some(0));
    assert_eq!(
        layer.get("source_kind").and_then(JsonValue::as_str),
        Some("user")
    );
    assert_eq!(
        layer.get("source_path").and_then(JsonValue::as_str),
        Some(expected_path.to_string_lossy().as_ref())
    );
    assert_eq!(
        layer.get("enabled").and_then(JsonValue::as_bool),
        Some(true)
    );
    let deprecated = layer
        .get("deprecated_keys")
        .and_then(JsonValue::as_array)
        .expect("deprecated keys");
    let deprecated_values: Vec<_> = deprecated.iter().filter_map(JsonValue::as_str).collect();
    assert!(deprecated_values.contains(&"experimental_instructions_file"));
    assert!(deprecated_values.contains(&"tools.web_search"));
    assert!(deprecated_values.contains(&"features.web_search"));
    assert_eq!(layer.get("source_domain").and_then(JsonValue::as_str), None);
    assert_eq!(layer.get("source_key").and_then(JsonValue::as_str), None);

    Ok(())
}

#[test]
fn layers_json_reports_context() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let cwd = codex_home.path().join("cwd");
    std::fs::create_dir_all(&cwd)?;
    let output = cmd
        .args([
            "config",
            "layers",
            "--json",
            "--cwd",
            cwd.to_string_lossy().as_ref(),
        ])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(parsed.get("profile").and_then(JsonValue::as_str), None);
    assert_eq!(
        parsed.get("cwd").and_then(JsonValue::as_str),
        Some(cwd.to_string_lossy().as_ref())
    );
    assert_eq!(
        parsed.get("layer_count").and_then(JsonValue::as_u64),
        parsed
            .get("layers")
            .and_then(JsonValue::as_array)
            .map(|layers| layers.len() as u64)
    );

    Ok(())
}

#[test]
fn warnings_json_reports_no_warnings() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd.args(["config", "warnings", "--json"]).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed.get("has_warnings").and_then(JsonValue::as_bool),
        Some(false)
    );
    assert_eq!(
        parsed.get("deprecated_count").and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(
        parsed.get("warnings_count").and_then(JsonValue::as_u64),
        Some(0)
    );

    let counts = parsed
        .get("counts")
        .and_then(JsonValue::as_object)
        .expect("counts");
    assert_eq!(
        counts
            .get("experimental_instructions_file")
            .and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(
        counts.get("tools.web_search").and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(
        counts
            .get("features.web_search")
            .and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(
        counts.get("unknown_features").and_then(JsonValue::as_u64),
        Some(0)
    );

    Ok(())
}

#[test]
fn warnings_text_reports_no_warnings() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let cwd = codex_home.path().join("cwd");
    std::fs::create_dir_all(&cwd)?;
    let output = cmd
        .args([
            "config",
            "warnings",
            "--cwd",
            cwd.to_string_lossy().as_ref(),
        ])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Config warnings:"));
    assert!(stdout.contains("Profile: (default)"));
    assert!(stdout.contains(&format!("CWD: {}", cwd.display())));
    assert!(stdout.contains("No configuration warnings found."));

    Ok(())
}

#[test]
fn layers_json_reports_session_flags_precedence() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd
        .args(["-c", "model=\"gpt-5\"", "config", "layers", "--json"])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    let layers = parsed
        .get("layers")
        .and_then(JsonValue::as_array)
        .expect("layers array");
    let session_layer = layers
        .iter()
        .find(|entry| entry.get("source").and_then(JsonValue::as_str) == Some("session-flags"))
        .expect("session flags layer");
    assert_eq!(
        session_layer.get("precedence").and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(
        session_layer.get("source_kind").and_then(JsonValue::as_str),
        Some("session-flags")
    );
    assert_eq!(
        session_layer
            .get("source_domain")
            .and_then(JsonValue::as_str),
        None
    );
    assert_eq!(
        session_layer.get("source_key").and_then(JsonValue::as_str),
        None
    );

    Ok(())
}

#[test]
fn warnings_json_reports_profile_context() -> Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::create_dir_all(codex_home.path())?;
    std::fs::write(
        codex_home.path().join("config.toml"),
        r#"[profiles.demo]
model = "gpt-5"
"#,
    )?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd
        .args(["config", "warnings", "--json", "--profile", "demo"])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed.get("profile").and_then(JsonValue::as_str),
        Some("demo")
    );

    Ok(())
}

#[test]
fn warnings_json_compact_parses() -> Result<()> {
    let codex_home = TempDir::new()?;

    let mut cmd = codex_command(codex_home.path())?;
    let output = cmd
        .args(["config", "warnings", "--json", "--compact"])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    let parsed: JsonValue = serde_json::from_str(&stdout)?;
    assert_eq!(
        parsed.get("has_warnings").and_then(JsonValue::as_bool),
        Some(false)
    );

    Ok(())
}
