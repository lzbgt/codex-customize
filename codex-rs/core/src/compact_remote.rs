use std::sync::Arc;

use crate::Prompt;
use crate::codex::Session;
use crate::codex::TurnContext;
use crate::error::CodexErr;
use crate::error::Result as CodexResult;
use crate::protocol::CompactedItem;
use crate::protocol::ContextCompactedEvent;
use crate::protocol::EventMsg;
use crate::protocol::RolloutItem;
use crate::protocol::TurnStartedEvent;
use codex_protocol::models::ResponseItem;
use tokio_util::sync::CancellationToken;

pub(crate) async fn run_inline_remote_auto_compact_task(
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>,
    cancellation_token: CancellationToken,
) {
    run_remote_compact_task_inner(&sess, &turn_context, &cancellation_token).await;
}

pub(crate) async fn run_remote_compact_task(
    sess: Arc<Session>,
    turn_context: Arc<TurnContext>,
    cancellation_token: CancellationToken,
) {
    let start_event = EventMsg::TurnStarted(TurnStartedEvent {
        model_context_window: turn_context.client.get_model_context_window(),
    });
    sess.send_event(&turn_context, start_event).await;

    run_remote_compact_task_inner(&sess, &turn_context, &cancellation_token).await;
}

async fn run_remote_compact_task_inner(
    sess: &Arc<Session>,
    turn_context: &Arc<TurnContext>,
    cancellation_token: &CancellationToken,
) {
    if let Err(err) =
        run_remote_compact_task_inner_impl(sess, turn_context, cancellation_token).await
    {
        let event = EventMsg::Error(
            err.to_error_event(Some("Error running remote compact task".to_string())),
        );
        sess.send_event(turn_context, event).await;
    }
}

async fn run_remote_compact_task_inner_impl(
    sess: &Arc<Session>,
    turn_context: &Arc<TurnContext>,
    cancellation_token: &CancellationToken,
) -> CodexResult<()> {
    let history = sess.clone_history().await;

    // Required to keep `/undo` available after compaction
    let ghost_snapshots: Vec<ResponseItem> = history
        .raw_items()
        .iter()
        .filter(|item| matches!(item, ResponseItem::GhostSnapshot { .. }))
        .cloned()
        .collect();

    let prompt = Prompt {
        input: history.for_prompt(),
        tools: vec![],
        parallel_tool_calls: false,
        base_instructions: sess.get_base_instructions().await,
        personality: turn_context.personality,
        output_schema: None,
    };

    let idle_timeout = turn_context.client.get_provider().stream_idle_timeout();
    let mut new_history = tokio::select! {
        _ = cancellation_token.cancelled() => {
            return Err(CodexErr::TurnAborted);
        }
        result = tokio::time::timeout(
            idle_timeout,
            turn_context.client.compact_conversation_history(&prompt)
        ) => {
            match result {
                Ok(history) => history?,
                Err(_) => {
                    let idle_secs = idle_timeout.as_secs_f64();
                    return Err(CodexErr::Stream(
                        format!("remote compact idle timeout ({idle_secs:.1}s)"),
                        None,
                    ));
                }
            }
        }
    };

    if !ghost_snapshots.is_empty() {
        new_history.extend(ghost_snapshots);
    }
    sess.replace_history(new_history.clone()).await;
    sess.recompute_token_usage(turn_context).await;

    let compacted_item = CompactedItem {
        message: String::new(),
        replacement_history: Some(new_history),
    };
    sess.persist_rollout_items(&[RolloutItem::Compacted(compacted_item)])
        .await;

    let event = EventMsg::ContextCompacted(ContextCompactedEvent {});
    sess.send_event(turn_context, event).await;

    Ok(())
}
