use yew::prelude::*;
use crate::task::*;
use yew_feather::edit_3::Edit3;

pub struct StatusWindow {
    link: ComponentLink<Self>,
    user_id: String,
    state: State,
}

struct State {
    user_status: UserStatus,
    edit_state: EditState,
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
    EditingTaskName(String), // where the String is the new task name
    EditingAwayReason(String), // where the String is the new away reason
}

#[derive(Debug)]
pub enum Msg {
    EditText,
    ConfirmEdits,
    CancelEdits,
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
                // user_status: UserStatus::Working(Task {
                //     date_added: chrono::Local::now(),
                //     title: String::from("Work on River"),
                //     status: TaskStatus::InProgress(0.25),
                // }),
                user_status: UserStatus::Away(String::from("on lunch")),
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("{:?}", msg);
        match msg {
            Msg::EditText => match &self.state.user_status {
                UserStatus::Working(task) =>  {
                    self.state.edit_state = EditState::EditingTaskName(task.title.clone());
                }
                UserStatus::Away(reason) =>  {
                    self.state.edit_state = EditState::EditingAwayReason(reason.to_string());
                }
                _ => ()
            },
            Msg::ConfirmEdits => match self.state.edit_state.clone() {
                EditState::EditingTaskName(value) => self.update_current_task_name(&value),
                EditState::EditingAwayReason(reason) => self.update_away_reason(&reason),
                _ => (),
            }
            Msg::CancelEdits => self.state.edit_state = EditState::NotEditing,
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
                    <div onclick=self.link.callback(|_| Msg::EditText)>
                        <Edit3 />
                    </div>
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
                EditState::EditingTaskName(value) => html! {
                    <input class="current-task-title" value=value placeholder="Enter a task name" />
                },
                EditState::EditingAwayReason(value) => html! {
                    <input class="current-task-title" value=value placeholder="Whatcha doin'?" />
                },
            }
        };

        html! {
            <div id="status-window">
                <div id="status-window-header">
                    { header() }
                </div>
                <div id="status-window-body">
                    { body() }
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
        if let UserStatus::Working(mut t) = self.state.user_status.clone() {
            // TODO: reflect changes in the server database
            // TODO: add history item
            self.state.user_status = UserStatus::Away(String::from(new_reason));
        }
    }
}
