use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum DiziErrorKind {
    // io related
    IoError(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    SerdeJson(serde_json::Error),
    ClipboardError,

    Glob,
    InvalidParameters,

    SendError,

    DecoderError,
    NoDevice,
    UnrecognizedFormat,
    StreamError(rodio::StreamError),

    UnrecognizedArgument,
    UnrecognizedCommand,
}

impl From<io::ErrorKind> for DiziErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::IoError(err)
    }
}

impl From<&globset::ErrorKind> for DiziErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl From<std::env::VarError> for DiziErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarNotPresent
    }
}

impl From<rodio::PlayError> for DiziErrorKind {
    fn from(err: rodio::PlayError) -> Self {
        match err {
            rodio::PlayError::DecoderError(_) => Self::DecoderError,
            rodio::PlayError::NoDevice => Self::NoDevice,
        }
    }
}

impl From<rodio::StreamError> for DiziErrorKind {
    fn from(err: rodio::StreamError) -> Self {
        Self::StreamError(err)
    }
}

impl From<rodio::decoder::DecoderError> for DiziErrorKind {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self::UnrecognizedFormat
    }
}

impl From<serde_json::Error> for DiziErrorKind {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

#[cfg(feature = "ffmpeg")]
impl From<ffmpeg_decoder::Error> for DiziErrorKind {
    fn from(_: ffmpeg_decoder::Error) -> Self {
        Self::DecoderError
    }
}
