use std::{collections::HashMap, fmt::Display};

use msg_proc::MessageRev;

pub const LOWEST_PRIORITY: u8 = 0;
pub const HEIGHTEST_PRIORITY: u8 = 255;
pub trait InteractManager: Send + Sync {
    /// 当前manager的消息优先级，数字越大优先级越高
    fn get_priority(&self) -> u8 {
        128
    }

    fn message_analyze(&self, msg: &MessageRev) -> Option<MessageCmd>;
}
#[derive(Debug,PartialEq, Eq)]
pub enum MainCmd {
    Nil,
    Cmd(String),
}

impl MainCmd {
    pub fn new<T: Display>(cmd: T) -> Self {
        Self::Cmd(cmd.to_string())
    }
}

impl Default for MainCmd {
    fn default() -> Self {
        MainCmd::Nil
    }
}

#[derive(Debug, Default,PartialEq, Eq)]
pub struct MessageCmd {
    msg_type: String,
    main_cmd: MainCmd,
    side_named_cmd: HashMap<String, String>,
    side_list_cmd: Vec<String>,
}

impl MessageCmd {
    pub fn new_empty() -> Self {
        MessageCmd::default()
    }

    pub fn new_main_only<T: Display>(src: &MessageRev, cmd: &T) -> Self {
        MessageCmd {
            msg_type: src.msg_type.clone(),
            main_cmd: MainCmd::new(cmd),
            ..Default::default()
        }
    }

    pub fn new_maped<T, I, K, V>(src: &MessageRev, cmd: &T, side: I) -> Self
    where
        T: Display,
        K: Display,
        V: Display,
        I: Iterator<Item = (K, V)>,
    {
        MessageCmd {
            msg_type: src.msg_type.clone(),
            main_cmd: MainCmd::new(cmd),
            side_named_cmd: side.map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            ..Default::default()
        }
    }

    pub fn new_listed<T, I, V>(src: &MessageRev, cmd: &T, listed: I) -> Self
    where
        T: Display,
        V: Display,
        I: Iterator<Item = V>,
    {
        MessageCmd {
            msg_type: src.msg_type.clone(),
            main_cmd: MainCmd::new(cmd),
            side_named_cmd: HashMap::default(),
            side_list_cmd: listed.map(|v| v.to_string()).collect(),
        }
    }

    pub fn new_full<T, I, K, V, IL, LV>(
        src: &MessageRev,
        cmd: &T,
        side_named: I,
        side_list: IL,
    ) -> Self
    where
        T: Display,
        K: Display,
        V: Display,
        LV: Display,
        IL: Iterator<Item = LV>,
        I: Iterator<Item = (K, V)>,
    {
        MessageCmd {
            msg_type: src.msg_type.clone(),
            main_cmd: MainCmd::new(cmd),
            side_named_cmd: side_named
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            side_list_cmd: side_list.map(|v| v.to_string()).collect(),
        }
    }
}

impl MessageCmd {
    pub fn get_src_type(&self) -> &String {
        &self.msg_type
    }

    pub fn get_map(&self, key: &str) -> Option<&String> {
        self.side_named_cmd.get(key)
    }

    pub fn set_names<T: Display>(&mut self, names: Vec<T>) {
        let data = { self.side_list_cmd.iter().zip(names.iter()) };
        for (v, k) in data {
            self.side_named_cmd.insert(k.to_string(), v.clone());
        }
    }

    pub fn get_cmd(&self) -> &MainCmd {
        &self.main_cmd
    }
}

#[macro_export]
macro_rules! message_cmd_generate {
    ( $m:expr , [$($k:expr => $v:expr),*] , [$($d:expr),*] ) => {
        MessageCmd::new_full(
            &$m,
            vec![
            $( ( $k.to_string(), $v.to_string() ) ),*
        ],
        vec![
            $( $d.to_string() ),*
        ])

    };
}
