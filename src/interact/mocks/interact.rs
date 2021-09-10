use msg_proc::{chain::chain_builder::ChainBuilder, send::contain::new_source_send};

use crate::{
    interact::utils::Channel,
    interactions::{
        context::ContextInteractHandle,
        error::{InteractError, InteractorResult},
        manage::MessageCmd,
        Interactor,
    },
};

pub struct MockInteractor;

impl Interactor for MockInteractor {
    fn do_interact(
        &self,
        cmd: MessageCmd,
        _msg: &Vec<Box<dyn msg_chain::MessageChain>>,
        sender: &Box<dyn msg_proc::Sender>,
        channel: &Channel,
    ) -> InteractorResult<Option<Box<dyn ContextInteractHandle>>> {
        let msg = ChainBuilder::new()
            .textln(format!("Main cmd: {:?}", cmd.get_cmd()))
            .textln(format!("SenderID: {}", sender.get_sender_id()))
            .textln("原始消息:无")
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

        // no context activiate
        Ok(None)
    }
}
