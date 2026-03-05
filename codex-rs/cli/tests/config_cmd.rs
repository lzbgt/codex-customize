use std::fs;
use std::path::Path;

use anyhow::Result;
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
