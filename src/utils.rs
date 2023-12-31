#[cfg(feature = "async")]
use redis::aio::ConnectionLike as AConnectionLike;
use std::time::{SystemTime, UNIX_EPOCH};

use redis::{ConnectionLike, FromRedisValue, RedisError, ToRedisArgs};
pub fn call_lua<R, S, T>(
    conn: &mut dyn ConnectionLike,
    script: &str,
    keys: R,
    args: S,
) -> Result<T, RedisError>
where
    R: ToRedisArgs,
    S: ToRedisArgs,
    T: FromRedisValue,
{
    redis::Script::new(script).key(keys).arg(args).invoke(conn)
}

#[cfg(feature = "async")]
pub async fn call_lua_async<C, R, S, T>(
    conn: &mut C,
    script: &str,
    keys: R,
    args: S,
) -> Result<T, RedisError>
where
    C: AConnectionLike,
    R: ToRedisArgs,
    S: ToRedisArgs,
    T: FromRedisValue,
{
    redis::Script::new(script)
        .key(keys)
        .arg(args)
        .invoke_async(conn)
        .await
}

pub fn now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as u64 * 1000u64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as u64;
    ms
}
