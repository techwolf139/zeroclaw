slint::include_modules!();

use slint::VecModel;
use std::rc::Rc;

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
    app.on_send_message(move |text| {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        // Add user message
        messages_clone.push(Message {
            role: "user".into(),
            content: text.into(),
            timestamp: "10:02".into(),
        });
        // Simulate response
        messages_clone.push(Message {
            role: "assistant".into(),
            content: format!("Echo: {}", text).into(),
            timestamp: "10:02".into(),
        });
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
    app.on_open_memory(move || {
        println!("Opening memory...");
        if let Some(app) = app_weak.upgrade() {
            app.set_show_memory(true);
        }
    });

    // Callback: open-llm
    let app_weak = app.as_weak();
    app.on_open_llm(move || {
        println!("Opening LLM...");
        if let Some(app) = app_weak.upgrade() {
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
