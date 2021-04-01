use yew::agent::{Agent, Context};

struct HistoryAgent {

}

enum Msg {
    SetUserId(String),
}

enum Input {

}

enum Output {

}

impl Agent for HistoryAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Input;
    type Output = Output;

    fn create(link: yew::worker::AgentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) {
        todo!()
    }

    fn handle_input(&mut self, msg: Self::Input, id: yew::worker::HandlerId) {
        todo!()
    }
}
