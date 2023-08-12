use crate::tokio::sync::RwLock;
use moon::{Lazy, *};
use redis;
pub mod school;
pub mod sql;

static REDISDB: Lazy<RwLock<redis::Client>> =
    Lazy::new(|| RwLock::new(redis::Client::open("redis://127.0.0.1:6379/").unwrap()));

async fn get_connection() -> redis::RedisResult<redis::Connection> {
    let client = REDISDB.read().await;
    client.get_connection()
}
pub async fn get_user<'a, U: Deserialize<'a> + redis::FromRedisValue>(
    auth: &'a str,
) -> redis::RedisResult<U> {
    let mut con = get_connection().await?;
    let user: U = redis::cmd("hget")
        .arg("sessions")
        .arg(auth)
        .query(&mut con)?;
    Ok(user)
}

pub async fn set_user(id: i32, auth_token: &AuthToken) -> redis::RedisResult<()> {
    let client = REDISDB.read().await;
    let mut con = client.get_connection()?;
    let _user: i32 = redis::cmd("hset")
        .arg("sessions")
        .arg(auth_token.clone().into_string())
        .arg(id)
        .query(&mut con)?;
    Ok(())
}

pub async fn del_user(id: i32, auth: String) -> redis::RedisResult<()> {
    let client = REDISDB.read().await;
    let mut con = client.get_connection()?;
    let _user = redis::cmd("hdel")
        .arg("sessions")
        .arg(id)
        .arg(auth)
        .query(&mut con)?;
    Ok(())
}
