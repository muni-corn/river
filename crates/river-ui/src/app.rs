use crate::components::River;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "municorn.river.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
}

pub enum Msg {

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        App {
            link,
            storage,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="column">
                <River user_id=10 />
            </div>
        }
    }
}
