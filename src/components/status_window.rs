use yew::prelude::*;
use crate::task::*;
use yew_feather::edit_3::Edit3;

struct State {
    user_status: UserStatus,
    edit_state: EditState,
}

enum UserStatus {
    // The user is working on a Task
    Working(Task),

    // The user is taking a break, with a given reason
    Away(String),

    // The user isn't present
    Out,
}

enum EditState {
    NotEditing,
    Editing(String), // where the String is the new task name
}

pub struct StatusWindow {
    user_id: String,
    state: State,
}

pub enum Msg {
    EditTaskName,
    ConfirmNewTaskName,
    CancelNewTaskName,
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
            user_id: props.user_id,
            state: State {
                edit_state: EditState::NotEditing,
                user_status: UserStatus::Away(String::from("on lunch")),
                // user_status: UserStatus::Working(Task {
                //     date_added: chrono::Local::now(),
                //     title: String::from("Work on River"),
                //     status: TaskStatus::InProgress(0.25),
                // }),
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let task_title = || {
            match &self.state.user_status {
                UserStatus::Working(task) => html! { <span class="current-task-title">{&task.title}</span> },
                UserStatus::Away(reason) => html! { <span class="away-reason">{reason}</span> },
                UserStatus::Out => html! { <span class="out">{"Out"}</span> },
            }
        };

        html! {
            <div id="status-window">
                <div id="status-window-header">
                    <Edit3 />
                </div>
                { task_title() }
            </div>
        }
    }
}
