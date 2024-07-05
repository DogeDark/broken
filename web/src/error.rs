#[allow(unused)]
#[derive(Debug)]
pub enum AppError {
    Socket(String),
    Parse(String),
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::Parse(value.to_string())
    }
}