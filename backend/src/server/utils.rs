use poem::http::StatusCode;
use serde::Serialize;

use super::search::{ResponseMessage, ResponseData};

#[derive(Debug, Serialize)]
pub struct Message {
    pub status: u16,
    pub message: String,
    pub usage: Option<Vec<String>>,
    pub demo: Option<ResponseMessage>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            status: 200,
            message: "".to_owned(),
            usage: None,
            demo: None,
        }
    }
}

impl Message {
    pub fn with_status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.as_u16();
        self
    }
    pub fn insert_usage(&mut self, usage: &str) -> &mut Self {
        let mut usages = vec![];
        if let Some(_usage) = self.usage.clone() {
            usages.extend(_usage);
        }
        usages.push(usage.to_owned());
        self.usage = Some(usages);
        self
    }
    pub fn insert_demo(&mut self, result: ResponseMessage) -> &mut Self {
        self.demo = Some(result);
        self
    }
    pub fn with_demo(&mut self) -> &mut Self {
        self.demo = Some(ResponseMessage {
            status: 200,
            data: vec![ResponseData {
                repo: "core".to_owned(),
                name: "pacman".to_owned(),
                version: "6.0.2-6".to_owned(),
                path: vec![
                    "/usr/share/bash-completion/completions/pacman".to_owned(),
                    "/usr/share/bash-completion/completions/pacman".to_owned(),
                ],
            }],
        });
        self
    }
    pub fn with_message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_owned();
        self
    }
}

#[derive(Debug, Serialize)]
pub struct RequestFailed {
    pub status: u16,
    pub error: String
}

impl Default for RequestFailed {
    fn default() -> Self {
        Self { status: StatusCode::NOT_FOUND.as_u16(), error: "".to_owned() }
    }
}

impl RequestFailed {
    pub fn with_error_msg(&mut self, msg: &str) -> &mut Self {
        self.error = msg.to_owned();
        self
    }
    pub fn with_status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.as_u16();
        self
    }
}
