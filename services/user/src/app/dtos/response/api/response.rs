use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDto<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ResponseDto<T> {
    pub fn success(message: &str, data: T) -> Self {
        ResponseDto {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        ResponseDto::<T> {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}
