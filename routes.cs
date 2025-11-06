
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PredictRequest {
    pub text: String,
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchPredictRequest {
    pub texts: Vec<String>,
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PredictResponse {
    pub success: bool,
    pub result: Option<crate::inference::InferenceResult>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchPredictResponse {
    pub success: bool,
    pub results: Option<Vec<crate::inference::InferenceResult>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}
