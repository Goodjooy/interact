use std::{
    fmt::Display,
    sync::mpsc::{self, SendError},
};

use msg_proc::send::cmd::CmdWithSendBody;

use super::manage::MessageCmd;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct MultiToOne {
    sign: String,
    names: Vec<String>,
}
impl MultiToOne {
    pub fn new<T: Display>(sign: T, names: &Vec<T>) -> Self {
        Self {
            sign: sign.to_string(),
            names: names.into_iter().map(|d| d.to_string()).collect(),
        }
    }

    pub fn load_keys<'a>(&self, cmd: &'a MessageCmd) -> Option<&'a String> {
        let mut tmp = self
            .names
            .iter()
            .map(|key| cmd.get_map(key))
            .filter(|op| op.is_some())
            .map(|op| op.unwrap());

        tmp.next().or_else(|| cmd.get_map(&self.sign))
    }

    pub fn get_first_key(&self) -> &String {
        &self.sign
    }

    pub fn all_names(&self) -> Vec<&String> {
        let mut t = vec![&self.sign];
        let _res = self.names.iter().inspect(|n| t.push(n)).collect::<Vec<_>>();
        t
    }
}

#[macro_export]
macro_rules! multi_name_key {
    [$s:expr , $($x:expr),*] => {
         MultiToOne::new(
             $s.to_string(),
            &vec![
                $(
                    $x.to_string(),
                )*
            ]
        )
    };
    ($n:ident=>[$s:expr , $($x:expr),*]) => {
        const $n : MultiToOne = crate::interact::MultiToOne::new(
            $s.to_string(),
           &vec![
               $(
                   $x.to_string(),
               )*
           ]
       );
   };
}

pub struct Channel {
    chan: mpsc::Sender<CmdWithSendBody>,
}

impl Channel {
    pub fn send(&self, data: CmdWithSendBody) -> Result<(), SendError<CmdWithSendBody>> {
        self.chan.send(data)?;

        Ok(())
    }
}

impl Channel {
    pub fn new(send: &mpsc::Sender<CmdWithSendBody>) -> Self {
        Self { chan: send.clone() }
    }
}

impl Clone for Channel {
    fn clone(&self) -> Self {
        Self {
            chan: self.chan.clone(),
        }
    }
}
