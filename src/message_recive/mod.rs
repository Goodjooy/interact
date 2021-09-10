use std::collections::HashMap;

use serde_json::Value;

pub mod recive_manage;

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct ReciveBody{
    pub syncId:String,
    pub data:HashMap<String,Value>
}

pub static ASYNC_ID:&str="-1";

