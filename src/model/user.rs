use serde::{Deserialize, Serialize};
use validator::Validate;

///
/// Entity struct, reflect all columns in the table
///
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,

    #[allow(dead_code)]
    pub password: String,
}

///
/// DTO structs, reflect request / response data
///
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(range(min = 10, max = 100))]
    pub age: i32,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 4))]
    pub name: Option<String>,
    #[validate(range(min = 10, max = 100))]
    pub age: Option<i32>,
    #[validate(length(min = 4))]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadUserDto {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
}

impl From<User> for ReadUserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            age: value.age,
            email: value.email,
        }
    }
}
