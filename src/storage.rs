// use std::sync::{Arc, Mutex};
use actix::{Actor, ArbiterService, Context, Handler, Message, Supervised};

// MESSAGES

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewSubscriber {
    pub server_id: usize,
}

struct ServerSubscriber {
    server_id: usize,
}

#[derive(Default)]
pub struct ServerStorate {
    subscribers: Vec<ServerSubscriber>
}

impl Actor for ServerStorate {
    type Context = Context<Self>;
}

impl Handler<NewSubscriber> for ServerStorate {
    type Result = ();

    fn handle(&mut self, subscriber: NewSubscriber, _ctx: &mut Context<Self>) -> Self::Result {
        for item in self.subscribers.iter() {
            if item.server_id == subscriber.server_id {
                return;
            }
        }
        self.subscribers.push(ServerSubscriber {
            server_id: subscriber.server_id,
        })
    }
}

impl Supervised for ServerStorate {}

impl ArbiterService for ServerStorate {}
