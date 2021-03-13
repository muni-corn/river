use yew::prelude::*;

enum UserStatus {
    Working(Task),
    Break(String),
    
    // The user isn't working, and it's not because they're taking a break;
    Out,
}

pub struct StatusWindow {
    user_id: String,
}

pub enum Msg {

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
            user_id: props.user_id
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
                <span class="current-task-title">{"Work on River"}</span>
            </div>
        }
    }
}
