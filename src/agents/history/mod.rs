use crate::{history_item::HistoryItem, HistoryId, UserId};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

pub struct HistoryAgent {
    link: AgentLink<Self>,
    event_subscribers: HashMap<HandlerId, Vec<UserId>>,
}

pub enum Request {
    /// Get all history items, optionally supplying a UserId to restrict the query to only the user
    /// provided.
    GetAllHistory(Option<UserId>),

    /// Get and return a single HistoryItem.
    GetSingle(HistoryId),

    /// Listen for history changes for the user id
    Subscribe(UserId),

    /// Delete a history item for the current user
    Delete(HistoryId),

    /// Hide a history item for the current user
    Hide(HistoryId),

    /// Add a new history item for the current user
    New(HistoryItem),
}

#[derive(Debug)]
pub enum Response {
    AllHistory(Option<UserId>, Vec<HistoryItem>),
    Single(HistoryItem),
    NewId(HistoryId),
    Info(String),
}

impl Agent for HistoryAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Result<Response, HistoryAgentError>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            event_subscribers: HashMap::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.event_subscribers.entry(id).or_insert_with(Vec::new);
    }

    fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
        match input {
            Request::GetAllHistory(id_opt) => if let Some(user_id) = id_opt {
                // TODO: get history of user
                self.link.respond(id, Ok(Response::Info("get_all_user".to_string())))
            } else {
                // TODO: get all history
                self.link.respond(id, Ok(Response::Info("get_all_all".to_string())))
            },
            Request::GetSingle(history_id) => {
                // TODO: get single history item
                self.link.respond(id, Ok(Response::Info("get_single".to_string())))
            },
            Request::Subscribe(user_id) => {
                // TODO: also subscribe via websocket
                self.link.respond(id, Ok(Response::Info("subscribe".to_string())));
                if let Some(v) = self.event_subscribers.get_mut(&id) {
                    v.push(user_id);
                }
            }
            Request::Delete(_) => {
                // TODO: delete a history item
                self.link.respond(id, Ok(Response::Info("deleted".to_string())))
            }
            Request::Hide(_) => {
                // TODO: hide a history item
                self.link.respond(id, Ok(Response::Info("hidden".to_string())))
            }
            Request::New(_) => {
                // TODO: add a new history item to the database
                self.link.respond(id, Ok(Response::Info("new".to_string())))
            }
        }
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
