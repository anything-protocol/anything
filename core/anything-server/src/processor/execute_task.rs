use std::sync::Arc;

use postgrest::Postgrest;

use crate::bundler::bundle_tasks_cached_context;
use crate::processor::process_trigger_utils::process_trigger_task;
use crate::system_actions::formatter_actions::{
    date_formatter::process_date_task, text_formatter::process_text_task,
};
use crate::system_actions::output_action::process_response_task;

use crate::system_actions::process_http_task::process_http_task;
use crate::task_types::Task;
use crate::AppState;

use serde_json::{json, Value};

use crate::task_types::ActionType;

pub async fn execute_task(
    state: Arc<AppState>,
    client: &Postgrest,
    task: &Task,
) -> Result<Option<Value>, Value> {
    println!("[PROCESS TASK] Processing task {}", task.task_id);

    // Clone state before using it in join
    let state_clone = Arc::clone(&state);

    //Bundle context with results from cache
    let bundled_context = bundle_tasks_cached_context(state, client, task, true).await;

    let http_client = state_clone.http_client.clone();

    match bundled_context {
        Ok(bundled_context) => {
            let task_result = if task.r#type == ActionType::Trigger.as_str().to_string() {
                println!("[PROCESS TASK] Processing trigger task {}", task.task_id);
                process_trigger_task(&bundled_context)
            } else {
                println!("[PROCESS TASK] Processing regular task {}", task.task_id);
                match &task.plugin_id {
                    Some(plugin_id) => match plugin_id.as_str() {
                        "http" => process_http_task(&http_client, &bundled_context).await,
                        "response" => {
                            process_response_task(
                                state_clone,
                                task.flow_session_id.clone(),
                                &bundled_context,
                            )
                            .await
                        }
                        "format_text" => process_text_task(&bundled_context),
                        "format_date" => process_date_task(&bundled_context),
                        _ => process_missing_plugin(plugin_id, &task.task_id.to_string()),
                    },
                    None => process_no_plugin_id(&task.task_id.to_string()),
                }
            };

            match task_result {
                Ok(result) => Ok(result),
                Err(e) => Err(json!({
                    "error": e.to_string()
                })),
            }
        }
        Err(e) => Err(json!({
            "error": format!("Failed to bundle task context: {}", e)
        })),
    }
}

// pub async fn execute_task(
//     state: Arc<AppState>,
//     client: &Postgrest,
//     task: &Task,
// ) -> Result<Option<Value>, Value> {
//     println!("[PROCESS TASK] Processing task {}", task.task_id);

//     // Clone state before using it in join
//     let state_clone = Arc::clone(&state);

//     //Bundle context with results from cache
//     let bundled_context = bundle_tasks_cached_context(state, client, task, true).await;

//     let http_client = state_clone.http_client.clone();

//     match bundled_context {
//         Ok(bundled_context) => {
//             if task.r#type == ActionType::Trigger.as_str().to_string() {
//                 println!("[PROCESS TASK] Processing trigger task {}", task.task_id);
//                 process_trigger_task(&bundled_context)
//             } else {
//                 println!("[PROCESS TASK] Processing regular task {}", task.task_id);
//                 match &task.plugin_id {
//                     Some(plugin_id) => match plugin_id.as_str() {
//                         "http" => process_http_task(&http_client, &bundled_context).await,
//                         "response" => {
//                             process_response_task(
//                                 state_clone,
//                                 task.flow_session_id.clone(),
//                                 &bundled_context,
//                             )
//                             .await
//                         }
//                         "format_text" => process_text_task(&bundled_context).await,
//                         "format_date" => process_date_task(&bundled_context).await,
//                         _ => Ok(Some(json!({
//                             "message": format!("Processed task {} with plugin_id {}", task.task_id, plugin_id)
//                         }))),
//                     },
//                     None => Ok(Some(json!({
//                         "message": format!("Processed task {} with no plugin_id", task.task_id)
//                     }))),
//                 }
//             }
//         }
//         Err(e) => Err(json!({
//             "error": format!("Failed to bundle task context: {}", e)
//         })),
//     }
// }

//this worked
// pub async fn execute_task(
//     state: Arc<AppState>,
//     client: &Postgrest,
//     task: &Task,
// ) -> Result<Option<Value>, Value> {
//     println!("[PROCESS TASK] Processing task {}", task.task_id);

//     // Clone state before using it in join
//     let state_clone = Arc::clone(&state);

//     //Bundle context with results from cache
//     let bundled_context = bundle_tasks_cached_context(state, client, task, true).await;

//     let http_client = state_clone.http_client.clone();

//     let result = match bundled_context {
//         Ok(bundled_context) => {
//             let task_result = if task.r#type == ActionType::Trigger.as_str().to_string() {
//                 println!("[PROCESS TASK] Processing trigger task {}", task.task_id);
//                 process_trigger_task(&bundled_context)
//             } else {
//                 println!("[PROCESS TASK] Processing regular task {}", task.task_id);
//                 match &task.plugin_id {
//                     Some(plugin_id) => match plugin_id.as_str() {
//                         "http" => process_http_task(&http_client, &bundled_context).await,
//                         "response" => {
//                             process_response_task(
//                                 state_clone,
//                                 task.flow_session_id.clone(),
//                                 &bundled_context,
//                             )
//                             .await
//                         }
//                         "format_text" => process_text_task(&bundled_context).await,
//                         "format_date" => process_date_task(&bundled_context).await,
//                         _ => Ok(Some(json!({
//                             "message": format!("Processed task {} with plugin_id {}", task.task_id, plugin_id)
//                         }))),
//                     },
//                 }
//             };

//             match task_result {
//                 Ok(result) => Ok(result.unwrap_or(json!(null))),
//                 Err(e) => Err(json!({
//                     "error": e.to_string()
//                 })),
//             }
//         }
//         Err(e) => Err(json!({
//             "error": format!("Failed to bundle task context: {}", e)
//         })),
//     };

//     result
// }

pub fn process_missing_plugin(
    plugin_id: &str,
    task_id: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Some(json!({
        "message": format!("Processed task {} :: plugin_id {} does not exist.", task_id, plugin_id)
    })))
}

pub fn process_no_plugin_id(
    task_id: &str,
) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Some(json!({
        "message": format!("Processed task {} :: no plugin_id found.", task_id)
    })))
}
