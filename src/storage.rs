use crate::errors::TerminatorErrors;
use log::error;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::collections::HashMap;

const REDIS_SERVER_PREFIX: &str = "terminator:servers";

pub struct ServerItem {
    pub id: usize,
    pub url: String,
    pub password: String,
}

pub struct RedisStorage {
    connection: ConnectionManager,
}

pub struct RedisConfig {
    pub host: String,
    pub port: u32,
}

impl RedisStorage {
    pub async fn new(config: RedisConfig) -> Result<Self, TerminatorErrors> {
        let client = redis::Client::open(format!("redis://{}:{}", config.host, config.port))
            .map_err(|err| {
                error!("Cannot init redis client: {}", err);
                TerminatorErrors::RedisError(format!("Cannot init redis client: {}", err))
            })?;
        let connection = client.get_tokio_connection_manager().await.map_err(|err| {
            error!("Cannot create redis connection: {}", err);
            TerminatorErrors::RedisError(format!("Cannot init redis client: {}", err))
        })?;

        Ok(Self { connection })
    }

    fn key(server_id: usize) -> String {
        format!("{}:{}", REDIS_SERVER_PREFIX, server_id)
    }

    pub async fn get(&mut self, server_id: usize) -> Result<Option<ServerItem>, TerminatorErrors> {
        let response: HashMap<String, String> = self
            .connection
            .hgetall(Self::key(server_id))
            .await
            .map_err(|err| {
                error!("Cannot check key {}", err);
                TerminatorErrors::RedisError(format!("Cannot check key {}", err))
            })?;

        if response.is_empty() {
            return Ok(None);
        }

        let server_url = response.get("url").ok_or_else(|| {
            error!("Couldn't find 'url' field in Redis");
            TerminatorErrors::ParseError("Couldn't find 'url' field in Redis".to_string())
        })?;
        let server_password = response.get("password").ok_or_else(|| {
            error!("Couldn't find 'password' field in Redis");
            TerminatorErrors::ParseError("Couldn't find 'password' field in Redis".to_string())
        })?;

        Ok(Some(ServerItem {
            id: server_id,
            url: server_url.clone(),
            password: server_password.clone(),
        }))
    }

    pub async fn store(&mut self, server: ServerItem) -> Result<(), TerminatorErrors> {
        self.connection
            .hset_multiple(
                Self::key(server.id),
                &[
                    ("url".to_string(), server.url),
                    ("password".to_string(), server.password),
                ],
            )
            .await
            .map_err(|err| {
                error!("Cannot set key {}", err);
                TerminatorErrors::RedisError(format!("Cannot set key {}", err))
            })?;
        Ok(())
    }
}
