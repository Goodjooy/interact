use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use msg_proc::recive::{load_recive_data, MessageRev};


use super::{ReciveBody, ASYNC_ID};

pub struct ReciveManage {
    data_recive: Receiver<ReciveBody>,
    msg_sedner: Sender<MessageRev>,
    //TODO: event sender
}

impl ReciveManage {
    // 开始接收消息并发送到指定位置
    pub fn start_recive_data(self) -> JoinHandle<()> {
        std::thread::spawn(move || {
            while let Ok(data) = self.data_recive.recv() {
                if data.syncId == ASYNC_ID {
                    if let Some(msg) = load_recive_data(&data.data) {
                        self.msg_sedner.send(msg).expect("Recive Message Failure");
                    }
                } else {

                }
            }
        })
    }
}

