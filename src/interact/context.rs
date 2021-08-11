use std::sync::mpsc::SendError;

use msg_chain::MessageChain;
use msg_proc::{Sender, send::body::SendBody};

use super::utils::Channel;

pub enum ActiveMod {
    SameUserInSameGroup,
    SameUserInAnyGroup,
    AnyUserInSameGroup,
    AnyUserInAnyGroup,
}
pub trait ContextInteractHandle {
    fn get_sign(&self) -> String;

    fn active_mod(&self) -> ActiveMod {
        ActiveMod::SameUserInSameGroup
    }

    fn do_follow_interact(
        &mut self,
        msg: Vec<Box<dyn MessageChain>>,
        sender: Box<dyn Sender>,
        channel: Channel,
    ) -> Result<Option<()>,SendError<SendBody>>;
}