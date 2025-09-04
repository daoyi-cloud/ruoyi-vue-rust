use crate::config;
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs, aio::MultiplexedConnection};
use std::sync::Arc;
use tokio::sync::OnceCell;

type RedisClient = Client;

static REDIS: OnceCell<Arc<RedisClient>> = OnceCell::const_new();

const CONNECTION_TEST_KEY: &str = "connection_test_key";

async fn init() -> anyhow::Result<Arc<RedisClient>> {
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

    // 测试连接
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn
        .set(CONNECTION_TEST_KEY, xid::new().to_string())
        .await?;
    let val: String = conn.get(CONNECTION_TEST_KEY).await?;

    tracing::info!("Redis connected successfully, {CONNECTION_TEST_KEY} = {val}");
    Ok(Arc::new(client))
}

/// 初始化Redis客户端
pub async fn init_redis() -> anyhow::Result<()> {
    REDIS.get_or_try_init(|| init()).await?;
    Ok(())
}

/// 获取Redis客户端实例
fn get_client() -> anyhow::Result<&'static Arc<RedisClient>> {
    REDIS
        .get()
        .ok_or_else(|| anyhow::anyhow!("Redis client not initialized"))
}

/// 测试Redis连接
pub async fn test_redis() -> anyhow::Result<String> {
    let v: String = get(CONNECTION_TEST_KEY).await?;
    tracing::info!("Redis test success...{CONNECTION_TEST_KEY}={v}");
    Ok(v)
}

/// 获取Redis中指定键的值
///
/// # 参数
/// * `key` - 要获取的键
///
/// # 返回值
/// 返回键对应的值，如果键不存在则返回错误
#[allow(dead_code)]
pub async fn get<T: FromRedisValue>(key: &str) -> anyhow::Result<T> {
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let result = conn.get(key).await?;
    Ok(result)
}

/// 设置键值对并指定过期时间
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
#[allow(dead_code)]
pub async fn set_ex<K, V>(key: K, value: V, seconds: u64) -> anyhow::Result<()>
where
    K: ToRedisArgs + Send + Sync + 'static,
    V: ToRedisArgs + Send + Sync + 'static,
{
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

/// 设置键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
#[allow(dead_code)]
pub async fn set<K, V>(key: K, value: V) -> anyhow::Result<()>
where
    K: ToRedisArgs + Send + Sync + 'static,
    V: ToRedisArgs + Send + Sync + 'static,
{
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.set(key, value).await?;
    Ok(())
}

/// 删除指定键
///
/// # 参数
/// * `key` - 要删除的键
#[allow(dead_code)]
pub async fn del<K>(key: K) -> anyhow::Result<()>
where
    K: ToRedisArgs + Send + Sync + 'static,
{
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.del(key).await?;
    Ok(())
}

/// 检查键是否存在
///
/// # 参数
/// * `key` - 要检查的键
pub async fn exists<K>(key: K) -> anyhow::Result<bool>
where
    K: ToRedisArgs + Send + Sync + 'static,
{
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let result = conn.exists(key).await?;
    Ok(result)
}

/// 设置带TTL的键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
pub async fn set_with_expire<K, V>(key: K, value: V, seconds: u64) -> anyhow::Result<()>
where
    K: ToRedisArgs + Send + Sync + 'static,
    V: ToRedisArgs + Send + Sync + 'static,
{
    let client = get_client()?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: () = conn.set_ex(key, value, seconds).await?;
    Ok(())
}

/// 获取Redis的原始异步连接
///
/// # 返回值
/// 返回一个Redis异步连接
pub async fn raw_connection() -> anyhow::Result<MultiplexedConnection> {
    let client = get_client()?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(conn)
}
