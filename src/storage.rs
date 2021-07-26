// use std::sync::{Arc, Mutex};
use actix::{Actor, Context, Handler, Message, MessageResult, Supervised, SystemService};

#[derive(Clone)]
pub struct ServerSubscriber {
    server_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewSubscriber {
    pub server_id: usize,
}

pub struct SubscribersResponse {
    pub subscribers: Vec<ServerSubscriber>,
}

#[derive(Message)]
#[rtype(result = "SubscribersResponse")]
pub struct GetSubscribers {}

#[derive(Default, Clone)]
pub struct ServerStorage {
    subscribers: Vec<ServerSubscriber>,
}

impl Actor for ServerStorage {
    type Context = Context<Self>;
}

impl Handler<NewSubscriber> for ServerStorage {
    type Result = ();

    fn handle(&mut self, subscriber: NewSubscriber, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Subscriber");
        for item in self.subscribers.iter() {
            if item.server_id == subscriber.server_id {
                return;
            }
        }
        println!("Added");
        self.subscribers.push(ServerSubscriber {
            server_id: subscriber.server_id,
        })
    }
}

impl Handler<GetSubscribers> for ServerStorage {
    type Result = MessageResult<GetSubscribers>;

    fn handle(&mut self, _data: GetSubscribers, _ctx: &mut Context<Self>) -> Self::Result {
        let mut subscribers = Vec::new();
        for item in self.subscribers.iter() {
            subscribers.push(ServerSubscriber {
                server_id: item.server_id.clone(),
            });
        }
        MessageResult(SubscribersResponse { subscribers })
    }
}

impl Supervised for ServerStorage {}

impl SystemService for ServerStorage {}
