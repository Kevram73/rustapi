use serde::{Deserialize, Serialize};

// DTOs génériques pour les réponses API
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl PaginationParams {
    pub fn offset(&self) -> u64 {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(20);
        (page.saturating_sub(1)) * limit
    }

    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(20)
    }
}

