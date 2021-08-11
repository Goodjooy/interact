use std::{collections::HashMap, sync::mpsc::SendError};

use msg_proc::{
    chain::{chain_builder::ChainBuilder, chain_handle::ToMsgHandle},
    send::{body::SendBody, contain::new_group_send},
};
use serde_json::Value;

use crate::{message_cmd_generate, multi_name_key};

use super::{manage::MessageCmd, Channel, InteractManager, Interactor};

struct ShapeCommand;

impl InteractManager for ShapeCommand {
    fn get_priority(&self) -> u8 {
        128
    }

    fn message_analyze(
        &self,
        msg: &Vec<Box<dyn msg_chain::MessageChain>>,
        sender: &Box<dyn msg_proc::Sender>,
    ) -> Option<super::MessageCmd> {
        let chain_handle = msg.to_msg_handle();
        if (chain_handle.is_at_target(964413011)
            || match chain_handle.conbin_plain() {
                Some(ok) => ok.starts_with("#"),
                None => false,
            })
            && sender.get_group_from().is_some()
        {
            let text = chain_handle
                .conbin_plain()
                .unwrap()
                .trim_start_matches('#')
                .trim()
                .to_string();
            let mut cmds = text.split_whitespace();
            let main_cmd = cmds.next()?.to_string();
            let mut side_named = HashMap::new();
            let mut side_unamed = Vec::new();

            for data in cmds {
                let t = data
                    .split_once(':')
                    .or_else(|| data.split_once("："))
                    .or_else(|| data.split_once('='))
                    .or_else(|| data.split_once('-'));
                match t {
                    Some((k, v)) => {
                        side_named.insert(k.to_string(), v.to_string());
                    }
                    None => {
                        side_unamed.push(data.to_string());
                    }
                };
            }

            let res = MessageCmd::new_full(&main_cmd, side_named.iter(), side_unamed.iter());

            Some(res)
        } else {
            print!("no match");
            None
        }
    }
}

struct HelloInteract;

//multi_name_key!(TARGET =>["目标","target"]);

impl HelloInteract {
    fn new() -> Box<dyn Interactor> {
        Box::new(Self)
    }
}

impl Interactor for HelloInteract {
    fn do_interact(
        &self,
        cmd: MessageCmd,
        _msg: Vec<Box<dyn msg_chain::MessageChain>>,
        sender: Box<dyn msg_proc::Sender>,
        channel: &Channel,
    ) -> Result<Option<Box<dyn super::ContextInteractHandle>>, SendError<SendBody>> {
        let v = multi_name_key!["name", "用户名"];

        let n = String::from("None");
        let name = v.load_keys(&cmd).unwrap_or(&n);

        let msg = ChainBuilder::new()
            .text(format!("你好呀！ {}", name))
            .simplify()
            .build();

        let send = new_group_send(&sender, msg, None);

        channel.send(send)?;
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::interact::manager_contain::InteractorManageContainer;
use std::sync::mpsc::channel;

    use msg_proc::Sender;

   

    use super::*;

    struct MockSender;
    impl Sender for MockSender {
        fn get_sender_id(&self) -> &u64 {
            &0
        }

        fn get_group_from(&self) -> Option<&u64> {
            Some(&0)
        }
    }
    #[test]
    fn test_manager_container() {
        let mock_sender: Box<dyn Sender> = Box::new(MockSender);
        let container = InteractorManageContainer::new(
            Box::new(ShapeCommand),
            vec![(multi_name_key!["hello","BBB","你好"], HelloInteract::new())],None
        );
        let msg = ChainBuilder::new().text("#你好 name:我").build();
        let res = container
            .get_manager()
            .message_analyze(&msg, &mock_sender)
            .unwrap();

        let handle = container.get_handle(&res.get_cmd()).unwrap();
        let (se, re) = channel();
        let chan = Channel::new(&se);

        let _res = handle.do_interact(res, msg, mock_sender, &chan);

        let r = re.recv();
        println!("{:?}", &r);
    }
}
