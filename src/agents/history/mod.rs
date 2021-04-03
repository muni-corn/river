use crate::{history_item::HistoryItem, HistoryId, UserId};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

struct HistoryAgent {
    event_subscribers: HashMap<HandlerId, Vec<UserId>>,
}

enum Request {
    /// Get all history items, optionally supplying a UserId to restrict the query to only the user
    /// provided.
    GetAllHistory(Option<UserId>),

    /// Get and return a single HistoryItem.
    GetSingle(HistoryId),

    /// Listen for history changes for the user id
    Subscribe(UserId),
}

enum Output {
    AllHistory(Option<UserId>, Vec<HistoryItem>),
    Single(HistoryItem),
}

impl Agent for HistoryAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Result<Output, HistoryAgentError>;

    fn create(_link: AgentLink<Self>) -> Self {
        Self {
            event_subscribers: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {}

    fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
        match input {
            Request::GetAllHistory(id_opt) => (),
            Request::GetSingle(history_id) => (),
            Request::Subscribe(user_id) => {
                if let Some(v) = self.event_subscribers.get_mut(&id) {
                    v.push(user_id);
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.event_subscribers.entry(id).or_insert_with(Vec::new);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.event_subscribers.remove(&id);
    }
}

/// An error, the string being a message
#[derive(Debug)]
pub struct HistoryAgentError(String);

impl Display for HistoryAgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "history agent error: {}", self.0)
    }
}

impl Error for HistoryAgentError {}
