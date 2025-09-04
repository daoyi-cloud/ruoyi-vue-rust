use crate::config;
use r2d2::Pool;
use redis::{Client, Commands, FromRedisValue, ToRedisArgs};
use std::sync::LazyLock;

type RedisClient = Pool<Client>;

static REDIS: LazyLock<RedisClient> =
    LazyLock::new(|| init().expect("Failed to initialize Redis pool"));

const CONNECTION_TEST_KEY: &str = "connection_test_key";

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
    let pool = Pool::builder().build(client)?;
    let mut conn = pool.get()?;
    let _: () = conn.set(CONNECTION_TEST_KEY, xid::new().to_string())?;
    let val: String = conn.get(CONNECTION_TEST_KEY)?;
    tracing::info!("Redis connected successfully, {CONNECTION_TEST_KEY} = {val}");
    Ok(pool)
}

/// 测试Redis连接
pub fn test_redis() -> anyhow::Result<String> {
    let v: String = get(CONNECTION_TEST_KEY)?;
    tracing::info!("Redis test success...{CONNECTION_TEST_KEY}={v}");
    Ok(v)
}

/// 获取Redis连接池实例
fn get_client() -> &'static RedisClient {
    &REDIS
}

/// 获取Redis中指定键的值
///
/// # 参数
/// * `key` - 要获取的键
///
/// # 返回值
/// 返回键对应的值，如果键不存在则返回错误
#[allow(dead_code)]
pub fn get<K: ToRedisArgs, T: FromRedisValue>(key: K) -> anyhow::Result<T> {
    let mut client = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let result = client.get(key)?;
    Ok(result)
}

/// 设置键值对并指定过期时间
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
#[allow(dead_code)]
pub fn set_ex<K: ToRedisArgs, V: ToRedisArgs>(
    key: K,
    value: V,
    seconds: u64,
) -> anyhow::Result<()> {
    let mut client = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let _: () = client.set_ex(key, value, seconds)?;
    Ok(())
}

/// 设置键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
#[allow(dead_code)]
pub fn set<K: ToRedisArgs, V: ToRedisArgs>(key: K, value: V) -> anyhow::Result<()> {
    let mut client = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let _: () = client.set(key, value)?;
    Ok(())
}

/// 删除指定键
///
/// # 参数
/// * `key` - 要删除的键
#[allow(dead_code)]
pub fn del<K: ToRedisArgs>(key: K) -> anyhow::Result<()> {
    let mut client = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let _: () = client.del(key)?;
    Ok(())
}

/// 检查键是否存在
///
/// # 参数
/// * `key` - 要检查的键
pub fn exists<K: ToRedisArgs>(key: K) -> anyhow::Result<bool> {
    let mut conn = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let result = conn.exists(key)?;
    Ok(result)
}

/// 设置带TTL的键值对
///
/// # 参数
/// * `key` - 键
/// * `value` - 值
/// * `seconds` - 过期时间（秒）
pub fn set_with_expire<K: ToRedisArgs, V: ToRedisArgs>(
    key: K,
    value: V,
    seconds: u64,
) -> anyhow::Result<()> {
    let mut conn = get_client().get().map_err(|e| anyhow::anyhow!(e))?;
    let _: () = conn.set_ex(key, value, seconds)?;
    Ok(())
}

/// 获取Redis的原始连接
///
/// # 返回值
/// 返回一个Redis连接
pub fn raw_connection() -> anyhow::Result<r2d2::PooledConnection<Client>> {
    get_client().get().map_err(|e| anyhow::anyhow!(e))
}
