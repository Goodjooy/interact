

mod interact;
mod interact_handle;

mod message_recive;
mod message_send;

mod filter;

pub mod interactions {
    pub use crate::interact::Interactor;

    pub mod context {
        pub use crate::interact::context::{ActiveMod, ContextInteractHandle,ALIVE,DEATH};
    }
    pub mod manage {
        pub use crate::interact::manage::*;
        pub use crate::message_cmd_generate;

        pub use crate::interact::manager_contain::*;
    }
    pub mod util{
        pub use crate::interact::utils::*;
    }
    pub mod error {
        pub use crate::interact::error::*;
    }
    pub mod handles {
        pub use crate::interact_handle::handle_builder::InteractHandleBuilder;
        pub use crate::interact_handle::InteractHandle;
    }
}

pub mod communicate {
    pub mod revice {
        pub use crate::message_recive::recive_manage::*;
        pub use crate::message_recive::{ReciveBody, ASYNC_ID};
    }

    pub mod sender {
        pub use crate::message_send::*;
    }
}

#[cfg(feature="unknow")]
mod unknow{}
