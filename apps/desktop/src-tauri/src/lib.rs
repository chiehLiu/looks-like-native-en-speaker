use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct RewriteResult {
    natural: String,
    professional: String,
    casual: String,
    notes: Vec<String>,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

#[tauri::command]
async fn rewrite(text: String, api_key: String) -> Result<RewriteResult, String> {
    let client = reqwest::Client::new();

    let prompt = format!(
        r#"Rewrite the following sentence to sound like a native English speaker.

Rules:
- Keep original meaning
- Improve fluency
- Provide 3 versions: natural, professional, casual
- Provide short explanation of changes
- Keep output concise

Input:
{}

Respond ONLY in this exact JSON format, no markdown, no code fences:
{{"natural": "...", "professional": "...", "casual": "...", "notes": ["change 1", "change 2"]}}"#,
        text
    );

    let body = serde_json::json!({
        "model": "claude-sonnet-4-20250514",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let response_text = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!("API error ({}): {}", status, response_text));
    }

    let claude_response: ClaudeResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("Failed to parse API response: {}", e))?;

    let text_content = claude_response
        .content
        .iter()
        .find_map(|block| block.text.as_ref())
        .ok_or("No text in response")?;

    let result: RewriteResult =
        serde_json::from_str(text_content).map_err(|e| format!("Failed to parse rewrite result: {} — raw: {}", e, text_content))?;

    Ok(result)
}

#[tauri::command]
async fn toggle_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, _event| {
                    if shortcut.key == tauri_plugin_global_shortcut::Code::KeyF {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![rewrite, toggle_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
