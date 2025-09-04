use crate::config;
use r2d2::Pool;
use redis::{Client, Commands, FromRedisValue, ToRedisArgs};
use std::sync::LazyLock;

type RedisClient = Pool<Client>;

static REDIS: LazyLock<RedisClient> = LazyLock::new(|| init().unwrap());
fn init() -> anyhow::Result<Pool<Client>> {
    let redis_config = config::get().redis();
    let host = redis_config.host();
    let port = redis_config.port();
    let db = redis_config.database();
    let passwd = redis_config.password();
    let client = if passwd.is_empty() {
        Client::open(format!("redis://{host}:{port}/{db}"))?
    } else {
        Client::open(format!("redis://:{passwd}@{host}:{port}/{db}"))?
    };
    let pool = r2d2::Pool::builder().build(client)?;
    let mut conn = pool.get()?;
    let _: () = conn.set("CONNECTION_TEST_KEY", xid::new().to_string())?;
    let val: String = conn.get("CONNECTION_TEST_KEY")?;
    tracing::info!("Redis connected successfully, CONNECTION_TEST_KEY = {val}");
    Ok(pool)
}

pub fn test_redis() -> anyhow::Result<String> {
    init()?;
    let v: String = get("CONNECTION_TEST_KEY")?;
    tracing::info!("Redis test success...CONNECTION_TEST_KEY={v}");
    Ok(v)
}

fn get_client() -> &'static RedisClient {
    &REDIS
}

#[allow(dead_code)]
pub fn get<K: ToRedisArgs, T: FromRedisValue>(key: K) -> anyhow::Result<T> {
    let mut client = get_client().get()?;
    let result = client.get(key);
    Ok(result?)
}

#[allow(dead_code)]
pub fn set_ex<K: ToRedisArgs, V: ToRedisArgs>(
    key: K,
    value: V,
    seconds: u64,
) -> anyhow::Result<()> {
    let mut client = get_client().get()?;
    let _: () = client.set_ex(key, value, seconds)?;
    Ok(())
}

#[allow(dead_code)]
pub fn set<K: ToRedisArgs, V: ToRedisArgs>(key: K, value: V) -> anyhow::Result<()> {
    let mut client = get_client().get()?;
    let _: () = client.set(key, value)?;
    Ok(())
}

#[allow(dead_code)]
pub fn del<K: ToRedisArgs>(key: K) -> anyhow::Result<()> {
    let mut client = get_client().get()?;
    let _: () = client.del(key)?;
    Ok(())
}
