use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String<512>,
    pub timestamp: String<32>,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: &str) -> Self {
        let timestamp = Self::current_timestamp();
        let content = String::from(content);
        Self {
            role,
            content,
            timestamp,
        }
    }

    fn current_timestamp() -> String<32> {
        let mut buf = heapless::String::<32>::new();
        let secs = embassy_time::Tick64::now().as_secs();
        let _ = write!(buf, "{}", secs);
        buf
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiStatus {
    pub connected: bool,
    pub wifi_ssid: Option<String<32>>,
    pub signal_strength: Option<u8>,
    pub battery_level: Option<u8>,
    pub zeroclaw_connected: bool,
}

impl Default for UiStatus {
    fn default() -> Self {
        Self {
            connected: false,
            wifi_ssid: None,
            signal_strength: None,
            battery_level: None,
            zeroclaw_connected: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiCommand {
    #[serde(rename = "send_message")]
    SendMessage { text: String<512> },
    #[serde(rename = "get_status")]
    GetStatus,
    #[serde(rename = "connect_wifi")]
    ConnectWifi {
        ssid: String<32>,
        password: String<64>,
    },
    #[serde(rename = "disconnect")]
    Disconnect,
    #[serde(rename = "clear_messages")]
    ClearMessages,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiResponse {
    pub success: bool,
    pub messages: Option<Vec<ChatMessage>>,
    pub status: Option<UiStatus>,
    pub error: Option<String<128>>,
}

impl UiResponse {
    pub fn ok() -> Self {
        Self {
            success: true,
            messages: None,
            status: None,
            error: None,
        }
    }

    pub fn with_status(status: UiStatus) -> Self {
        Self {
            success: true,
            messages: None,
            status: Some(status),
            error: None,
        }
    }

    pub fn with_messages(messages: Vec<ChatMessage>) -> Self {
        Self {
            success: true,
            messages: Some(messages),
            status: None,
            error: None,
        }
    }

    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            messages: None,
            status: None,
            error: Some(String::from(msg)),
        }
    }
}
