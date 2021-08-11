use std::{collections::HashMap, rc::Rc};

use super::{
    manage::{InteractManager, MainCmd},
    utils::MultiToOne,
    Interactor,
};

pub struct InteractorManageContainer {
    manager: Box<dyn InteractManager>,
    mid_map: HashMap<String, Rc<MultiToOne>>,
    handles: HashMap<Rc<MultiToOne>, Box<dyn Interactor>>,
    nil_handle: Option<Box<dyn Interactor>>,
}

impl InteractorManageContainer {
    pub fn new(
        manager: Box<dyn InteractManager>,
        handles: Vec<(MultiToOne, Box<dyn Interactor>)>,
        nil_handle: Option<Box<dyn Interactor>>,
    ) -> Self {
        let mut mid = HashMap::new();
        let mut hand = HashMap::new();

        for (key, value) in handles {
            let rc = Rc::new(key);
            let _res = rc
                .all_names()
                .iter()
                .map(|f| match mid.insert(f.to_string(), Rc::clone(&rc)) {
                    Some(_) => panic!("The Key Had Been Used : {}", f),
                    None => (),
                })
                .collect::<Vec<_>>();

            hand.insert(Rc::clone(&rc), value);
        }

        Self {
            manager,
            mid_map: mid,
            handles: hand,
            nil_handle,
        }
    }
}

impl InteractorManageContainer {
    pub fn get_handle(&self, key: &MainCmd) -> Option<&Box<dyn Interactor>> {
        match key {
            MainCmd::Nil => match &self.nil_handle {
                Some(hanle) => Some(hanle),
                None => None,
            },
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
