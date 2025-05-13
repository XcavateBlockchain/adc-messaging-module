use core::fmt::Display;

#[derive(Debug, Clone)]
pub enum MessageBuilderError {
    MissingBody,
    SerializationError,
    MissingKey,
    MissingMessage,
    MissingMediaItem,
}

impl Display for MessageBuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            MessageBuilderError::MissingBody => "Missing body in the message",
            MessageBuilderError::SerializationError => "Error during message serialization",
            MessageBuilderError::MissingKey => "Missing at least one key in the message",
            MessageBuilderError::MissingMessage => "Missing message content",
            MessageBuilderError::MissingMediaItem => "Missing media item",
        };
        write!(f, "{}", msg)
    }
}
