use river_core::UserId;
use yew::prelude::*;
use super::{HistoryList, StatusWindow, TodoList};

pub struct River {
    user_id: UserId,
}

pub enum Msg {

}

#[derive(Clone, Properties)]
pub struct Props {
    pub user_id: UserId,
}

impl Component for River {
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
            <div class="column">
                <HistoryList user_id=self.user_id />
                <StatusWindow user_id=self.user_id />
                <TodoList user_id=self.user_id />
            </div>
        }
    }
}
