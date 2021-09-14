use msg_chain::MessageChain;
use msg_proc::{chain::chain_builder::ChainBuilder, send::contain::new_source_send, Sender};

use crate::{
    interact::utils::Channel,
    interact_result,
    interactions::{
        context::{ActiveMod::SameUserInSameGroup, ContextInteractHandle, ALIVE, DEATH},
        error::{InteractorResult},
        manage::MessageCmd,
        Interactor,
    },
};
use msg_proc::chain::chain_handle::ToMsgHandle;

pub struct MockContextInteractor;

impl Interactor for MockContextInteractor {
    fn do_interact(
        &self,
        cmd: MessageCmd,
        _msg: &Vec<Box<dyn MessageChain>>,
        sender: &Box<dyn Sender>,
        channel: &Channel,
    ) -> InteractorResult<Option<Box<dyn ContextInteractHandle>>> {
        let msg = ChainBuilder::new()
            .at(*sender.get_sender_id())
            .text("enable context: Same User in Same Group")
            .simplify()
            .build();

        channel.send(new_source_send(cmd.get_src_type(), sender, msg, None)?)?;

        interact_result!(MockContextHandle)
    }
}

pub struct MockContextHandle;

impl ContextInteractHandle for MockContextHandle {
    fn get_sign(&self) -> String {
        String::new()
    }

    fn do_follow_interact(
        &mut self,
        msg: &Vec<Box<dyn MessageChain>>,
        _sender: &Box<dyn Sender>,
        _channel: &Channel,
    ) -> InteractorResult<Option<()>> {
        let handle = msg.to_msg_handle();
        let txt = handle.conbin_plain().unwrap_or(String::new());
        if txt.starts_with("end") {
            DEATH
        } else {
            ALIVE
        }
    }

    fn active_mod(&self) -> crate::interactions::context::ActiveMod {
        SameUserInSameGroup
    }
}
