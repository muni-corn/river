use yew::prelude::*;
use crate::task::Task;

struct State {

}

enum UserStatus {
    // The user is working on a Task
    Working(Task),

    // The user is taking a break
    Break(String),
    
    // The user is out for the day
    Out,
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
            state: State {}
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="status-window">
                <div id="status-window-header">
                    <i data-feather="clock" />
                </div>
                <span class="current-task-title">{"Work on River"}</span>
            </div>
        }
    }
}
