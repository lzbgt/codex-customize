const THROUGHPUT_GUIDANCE: &str = r#"## Throughput and batching

- Prefer completing multiple related, high-leverage tasks in a single turn when feasible.
- Avoid micro-changes or single-line patches if broader, meaningful progress is available.
- If there are several adjacent improvements that compound (code, tests, docs), batch them together.
- Continue working until you have delivered substantial progress, not just a small tweak."#;

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
            "{input}\n\n## Throughput and batching\n\n- Prefer completing multiple related, high-leverage tasks in a single turn when feasible.\n- Avoid micro-changes or single-line patches if broader, meaningful progress is available.\n- If there are several adjacent improvements that compound (code, tests, docs), batch them together.\n- Continue working until you have delivered substantial progress, not just a small tweak."
        );
        assert_eq!(ensure_throughput_guidance(input.to_string()), expected);
    }

    #[test]
    fn leaves_existing_throughput_guidance_intact() {
        let input = "Header\n\n## Throughput and batching\n\n- already here";
        assert_eq!(ensure_throughput_guidance(input.to_string()), input);
    }
}
