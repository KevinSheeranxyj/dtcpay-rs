


#[derive(Debug,  serde::Serialize)]
pub enum ErrorType {
    InvalidRequestError,
    RouterError,
    ConnectorError,
}


struct ErrorResponse<'a> {
    error_type: ErrorType,
    message: Cow<'a, str>,
    code: String,
    extra: &'a Option<Extra>,
    stacktrace: Option<&'a serde_json::Value>,
}