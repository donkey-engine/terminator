use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminatorErrors {
    #[error("Redis error {0}")]
    RedisError(String),
    #[error("Subscriber error {0}")]
    SubscribeError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("RCON error: {0}")]
    RCONError(String),
}
