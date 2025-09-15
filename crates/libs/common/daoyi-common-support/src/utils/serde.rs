use humantime::parse_duration;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}
pub fn deserializer_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
    D: Deserializer<'de>,
{
    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

// 序列化函数
pub fn serialize_hashmap<S>(
    hashmap: &HashMap<String, String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let json_string = serde_json::to_string(hashmap).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}

// 反序列化函数
pub fn deserialize_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let json_string = String::deserialize(deserializer)?;
    serde_json::from_str(&json_string).map_err(serde::de::Error::custom)
}

// 序列化 Vec<String> 为 JSON 字符串
pub fn serialize_vec_string<S>(vec: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let json_string = serde_json::to_string(vec).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}

// 反序列化 JSON 字符串为 Vec<String>
pub fn deserialize_vec_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let json_string = String::deserialize(deserializer)?;
    serde_json::from_str(&json_string).map_err(serde::de::Error::custom)
}

pub mod datetime_format {
    use sea_orm::prelude::DateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn deserialize_human_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration(&s).map_err(serde::de::Error::custom)
}
