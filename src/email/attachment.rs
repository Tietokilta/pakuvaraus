#[derive(PartialEq, Eq)]
pub struct Attachment {
    pub filename: String,
    pub content_type: &'static str,
    pub data: Vec<u8>,
}
