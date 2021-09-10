//! # 消息交互器控制组
//! * 根据从`Reciver<MessageRev>` 收到的消息，按照 交互控制器优先级遍历找到第一个匹配的控制器，并进行对应交互

use std::process::Output;
use std::sync::Arc;
use std::sync::mpsc::{self, SendError};
use std::{collections::HashMap, sync::Mutex};

pub mod handle_builder;
pub mod interact_pool;

use msg_proc::{
    chain::chain_builder::ChainBuilder,
    send::{cmd::CmdWithSendBody, contain::new_source_send},
    MessageRev, Sender,
};
use threadpool::ThreadPool;

use crate::interact::error::InteractorResult;
use crate::interact::{
    context::{ActiveMod, ContextInteractHandle},
    manager_contain::InteractorManageContainer,
    utils::Channel,
};

#[derive(Hash, PartialEq, Eq)]
pub struct InteractContext {
    sender_id: u64,
    group_id: Option<u64>,
}

impl InteractContext {
    fn from_sender(sender: &Box<dyn Sender>) -> Self {
        Self {
            sender_id: *sender.get_sender_id(),
            group_id: sender.get_group_from().and_then(|u| Some(*u)),
        }
    }
}

pub struct InteractHandle {
    // 全部的交互管理器
    orded_handles: Vec<InteractorManageContainer>,

    //上下文状态保持
    // 不同模式分别保存
    // 对于每一模式逐一匹配
    user_global_holder: Mutex<HashMap<u64, Box<dyn ContextInteractHandle>>>,
    // 同用户
    user_holder: Mutex<HashMap<InteractContext, Box<dyn ContextInteractHandle>>>,
    // 不同用户不同群，同时只有一个
    global_holder: Mutex<Option<Box<dyn ContextInteractHandle>>>,
    // 不同用户同群聊，一个群一个
    group_holder: Mutex<HashMap<Option<u64>, Box<dyn ContextInteractHandle>>>,
}

impl InteractHandle {
    fn new(orded_handles: Vec<InteractorManageContainer>) -> Self {
        Self {
            orded_handles,
            user_global_holder: Mutex::new(HashMap::new()),
            user_holder: Mutex::new(HashMap::new()),
            global_holder: Mutex::new(None),
            group_holder: Mutex::new(HashMap::new()),
        }
    }

    pub fn action(&self, data: MessageRev, chan: &Channel) -> InteractorResult<()> {
        let msg = data;
        let msg_type = msg.msg_type;
        let sender = msg.sender;
        let message_chain = msg.chain;

        // 全局上下文响应
        {
            let mut holder = self.global_holder.lock().unwrap();
            if holder.is_some() {
                let handle = holder.as_mut().unwrap();

                let res = handle.do_follow_interact(&message_chain, &sender, chan)?;
                if res.is_none() {
                    *holder = None;
                }
                return Ok(());
            }
        }
        // 单人全局上下文响应
        {
            let mut holder = self.user_global_holder.lock().unwrap();
            let key = sender.get_sender_id();
            if let Some(handles) = holder.get_mut(&key) {
                let res = handles.do_follow_interact(&message_chain, &sender, chan)?;
                if res.is_none() {
                    holder.remove(&key);
                }
                return Ok(());
            }
        }

        // 群内全局上下文
        {
            let mut holder = self.group_holder.lock().unwrap();
            let key = sender.get_group_from().and_then(|f| Some(*f));
            if let Some(handle) = holder.get_mut(&key) {
                let res = handle.do_follow_interact(&message_chain, &sender, chan)?;
                if res.is_none() {
                    holder.remove(&key);
                }
                return Ok(());
            }
        }

        // 单人单群上下文
        {
            let mut holder = self.user_holder.lock().unwrap();
            let key = InteractContext::from_sender(&sender);
            if let Some(handle) = holder.get_mut(&key) {
                let res = handle.do_follow_interact(&message_chain, &sender, chan)?;
                if res.is_none() {
                    holder.remove(&key);
                }
                return Ok(());
            }
        }

        // 新建上下文
        {
            for handle_manage in &self.orded_handles {
                if let Some(cmd) = handle_manage
                    .get_manager()
                    .message_analyze(&message_chain, &sender)
                {
                    if let Some(handle) = handle_manage.get_handle(cmd.get_cmd()) {
                        let res = handle.do_interact(cmd, &message_chain, &sender, chan)?;
                        if let Some(context) = res {
                            match context.active_mod() {
                                ActiveMod::SameUserInSameGroup => {
                                    let mut holder = self.user_holder.lock().unwrap();
                                    let key = InteractContext::from_sender(&sender);
                                    if !holder.contains_key(&key) {
                                        holder.insert(key, context);
                                    } else {
                                        context_create_failure(chan, &msg_type, &sender)?;
                                    }
                                }
                                ActiveMod::SameUserInAnyGroup => {
                                    let mut holder = self.user_global_holder.lock().unwrap();
                                    let key = *sender.get_sender_id();
                                    if !holder.contains_key(&key) {
                                        holder.insert(*sender.get_sender_id(), context);
                                    } else {
                                        context_create_failure(chan, &msg_type, &sender)?;
                                    }
                                }
                                ActiveMod::AnyUserInSameGroup => {
                                    let mut holder = self.group_holder.lock().unwrap();
                                    let key = sender.get_group_from().and_then(|f| Some(*f));
                                    if !holder.contains_key(&key) {
                                        holder.insert(key, context);
                                    } else {
                                        context_create_failure(chan, &msg_type, &sender)?;
                                    }
                                }
                                ActiveMod::AnyUserInAnyGroup => {
                                    let mut holder = self.global_holder.lock().unwrap();
                                    if holder.is_none() {
                                        *holder = Some(context);
                                    } else {
                                        context_create_failure(chan, &msg_type, &sender)?;
                                    }
                                }
                            }
                        }
                    } else {
                        // 解析到指令但是没有对应指令，寻找后续
                        continue;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn work_in_thread<I: Iterator<Item = MessageRev>, F: Fn(InteractorResult<()>)>(
        self,
        input: &mut I,
        chan: &Channel,
        pool: &ThreadPool,
        res_handle_chan: mpsc::Sender<InteractorResult<()>>,
    ) {
        let self_data=Arc::new(self);
        while let Some(data) = input.next() {
            let channel = chan.clone();
            let res_chan = res_handle_chan.clone();
            let sdata=Arc::clone(&self_data);
            pool.execute(move || {
                let res = sdata.action(data, &channel);
                res_chan.send(res).expect("failure to send Result info");
            });
        }
    }
}
fn context_create_failure(
    chan: &Channel,
    msg_type: &String,
    sender: &Box<dyn Sender>,
) -> Result<(), SendError<CmdWithSendBody>> {
    let msg = ChainBuilder::new()
        .text_repeat_ln("-", 6)
        .textln("创建上下文交互失败")
        .textln(format!("指令发起者：{}", sender.get_sender_id()))
        .text_repeat_ln("-", 6)
        .simplify()
        .build();

    let res = new_source_send(&msg_type, sender, msg, None).expect("Source Send Failure");
    chan.send(res)
}
