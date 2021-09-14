use crate::interact::context::ContextInteractHandle;

use msg_chain::MessageChain;
use msg_proc::Sender;

use self::{error::InteractorResult, manage::MessageCmd, utils::Channel};

pub mod context;
pub mod error;
pub mod manage;
pub mod manager_contain;
pub mod utils;

pub mod mocks;

pub trait Interactor: Send + Sync {
    fn do_interact(
        &self,
        cmd: MessageCmd,
        msg: &Vec<Box<dyn MessageChain>>,
        sender: &Box<dyn Sender>,
        channel: &Channel,
    ) -> InteractorResult<Option<Box<dyn ContextInteractHandle>>>;
}

#[macro_export]
macro_rules! interact_result {
    () => {
        Ok(None)
    };
    ($x:expr)=>{
        {
            let data:Box<dyn ContextInteractHandle>=Box::new($x);
            Ok(Some(data))
        }
    };
}