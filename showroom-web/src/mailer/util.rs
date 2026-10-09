use aws_sdk_sesv2::{
    error::{DisplayErrorContext, ProvideErrorMetadata, SdkError},
    types::Content,
};

pub fn convert_ses_content(data: impl Into<String>) -> Content {
    Content::builder().data(data.into()).build().unwrap()
}

pub fn ses_error<E, R>(action: &str, error: &SdkError<E, R>) -> String
where
    E: ProvideErrorMetadata + std::error::Error + 'static,
    R: std::fmt::Debug,
{
    match error.message() {
        Some(message) => format!("{action}: {message}"),
        None => format!("{action}: {}", DisplayErrorContext(error)),
    }
}

pub fn is_retryable<E: ProvideErrorMetadata, R>(error: &SdkError<E, R>) -> bool {
    match error {
        SdkError::TimeoutError(_) | SdkError::DispatchFailure(_) | SdkError::ResponseError(_) => true,
        _ => matches!(
            error.code(),
            Some("TooManyRequestsException" | "Throttling" | "ThrottlingException" | "InternalFailure" | "ServiceUnavailable")
        ),
    }
}
