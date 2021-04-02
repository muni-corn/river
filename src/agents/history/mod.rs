use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::{history_item::HistoryItem, HistoryId, UserId};

struct HistoryAgent {
    link: AgentLink<Self>,
    user_id: String,
    data: Vec<HistoryItem>,
}

enum Msg {
    SetUserId(String),
}

enum Request {
    GetAllHistory,
    GetSingle(HistoryId),
    Subscribe(UserId),
}

enum Output {
    AllHistory(Vec<HistoryItem>),
    Single(HistoryItem),
}

impl Agent for HistoryAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Output;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,

        }
    }

    fn update(&mut self, msg: Self::Message) {
        todo!()
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        todo!()
    }
}
