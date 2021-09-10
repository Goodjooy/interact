//! 在线程池中执行任务
//! 线程池中任务
//! 任务会直接注册进入，结果再通过chan发送出来
//!
//!

use std::{process::Output, rc::Rc, sync::{Arc, mpsc::Sender}};

use futures::stream::poll_fn;
use msg_chain::MessageChain;
use threadpool::ThreadPool;

use crate::interact::{
    self, context::ContextInteractHandle, error::InteractorResult, manage::MessageCmd,
    utils::Channel, Interactor,
};
const size: usize = 32;
pub struct InteractHandlePool {
    // 线程池，用于执行交互任务
    pool: threadpool::ThreadPool,
    // 执行结果的发送方向
    
}



