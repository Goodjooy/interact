use msg_chain::MessageChain;
use msg_proc::Sender;

use super::{error::InteractorResult, utils::Channel};

#[derive(Hash, PartialEq, Eq)]
pub enum ActiveMod {
    SameUserInSameGroup,
    SameUserInAnyGroup,
    AnyUserInSameGroup,
    AnyUserInAnyGroup,
}
pub trait ContextInteractHandle: Sync + Send {
    fn get_sign(&self) -> String;

    fn active_mod(&self) -> ActiveMod {
        ActiveMod::SameUserInSameGroup
    }

    fn do_follow_interact(
        &mut self,
        msg: &Vec<Box<dyn MessageChain>>,
        sender: &Box<dyn Sender>,
        channel: &Channel,
    ) -> InteractorResult<Option<()>>;
}
