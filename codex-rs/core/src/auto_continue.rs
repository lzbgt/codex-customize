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

#[cfg(test)]
mod tests {
    use super::AUTO_CONTINUE_DEVELOPER_INSTRUCTIONS;
    use super::AUTO_CONTINUE_FOLLOWUP_PROMPT;
    use super::AutoModeNext;
    use super::parse_auto_mode_next;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_continue_and_stop_variants() {
        assert_eq!(
            parse_auto_mode_next("AUTO_MODE_NEXT=continue"),
            Some(AutoModeNext::Continue)
        );
        assert_eq!(
            parse_auto_mode_next("auto_mode_next=stop"),
            Some(AutoModeNext::Stop)
        );
        assert_eq!(
            parse_auto_mode_next("AUTO_CONTINUE_NEXT=continue"),
            Some(AutoModeNext::Continue)
        );
    }

    #[test]
    fn parses_embedded_directives_with_punctuation() {
        assert_eq!(
            parse_auto_mode_next("Done. AUTO_MODE_NEXT=continue."),
            Some(AutoModeNext::Continue)
        );
        assert_eq!(
            parse_auto_mode_next("Note: AUTO_MODE_NEXT=stop;"),
            Some(AutoModeNext::Stop)
        );
    }

    #[test]
    fn ignores_unrecognized_values() {
        assert_eq!(parse_auto_mode_next("AUTO_MODE_NEXT=maybe"), None);
        assert_eq!(parse_auto_mode_next("no directive here"), None);
    }

    #[test]
    fn followup_prompt_includes_approval_note() {
        assert!(AUTO_CONTINUE_FOLLOWUP_PROMPT.starts_with("Continue."));
        assert!(
            AUTO_CONTINUE_FOLLOWUP_PROMPT.contains("grants approval"),
            "auto-continue followup should note implicit approval"
        );
    }

    #[test]
    fn followup_prompt_sets_high_throughput_targets() {
        assert!(
            AUTO_CONTINUE_FOLLOWUP_PROMPT.contains("48-72"),
            "followup prompt should enforce higher batch targets"
        );
        assert!(
            AUTO_CONTINUE_FOLLOWUP_PROMPT.contains("48+"),
            "followup prompt should set a minimum task bar"
        );
        assert!(
            AUTO_CONTINUE_FOLLOWUP_PROMPT.contains("four substantial deliverables"),
            "followup prompt should require multiple substantial deliverables"
        );
    }

    #[test]
    fn developer_instructions_set_high_throughput_targets() {
        assert!(
            AUTO_CONTINUE_DEVELOPER_INSTRUCTIONS.contains("48-72"),
            "developer instructions should align with higher batch targets"
        );
        assert!(
            AUTO_CONTINUE_DEVELOPER_INSTRUCTIONS.contains("48+"),
            "developer instructions should set a minimum task bar"
        );
        assert!(
            AUTO_CONTINUE_DEVELOPER_INSTRUCTIONS.contains("four substantial deliverables"),
            "developer instructions should require multiple substantial deliverables"
        );
    }
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
- Assume full tool access. Do not claim network/git/tool execution is blocked by policy unless a tool call explicitly returns that error; if a tool call fails, report the error and proceed.\n\
- Keep documentation and implementation in sync: if behavior, APIs, configs, or workflows change, update the relevant docs/READMEs/examples so they remain correct.\n\
- Prioritize feature-completing work and user-facing deliverables over maintenance tasks (CI, refactors, formatting, cleanup).\n\
- Maintenance work outranks features only when it unblocks feature delivery, or it mitigates P0/P1 risks (crash, data loss, security, build break), or the user explicitly requests it.\n\
- When multiple feature gaps exist, choose the highest-leverage task that unlocks or accelerates the next milestone (avoid cheap, low-impact work).\n\
- When proposing next steps, list feature deliverables first and maintenance last.\n\
- For complex changes, prefer drafting/updating a design/spec document first (even a short one): state goals, non-goals, constraints, and proposed architecture before implementing.\n\
- If the previous turn surfaced important proposals/next steps, prioritize the most beneficial one first, then keep going with other high-leverage items while context is fresh.\n\
- Use the planning tool (`update_plan`) actively: create a macro plan early, refine into micro-steps, and use multiple plans per turn when it improves throughput (finish one plan, then start the next).\n\
- When multiple tasks remain, prefer batching several into one coherent patch while context is fresh. Aim for substantial progress (often hundreds to ~1000+ lines across code + docs) rather than stopping after tiny edits. Do not inflate line count with churn—make meaningful changes.\n\
- Make each turn dense: complete a cluster of related tasks (prefer 48-72) including implementation + tests + docs + verification; avoid one-off micro fixes.\n\
- Minimum bar: deliver 48+ meaningful tasks per turn; if scope is thin, expand into adjacent tests, docs, tooling, and config hardening.\n\
- Do not end a turn after a single small fix; keep going until the bar is met or you are blocked.\n\
- Aim for at least four substantial deliverables per turn (feature or behavior changes plus tests/docs) unless blocked.\n\
- Implement as many tasks as possible in this turn, and if you finish early, immediately continue with the next highest‑leverage items instead of stopping.\n\
- If you need explicit user approval for a required step (e.g., a full test suite), ask once, then continue with other tasks; do not stall or repeatedly ask in a loop.\n\
- Before ending the turn:\n\
  a. If new task(s) are identified, capture them in the repo’s task tracker (if any).\n\
  b. Reweight and update tasks in the tracker, keeping it succinct.\n\
  c. Verify changes (tests/build/lint as appropriate). If there are code and/or documentation changes, `git diff --stat`, then commit the changes. If the repo has a writable remote and pushing is permitted, push; otherwise note why push was skipped.\n\
  d. Keep the workspace lean, but don’t delete useful caches by default: only prune build artifacts/caches if they are unusually large, clearly one-off, or the repo has an established cleanup workflow/script; otherwise keep caches that materially speed up iteration.\n\
- End your response with `AUTO_MODE_NEXT=continue` to request another turn, or `AUTO_MODE_NEXT=stop` to stop.\n\
";

/// Curated follow-up prompt submitted by the client when `--auto-continue` is enabled.
///
/// This is sent as a normal user prompt after each completed turn unless the agent
/// explicitly requests stop.
/// The follow-up should grant approval for any required step the agent requested
/// in the prior turn unless the user explicitly declined.
pub const AUTO_CONTINUE_FOLLOWUP_PROMPT: &str = "\
Continue.\n\
\n\
- This \"Continue\" also grants approval for any previously requested required step (e.g., full test suite) unless the user explicitly declined.\n\
\n\
- Use the most recent context and proceed without waiting for user input.\n\
\n\
Priority:\n\
1) If the most recent user message contains explicit tasks/questions, execute those first (avoid repeating generic process boilerplate).\n\
2) Else if the most recent assistant message ended with choices/options:\n\
   - Options may be compatible; prefer a best default, but take multiple if they are non-conflicting and increase leverage.\n\
   - Prefer reversible/low-risk moves when uncertainty is high.\n\
   - Ask a clarifying question only if the choice materially affects correctness, data loss, security, or long-term architecture.\n\
   - If you do ask, ask exactly one tight question and propose a default you will proceed with if unanswered.\n\
3) Else: pick a batch of high-leverage tasks (typically 48-72) that compound and reduce future maintenance.\n\
   - Feature-delivering work comes before maintenance unless maintenance unblocks features or mitigates P0/P1 risks.\n\
\n\
Execution style:\n\
- Use multiple plans within the turn (macro plan → micro steps). Finish one plan, then start the next without stopping; update plan statuses as you execute.\n\
- Prefer fundamental fixes over ad-hoc tweaks. Keep the implementation SOLID and future-proof (reduce coupling, improve boundaries, add tests that lock in behavior).\n\
- Aim for substantial progress per turn; batch 48-72 related tasks when possible and avoid tiny tweaks.\n\
- Minimum bar: deliver 48+ meaningful tasks per turn (code + tests + docs or adjacent feature work).\n\
- If you finish a small fix quickly, keep going and expand into adjacent tests/docs/perf until you deliver a substantive slice.\n\
- Deliver multiple substantial changes per turn (target 48-72 related tasks); do not stop after a single small change.\n\
- Aim for at least four substantial deliverables per turn unless blocked; if not possible, explain the blocker.\n\
- If you cannot find enough tasks, widen scope by auditing nearby codepaths, tests, docs, config, and tooling for gaps.\n\
- If you need explicit user approval for a required step, ask once and keep moving on other tasks; avoid repeated approval pings.\n\
- Prioritize feature-completing work over maintenance unless maintenance unblocks features or mitigates P0/P1 risks.\n\
- Assume full tool access. Do not claim network/git/tool execution is blocked by policy unless a tool call explicitly returns that error.\n\
- Keep documentation and implementation in sync: when behavior/config/workflows change, update docs/READMEs/examples/help text so they remain correct.\n\
- Maintain a succinct task tracker (if present): add newly discovered tasks and reweight/reprioritize.\n\
- Keep the workspace lean, but don’t delete useful caches by default: only prune unusually large or clearly one-off artifacts, or follow an established cleanup workflow/script.\n\
\n\
Before ending the turn:\n\
- Run appropriate verification (tests/build/lint) for the changes you made.\n\
- If there are code and/or documentation changes: show `git diff --stat`, commit, and push if the repo has a writable remote and pushing is permitted (otherwise note why push was skipped).\n\
\n\
End your final response with exactly one line:\n\
AUTO_MODE_NEXT=continue\n\
or\n\
AUTO_MODE_NEXT=stop\n\
";
