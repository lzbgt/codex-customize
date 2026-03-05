use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use codex_common::CliConfigOverrides;
use codex_core::config::Config;
use codex_core::config::ConfigOverrides;
use codex_core::config::deprecated_features_web_search_sources;
use codex_core::config::deprecated_instructions_file_sources;
use codex_core::config::deprecated_tools_web_search_sources;
use codex_core::config::describe_layer_source;
use codex_core::config::unknown_feature_keys;
use codex_core::config_loader::ConfigLayerStack;
use codex_core::config_loader::ConfigLayerStackOrdering;
use std::path::PathBuf;
use toml::Value as TomlValue;

#[derive(Debug, Parser)]
pub struct ConfigCli {
    #[clap(flatten)]
    pub config_overrides: CliConfigOverrides,

    /// Configuration profile from config.toml to use for diagnostics.
    #[arg(long = "profile", short = 'p')]
    pub config_profile: Option<String>,

    /// Working directory to use when resolving project config layers.
    #[arg(long = "cwd", value_name = "DIR")]
    pub cwd: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum ConfigSubcommand {
    /// Show configuration layers (highest precedence first).
    Layers,

    /// Show deprecated keys and unknown feature warnings.
    Warnings,
}

#[derive(Debug, Clone)]
pub struct ConfigContext {
    pub config_profile: Option<String>,
    pub cwd: Option<PathBuf>,
    pub web_search: bool,
}

impl ConfigCli {
    pub async fn run(self, context: ConfigContext) -> Result<()> {
        let ConfigCli {
            config_overrides,
            config_profile,
            cwd,
            subcommand,
        } = self;

        let mut cli_overrides = config_overrides
            .parse_overrides()
            .map_err(anyhow::Error::msg)?;
        if context.web_search {
            cli_overrides.push((
                "web_search".to_string(),
                TomlValue::String("live".to_string()),
            ));
        }

        let profile = config_profile.or(context.config_profile);
        let cwd = cwd.or(context.cwd);
        let overrides = ConfigOverrides {
            config_profile: profile.clone(),
            cwd,
            ..Default::default()
        };

        let config =
            Config::load_with_cli_overrides_and_harness_overrides(cli_overrides, overrides)
                .await
                .context("failed to load configuration")?;

        match subcommand {
            ConfigSubcommand::Layers => print_layers(&config),
            ConfigSubcommand::Warnings => print_warnings(&config.config_layer_stack),
        }

        Ok(())
    }
}

fn print_layers(config: &Config) {
    safe_println!("Config layers (high → low precedence):");
    if let Some(profile) = config.active_profile.as_deref() {
        safe_println!("Profile: {profile}");
    } else {
        safe_println!("Profile: (default)");
    }
    safe_println!("CWD: {}", config.cwd.display());

    let layers = config
        .config_layer_stack
        .get_layers(ConfigLayerStackOrdering::HighestPrecedenceFirst, true);
    if layers.is_empty() {
        safe_println!("No configuration layers found.");
        return;
    }

    let instructions_sources = deprecated_instructions_file_sources(&config.config_layer_stack);
    let tools_sources = deprecated_tools_web_search_sources(&config.config_layer_stack);
    let features_sources = deprecated_features_web_search_sources(&config.config_layer_stack);

    for layer in layers {
        let mut details = Vec::new();
        details.push(format!("version={}", layer.version));
        if let Some(reason) = layer.disabled_reason.as_deref() {
            details.push(format!("disabled={reason}"));
        }
        let mut deprecated = Vec::new();
        if instructions_sources.contains(&layer.name) {
            deprecated.push("experimental_instructions_file");
        }
        if tools_sources.contains(&layer.name) {
            deprecated.push("tools.web_search");
        }
        if features_sources.contains(&layer.name) {
            deprecated.push("features.web_search");
        }
        if !deprecated.is_empty() {
            details.push(format!("deprecated={}", deprecated.join(", ")));
        }

        let detail_str = details.join("; ");
        safe_println!("- {} ({detail_str})", describe_layer_source(&layer.name));
    }
}

fn print_warnings(layers: &ConfigLayerStack) {
    let instructions_sources = deprecated_instructions_file_sources(layers);
    let tools_sources = deprecated_tools_web_search_sources(layers);
    let features_sources = deprecated_features_web_search_sources(layers);
    let unknown_features = unknown_feature_keys(layers);

    let mut warned = false;

    if !instructions_sources.is_empty() {
        warned = true;
        safe_println!("Deprecated: experimental_instructions_file");
        safe_println!("  Detected in: {}", format_sources(&instructions_sources));
        safe_println!(
            "  Move the setting to model_instructions_file in config.toml (or under a profile) to load instructions from a file."
        );
    }

    if !tools_sources.is_empty() {
        warned = true;
        safe_println!("Deprecated: tools.web_search");
        safe_println!("  Detected in: {}", format_sources(&tools_sources));
        safe_println!("  Use web_search = \"live\" | \"cached\" | \"disabled\" instead.");
        safe_println!(
            "  If you only need the raw tool toggle, set [features].web_search_request = true."
        );
    }

    if !features_sources.is_empty() {
        warned = true;
        safe_println!("Deprecated: features.web_search");
        safe_println!("  Detected in: {}", format_sources(&features_sources));
        safe_println!("  Use [features].web_search_request instead.");
        safe_println!(
            "  To enable the built-in web search tool, set web_search = \"live\" | \"cached\" | \"disabled\"."
        );
    }

    if !unknown_features.is_empty() {
        warned = true;
        safe_println!(
            "Unknown [features] keys ignored: {}",
            unknown_features.join(", ")
        );
        safe_println!("  See docs/config.md for supported feature flags.");
    }

    if !warned {
        safe_println!("No configuration warnings found.");
    }
}

fn format_sources(sources: &[codex_app_server_protocol::ConfigLayerSource]) -> String {
    sources
        .iter()
        .map(describe_layer_source)
        .collect::<Vec<_>>()
        .join(", ")
}
