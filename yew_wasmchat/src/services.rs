use super::Message;
use log::info;
use stdweb::Value;
use yew::prelude::*;

pub struct PubnubService {
    lib: Option<Value>,
    chat: Option<Value>,
}

impl PubnubService {
    pub fn new(publish_key: &str, subscribe_key: &str) -> Self {
        info!("Creating new instance of pubnub chatengine service");
        let chat_engine = js! {
            let ce = ChatEngineCore.create({
                publishKey: @{publish_key},
                subscribeKey: @{subscribe_key}
            });

            console.log("Chat engine created");

            return ce;
        };

        PubnubService {
            lib: Some(chat_engine),
            chat: None,
        }
    }

    pub fn send_message(&mut self, msg: &str) -> () {
        js! {
            let m = @{msg};
            myChat.emit("message", {
                text: m
            });
        }
    }

    pub fn connect(
        &mut self,
        topic: &str,
        nickname: &str,
        onmessage: Callback<Message>,
        onoffline: Callback<String>,
        ononline: Callback<String>,
    ) -> () {
        let lib = self.lib.as_ref().expect("No pubnub library!");

        let chat_callback = move |text: String, source: String| {
            let msg = Message { text, from: source };
            onmessage.emit(msg);
        };

        let useroffline_callback = move |username: String| {
            onoffline.emit(username);
        };

        let useronline_callback = move |username: String| {
            ononline.emit(username);
        };

        let chat = js! {
            var pn = @{lib};
            var chat_callback = @{chat_callback};
            var online_cb = @{useronline_callback};
            var offline_cb = @{useroffline_callback};
            pn.on("$.ready", function (data) {
                console.log("PubNub Chat Engine ready");
                //  set global var
                me = data.me;
                // create a new ChatEngine
                myChat = new pn.Chat(@{topic});
                myChat.on("$.connected", () => {
                    console.log("chat connected");
                    myChat.on("message", (message) => {
                        chat_callback(message.data.text, message.sender.state.nickName);
                        console.log(
                            "message: " + message.data.text + " from " +
                            message.sender.state.nickName
                        );
                    });

                    myChat.on("$.online.*", (data) => {
                        console.log("online", data.user);
                        online_cb(data.user.state.nickName);
                    });

                    myChat.on("$.offline.*", (data) => {
                        console.log("offline", data.user);
                        offline_cb(data.user.state.nickName);
                    });
                });
            });

            pn.connect(String(new Date().getTime()), {
                nickName: @{nickname}
            });

            console.log("connecting");
            return myChat;
        };

        self.chat = Some(chat)
    }
}
