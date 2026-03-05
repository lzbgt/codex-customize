use clap::Parser;
use clap::ValueHint;
use codex_common::ApprovalModeCliArg;
use codex_common::CliConfigOverrides;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Optional user prompt to start the session.
    #[arg(value_name = "PROMPT", value_hint = clap::ValueHint::Other)]
    pub prompt: Option<String>,

    /// Optional image(s) to attach to the initial prompt.
    #[arg(long = "image", short = 'i', value_name = "FILE", value_delimiter = ',', num_args = 1..)]
    pub images: Vec<PathBuf>,

    // Internal controls set by the top-level `codex resume` subcommand.
    // These are not exposed as user flags on the base `codex` command.
    #[clap(skip)]
    pub resume_picker: bool,

    #[clap(skip)]
    pub resume_last: bool,

    /// Internal: resume a specific recorded session by id (UUID). Set by the
    /// top-level `codex resume <SESSION_ID>` wrapper; not exposed as a public flag.
    #[clap(skip)]
    pub resume_session_id: Option<String>,

    /// Internal: show all sessions (disables cwd filtering and shows CWD column).
    #[clap(skip)]
    pub resume_show_all: bool,

    // Internal controls set by the top-level `codex fork` subcommand.
    // These are not exposed as user flags on the base `codex` command.
    #[clap(skip)]
    pub fork_picker: bool,

    #[clap(skip)]
    pub fork_last: bool,

    /// Internal: fork a specific recorded session by id (UUID). Set by the
    /// top-level `codex fork <SESSION_ID>` wrapper; not exposed as a public flag.
    #[clap(skip)]
    pub fork_session_id: Option<String>,

    /// Internal: show all sessions (disables cwd filtering and shows CWD column).
    #[clap(skip)]
    pub fork_show_all: bool,

    /// Model the agent should use.
    #[arg(long, short = 'm')]
    pub model: Option<String>,

    /// Convenience flag to select the local open source model provider. Equivalent to -c
    /// model_provider=oss; verifies a local LM Studio or Ollama server is running.
    #[arg(long = "oss", default_value_t = false)]
    pub oss: bool,

    /// Specify which local provider to use (lmstudio, ollama, or ollama-chat).
    /// If not specified with --oss, will use config default or show selection.
    #[arg(long = "local-provider")]
    pub oss_provider: Option<String>,

    /// Configuration profile from config.toml to specify default options.
    #[arg(long = "profile", short = 'p')]
    pub config_profile: Option<String>,

    /// Select the sandbox policy to use when executing model-generated shell
    /// commands.
    #[arg(long = "sandbox", short = 's')]
    pub sandbox_mode: Option<codex_common::SandboxModeCliArg>,

    /// Configure when the model requires human approval before executing a command.
    #[arg(long = "ask-for-approval", short = 'a')]
    pub approval_policy: Option<ApprovalModeCliArg>,

    /// Convenience alias for low-friction sandboxed automatic execution (-a on-request, --sandbox workspace-write).
    #[arg(long = "full-auto", default_value_t = false)]
    pub full_auto: bool,

    /// Skip all confirmation prompts, execute commands without sandboxing,
    /// bypass managed policy constraints, disable exec-policy enforcement,
    /// force-enable unrestricted tool use (shell/unified_exec/apply_patch/view_image
    /// plus live web_search), allow full host/network access, and enable auto-continue.
    /// Also resets the shell environment policy to inherit all variables with
    /// default excludes disabled.
    /// EXTREMELY DANGEROUS. Intended solely for running in environments that
    /// are externally sandboxed.
    #[arg(
        long = "dangerously-bypass-approvals-and-sandbox",
        alias = "yolo",
        default_value_t = false,
        conflicts_with_all = ["approval_policy", "full_auto"]
    )]
    pub dangerously_bypass_approvals_and_sandbox: bool,

    /// Tell the agent to use the specified directory as its working root.
    #[clap(long = "cd", short = 'C', value_name = "DIR")]
    pub cwd: Option<PathBuf>,

    /// Enable live web search. When enabled, the native Responses `web_search` tool is available to the model (no per‑call approval).
    #[arg(long = "search", default_value_t = false)]
    pub web_search: bool,

    /// Additional directories that should be writable alongside the primary workspace.
    #[arg(long = "add-dir", value_name = "DIR", value_hint = ValueHint::DirPath)]
    pub add_dir: Vec<PathBuf>,

    /// After each completed turn, automatically continue with the next most urgent task(s)
    /// without waiting for user input (unless the agent explicitly requests stop).
    /// The follow-up "Continue" also grants approval for any required step unless declined.
    /// The follow-up prompt targets 12-18 related tasks per turn (minimum 12).
    #[arg(long = "auto-continue", default_value_t = false)]
    pub auto_continue: bool,

    /// Optional safety override: maximum number of turns to run when `--auto-continue` is enabled
    /// (including the initial turn).
    #[arg(long = "auto-continue-max-turns", value_name = "N")]
    pub auto_continue_max_turns: Option<usize>,

    /// Disable alternate screen mode
    ///
    /// Runs the TUI in inline mode, preserving terminal scrollback history. This is useful
    /// in terminal multiplexers like Zellij that follow the xterm spec strictly and disable
    /// scrollback in alternate screen buffers.
    #[arg(long = "no-alt-screen", default_value_t = false)]
    pub no_alt_screen: bool,

    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_auto_continue_flags() {
        let cli = Cli::parse_from(["codex", "--auto-continue", "--auto-continue-max-turns", "3"]);
        assert!(cli.auto_continue);
        assert_eq!(cli.auto_continue_max_turns, Some(3));
    }
}
