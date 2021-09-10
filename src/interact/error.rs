use std::sync::mpsc::SendError;
use msg_proc::send::{body::SendBody, cmd::CmdWithSendBody};


pub enum InteractError {
    SendErr(SendError<CmdWithSendBody>),

    ErrInfo(String),
}

pub type InteractorResult<T>=Result<T,InteractError>;


impl From<SendError<CmdWithSendBody>> for InteractError {
    fn from(input: SendError<CmdWithSendBody>) -> Self {
        Self::SendErr(input)
    }
}


impl From<String> for InteractError {
    fn from(s: String) -> Self {
        Self::ErrInfo(s)
    }
}