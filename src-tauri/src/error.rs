use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub sample: Option<String>,
    pub message: String,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            sample: None,
            message: message.into(),
        }
    }

    pub fn with_sample(
        code: impl Into<String>,
        sample: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            sample: Some(sample.into()),
            message: message.into(),
        }
    }
}
