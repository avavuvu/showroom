use aws_sdk_sesv2::types::Content;

pub fn convert_ses_content(data: impl Into<String>) -> Content {
    Content::builder().data(data.into()).build().unwrap()
}
