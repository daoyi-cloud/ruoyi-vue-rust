use serde::{Deserialize, Serialize};

pub mod sms_code;
// 定义 KeyValue 结构体，类似 Java 中的 KeyValue 类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}
