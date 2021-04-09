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
            <div class="container">
                <div class="columns is-vcentered">
                    <div class="column">
                        <River user_id=10 />
                    </div>
                    <div class="column">
                        <div class="container">
                            <div class="card">
                                <div class="card-header">
                                    <div class="card-header-title">
                                        <span class="title is-2">{"Hello!"}</span>
                                    </div>
                                </div>
                                <div class="card-content">
                                    <p>{"Welcome to River."}</p>
                                    <p>{"Your teammates will appear in this list. If you need to know what they're working on, check in here."}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
