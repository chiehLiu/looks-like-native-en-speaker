use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[derive(Serialize, Deserialize)]
struct RewriteResult {
    natural: String,
    professional: String,
    casual: String,
    notes: Vec<String>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
struct Candidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: Option<String>,
}

#[tauri::command]
async fn rewrite(text: String) -> Result<RewriteResult, String> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY not set. Add it to your environment.".to_string())?;

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

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    let body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    { "text": prompt }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 2048
        }
    });

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!("API error ({}): {}", status, response_text));
    }

    let gemini_response: GeminiResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    let text_content = gemini_response
        .candidates
        .as_ref()
        .and_then(|c| c.first())
        .and_then(|c| c.content.parts.first())
        .and_then(|p| p.text.as_ref())
        .ok_or("No text in response")?;

    // Strip markdown code fences if present
    let cleaned = text_content
        .trim()
        .strip_prefix("```json")
        .or_else(|| text_content.trim().strip_prefix("```"))
        .unwrap_or(text_content.trim());
    let cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned).trim();

    let result: RewriteResult = serde_json::from_str(cleaned).map_err(|e| {
        format!(
            "Failed to parse rewrite result: {} — raw: {}",
            e, text_content
        )
    })?;

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
    // Load .env: try ~/.config/looks-like-native/.env, then working directory
    if let Some(home) = dirs::home_dir() {
        let config_env = home.join(".config/looks-like-native/.env");
        if config_env.exists() {
            let _ = dotenvy::from_path(&config_env);
        } else {
            let _ = dotenvy::dotenv();
        }
    } else {
        let _ = dotenvy::dotenv();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed
                        && shortcut.key == Code::KeyL
                    {
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
        .setup(|app| {
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyL);
            app.global_shortcut().register(shortcut)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![rewrite, toggle_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
