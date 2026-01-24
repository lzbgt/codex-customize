#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoModeNext {
    Continue,
    Stop,
}

/// Parse an `AUTO_MODE_NEXT=continue|stop` directive from an agent message.
///
/// Supported forms (case-insensitive, whitespace-tolerant):
/// - `AUTO_MODE_NEXT=continue`
/// - `auto_mode_next=stop`
/// - `AUTO_CONTINUE_NEXT=continue` (legacy alias)
///
/// Returns `None` when no directive is found.
pub fn parse_auto_mode_next(text: &str) -> Option<AutoModeNext> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Be forgiving: the directive may be indented or embedded in prose.
        // We only accept ASCII keys; positions in `trimmed` and `upper` match.
        let upper = trimmed.to_ascii_uppercase();
        let (_key, rest) = if let Some(pos) = upper.find("AUTO_MODE_NEXT=") {
            ("AUTO_MODE_NEXT=", &trimmed[pos + "AUTO_MODE_NEXT=".len()..])
        } else if let Some(pos) = upper.find("AUTO_CONTINUE_NEXT=") {
            (
                "AUTO_CONTINUE_NEXT=",
                &trimmed[pos + "AUTO_CONTINUE_NEXT=".len()..],
            )
        } else {
            continue;
        };

        let value = rest
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_end_matches(['#', ';', ',', '.']);

        if value.eq_ignore_ascii_case("continue") {
            return Some(AutoModeNext::Continue);
        }
        if value.eq_ignore_ascii_case("stop") {
            return Some(AutoModeNext::Stop);
        }
        return None;
    }

    None
}

/// Developer instructions injected when `--auto-continue` is enabled.
///
/// The key requirement is that *every* final response ends with an explicit
/// `AUTO_MODE_NEXT=continue|stop` marker so the frontend can deterministically
/// decide whether to auto-run the next turn.
pub const AUTO_CONTINUE_DEVELOPER_INSTRUCTIONS: &str = "\
- This project is in a rolling style. Until stable, all revisions/refactors are granted, and compatibility with former versions is not mandatory, given changes meet the rules below.\n\
- Make decisions based on making this project modern, elegant, efficient, technically solid, and future-proof.\n\
- If there is uncertainty, refer to the repository’s documentation (if any) and conduct further research to ensure updates and accuracy.\n\
- Prefer fundamental, high-leverage work over cheap tweaks: address root causes, reduce future maintenance burden, and pick the change that compounds.\n\
- Keep the implementation SOLID and maintainable: single-responsibility components, clear boundaries, minimal coupling, and testable units.\n\
- Optimize for correctness and robustness first, then performance (measure when possible).\n\
- If the previous turn surfaced important proposals/next steps, prioritize the most beneficial one before switching to unrelated work.\n\
- Use the planning tool (`update_plan`) actively: create a macro plan early, refine into micro-steps, and update statuses multiple times within the same turn when it helps execution.\n\
- Implement as many tasks as possible in this turn.\n\
- Before ending the turn:\n\
  a. If new task(s) are identified, capture them in the repo’s task tracker (if any).\n\
  b. Reweight and update tasks in the tracker, keeping it succinct.\n\
  c. If tests pass and there are code and/or documentation changes, `git diff --stat`, then commit the changes. Pushing to the remote repository is optional and not a blocker for continuation.\n\
  d. If you built large local artifacts, prune build outputs/caches (keep only the necessary deliverables).\n\
- IMPORTANT: Doing a git commit and/or push does not imply stopping. Keep going unless you explicitly output `AUTO_MODE_NEXT=stop`.\n\
- End your response with `AUTO_MODE_NEXT=continue` to request another turn, or `AUTO_MODE_NEXT=stop` to stop.\n\
";

/// Curated follow-up prompt submitted by the client when `--auto-continue` is enabled.
///
/// This is sent as a normal user prompt after each completed turn unless the agent
/// explicitly requests stop.
pub const AUTO_CONTINUE_FOLLOWUP_PROMPT: &str = "\
Continue.\n\
\n\
- Pick the highest-leverage next step based on the most recent turn (prefer fundamental fixes over ad-hoc tweaks; avoid “cheap work”).\n\
- If the previous turn proposed important next steps, do the most beneficial one first.\n\
- Use multiple plans within the turn when helpful (macro plan → micro steps), and update plan statuses as you execute.\n\
- Keep the implementation SOLID and future-proof: reduce coupling, improve boundaries, and add tests that lock in behavior.\n\
- If this repo uses a task tracker (e.g., TODO.md / issues), capture new tasks and re-prioritize succinctly.\n\
- Keep the workspace lean, but don’t delete useful caches by default: only prune build artifacts/caches if they are unusually large, clearly one-off, or the repo has an established cleanup workflow/script; otherwise keep caches that materially speed up iteration.\n\
\n\
End your final response with exactly one line:\n\
AUTO_MODE_NEXT=continue\n\
or\n\
AUTO_MODE_NEXT=stop\n\
";
