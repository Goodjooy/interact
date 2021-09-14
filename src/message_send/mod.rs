use std::{
    sync::mpsc::Receiver,
    thread::{sleep, JoinHandle},
    time::Duration,
};

use msg_proc::send::{body::SendBody, cmd::CmdWithSendBody};

use serde_json::{Value, to_string, to_string_pretty};
use tokio::io::AsyncWriteExt;

mod img_pre_upload;
mod text2img;
pub struct SendHandle {
    auth_key: String,
    port: String,
    chan: Receiver<CmdWithSendBody>,
}

#[derive(serde::Serialize)]
pub struct WSSendBody {
    pub syncId: String,
    pub command: String,
    pub subCommand: Option<String>,
    pub content: SendBody,
}

impl SendHandle {
    pub fn new(verify_key: String, port: String, chan: Receiver<CmdWithSendBody>) -> Self {
        Self {
            auth_key: verify_key,
            port,
            chan,
        }
    }

    pub fn set_verify_code(&mut self, code: &str) {
        self.auth_key = code.to_string();
    }

    pub fn start_http_sender(self) -> JoinHandle<()> {
        let handle = std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Tokio Runtime create failure");
            while let Ok(mut data) = self.chan.recv() {
                data.set_session_key(&self.auth_key);
                
                println!("Get Send Task | Cmd: {} \n body :{}",&(data.cmd).main_cmd, to_string(&data.body).unwrap());

                let url = format!("{}/{}", self.port, &data.cmd.main_cmd);
                runtime.spawn(async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .post(url)
                        .json(&data.body)
                        .send()
                        .await
                        .expect("Send Message Failure");

                    let mut out = tokio::io::stdout();
                    let body=res.json::<Value>().await;
                    let info = format!(
                        "Send Message Success body: [{:?}]\n ",
                        body
                    );
                    out.write_all(info.as_bytes()).await.unwrap();
                });

                // 延时发送
                let delay = (rand::random::<f32>() * 5.0) as u64;
                sleep(Duration::from_secs(delay));
            }
        });
        handle
    }
}
