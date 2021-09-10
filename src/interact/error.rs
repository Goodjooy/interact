use std::sync::mpsc::SendError;
use msg_proc::send::{cmd::CmdWithSendBody};

#[derive(Debug)]
pub enum InteractError {
    SendErr(SendError<CmdWithSendBody>),
    ConstructSendFromSrouceFailure{src_type:String,sender_id:u64},
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