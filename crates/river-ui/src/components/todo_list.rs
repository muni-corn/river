use river_core::UserId;
use yew::prelude::*;

pub struct TodoList {
    user_id: UserId
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub user_id: UserId
}

impl Component for TodoList {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            user_id: props.user_id,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="todo-list">
                <div class="has-text-centered">
                    { "To-do" }
                </div>
            </div>
        }
    }
}
