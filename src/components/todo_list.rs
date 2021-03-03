use yew::prelude::*;

pub struct TodoList {
    
}

pub enum Msg {

}

#[derive(Clone, Properties)]
pub struct Props {

}

impl Component for TodoList {
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
            { "TodoList" }
        }
    }
}
