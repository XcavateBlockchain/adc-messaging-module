use core::fmt::Display;

#[derive(Debug, Clone)]
pub enum MessageBuilderError {
    MissingId,
    MissingBody,
    SerializationError,
    MissingLatestKey,
    MissingMessage,
    MissingMediaItem,
}

impl Display for MessageBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            MessageBuilderError::MissingId => "Missing ID in the message",
            MessageBuilderError::MissingBody => "Missing body in the message",
            MessageBuilderError::SerializationError => "Error during message serialization",
            MessageBuilderError::MissingLatestKey => "Missing latest key",
            MessageBuilderError::MissingMessage => "Missing message content",
            MessageBuilderError::MissingMediaItem => "Missing media item",
        };
        write!(f, "{}", msg)
    }
}
