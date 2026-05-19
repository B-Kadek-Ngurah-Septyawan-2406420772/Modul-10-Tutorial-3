use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    username: String,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
            .is_ok()
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            username,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if value.trim().is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(value),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let online_count = self.users.len();
        let message_count = self.messages.len();
        let message_list = if self.messages.is_empty() {
            html! {
                <div class="mx-auto mt-24 max-w-lg text-center">
                    <div class="mx-auto flex h-16 w-16 items-center justify-center rounded-2xl bg-cyan-100 text-3xl font-bold text-cyan-700">{"#"}</div>
                    <div class="mt-5 text-2xl font-semibold text-slate-900">{"No messages yet"}</div>
                    <div class="mt-2 text-sm text-slate-500">{"Send the first message and it will appear instantly in every connected browser."}</div>
                </div>
            }
        } else {
            self.messages
                .iter()
                .map(|m| {
                    let user = self.users.iter().find(|u| u.name == m.from);
                    let avatar = user.map(|u| u.avatar.clone()).unwrap_or_else(|| {
                        format!(
                            "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                            m.from
                        )
                    });
                    let is_current = m.from == self.username;
                    html! {
                        <div class={classes!("mb-5", "flex", "w-full", if is_current { "justify-end" } else { "justify-start" })}>
                            <div class={classes!("flex", "max-w-2xl", "items-end", "gap-3", if is_current { "flex-row-reverse" } else { "flex-row" })}>
                                <img class="h-10 w-10 rounded-full bg-white shadow-sm ring-1 ring-slate-200" src={avatar} alt="avatar"/>
                                <div class={classes!(
                                    "rounded-2xl", "px-5", "py-4", "shadow-sm", "ring-1",
                                    if is_current { "rounded-br-sm" } else { "rounded-bl-sm" },
                                    if is_current { "bg-cyan-600" } else { "bg-white" },
                                    if is_current { "text-white" } else { "text-slate-800" },
                                    if is_current { "ring-cyan-700/20" } else { "ring-slate-200" },
                                )}>
                                    <div class={classes!("mb-1", "text-sm", "font-semibold", if is_current { "text-cyan-50" } else { "text-slate-950" })}>
                                        {if is_current { format!("{} (you)", m.from) } else { m.from.clone() }}
                                    </div>
                                    <div class={classes!("text-sm", if is_current { "text-cyan-50" } else { "text-slate-600" })}>
                                        if m.message.ends_with(".gif") {
                                            <img class="mt-3 rounded-xl" src={m.message.clone()}/>
                                        } else {
                                            {m.message.clone()}
                                        }
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Html>()
        };

        html! {
            <div class="flex min-h-screen w-screen bg-slate-50 text-slate-950">
                <aside class="h-screen w-72 flex-none border-r border-slate-800 bg-slate-950 text-white">
                    <div class="border-b border-slate-800 px-5 py-6">
                        <div class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-300">{"YewChat"}</div>
                        <div class="mt-2 text-2xl font-semibold">{"Orbit Room"}</div>
                        <div class="mt-3 flex items-center gap-2 text-sm text-slate-300">
                            <span class="h-2.5 w-2.5 rounded-full bg-emerald-400"></span>
                            <span>{format!("{online_count} online")}</span>
                        </div>
                    </div>
                    <div class="px-5 pb-2 pt-5 text-xs font-semibold uppercase tracking-[0.18em] text-slate-400">{"Active users"}</div>
                    {
                        self.users.iter().map(|u| {
                            let is_current = u.name == self.username;
                            html! {
                                <div class={classes!(
                                    "mx-4", "my-3", "flex", "items-center", "gap-3", "rounded-xl", "border", "p-3",
                                    if is_current { "border-cyan-400/40" } else { "border-white/10" },
                                    if is_current { "bg-cyan-500/15" } else { "bg-white/5" },
                                )}>
                                    <div class="relative">
                                        <img class="h-12 w-12 rounded-full bg-white/10 ring-2 ring-white/10" src={u.avatar.clone()} alt="avatar"/>
                                        <span class="absolute bottom-0 right-0 h-3 w-3 rounded-full bg-emerald-400 ring-2 ring-slate-950"></span>
                                    </div>
                                    <div class="min-w-0 flex-grow">
                                        <div class="flex items-center justify-between gap-2">
                                            <div class="truncate text-sm font-semibold">{u.name.clone()}</div>
                                            if is_current {
                                                <span class="rounded-full bg-cyan-400 px-2 py-0.5 text-[10px] font-bold uppercase text-slate-950">{"You"}</span>
                                            }
                                        </div>
                                        <div class="mt-1 text-xs text-slate-400">{"Ready to chat"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </aside>
                <main class="flex h-screen grow flex-col">
                    <header class="flex h-20 w-full items-center justify-between border-b border-slate-200 bg-white px-8">
                        <div>
                            <div class="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">{"Live broadcast"}</div>
                            <div class="text-2xl font-semibold text-slate-950">{"Chat room"}</div>
                        </div>
                        <div class="flex items-center gap-3">
                            <div class="rounded-full bg-slate-100 px-4 py-2 text-sm font-medium text-slate-600">{format!("{message_count} messages")}</div>
                            <div class="rounded-full bg-emerald-50 px-4 py-2 text-sm font-medium text-emerald-700">{"Connected"}</div>
                        </div>
                    </header>
                    <section class="grow overflow-auto border-b border-slate-200 bg-[radial-gradient(circle_at_top_left,_#ecfeff,_transparent_34%),linear-gradient(180deg,_#ffffff,_#f8fafc)] px-10 py-8">
                        {message_list}
                    </section>
                    <div class="flex h-20 w-full items-center gap-3 bg-white px-8">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Type a message or paste a .gif link" class="block h-12 w-full rounded-2xl bg-slate-100 px-5 text-slate-800 outline-none ring-1 ring-transparent transition focus:bg-white focus:ring-cyan-400" name="message" required=true />
                        <button onclick={submit} class="flex h-12 w-12 items-center justify-center rounded-2xl bg-cyan-600 text-white shadow-sm transition hover:bg-cyan-700">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </main>
            </div>
        }
    }
}
