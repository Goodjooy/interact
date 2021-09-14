use msg_proc::{send::cmd::CmdWithSendBody, SendBodyTypeNotFoundError};
use std::sync::mpsc::SendError;

#[derive(Debug)]
pub enum InteractError {
    ChannelSend(SendError<CmdWithSendBody>),
    SendBodyTypeNotSupport(String),
    ErrInfo(String),
}

pub type InteractorResult<T> = Result<T, InteractError>;

impl From<SendError<CmdWithSendBody>> for InteractError {
    fn from(input: SendError<CmdWithSendBody>) -> Self {
        Self::ChannelSend(input)
    }
}

impl From<SendBodyTypeNotFoundError> for InteractError {
    fn from(err: SendBodyTypeNotFoundError) -> Self {
        Self::SendBodyTypeNotSupport(err.target_mod)
    }
}

impl From<String> for InteractError {
    fn from(s: String) -> Self {
        Self::ErrInfo(s)
    }
}
