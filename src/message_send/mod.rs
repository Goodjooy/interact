use std::{sync::mpsc::Receiver, thread::JoinHandle};

use msg_proc::send::{body::SendBody, cmd::CmdWithSendBody};

use tokio::io::AsyncWriteExt;

pub struct SendHandle {
    auth_key: String,
    port: String,
    chan: Receiver<CmdWithSendBody>,
}
struct BodyTransfrom {}

#[derive(serde::Serialize)]
pub struct WSSendBody {
    pub syncId: String,
    pub command: String,
    pub subCommand: Option<String>,
    pub content: SendBody,
}

impl SendHandle {
    pub fn start_http_sender(self) -> JoinHandle<()> {
        let handle = std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Tokio Runtime create failure");
            while let Ok(mut data) = self.chan.recv() {
                data.set_session_key(&self.auth_key);
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
                    let info = format!("Send Message Success [{}]", res.status());
                    out.write_all(info.as_bytes()).await.unwrap();
                });
            }
        });
        handle
    }
}
