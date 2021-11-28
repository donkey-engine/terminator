use crate::errors::TerminatorErrors;
use crate::storage::RedisStorage;
use log::error;
use rcon::{AuthRequest, RCONClient, RCONConfig, RCONRequest};
use std::collections::HashMap;

struct RCONData {
    client: RCONClient,
}

pub struct ServerFacade {
    rcon_connections: HashMap<usize, RCONData>,
    storage: RedisStorage,
}

impl ServerFacade {
    pub fn init(storage: RedisStorage) -> Self {
        Self {
            rcon_connections: HashMap::new(),
            storage,
        }
    }

    pub async fn execute(
        &mut self,
        server_id: usize,
        command: String,
    ) -> Result<String, TerminatorErrors> {
        if let std::collections::hash_map::Entry::Vacant(_) = self.rcon_connections.entry(server_id)
        {
            // Restore data from storage and reconnect
            let server_data = self.storage.get(server_id).await?.ok_or_else(|| {
                error!("There is no subscriber with ID: {}", server_id);
                TerminatorErrors::SubscribeError(format!(
                    "There is no subscriber with ID: {}",
                    server_id
                ))
            })?;

            let mut rcon_client = RCONClient::new(RCONConfig {
                url: server_data.url.clone(),
                ..Default::default()
            })
            .map_err(|err| {
                error!("Cannot create RCON client: {}", err);
                TerminatorErrors::RCONError(format!("Cannot create RCON client: {}", err))
            })?;

            let auth_result = rcon_client
                .auth(AuthRequest::new(server_data.password))
                .map_err(|err| {
                    error!("Cannot auth with RCON: {}", err);
                    TerminatorErrors::RCONError(format!("Cannot auth with RCON: {}", err))
                })?;

            if !auth_result.is_success() {
                error!("Bad auth credentials");
                return Err(TerminatorErrors::RCONError(String::from(
                    "Bad auth credentials",
                )));
            }

            self.rcon_connections.insert(
                server_id,
                RCONData {
                    client: rcon_client,
                },
            );
        }

        let rcon_data = self.rcon_connections.get_mut(&server_id).unwrap();
        let rcon_client = &mut rcon_data.client;
        let rcon_response = &rcon_client
            .execute(RCONRequest::new(command))
            .map_err(|err| {
                error!("RCON command execute error: {}", err);
                TerminatorErrors::RCONError(format!("RCON command execute error: {}", err))
            })?;

        Ok(rcon_response.body.clone())
    }
}
