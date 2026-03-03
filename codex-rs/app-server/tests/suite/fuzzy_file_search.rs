use anyhow::Result;
use anyhow::anyhow;
use app_test_support::McpProcess;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::RequestId;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::TempDir;
use tokio::time::timeout;

const DEFAULT_READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_fuzzy_file_search_sorts_and_includes_indices() -> Result<()> {
    // Prepare a temporary Codex home and a separate root with test files.
    let codex_home = TempDir::new()?;
    let root = TempDir::new()?;

    // Create files designed to have deterministic ordering for query "abe".
    std::fs::write(root.path().join("abc"), "x")?;
    std::fs::write(root.path().join("abcde"), "x")?;
    std::fs::write(root.path().join("abexy"), "x")?;
    std::fs::write(root.path().join("zzz.txt"), "x")?;
    let sub_dir = root.path().join("sub");
    std::fs::create_dir_all(&sub_dir)?;
    let sub_abce_path = sub_dir.join("abce");
    std::fs::write(&sub_abce_path, "x")?;
    let sub_abce_rel = sub_abce_path
        .strip_prefix(root.path())?
        .to_string_lossy()
        .to_string();

    // Start MCP server and initialize.
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let root_path = root.path().to_string_lossy().to_string();
    // Send fuzzyFileSearch request.
    let request_id = mcp
        .send_fuzzy_file_search_request("abe", vec![root_path.clone()], None)
        .await?;

    // Read response and verify shape and ordering.
    let resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;

    let value = resp.result;
    let files = value
        .get("files")
        .and_then(|files| files.as_array())
        .ok_or_else(|| anyhow!("files key missing"))?;
    assert_eq!(files.len(), 3);

    let mut by_path = std::collections::HashMap::new();
    for file in files {
        let path = file
            .get("path")
            .and_then(|path| path.as_str())
            .ok_or_else(|| anyhow!("path key missing"))?;
        by_path.insert(path.to_string(), file);
    }

    let abexy = by_path
        .get("abexy")
        .ok_or_else(|| anyhow!("abexy missing"))?;
    assert_eq!(abexy["root"], root_path);
    assert_eq!(abexy["file_name"], "abexy");
    assert_eq!(abexy["indices"], json!([0, 1, 2]));

    let abcde = by_path
        .get("abcde")
        .ok_or_else(|| anyhow!("abcde missing"))?;
    assert_eq!(abcde["root"], root_path);
    assert_eq!(abcde["file_name"], "abcde");
    assert_eq!(abcde["indices"], json!([0, 1, 4]));

    let sub_abce = by_path
        .get(&sub_abce_rel)
        .ok_or_else(|| anyhow!("sub/abce missing"))?;
    assert_eq!(sub_abce["root"], root_path);
    assert_eq!(sub_abce["file_name"], "abce");
    assert_eq!(sub_abce["indices"], json!([4, 5, 7]));

    let scores = files
        .iter()
        .map(|file| {
            file.get("score")
                .and_then(serde_json::Value::as_i64)
                .ok_or_else(|| anyhow!("score missing"))
        })
        .collect::<Result<Vec<_>>>()?;
    for window in scores.windows(2) {
        assert!(window[0] >= window[1], "scores not sorted desc: {scores:?}");
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_fuzzy_file_search_accepts_cancellation_token() -> Result<()> {
    let codex_home = TempDir::new()?;
    let root = TempDir::new()?;

    std::fs::write(root.path().join("alpha.txt"), "contents")?;

    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let root_path = root.path().to_string_lossy().to_string();
    let request_id = mcp
        .send_fuzzy_file_search_request("alp", vec![root_path.clone()], None)
        .await?;

    let request_id_2 = mcp
        .send_fuzzy_file_search_request(
            "alp",
            vec![root_path.clone()],
            Some(request_id.to_string()),
        )
        .await?;

    let resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id_2)),
    )
    .await??;

    let files = resp
        .result
        .get("files")
        .ok_or_else(|| anyhow!("files key missing"))?
        .as_array()
        .ok_or_else(|| anyhow!("files not array"))?
        .clone();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0]["root"], root_path);
    assert_eq!(files[0]["path"], "alpha.txt");

    Ok(())
}
