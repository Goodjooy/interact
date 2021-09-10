use crate::interact::context::ContextInteractHandle;

use std::{ sync::mpsc::SendError};

use msg_chain::MessageChain;
use msg_proc::{send::cmd::CmdWithSendBody, Sender};

use self::{error::InteractorResult, manage::MessageCmd, utils::Channel};

pub mod context;
pub mod manage;
pub mod manager_contain;
pub mod utils;
pub mod error;
pub trait Interactor: Send +Sync{
    fn do_interact(
        &self,
        cmd: MessageCmd,
        msg: &Vec<Box<dyn MessageChain>>,
        sender: &Box<dyn Sender>,
        channel: &Channel,
    ) -> InteractorResult<Option<Box<dyn ContextInteractHandle>>>;
}

