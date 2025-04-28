#[derive(Debug, Clone)]
pub enum MessageBuilderError {
    MissingId,
    MissingBody,
    SerializationError,
    MissingLatestKey,
    MissingPreviousKeys,
    MissingMessage
}
