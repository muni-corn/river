use yew::prelude::*;
use super::{HistoryList, StatusWindow, TodoList};

pub struct River {
    
}

pub enum Msg {

}

#[derive(Clone, Properties)]
pub struct Props {

}

impl Component for River {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <HistoryList />
                <StatusWindow />
                <TodoList />
            </>
        }
    }
}
