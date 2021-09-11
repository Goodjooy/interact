use msg_proc::chain::chain_handle::ToMsgHandle;

use crate::interactions::manage::{InteractManager, MessageCmd};

pub struct MockManage;

impl InteractManager for MockManage {
    fn get_priority(&self) -> u8 {
        64
    }

    fn message_analyze(&self, msg: &msg_proc::MessageRev) -> Option<MessageCmd> {
        let text = msg.chain.to_msg_handle();
        let main_cmd = text.conbin_plain()?;
        if main_cmd.starts_with("CMD") {
            let msg_cmd = MessageCmd::new_main_only(msg, &"CMD");
            Some(msg_cmd)
        }else if main_cmd.starts_with("CON"){
            Some(MessageCmd::new_main_only(msg, &"CON"))
        }
        
        else {
            None
        }
    }
}
