use crate::models::users::{get_response_to_user, put_response_to_user, AccountIdentifier, User};
use crate::repository::ddb::DDBRepository;

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{
    get, post,
    web::Path,
    web::{Data, Json},
};
use actix_web::{HttpResponse, ResponseError};
use aws_sdk_dynamodb::model::AttributeValue;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserCreationFailure,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserCreationFailure => StatusCode::FAILED_DEPENDENCY,
        }
    }
}

#[get("/accounts/{id}")]
pub async fn get_user(
    ddb_repo: Data<DDBRepository>,
    account_identifier: Path<AccountIdentifier>,
) -> Result<Json<User>, UserError> {
    let client = &ddb_repo.client;

    let output = client
        .get_item()
        .table_name("Users")
        .key("Id", AttributeValue::S(account_identifier.id.to_string()))
        .send()
        .await;

    let user = get_response_to_user(output);
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(UserError::UserNotFound),
    }
}

#[post("/accounts")]
pub async fn create_user(
    ddb_repo: Data<DDBRepository>,
    body: Json<User>,
) -> Result<Json<User>, UserError> {
    println!("{:?}", body);
    let client = ddb_repo.client.clone();

    let request = client
        .put_item()
        .table_name("Users")
        .item("Id", AttributeValue::S(body.id.to_string()))
        .item("name", AttributeValue::S(body.name.to_string()))
        .item("title", AttributeValue::S(body.title.to_string()));

    let user = put_response_to_user(request.send().await);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(UserError::UserCreationFailure),
    }
}
