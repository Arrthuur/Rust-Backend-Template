use aws_sdk_dynamodb::{
    output::{GetItemOutput, PutItemOutput},
    types::SdkError,
};
use serde::{Deserialize, Serialize};

use crate::repository::ddb::{required_item_value, DDBError};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountIdentifier {
    pub id: String,
}

pub fn get_response_to_user<T>(
    response: Result<GetItemOutput, SdkError<T>>,
) -> Result<User, DDBError> {
    match response {
        Ok(output) => match output.item {
            Some(item) => {
                return Ok(User {
                    id: required_item_value("Id", &item)?,
                    name: required_item_value("name", &item)?,
                    title: required_item_value("title", &item)?,
                })
            }
            None => Err(DDBError),
        },
        Err(_) => Err(DDBError),
    }
}

pub fn put_response_to_user<T>(
    response: Result<PutItemOutput, SdkError<T>>,
) -> Result<User, DDBError> {
    match response {
        Ok(output) => match output.attributes {
            Some(item) => {
                return Ok(User {
                    id: required_item_value("Id", &item)?,
                    name: required_item_value("name", &item)?,
                    title: required_item_value("title", &item)?,
                })
            }
            None => Err(DDBError),
        },
        Err(_) => Err(DDBError),
    }
}
