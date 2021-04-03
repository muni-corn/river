use yew::prelude::*;

use crate::agents::history::{self, *};
use super::{HistoryList, StatusWindow, TodoList};

pub struct River {
    link: ComponentLink<Self>,
    user_id: String,
    history_agent_bridge: Box<dyn Bridge<HistoryAgent>>,
}

pub enum Msg {
    HistoryAgentResponse(Result<history::Response, HistoryAgentError>),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub user_id: String,
}

impl Component for River {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let history_callback = link.callback(Msg::HistoryAgentResponse);
        Self {
            link,
            history_agent_bridge: HistoryAgent::bridge(history_callback),
            user_id: props.user_id
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.history_agent_bridge.send(history::Request::Delete("fakeid".to_string()));
        match msg {
            Msg::HistoryAgentResponse(result) => match result {
                Ok(out) => log::info!("{:?}", out),
                Err(e) => log::error!("{}", e),
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="column">
                <HistoryList user_id=self.user_id.clone() />
                <StatusWindow user_id=self.user_id.clone() />
                <TodoList user_id=self.user_id.clone() />
            </div>
        }
    }
}
