use crate::app::serde::deserializer_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1, message = "页码必须大于0"))]
    #[serde(default = "default_page", deserialize_with = "deserializer_number")]
    pub page: u64,
    #[validate(range(min = 1, max = 1000, message = "每页数量必须在1-1000之间"))]
    #[serde(default = "default_size", deserialize_with = "deserializer_number")]
    pub size: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub total: u64,
    pub size: u64,
    pub page: u64,
    pub total_pages: u64,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(size: u64, page: u64, total: u64, items: Vec<T>) -> Self {
        Self {
            total,
            size,
            page,
            total_pages: if size == 0 {
                0
            } else {
                (total + size - 1) / size
            },
            items,
        }
    }

    pub fn from_pagination(pagination: PaginationParams, total: u64, items: Vec<T>) -> Self {
        Self::new(pagination.size, pagination.page, total, items)
    }
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}
fn default_size() -> u64 {
    DEFAULT_SIZE
}
