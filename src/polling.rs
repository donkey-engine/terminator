use crate::storage::{GetSubscribers, ServerStorage, ServerSubscriber};
use actix::Addr;

struct PollingHandler {
    server_storage: Addr<ServerStorage>,
}

impl PollingHandler {
    pub fn new(storage: Addr<ServerStorage>) -> PollingHandler {
        PollingHandler {
            server_storage: storage,
        }
    }

    async fn handle_item(&mut self, _item: &ServerSubscriber) {
        // TODO
    }

    pub async fn handle_all(&mut self) {
        if let Ok(result) = self.server_storage.send(GetSubscribers {}).await {
            for server_subscriber in result.subscribers.iter() {
                self.handle_item(server_subscriber).await;
            }
        };
    }
}

pub fn run(storage: Addr<ServerStorage>) -> impl std::future::Future<Output = ()> {
    return async move {
        let mut polling_handler = PollingHandler::new(storage);

        loop {
            polling_handler.handle_all().await;
        }
    };
}
