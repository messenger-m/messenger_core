use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait, ColumnTrait, QueryFilter};
use crate::entity::user;
use bcrypt::verify;

#[derive(Clone)]
pub struct CoreService {
    db: DatabaseConnection,
}

impl CoreService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<bool, String> {
    println!("Core received register: user {} password {}", username, password);

    // Создаем новую запись через ActiveModel
    let new_user = user::ActiveModel {
        login: Set(username.to_owned()),
        password_hash: Set(password.to_owned()),
        ..Default::default()
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

    pub async fn login_user(&self, username: &str, password: &str) -> Result<bool, String> {
        println!("Core received login: {}", username);

        let user = user::Entity::find()
            .filter(user::Column::Login.eq(username))
            .one(&self.db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let user = user.ok_or("User not found".to_string())?;
        println!("user: {}", user.login);

        if !verify(password, &user.password_hash).map_err(|e| e.to_string())? {
            return Err("Invalid password".into());
        }

        Ok(true)
    }
}