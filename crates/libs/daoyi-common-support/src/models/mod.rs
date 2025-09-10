// 定义 KeyValue 结构体，类似 Java 中的 KeyValue 类
#[derive(Debug, Clone)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}
