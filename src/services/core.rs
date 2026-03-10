use sea_orm::{DatabaseConnection, ActiveModelTrait, Set};
use crate::entity::user;

#[derive(Clone)]
pub struct CoreService {
    db: DatabaseConnection,
}

impl CoreService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: &str, password: &str, token: &str) -> Result<bool, String> {
    println!("Core received register: user {} password {} token {}", username, password, token);

    // Создаем новую запись через ActiveModel
    let new_user = user::ActiveModel {
        login: Set(username.to_owned()),
        password_hash: Set(password.to_owned()), // здесь позже можно хешировать!
        token: Set(token.to_owned()),
        ..Default::default() // id с автоинкрементом
    };

    // Вставляем в БД
    match new_user.insert(&self.db).await {
        Ok(_) => Ok(true),
        Err(e) => {
            eprintln!("Failed to insert user: {}", e);
            Err(format!("Failed to create user: {}", e))
        }
    }
}

    pub async fn login_user(&self, username: &str, password: &str) -> Result<String, String> {
        println!("Core received login: {} / {}", username, password);
        Ok("jwt_token".into())
    }
}