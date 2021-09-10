use crate::interact::manager_contain::InteractorManageContainer;

use super::InteractHandle;

pub struct InteractHandleBuilder {
    data: Vec<InteractorManageContainer>,
}

impl InteractHandleBuilder {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn add_manage(mut self, handle: InteractorManageContainer) -> Self {
        self.data.push(handle);
        self
    }

    pub fn build(mut self) -> InteractHandle {
        self.data.sort_by_key(|f| f.get_manager().get_priority());
        InteractHandle::new(self.data)
    }
}
