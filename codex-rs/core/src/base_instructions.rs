const THROUGHPUT_GUIDANCE: &str = r#"## Throughput and batching

- Prefer completing multiple related, high-leverage tasks in a single turn (aim for 36-48 when feasible).
- If the task is open-ended or the user says "Continue", proactively select a batch of adjacent improvements (code, tests, docs) and finish them.
- Avoid micro-changes; expand to the next meaningful slice that compounds and reduces follow-on work.
- Keep going until you deliver a substantial result; only pause when blocked by missing info or risk."#;

pub(crate) fn ensure_throughput_guidance(base_instructions: String) -> String {
    if base_instructions.contains("Throughput and batching") {
        return base_instructions;
    }
    format!("{base_instructions}\n\n{THROUGHPUT_GUIDANCE}")
}

#[cfg(test)]
mod tests {
    use super::ensure_throughput_guidance;
    use pretty_assertions::assert_eq;

    #[test]
    fn appends_throughput_guidance_when_missing() {
        let input = "Hello";
        let expected = format!(
            "{input}\n\n## Throughput and batching\n\n- Prefer completing multiple related, high-leverage tasks in a single turn (aim for 36-48 when feasible).\n- If the task is open-ended or the user says \"Continue\", proactively select a batch of adjacent improvements (code, tests, docs) and finish them.\n- Avoid micro-changes; expand to the next meaningful slice that compounds and reduces follow-on work.\n- Keep going until you deliver a substantial result; only pause when blocked by missing info or risk."
        );
        assert_eq!(ensure_throughput_guidance(input.to_string()), expected);
    }

    #[test]
    fn leaves_existing_throughput_guidance_intact() {
        let input = "Header\n\n## Throughput and batching\n\n- already here";
        assert_eq!(ensure_throughput_guidance(input.to_string()), input);
    }
}
