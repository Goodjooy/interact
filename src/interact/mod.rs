use std::{collections::HashMap, sync::mpsc};

use msg_chain::MessageChain;
use msg_proc::{
    send::body::{self, MsgSend},
    Sender,
};
use serde_json::Value;

const LOWEST_PRIORITY: u8 = 0;
const HEIGHTEST_PRIORITY: u8 = 255;
trait InteractManager {
    /// 当前manager的消息优先级，数字越大优先级越高
    fn get_priority(&self) -> u8 {
        LOWEST_PRIORITY
    }

    fn message_analyze(
        &self,
        msg: Vec<Box<dyn MessageChain>>,
        sender: Box<dyn Sender>,
    ) -> Option<MesageCmd>;
}

struct MesageCmd {
    main_cmd: String,
    side_named_cmd: HashMap<String, String>,
    side_list_cmd: Vec<String>,
}

struct InteractorManageContainer {
    manager: Box<dyn InteractManager>,
    handles: HashMap<String, Box<dyn Fn() -> dyn Interactor>>,
}

trait Interactor {
    fn do_interact(
        self,
        cmd: MesageCmd,
        msg: Vec<Box<dyn MessageChain>>,
        sender: Box<dyn Sender>,
        channel: &mut mpsc::Sender<Value>,
    ) -> Option<Box<dyn ContextInteractHandle>>;
}

enum ActiveMod {
    SameUserInSameGroup,
    SameUserInAnyGroup,
    AnyUserInSameGroup,
    AnyUserInAnyGroup,
}
trait ContextInteractHandle {
    fn get_sign(&self) -> String;

    fn active_mod(&self) -> ActiveMod {
        ActiveMod::SameUserInSameGroup
    }

    fn do_follow_interact(
        &mut self,
        msg: Vec<Box<dyn MessageChain>>,
        sender: Box<dyn Sender>,
        channel: &mut mpsc::Sender<Value>,
    ) -> Option<()>;
}
