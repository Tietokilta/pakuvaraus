pub struct Attachment<'a> {
    pub filename: String,
    pub content_type: &'a str,
    pub data: Vec<u8>,
}
