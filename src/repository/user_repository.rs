use axum_typed_multipart::async_trait;
use bcrypt;
use sqlx::Error as SqlxError;
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;

use crate::{
    config::pg_database::{PgDatabase, PgDatabaseTrait},
    model::user::{CreateUserDto, UpdateUserDto, User},
};

pub type UserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn create(&self, user: CreateUserDto) -> Result<User, SqlxError>;
    async fn get(&self, id: i32) -> Result<User, SqlxError>;
    #[allow(dead_code)]
    async fn get_by_email(&self, email: String) -> Result<User, SqlxError>;
    async fn get_all(&self) -> Result<Vec<User>, SqlxError>;
    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, SqlxError>;
    async fn delete(&self, id: i32) -> Result<bool, SqlxError>;
}

pub struct UserRepositoryImpl {
    db_conn: Arc<PgDatabase>,
}

impl UserRepositoryImpl {
    pub fn new(conn: Arc<PgDatabase>) -> Self {
        UserRepositoryImpl { db_conn: conn }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepositoryImpl {
    async fn create(&self, user: CreateUserDto) -> Result<User, SqlxError> {
        // Must include 'RETURNING *' to return the new record
        sqlx::query_as(
            r#"INSERT INTO users(name, age, email, password) VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(user.name)
        .bind(user.age)
        .bind(user.email)
        .bind(bcrypt::hash(user.password, 4).unwrap())
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get(&self, id: i32) -> Result<User, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password
            FROM users u
            WHERE u.id = $1"#,
        )
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get_by_email(&self, email: String) -> Result<User, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password
            FROM users u
            WHERE u.email = $1"#,
        )
        .bind(email)
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get_all(&self) -> Result<Vec<User>, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password
            FROM users u"#,
        )
        .fetch_all(self.db_conn.get_pool())
        .await
    }

    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, SqlxError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(name) = user.name {
            separated.push("name = ").push_bind_unseparated(name);
        }
        if let Some(age) = user.age {
            separated.push("age = ").push_bind_unseparated(age);
        }
        if let Some(password) = user.password {
            separated
                .push("password = ")
                .push_bind_unseparated(bcrypt::hash(password, 4).unwrap());
        }

        let query = query_builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" RETURNING *")
            .build_query_as();

        query.fetch_one(self.db_conn.get_pool()).await
    }

    async fn delete(&self, id: i32) -> Result<bool, SqlxError> {
        let user = self.get(id).await?;
        let mut tx = self.db_conn.get_pool().begin().await?;

        sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(user.id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dto_creation() {
        let create_dto = CreateUserDto {
            name: "Test User".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(!create_dto.name.is_empty());
        assert!(create_dto.age > 0);
    }

    #[test]
    fn test_create_user_dto_validation() {
        let valid_dto = CreateUserDto {
            name: "Test User".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert_eq!(valid_dto.name, "Test User");
        assert_eq!(valid_dto.age, 25);
        assert_eq!(valid_dto.email, "test@example.com");
    }

    #[test]
    fn test_update_user_dto_validation() {
        let update_dto = UpdateUserDto {
            name: Some("Updated User".to_string()),
            age: Some(30),
            password: Some("newpassword123".to_string()),
        };

        assert_eq!(update_dto.name, Some("Updated User".to_string()));
        assert_eq!(update_dto.age, Some(30));
    }
}
