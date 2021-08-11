use crate::interact::context::ContextInteractHandle;
use crate::interact::utils::MultiToOne;
use std::{
    sync::mpsc::SendError,
};

use msg_chain::MessageChain;
use msg_proc::{Sender, send::body::SendBody};


use self::{manage::{InteractManager, MessageCmd}, utils::Channel};

mod manage;
mod sharp_cmd;
mod utils;
mod manager_contain;
mod context;

 pub  trait  Interactor  {
    fn do_interact(
        &self,
        cmd: MessageCmd,
        msg: Vec<Box<dyn MessageChain>>,
        sender: Box<dyn Sender>,
        channel: &Channel,
    ) -> Result<Option<Box<dyn ContextInteractHandle>>,SendError<SendBody>>;
}




