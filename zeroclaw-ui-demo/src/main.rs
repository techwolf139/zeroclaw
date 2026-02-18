slint::include_modules!();

use slint::VecModel;
use std::rc::Rc;

fn call_chat_api(server_url: &str, message: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/v1/chat", server_url);

    let body = serde_json::json!({
        "message": message,
        "model": "claude-sonnet-4-20250514"
    });

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        Ok(json["response"].as_str().unwrap_or("").to_string())
    } else {
        Err(format!("API error: {}", response.status()))
    }
}

fn get_memories(server_url: &str) -> Result<Vec<String>, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/v1/memories", server_url);

    let response = client.get(&url).send().map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        let memories = json["memories"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|m| m["content"].as_str().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_default();
        Ok(memories)
    } else {
        Err(format!("API error: {}", response.status()))
    }
}

fn get_models(server_url: &str) -> Result<Vec<String>, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/v1/models", server_url);

    let response = client.get(&url).send().map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        let models = json["models"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|m| m["id"].as_str().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_default();
        Ok(models)
    } else {
        Err(format!("API error: {}", response.status()))
    }
}

fn main() {
    let app = MainWindow::new().unwrap();

    let messages = Rc::new(VecModel::default());
    messages.push(Message {
        role: "assistant".into(),
        content: "Hello! I am ZeroClaw on ESP32.".into(),
        timestamp: "10:00".into(),
    });
    messages.push(Message {
        role: "user".into(),
        content: "Hi! Ready to chat.".into(),
        timestamp: "10:01".into(),
    });
    messages.push(Message {
        role: "user".into(),
        content: "Hi! Ready to chat.".into(),
        timestamp: "10:01".into(),
    });

    app.set_messages(messages.clone().into());
    app.set_status_text("Ready".into());
    app.set_is_connected(true);
    app.set_wifi_signal(3);
    app.set_server_url("http://192.168.1.100:8080".into());
    app.set_show_menu(false);
    app.set_show_memory(false);
    app.set_show_llm(false);
    app.set_show_skill(false);

    // Callback: send-message
    let messages_clone = messages.clone();
    let server_url = app.get_server_url().to_string();
    app.on_send_message(move |text| {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        let server_url = server_url.clone();

        messages_clone.push(Message {
            role: "user".into(),
            content: text.into(),
            timestamp: "10:02".into(),
        });

        // Call API
        match call_chat_api(&server_url, text) {
            Ok(response) => {
                messages_clone.push(Message {
                    role: "assistant".into(),
                    content: response.into(),
                    timestamp: "10:02".into(),
                });
            }
            Err(e) => {
                messages_clone.push(Message {
                    role: "assistant".into(),
                    content: format!("Error: {}", e).into(),
                    timestamp: "10:02".into(),
                });
            }
        }
    });

    // Callback: open-menu
    let app_weak = app.as_weak();
    app.on_open_menu(move || {
        println!("Opening menu...");
        if let Some(app) = app_weak.upgrade() {
            app.set_show_menu(true);
        }
    });

    // Callback: open-memory
    let app_weak = app.as_weak();
    let server_url = app.get_server_url().to_string();
    app.on_open_memory(move || {
        println!("Loading memories...");
        let server_url = server_url.clone();
        if let Some(app) = app_weak.upgrade() {
            let server_url = server_url.clone();
            match get_memories(&server_url) {
                Ok(memories) => {
                    println!("Loaded {} memories", memories.len());
                }
                Err(e) => {
                    println!("Error loading memories: {}", e);
                }
            }
            app.set_show_memory(true);
        }
    });

    // Callback: open-llm
    let app_weak = app.as_weak();
    let server_url = app.get_server_url().to_string();
    app.on_open_llm(move || {
        println!("Loading models...");
        let server_url = server_url.clone();
        if let Some(app) = app_weak.upgrade() {
            let server_url = server_url.clone();
            match get_models(&server_url) {
                Ok(models) => {
                    println!("Loaded {} models", models.len());
                }
                Err(e) => {
                    println!("Error loading models: {}", e);
                }
            }
            app.set_show_llm(true);
        }
    });

    // Callback: open-skill
    let app_weak = app.as_weak();
    app.on_open_skill(move || {
        println!("Opening skill...");
        if let Some(app) = app_weak.upgrade() {
            app.set_show_skill(true);
        }
    });

    // Callback: close-menu
    let app_weak = app.as_weak();
    app.on_close_menu(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_show_menu(false);
        }
    });

    // Callback: close-memory
    let app_weak = app.as_weak();
    app.on_close_memory(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_show_memory(false);
        }
    });

    // Callback: close-llm
    let app_weak = app.as_weak();
    app.on_close_llm(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_show_llm(false);
        }
    });

    // Callback: close-skill
    let app_weak = app.as_weak();
    app.on_close_skill(move || {
        if let Some(app) = app_weak.upgrade() {
            app.set_show_skill(false);
        }
    });

    // Callback: reconnect
    let app_weak = app.as_weak();
    app.on_reconnect(move || {
        println!("Reconnecting...");
        if let Some(app) = app_weak.upgrade() {
            app.set_status_text("Reconnecting...".into());
            app.set_is_connected(false);
            // Simulate reconnect success
            app.set_status_text("Connected".into());
            app.set_is_connected(true);
            app.set_show_menu(false);
        }
    });

    // Callback: disconnect-wifi
    let app_weak = app.as_weak();
    app.on_disconnect_wifi(move || {
        println!("Disconnecting WiFi...");
        if let Some(app) = app_weak.upgrade() {
            app.set_status_text("Disconnected".into());
            app.set_is_connected(false);
            app.set_wifi_signal(0);
            app.set_show_menu(false);
        }
    });

    println!("ZeroClaw UI Demo started!");
    println!("Window: 320x240 pixels");
    println!("Status: Connected");
    println!("Click [S] for settings, Send or Enter to test");

    app.run().unwrap();
}
