use river_core::*;
use serde::{Deserialize, Serialize};

use yew::{prelude::*, services::websocket::*, format::Json};
use yew_feather::{check::Check, chevron_down::ChevronDown, edit_3::Edit3, x::X};

pub struct StatusWindow {
    link: ComponentLink<Self>,
    user_id: String,
    state: State,
    is_dropdown_active: bool,

    ws: Option<WebSocketTask>,
}

struct State {
    user_status: UserStatus,
    edit_state: EditState,
    edit_value: String,
}

#[derive(Clone)]
enum EditState {
    NotEditing,
    Editing(UserStatusCategory),
}

#[derive(Debug)]
pub enum Msg {
    EditText,
    ConfirmEdits,
    CancelEdits,
    UpdateEditValue(String),
    Nothing,
    ToggleDropdown,
    ChangeUserStatus(UserStatusCategory),

    WebSocket(WebSocketAction),
    WebSocketResponseReady(Result<WebSocketResponse, anyhow::Error>),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub user_id: String,
}

impl Component for StatusWindow {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            user_id: props.user_id,
            state: State {
                edit_state: EditState::NotEditing,
                user_status: UserStatus::Working(Task {
                    date_added: chrono::Local::now(),
                    title: String::from("Work on River"),
                    status: TaskStatus::InProgress(0.25),
                }),
                // user_status: UserStatus::Away(String::from("on lunch")),
                edit_value: String::new(),
            },
            is_dropdown_active: false,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditText => match &self.state.user_status {
                UserStatus::Working(task) => {
                    self.state.edit_state = EditState::Editing(UserStatusCategory::Working);
                    self.state.edit_value = task.title.clone();
                }
                UserStatus::Away(reason) => {
                    self.state.edit_state = EditState::Editing(UserStatusCategory::Away);
                    self.state.edit_value = reason.clone();
                }
                UserStatus::Out => {
                    self.state.edit_state = EditState::Editing(UserStatusCategory::Out);
                }
            },
            Msg::ConfirmEdits => {
                if let EditState::Editing(cat) = self.state.edit_state {
                    // meow
                    match cat {
                        UserStatusCategory::Working => {
                            self.update_current_task_name(&self.state.edit_value.clone())
                        }
                        UserStatusCategory::Away => {
                            self.update_away_reason(&self.state.edit_value.clone())
                        }
                        UserStatusCategory::Out => self.update_as_out(),
                    }
                }
            }
            Msg::CancelEdits => self.state.edit_state = EditState::NotEditing,
            Msg::UpdateEditValue(val) => {
                self.update_edit_value(&val);
                return false;
            }
            Msg::ToggleDropdown => self.is_dropdown_active = !self.is_dropdown_active,
            Msg::ChangeUserStatus(cat) => {
                match cat {
                    // meow
                    UserStatusCategory::Working => {
                        self.state.edit_state = EditState::Editing(UserStatusCategory::Working);
                        self.state.edit_value = String::new();
                    }
                    UserStatusCategory::Away => {
                        self.state.edit_state = EditState::Editing(UserStatusCategory::Away);
                        self.state.edit_value = String::new();
                    }
                    UserStatusCategory::Out => {
                        self.state.edit_state = EditState::Editing(UserStatusCategory::Out);
                    }
                }
                self.is_dropdown_active = false;
            }
            Msg::Nothing => (),
            Msg::WebSocket(action) => self.do_web_socket(action),
            Msg::WebSocketResponseReady(res) => log::info!("{:?}", res),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let header = || {
            let dropdown = |cat: UserStatusCategory| {
                // meow
                let dropdown_item = |cat: UserStatusCategory| {
                    html! {
                        // meow
                        <a class="dropdown-item" onclick=self.link.callback(move |_| Msg::ChangeUserStatus(cat))>{cat.display()}</a>
                    }
                };
                let dropdown_class = || {
                    if self.is_dropdown_active {
                        "dropdown is-active"
                    } else {
                        "dropdown"
                    }
                };
                html! {
                    <div class={ dropdown_class() }>
                        <div class="dropdown-trigger">
                            <button class="button" onclick=self.link.callback(|_| Msg::ToggleDropdown)>
                                <span class="generic-status">{cat.display()}</span>
                                <span class="icon">
                                    <ChevronDown />
                                </span>
                            </button>
                        </div>
                        <div class="dropdown-menu">
                            <div class="dropdown-content">
                                { dropdown_item(UserStatusCategory::Working) }
                                { dropdown_item(UserStatusCategory::Away) }
                                { dropdown_item(UserStatusCategory::Out) }
                            </div>
                        </div>
                    </div>
                }
            };

            match &self.state.edit_state {
                EditState::NotEditing => html! {
                    <div class="level">
                        <div class="level-left">
                            <span class="generic-status">{UserStatusCategory::from(self.state.user_status.clone()).display()}</span>
                        </div>
                        <div class="level-right">
                            <span class="is-clickable px-2" onclick=self.link.callback(|_| Msg::EditText)>
                                <Edit3 size=Some(32) />
                            </span>
                        </div>
                    </div>
                },
                EditState::Editing(cat) => html! {
                    // meow
                    <div class="level">
                        <div class="level-left">
                            { dropdown(*cat) }
                        </div>
                        <div class="level-right">
                        </div>
                    </div>
                },
                _ => html! {},
            }
        };

        let task_title = || match &self.state.user_status {
            UserStatus::Working(task) => html! { <span class="subject bold">{&task.title}</span> },
            UserStatus::Away(reason) => html! { <span class="subject">{reason}</span> },
            UserStatus::Out => html! { <span class="subject">{"Out for now"}</span> },
        };

        let body = || {
            match &self.state.edit_state {
                EditState::NotEditing => html! {
                    { task_title() }
                },
                EditState::Editing(cat) => {
                    // meow
                    let placeholder = match cat {
                        UserStatusCategory::Working => "Enter a task name",
                        UserStatusCategory::Away => "Whatcha doin'?",
                        _ => "Enter something?",
                    };

                    match cat {
                        UserStatusCategory::Working | UserStatusCategory::Away => {
                            let class = match cat {
                                UserStatusCategory::Working => "input subject bold",
                                UserStatusCategory::Away => "input subject",
                                _ => "subject",
                            };
                            html! {
                                <input class=class
                                    value=self.state.edit_value
                                    placeholder=placeholder
                                    oninput=self.link.callback(|e: InputData| Msg::UpdateEditValue(e.value))
                                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                                        if e.key() == "Enter" { Msg::ConfirmEdits } else { Msg::Nothing }
                                    }) />
                            }
                        }
                        UserStatusCategory::Out => html! {
                            <span class="subject">{"Out for now"}</span>
                        },
                    }
                }
            }
        };

        let footer = || match &self.state.edit_state {
            EditState::NotEditing => html! {},
            EditState::Editing(_) => html! {
                <div class="has-text-right">
                    <span onclick=self.link.callback(|_| Msg::CancelEdits) class="is-clickable px-2">
                        <X size=Some(32) />
                    </span>
                    <span onclick=self.link.callback(|_| Msg::ConfirmEdits) class="is-clickable px-2">
                        <Check size=Some(32) />
                    </span>
                </div>
            },
        };

        html! {
            <div id="status-window">
                <div id="header" class="pb-2">
                    { header() }
                </div>
                <div id="body">
                    <button class="button" onclick=self.link.callback(|_| Msg::WebSocket(WebSocketAction::Connect))>{"connect"}</button>
                    <button class="button" onclick=self.link.callback(|_| Msg::WebSocket(WebSocketAction::Send(false)))>{"send"}</button>
                    { body() }
                </div>
                <div id="footer" class="pt-2">
                    { footer() }
                </div>
            </div>
        }
    }
}

impl StatusWindow {
    fn update_current_task_name(&mut self, new_name: &str) {
        if let EditState::Editing(UserStatusCategory::Working) = self.state.edit_state {
            if let UserStatus::Working(mut t) = self.state.user_status.clone() {
                t.title = String::from(new_name);
                self.state.user_status = UserStatus::Working(t);
                // TODO: reflect changes in the server database
                // TODO: add history item
                self.state.edit_state = EditState::NotEditing;
            } else {
                self.state.user_status = UserStatus::Working(Task {
                    date_added: chrono::Local::now(),
                    title: new_name.to_string(),
                    status: TaskStatus::InProgress(0.0),
                });
                // Add a new task with the new name
                // TODO: add task in the server database
                // TODO: add history item
                self.state.edit_state = EditState::NotEditing;
            }
        }
    }

    fn update_away_reason(&mut self, new_reason: &str) {
        if let EditState::Editing(UserStatusCategory::Away) = self.state.edit_state {
            self.state.user_status = UserStatus::Away(String::from(new_reason));
            // TODO: reflect changes in the server database
            // TODO: add history item
            self.state.edit_state = EditState::NotEditing;
        }
    }

    fn update_as_out(&mut self) {
        if let EditState::Editing(UserStatusCategory::Out) = self.state.edit_state {
            self.state.user_status = UserStatus::Out;
            // TODO: reflect changes in the server database
            // TODO: add history item
            self.state.edit_state = EditState::NotEditing;
        }
    }

    fn update_edit_value(&mut self, val: &str) {
        self.state.edit_value = val.to_string();
    }

    fn do_web_socket(&mut self, action: WebSocketAction) {
        match action {
            WebSocketAction::Connect => {
                log::info!("connecting");
                let callback = self.link.callback(|Json(data): Json<Result<WsResponse, anyhow::Error>>| Msg::WebSocketResponseReady(data));
                let notification = self.link.batch_callback(|status| match status {
                    WebSocketStatus::Opened => vec![],
                    WebSocketStatus::Closed | WebSocketStatus::Error => {
                        vec![WebSocketAction::Terminated.into()]
                    }
                });

                let task =
                    WebSocketService::connect("ws://localhost:9001/", callback, notification)
                    .unwrap();

                self.ws = Some(task);
                log::info!("done?");
            }
            WebSocketAction::Disconnect => { log::warn!("Disconnecting") }
            WebSocketAction::Send(as_binary) => {
                let request = WebSocketRequest { value: 321 };
                if as_binary {
                    self.ws.as_mut().unwrap().send_binary(Json(&request));
                } else {
                    self.ws.as_mut().unwrap().send(Json(&request));
                }
            }
            WebSocketAction::Terminated => {log::error!("WS connection terminated")}
        }
    }
}

impl From<WebSocketAction> for Msg {
    fn from(action: WebSocketAction) -> Self {
        Msg::WebSocket(action)
    }
}
