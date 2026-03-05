use std::path::Path;
use std::path::PathBuf;

pub fn codex_bin() -> anyhow::Result<PathBuf> {
    match codex_utils_cargo_bin::cargo_bin("codex") {
        Ok(path) => Ok(path),
        Err(err) => {
            if should_attempt_build() {
                build_codex_cli()?;
                return Ok(codex_utils_cargo_bin::cargo_bin("codex")?);
            }
            Err(err.into())
        }
    }
}

fn should_attempt_build() -> bool {
    std::env::var_os("RUNFILES_DIR").is_none()
}

fn build_codex_cli() -> anyhow::Result<()> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("missing workspace root"))?;
    let status = std::process::Command::new(cargo)
        .current_dir(workspace_root)
        .args(["build", "-p", "codex-cli", "--bin", "codex"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("failed to build codex binary for tests")
    }
}
