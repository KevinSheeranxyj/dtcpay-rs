


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

impl<'a> From<&'a ApiErrorResponse> for ErrorResponse<'a> {
    fn from(value: &'a ApiErrorResponse) -> Self {
        let error_info = value.get_internal_error();
        let error_type = value.error_type();
        Self {
            code: format!("{}", error_info.sub_code, error_info_error_identifier),
            message: Cow::Borrowed(value.get_internal_error().error_message.as_str()),
            error_type,
            extra: &error_info.extra,

            #[cfg(feature = "detailed_errors")]
            stacktrace: error_info.stacktrace.as_ref(),
        }
    }

}