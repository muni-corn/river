use yew::prelude::*;

pub struct TodoList {
    user_id: String
}

pub enum Msg {

}

#[derive(Clone, Properties)]
pub struct Props {
    pub user_id: String
}

impl Component for TodoList {
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
            <div id="todo-list">
                { "TodoList" }
            </div>
        }
    }
}
