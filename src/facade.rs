use crate::errors::TerminatorErrors;
use log::error;
use rcon::{AuthRequest, RCONClient, RCONConfig, RCONRequest};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::collections::HashMap;

const REDIS_SERVER_PREFIX: &str = "terminator:servers";

pub struct RedisConfig {
    pub host: String,
    pub port: u32,
}

struct RCONData {
    client: RCONClient,
}

pub struct ServerFacade {
    rcon_connections: HashMap<usize, RCONData>,
    redis: ConnectionManager,
}

impl ServerFacade {
    pub async fn init(redis_config: RedisConfig) -> Result<Self, TerminatorErrors> {
        let redis_client = redis::Client::open(format!(
            "redis://{}:{}",
            redis_config.host, redis_config.port
        ))
        .map_err(|err| {
            error!("Cannot init redis client: {}", err);
            TerminatorErrors::RedisError(format!("Cannot init redis client: {}", err))
        })?;
        let redis_connection =
            redis_client
                .get_tokio_connection_manager()
                .await
                .map_err(|err| {
                    error!("Cannot create redis connection: {}", err);
                    TerminatorErrors::RedisError(format!("Cannot init redis client: {}", err))
                })?;

        Ok(Self {
            rcon_connections: HashMap::new(),
            redis: redis_connection,
        })
    }

    pub async fn execute(
        &mut self,
        server_id: usize,
        command: String,
    ) -> Result<String, TerminatorErrors> {
        if let std::collections::hash_map::Entry::Vacant(_) = self.rcon_connections.entry(server_id)
        {
            // Restore data from redis and reconnect
            let server_data: HashMap<String, String> = self
                .redis
                .hgetall(format!("{}:{}", REDIS_SERVER_PREFIX, server_id))
                .await
                .map_err(|err| {
                    error!("Cannot check key {}", err);
                    TerminatorErrors::RedisError(format!("Cannot check key {}", err))
                })?;

            if server_data.is_empty() {
                error!("There is no subscriber with ID: {}", server_id);
                return Err(TerminatorErrors::SubscribeError(format!(
                    "There is no subscriber with ID: {}",
                    server_id
                )));
            }

            // FIXME исправить unwrap на обработку ошибок
            let server_url = server_data.get("url").unwrap();
            let server_password = server_data.get("password").unwrap();

            let mut rcon_client = RCONClient::new(RCONConfig {
                url: server_url.clone(),
                ..Default::default()
            })
            .map_err(|err| {
                error!("Cannot create RCON client: {}", err);
                TerminatorErrors::RCONError(format!("Cannot create RCON client: {}", err))
            })?;

            let auth_result = rcon_client
                .auth(AuthRequest::new(server_password.clone()))
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
