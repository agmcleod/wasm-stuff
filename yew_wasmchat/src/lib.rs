#![recursion_limit = "512"]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use std::collections::HashSet;

use log::info;
use serde::{Deserialize, Serialize};
use strum;

use yew::{html, html_impl, prelude::*};

pub mod services;

use services::PubnubService;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub text: String,
    pub from: String,
}

pub struct Model {
    alias: String,
    pending_text: String,
    messages: Vec<Message>,
    users: HashSet<String>,
}

#[derive(Debug)]
pub enum Msg {
    SendChat,
    AddMessage(Message),
    Connect,
    EnterName(String),
    UserOffline(String),
    UserOnline(String),
    UpdatePendingText(String),
    NoOp,
}

impl<C> Component<C> for Model
where
    C: AsMut<PubnubService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
        Model {
            alias: "".into(),
            pending_text: "".into(),
            messages: Vec::new(),
            users: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<C, Self>) -> ShouldRender {
        match msg {
            Msg::AddMessage(message) => self.messages.push(message),
            Msg::UserOnline(nick) => {
                info!("Adding User {:?}", nick);
                self.users.insert(nick);
            }
            Msg::UserOffline(nick) => {
                info!("Removing User {:?}", nick);
                self.users.remove(&nick);
            }
            Msg::SendChat => {
                info!("Called Send chat");
                env.as_mut().send_message(&self.pending_text);
                self.pending_text = "".into();
            }
            Msg::Connect => {
                let on_message = env.send_back(|msg| Msg::AddMessage(msg));
                let on_offline = env.send_back(|user| Msg::UserOffline(user));
                let on_online = env.send_back(|user| Msg::UserOnline(user));

                env.as_mut().connect(
                    "chatengine-demochat",
                    &self.alias,
                    on_message,
                    on_offline,
                    on_online,
                );
            }
            Msg::EnterName(n) => self.alias = n,
            Msg::UpdatePendingText(s) => self.pending_text = s,
            Msg::NoOp => {}
        }
        true
    }
}

impl<C> Renderable<C, Model> for Model
where
    C: AsMut<PubnubService> + 'static,
{
    fn view(&self) -> Html<C, Self> {
        html! {
            <div class="wrapper",>
                <div class="chat-text",>
                    <h1>{ "Messages" }</h1><br/>
                    <ul class="message-list",>
                        { for self.messages.iter().enumerate().map(view_message) }
                    </ul>
                </div>
                <div class="users",>
                    <h1>{ "Users" }</h1><br/>
                    <ul class="user-list",>
                        { for self.users.iter().enumerate().map(view_user) }
                    </ul>
                </div>
                <div class="connect",>
                    <input placeholder="Your Name",
                        value=&self.alias,
                        oninput=|e| Msg::EnterName(e.value),>
                    </input>
                    <button onclick=|_| Msg::Connect,>{ "Connect" }</button>
                </div>
                <div class="text-entry",>
                    <input placeholder="Message Text",
                        class="pending-text",
                        value=&self.pending_text,
                        oninput=|e| Msg::UpdatePendingText(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::SendChat } else { Msg::NoOp }
                        },>
                    </input>
                </div>
            </div>
        }
    }
}

fn view_message<C>((_id, message): (usize, &Message)) -> Html<C, Model>
where
    C: AsMut<PubnubService> + 'static,
{
    html! {
        <li>
            <label>
                <span class="sender",>{"["}{&message.from}{"]"}</span>
                <span class="chatmsg",>{&message.text}</span>
            </label>
        </li>
    }
}

fn view_user<C>((_idx, user): (usize, &String)) -> Html<C, Model>
where
    C: AsMut<PubnubService> + 'static,
{
    html! {
        <li>
            <label>{ user }</label>
        </li>
    }
}
