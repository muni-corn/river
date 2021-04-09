use super::{HistoryList, StatusWindow, TodoList};
use river_core::UserId;
use yew::{
    format::{Nothing, Text},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

pub struct River {
    link: ComponentLink<Self>,
    user_id: UserId,
}

pub enum Msg {
    SendHttpRequest,
    FetchReady,
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
            link,
            user_id: props.user_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if matches!(msg, Msg::SendHttpRequest) {
            self.fetch_text();
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="column">
                <button class="button" onclick=self.link.callback(|_| Msg::SendHttpRequest)>{"Http"}</button>
                <HistoryList user_id=self.user_id />
                <StatusWindow user_id=self.user_id />
                <TodoList user_id=self.user_id />
            </div>
        }
    }
}

impl River {
    fn fetch_text(&mut self) -> FetchTask {
        let callback = self.link.callback(move |res: Response<Text>| {
            log::info!("{:#?}", res);
            Msg::FetchReady
        });

        let request = Request::get("/api/test/15").body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}
