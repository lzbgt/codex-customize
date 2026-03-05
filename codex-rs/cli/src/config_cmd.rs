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
    Layers(ConfigLayersArgs),

    /// Show deprecated keys and unknown feature warnings.
    Warnings(ConfigWarningsArgs),
}

#[derive(Debug, Parser)]
pub struct ConfigLayersArgs {
    /// Emit the layer summary as JSON.
    #[arg(long = "json")]
    pub json: bool,
}

#[derive(Debug, Parser)]
pub struct ConfigWarningsArgs {
    /// Emit warnings as JSON.
    #[arg(long = "json")]
    pub json: bool,
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
            ConfigSubcommand::Layers(args) => print_layers(&config, args.json),
            ConfigSubcommand::Warnings(args) => {
                print_warnings(&config, args.json);
            }
        }

        Ok(())
    }
}

fn print_layers(config: &Config, as_json: bool) {
    if as_json {
        print_layers_json(config);
        return;
    }

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

fn print_warnings(config: &Config, as_json: bool) {
    if as_json {
        print_warnings_json(config);
        return;
    }

    let instructions_sources = deprecated_instructions_file_sources(&config.config_layer_stack);
    let tools_sources = deprecated_tools_web_search_sources(&config.config_layer_stack);
    let features_sources = deprecated_features_web_search_sources(&config.config_layer_stack);
    let unknown_features = unknown_feature_keys(&config.config_layer_stack);

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

fn print_layers_json(config: &Config) {
    let layers = config
        .config_layer_stack
        .get_layers(ConfigLayerStackOrdering::HighestPrecedenceFirst, true);
    let instructions_sources = deprecated_instructions_file_sources(&config.config_layer_stack);
    let tools_sources = deprecated_tools_web_search_sources(&config.config_layer_stack);
    let features_sources = deprecated_features_web_search_sources(&config.config_layer_stack);

    let layers_json = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| {
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
            serde_json::json!({
                "precedence": index,
                "source": describe_layer_source(&layer.name),
                "version": layer.version,
                "enabled": !layer.is_disabled(),
                "disabled_reason": layer.disabled_reason,
                "deprecated_keys": deprecated,
            })
        })
        .collect::<Vec<_>>();

    let payload = serde_json::json!({
        "profile": config.active_profile.clone(),
        "cwd": config.cwd,
        "layer_count": layers_json.len(),
        "layers": layers_json,
    });
    let output = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string());
    safe_println!("{output}");
}

fn print_warnings_json(config: &Config) {
    let instructions_sources = deprecated_instructions_file_sources(&config.config_layer_stack);
    let tools_sources = deprecated_tools_web_search_sources(&config.config_layer_stack);
    let features_sources = deprecated_features_web_search_sources(&config.config_layer_stack);
    let unknown_features = unknown_feature_keys(&config.config_layer_stack);
    let has_warnings = !instructions_sources.is_empty()
        || !tools_sources.is_empty()
        || !features_sources.is_empty()
        || !unknown_features.is_empty();

    let payload = serde_json::json!({
        "profile": config.active_profile.clone(),
        "cwd": config.cwd,
        "has_warnings": has_warnings,
        "deprecated": {
            "experimental_instructions_file": format_sources_json(&instructions_sources),
            "tools.web_search": format_sources_json(&tools_sources),
            "features.web_search": format_sources_json(&features_sources),
        },
        "unknown_features": unknown_features,
        "counts": {
            "experimental_instructions_file": instructions_sources.len(),
            "tools.web_search": tools_sources.len(),
            "features.web_search": features_sources.len(),
            "unknown_features": unknown_features.len(),
        }
    });
    let output = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string());
    safe_println!("{output}");
}

fn format_sources_json(sources: &[codex_app_server_protocol::ConfigLayerSource]) -> Vec<String> {
    sources.iter().map(describe_layer_source).collect()
}
