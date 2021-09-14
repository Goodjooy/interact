//! Mock Unit
//! ***用于单元测试的假对象部分，不可用于项目其他部分***

use msg_chain::MessageChain;
use msg_proc::{MessageRev, Permission, Sender};

use crate::{interact::mocks::{context::MockContextInteractor, interact::MockInteractor}, multi_name_key};

use self::manage::MockManage;
use crate::interact::utils::MultiToOne;

use super::manager_contain::{ContainerBuilder, InteractorManageContainer};
pub mod interact;
pub mod context;
pub mod manage;

pub fn create_data()->InteractorManageContainer{
    ContainerBuilder::new(Box::new(MockManage))
    .add_handle(multi_name_key!["CMD",], Box::new(MockInteractor))
    .add_handle(multi_name_key!["CON",], Box::new(MockContextInteractor))
    .build()
}


pub fn create_mock_msg_rev(msg:Vec<Box<dyn MessageChain>>)->MessageRev{
    MessageRev { msg_type: String::from("GroupMessage"), sender: Box::new(MockSender), chain: msg }
}


pub struct MockSender;

impl Sender for MockSender {
 

    fn get_group_from(&self) -> Option<&u64> {
        Some(&1234567)
    }



    fn get_sender_id(&self) -> &u64 {
        &1141451919
    }
}

