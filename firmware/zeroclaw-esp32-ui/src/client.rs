use heapless::String;
use log::{info, error};
use serde::{Deserialize, Serialize};

use crate::protocol::{ChatMessage, MessageRole, UiResponse};

pub struct ZeroClawClient {
    base_url: String<128>,
    api_key: Option<String<64>>,
    connected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebhookRequest {
    message: String<1024>,
}

#[derive(Debug, Deserialize)]
struct WebhookResponse {
    response: Option<String<2048>>,
    #[serde(default)]
    error: Option<String<256>>,
}

impl ZeroClawClient {
    pub fn new(base_url: &str) -> Result<Self, ClientError> {
        let mut url = String::new();
        url.push_str(base_url)
            .map_err(|_| ClientError::UrlTooLong)?;
        
        Ok(Self {
            base_url: url,
            api_key: None,
            connected: false,
        })
    }

    pub fn set_api_key(&mut self, key: &str) -> Result<(), ClientError> {
        let mut api_key = String::new();
        api_key
            .push_str(key)
            .map_err(|_| ClientError::ApiKeyTooLong)?;
        
        self.api_key = Some(api_key);
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }

    pub async fn send_message(&mut self, message: &str) -> Result<String, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/webhook")
            .map_err(|_| ClientError::UrlTooLong)?;
        
        info!("Sending message to: {}", url);
        
        let mut request_body = String::<1024>::new();
        request_body
            .push_str(message)
            .map_err(|_| ClientError::MessageTooLong)?;
        
        let client = esp_idf_svc::http::client::EspHttpConnection::new(
            &esp_idf_svc::http::client::Configuration::default(),
        )
        .map_err(|e| ClientError::ConnectionFailed(e.to_string()))?;
        
        let mut request = client
            .post(url.as_str(), "application/json")
            .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
        
        if let Some(ref key) = self.api_key {
            let mut auth_header = String::<64>::new();
            let _ = write!(auth_header, "Bearer {}", key);
            request
                .set_header("Authorization", auth_header.as_str())
                .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
        }
        
        request
            .send(request_body.as_bytes())
            .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
        
        let status = request
            .status_code()
            .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
        
        if status != 200 {
            error!("HTTP error: {}", status);
            return Err(ClientError::HttpError(status));
        }
        
        let mut response_body = [0u8; 2048];
        let bytes_read = request
            .read(&mut response_body)
            .map_err(|e| ClientError::ReadFailed(e.to_string()))?;
        
        let response_str =
            core::str::from_utf8(&response_body[..bytes_read]).map_err(|_| ClientError::InvalidResponse)?;
        
        let webhook_resp: WebhookResponse = serde_json_core::from_str(response_str)
            .map_err(|_| ClientError::ParseError)?
            .0;
        
        if let Some(err) = webhook_resp.error {
            error!("Server error: {}", err);
            return Err(ClientError::ServerError(err));
        }
        
        webhook_resp
            .response
            .map(|r| {
                self.connected = true;
                r
            })
            .ok_or(ClientError::NoResponse)
    }

    pub async fn check_connection(&mut self) -> Result<bool, ClientError> {
        let mut url = self.base_url.clone();
        url.push_str("/health").map_err(|_| ClientError::UrlTooLong)?;
        
        let client = esp_idf_svc::http::client::EspHttpConnection::new(
            &esp_idf_svc::http::client::Configuration::default(),
        )
        .map_err(|e| ClientError::ConnectionFailed(e.to_string()))?;
        
        let request = client
            .get(url.as_str())
            .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
        
        match request.send(&[]) {
            Ok(resp) => {
                let status = resp
                    .status_code()
                    .map_err(|e| ClientError::RequestFailed(e.to_string()))?;
                
                let connected = status == 200;
                self.connected = connected;
                Ok(connected)
            }
            Err(e) => {
                warn!("Health check failed: {}", e);
                self.connected = false;
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClientError {
    UrlTooLong,
    ApiKeyTooLong,
    MessageTooLong,
    ConnectionFailed(String),
    RequestFailed(String),
    HttpError(u16),
    ReadFailed(String),
    InvalidResponse,
    ParseError,
    ServerError(String<256>),
    NoResponse,
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ClientError::UrlTooLong => write!(f, "URL too long"),
            ClientError::ApiKeyTooLong => write!(f, "API key too long"),
            ClientError::MessageTooLong => write!(f, "Message too long"),
            ClientError::ConnectionFailed(e) => write!(f, "Connection failed: {}", e),
            ClientError::RequestFailed(e) => write!(f, "Request failed: {}", e),
            ClientError::HttpError(code) => write!(f, "HTTP error: {}", code),
            ClientError::ReadFailed(e) => write!(f, "Read failed: {}", e),
            ClientError::InvalidResponse => write!(f, "Invalid response"),
            ClientError::ParseError => write!(f, "Failed to parse response"),
            ClientError::ServerError(e) => write!(f, "Server error: {}", e),
            ClientError::NoResponse => write!(f, "No response from server"),
        }
    }
}
