use axum_typed_multipart::async_trait;
use sqlx::Error as SqlxError;
use std::sync::Arc;

use crate::model::user::{CreateUserDto, ReadUserDto, UpdateUserDto};
use crate::repository::user_repository::UserRepository;
use crate::util::api_response::{ApiError, ServiceError};

pub type UserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserServiceTrait {
    async fn create(&self, user: CreateUserDto) -> Result<ReadUserDto, ApiError>;
    async fn get(&self, id: u32) -> Result<ReadUserDto, ApiError>;
    async fn get_all(&self) -> Result<Vec<ReadUserDto>, ApiError>;
    async fn delete(&self, id: u32) -> Result<bool, ApiError>;
    async fn update(&self, id: u32, user: UpdateUserDto) -> Result<ReadUserDto, ApiError>;
}

pub struct UserServiceImpl {
    user_repository: UserRepository,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepository) -> Self {
        UserServiceImpl { user_repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserServiceImpl {
    async fn create(&self, user: CreateUserDto) -> Result<ReadUserDto, ApiError> {
        self.user_repository
            .create(user)
            .await
            .map(|u| u.into())
            .map_err(|e| {
                match e {
                    SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                    _ => ServiceError::Unknown(e.to_string()),
                }
                .into()
            })
    }

    async fn get(&self, id: u32) -> Result<ReadUserDto, ApiError> {
        self.user_repository
            .get(id as i32)
            .await
            .map(|u| u.into())
            .map_err(|e| {
                match e {
                    SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                    _ => ServiceError::Unknown(e.to_string()),
                }
                .into()
            })
    }

    async fn get_all(&self) -> Result<Vec<ReadUserDto>, ApiError> {
        self.user_repository
            .get_all()
            .await
            .map(|u_ls| u_ls.into_iter().map(|u| u.into()).collect())
            .map_err(|e| {
                match e {
                    SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                    _ => ServiceError::Unknown(e.to_string()),
                }
                .into()
            })
    }

    async fn delete(&self, id: u32) -> Result<bool, ApiError> {
        self.user_repository.delete(id as i32).await.map_err(|e| {
            match e {
                SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                _ => ServiceError::Unknown(e.to_string()),
            }
            .into()
        })
    }

    async fn update(&self, id: u32, user: UpdateUserDto) -> Result<ReadUserDto, ApiError> {
        self.user_repository
            .update(id as i32, user)
            .await
            .map(|u| u.into())
            .map_err(|e| {
                match e {
                    SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                    SqlxError::RowNotFound => {
                        ServiceError::NotFound(format!("User not found with id: {id}"))
                    }
                    _ => ServiceError::Unknown(e.to_string()),
                }
                .into()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::user::User;
    use axum_typed_multipart::async_trait;
    use sqlx::Error as SqlxError;

    struct MockUserRepository;

    #[async_trait]
    impl crate::repository::user_repository::UserRepositoryTrait for MockUserRepository {
        async fn create(&self, user: CreateUserDto) -> Result<User, SqlxError> {
            Ok(User {
                id: 1,
                name: user.name,
                age: user.age,
                email: user.email,
                password: "hashed_password".to_string(),
            })
        }

        async fn get(&self, id: i32) -> Result<User, SqlxError> {
            if id == 1 {
                Ok(User {
                    id: 1,
                    name: "Test User".to_string(),
                    age: 25,
                    email: "test@example.com".to_string(),
                    password: "hashed_password".to_string(),
                })
            } else {
                Err(SqlxError::RowNotFound)
            }
        }

        async fn get_by_email(&self, _email: String) -> Result<User, SqlxError> {
            Ok(User {
                id: 1,
                name: "Test User".to_string(),
                age: 25,
                email: "test@example.com".to_string(),
                password: "hashed_password".to_string(),
            })
        }

        async fn get_all(&self) -> Result<Vec<User>, SqlxError> {
            Ok(vec![User {
                id: 1,
                name: "Test User".to_string(),
                age: 25,
                email: "test@example.com".to_string(),
                password: "hashed_password".to_string(),
            }])
        }

        async fn update(&self, _id: i32, user: UpdateUserDto) -> Result<User, SqlxError> {
            Ok(User {
                id: 1,
                name: user.name.unwrap_or("Test User".to_string()),
                age: user.age.unwrap_or(25),
                email: "test@example.com".to_string(),
                password: "hashed_password".to_string(),
            })
        }

        async fn delete(&self, _id: i32) -> Result<bool, SqlxError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let repo: crate::repository::user_repository::UserRepository = Arc::new(MockUserRepository);
        let service = UserServiceImpl::new(repo);

        let create_dto = CreateUserDto {
            name: "Test User".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = service.create(create_dto).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, 25);
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let repo: crate::repository::user_repository::UserRepository = Arc::new(MockUserRepository);
        let service = UserServiceImpl::new(repo);

        let result = service.get(1).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test User");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let repo: crate::repository::user_repository::UserRepository = Arc::new(MockUserRepository);
        let service = UserServiceImpl::new(repo);

        let result = service.get(999).await;
        assert!(result.is_err());
    }
}
