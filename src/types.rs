/// Contains statically-typed stuff, of what is usually stringly-typed (f.ex. Standardized Error
/// Message).

use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,

    pub message: String,
}
