use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{MethodRouter, patch},
};
use axum_extra::extract::WithRejection;

use crate::model::user::{CreateUserDto, ReadUserDto, UpdateUserDto};
use crate::service::user_service::UserService;
use crate::util::{
    api_request::ValidatedJsonOrForm,
    api_response::{ApiError, ApiSuccess},
};
use axum::routing::{delete, get, post};

pub fn routes(state: UserService) -> Router {
    Router::new()
        .route(
            "/users",
            MethodRouter::new()
                .merge(get(get_users))
                .merge(post(create_user)),
        )
        .route(
            "/users/{id}",
            MethodRouter::new()
                .merge(get(get_user))
                .merge(patch(update_user))
                .merge(delete(delete_user)),
        )
        .with_state(state)
}

async fn create_user(
    State(service): State<UserService>,
    WithRejection(ValidatedJsonOrForm(user), _): WithRejection<
        ValidatedJsonOrForm<CreateUserDto>,
        ApiError,
    >,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service.create(user).await.map(ApiSuccess::new).map(Json)
}

async fn get_user(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service.get(id).await.map(ApiSuccess::new).map(Json)
}

async fn get_users(
    State(service): State<UserService>,
) -> Result<Json<ApiSuccess<Vec<ReadUserDto>>>, ApiError> {
    service.get_all().await.map(ApiSuccess::new).map(Json)
}

async fn update_user(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
    WithRejection(ValidatedJsonOrForm(user), _): WithRejection<
        ValidatedJsonOrForm<UpdateUserDto>,
        ApiError,
    >,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service
        .update(id, user)
        .await
        .map(ApiSuccess::new)
        .map(Json)
}

async fn delete_user(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<bool>>, ApiError> {
    service.delete(id).await.map(ApiSuccess::new).map(Json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_typed_multipart::async_trait;
    use std::sync::Arc;

    struct MockUserService;

    #[async_trait]
    impl crate::service::user_service::UserServiceTrait for MockUserService {
        async fn create(&self, _user: CreateUserDto) -> Result<ReadUserDto, ApiError> {
            Ok(ReadUserDto {
                id: 1,
                name: "Test User".to_string(),
                age: 25,
                email: "test@example.com".to_string(),
            })
        }

        async fn get(&self, id: u32) -> Result<ReadUserDto, ApiError> {
            if id == 1 {
                Ok(ReadUserDto {
                    id: 1,
                    name: "Test User".to_string(),
                    age: 25,
                    email: "test@example.com".to_string(),
                })
            } else {
                Err(ApiError::new(Some("User not found".to_string()), 404))
            }
        }

        async fn get_all(&self) -> Result<Vec<ReadUserDto>, ApiError> {
            Ok(vec![ReadUserDto {
                id: 1,
                name: "Test User".to_string(),
                age: 25,
                email: "test@example.com".to_string(),
            }])
        }

        async fn delete(&self, _id: u32) -> Result<bool, ApiError> {
            Ok(true)
        }

        async fn update(&self, _id: u32, _user: UpdateUserDto) -> Result<ReadUserDto, ApiError> {
            Ok(ReadUserDto {
                id: 1,
                name: "Updated User".to_string(),
                age: 30,
                email: "updated@example.com".to_string(),
            })
        }
    }

    #[test]
    fn test_routes_creation() {
        let service: UserService = Arc::new(MockUserService);
        let router = routes(service);
        assert!(!format!("{router:?}").is_empty());
    }
}
