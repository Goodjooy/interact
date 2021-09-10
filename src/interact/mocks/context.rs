use msg_chain::MessageChain;
use msg_proc::{Sender, chain::chain_builder::ChainBuilder, send::contain::new_source_send};

use crate::{interact::utils::Channel, interactions::{Interactor, context::{ActiveMod::SameUserInSameGroup, ContextInteractHandle}, error::{InteractError, InteractorResult}, manage::MessageCmd}};
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
        let msg=ChainBuilder::new()
        .at(*sender.get_sender_id())
        .text("enable context: Same User in Same Group")
        .simplify()
        .build();

        channel.send(
            new_source_send(cmd.get_src_type(), sender, msg, None).ok_or(
                InteractError::ConstructSendFromSrouceFailure {
                    src_type: cmd.get_src_type().clone(),
                    sender_id: *sender.get_sender_id(),
                },
            )?,
        )?;

        Ok(Some(Box::new(MockContextHandle)))
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
        let handle=msg.to_msg_handle();
        let txt=handle.conbin_plain().unwrap_or(String::new());
        if txt.starts_with("end"){
            
            Ok(None)
        }else {
            Ok(Some(()))
        }
    }

    fn active_mod(&self) -> crate::interactions::context::ActiveMod {
        SameUserInSameGroup
    }
}