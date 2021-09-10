use std::{collections::HashMap, sync::Arc};

use super::{
    manage::{InteractManager, MainCmd},
    utils::MultiToOne,
    Interactor,
};

pub struct InteractorManageContainer {
    manager: Box<dyn InteractManager>,
    mid_map: HashMap<String, Arc<MultiToOne>>,
    handles: HashMap<Arc<MultiToOne>, Box<dyn Interactor>>,
    nil_handle: Option<Arc<Box<dyn Interactor>>>,
}

impl InteractorManageContainer {
    pub fn new(
        manager: Box<dyn InteractManager>,
        handles: Vec<(MultiToOne, Box<dyn Interactor>)>,
        nil_handle: Option<Box<dyn Interactor>>,
    ) -> Self {
        // init handle map
        let mut mid = HashMap::new();
        let mut hand = HashMap::new();

        for (key, value) in handles {
            let rc = Arc::new(key);
            let _res = rc
                .all_names()
                .iter()
                .map(|f| match mid.insert(f.to_string(), Arc::clone(&rc)) {
                    Some(_) => panic!("The Key Had Been Used : {}", f),
                    None => (),
                })
                .collect::<Vec<_>>();

            hand.insert(Arc::clone(&rc), value);
        }

        let nil_handle = nil_handle.and_then(|f| Some(Arc::new(f)));

        Self {
            manager,
            mid_map: mid,
            handles: hand,
            nil_handle,
        }
    }
}

impl InteractorManageContainer {
    // using key get target handle
    pub fn get_handle(&self, key: &MainCmd) -> Option<&Box<dyn Interactor>> {
        match key {
            // if main cmd is Nil return nil handle or None
            MainCmd::Nil => match &self.nil_handle {
                Some(hanle) => Some(hanle),
                None => None,
            },
            // else load cmd handle throw its key
            MainCmd::Cmd(key) => {
                let mid = self.mid_map.get(key)?;
                self.handles.get(mid)
            }
        }
    }

    pub fn get_manager(&self) -> &Box<dyn InteractManager> {
        &self.manager
    }
}

pub struct ContainerBuilder {
    data: InteractorManageContainer,
}

impl ContainerBuilder {
    pub fn new(manage: Box<dyn InteractManager>) -> Self {
        Self {
            data: InteractorManageContainer {
                manager: manage,
                mid_map: HashMap::new(),
                handles: HashMap::new(),
                nil_handle: None,
            },
        }
    }
}

impl ContainerBuilder {
    pub fn add_handle(mut self, guider: MultiToOne, handle: Box<dyn Interactor>) -> Self {
        let rc = Arc::new(guider);
        let _res = rc
            .all_names()
            .iter()
            .map(
                |f| match self.data.mid_map.insert(f.to_string(), Arc::clone(&rc)) {
                    Some(_) => panic!("The Key Had Been Used : {}", f),
                    None => (),
                },
            )
            .collect::<Vec<_>>();

        self.data.handles.insert(Arc::clone(&rc), handle);

        self
    }

    pub fn add_nil_handle(mut self, handle: Box<dyn Interactor>) -> Self {
        self.data.nil_handle = Some(Arc::new(handle));
        self
    }

    pub fn build(self) -> InteractorManageContainer {
        self.data
    }
}

#[cfg(test)]
mod test {

    use std::sync::mpsc::channel;

    use msg_proc::chain::chain_builder::ChainBuilder;

    use crate::{
        interact::{
            mocks::{create_data, create_mock_msg_rev},
            utils::Channel,
        },
        interactions::manage::MessageCmd,
    };

    use super::*;

    #[test]
    fn test_no_context_work() {
        let manage = create_data();
        let msg = ChainBuilder::new().text("CMD ABAB").build();

        let (se, rv) = channel();
        let chan = Channel::new(&se);

        let msg = create_mock_msg_rev(msg);

        let res = manage.get_manager().message_analyze(&msg);

        // get cmd success
        assert_eq!(res, Some(MessageCmd::new_main_only(&msg, &"CMD")));

        let cmd = res.unwrap();
        let main_cmd = cmd.get_cmd();

        let res = manage.get_handle(main_cmd);

        // get interactor
        assert_eq!(res.is_none(), false);

        // do interactor
        let interacor = res.unwrap();

        let res = interacor.do_interact(cmd, &msg.chain, &msg.sender, &chan);

        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.is_none(), true);

        println!("{:#?}", rv.recv());
    }

    #[test]
    fn test_context_work() {
        let manage = create_data();
        let msg = ChainBuilder::new().text("CON ABAB").build();

        let (se, rv) = channel();
        let chan = Channel::new(&se);

        let msg = create_mock_msg_rev(msg);

        let res = manage.get_manager().message_analyze(&msg);

        // get cmd success
        assert_eq!(res, Some(MessageCmd::new_main_only(&msg, &"CON")));

        let cmd = res.unwrap();
        let main_cmd = cmd.get_cmd();

        let res = manage.get_handle(main_cmd);

        // get interactor
        assert_eq!(res.is_none(), false);

        // do interactor
        let interacor = res.unwrap();

        let res = interacor.do_interact(cmd, &msg.chain, &msg.sender, &chan);

        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res.is_none(), false);

        println!("{:#?}", rv.recv());

        let mut context = res.unwrap();

        let msg = ChainBuilder::new().text("abab").build();
        let msg = create_mock_msg_rev(msg);

        let res = context.do_follow_interact(&msg.chain, &msg.sender, &chan);

        // context continue
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res, Some(()));

        let msg = ChainBuilder::new().text("end").build();
        let msg = create_mock_msg_rev(msg);

        let res = context.do_follow_interact(&msg.chain, &msg.sender, &chan);

        // context continue
        assert_eq!(res.is_ok(), true);
        let res = res.unwrap();
        assert_eq!(res, None);
    }
}
