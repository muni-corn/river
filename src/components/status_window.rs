use yew::prelude::*;
use crate::task::*;
use yew_feather::{
    edit_3::Edit3,
    x::X,
    check::Check,
};

pub struct StatusWindow {
    link: ComponentLink<Self>,
    user_id: String,
    state: State,
}

struct State {
    user_status: UserStatus,
    edit_state: EditState,
    edit_value: String,
}

#[derive(Clone)]
enum UserStatus {
    // The user is working on a Task
    Working(Task),

    // The user is taking a break, with a given reason
    Away(String),

    // The user isn't present
    Out,
}

#[derive(Clone)]
enum EditState {
    NotEditing,
    Editing,
}

#[derive(Debug)]
pub enum Msg {
    EditText,
    ConfirmEdits,
    CancelEdits,
    UpdateEditValue(String),
    Nothing,
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
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditText => match &self.state.user_status {
                UserStatus::Working(task) =>  {
                    self.state.edit_state = EditState::Editing;
                    self.state.edit_value = task.title.clone();
                }
                UserStatus::Away(reason) =>  {
                    self.state.edit_state = EditState::Editing;
                    self.state.edit_value = reason.clone();
                }
                _ => ()
            },
            Msg::ConfirmEdits => match self.state.user_status {
                UserStatus::Working(_) => self.update_current_task_name(&self.state.edit_value.clone()),
                UserStatus::Away(_) => self.update_away_reason(&self.state.edit_value.clone()),
                _ => (),
            }
            Msg::CancelEdits => self.state.edit_state = EditState::NotEditing,
            Msg::UpdateEditValue(val) => {
                self.update_edit_value(&val);
                return false
            }
            _ => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let header = || {
            match &self.state.edit_state {
                EditState::NotEditing => html! {
                    <span class="is-clickable px-2" onclick=self.link.callback(|_| Msg::EditText)>
                        <Edit3 size=Some(32) />
                    </span>
                },
                _ => html! {}
            }
        };

        let task_title = || {
            match &self.state.user_status {
                UserStatus::Working(task) => html! { <span class="current-task-title">{&task.title}</span> },
                UserStatus::Away(reason) => html! { <span class="away-reason">{reason}</span> },
                UserStatus::Out => html! { <span class="out">{"Out"}</span> },
            }
        };

        let body = || {
            match &self.state.edit_state {
                EditState::NotEditing => html! {
                    { task_title() }
                },
                EditState::Editing => {
                    let placeholder = match self.state.user_status {
                        UserStatus::Working(_) => "Enter a task name",
                        UserStatus::Away(_) => "Whatcha doin'?",
                        _ => "Enter something?",
                    };

                    html! {
                        <input class="input current-task-title" 
                            value=self.state.edit_value 
                            placeholder=placeholder
                            oninput=self.link.callback(|e: InputData| Msg::UpdateEditValue(e.value)) 
                            onkeypress=self.link.callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::ConfirmEdits } else { Msg::Nothing }
                            }) />
                    }
                },
            }
        };

        let footer = || {
            match &self.state.edit_state {
                EditState::NotEditing => html! { },
                EditState::Editing => html! {
                    <div class="has-text-right">
                        <span onclick=self.link.callback(|_| Msg::CancelEdits) class="is-clickable px-2">
                            <X size=Some(32) />
                        </span>
                        <span onclick=self.link.callback(|_| Msg::ConfirmEdits) class="is-clickable px-2">
                            <Check size=Some(32) />
                        </span>
                    </div>
                },
            }
        };

        html! {
            <div id="status-window">
                <div id="header" class="pb-2">
                    { header() }
                </div>
                <div id="body">
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
        if let UserStatus::Working(mut t) = self.state.user_status.clone() {
            t.title = String::from(new_name);
            // TODO: reflect changes in the server database
            // TODO: add history item
            self.state.user_status = UserStatus::Working(t);
            self.state.edit_state = EditState::NotEditing;
        }
    }

    fn update_away_reason(&mut self, new_reason: &str) {
        if let UserStatus::Away(_) = self.state.user_status.clone() {
            // TODO: reflect changes in the server database
            // TODO: add history item
            self.state.user_status = UserStatus::Away(String::from(new_reason));
            self.state.edit_state = EditState::NotEditing;
        }
    }

    fn update_edit_value(&mut self, val: &str) {
        self.state.edit_value = val.to_string();
    }
}
