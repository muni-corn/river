use crate::components::River;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "harrisonthorne.river.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    value: String,
    edit_value: String,
}

pub enum Msg {

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State {
            value: "".into(),
            edit_value: "".into(),
        };
        App {
            link,
            storage,
            state,
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
                <River user_id=String::from("asdf") />
            </div>
        }
    }
}
