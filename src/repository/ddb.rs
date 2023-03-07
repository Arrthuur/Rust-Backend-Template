use std::collections::HashMap;

use aws_sdk_dynamodb::{model::AttributeValue, Client};

pub struct DDBRepository {
    pub client: Client,
}

#[derive(Debug)]
pub struct DDBError;

pub fn required_item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(DDBError),
        Err(DDBError) => Err(DDBError),
    }
}

pub fn item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(DDBError),
        },
        None => Ok(None),
    }
}

impl DDBRepository {
    pub fn init(client: Client) -> DDBRepository {
        DDBRepository { client }
    }
}
