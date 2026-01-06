use tokio_postgres::Client;
use crate::models::user::User;

pub async fn find_user_by_login(
    db: &Client,
    login: &str,
) -> Option<User> {
    let row  = db
        .query_opt(
            "SELECT id, login FROM users WHERE login = $1",
            &[&login],
        )
        .await
        .ok()??;

    Some(User {
        id: row.get(0),
        login: row.get(1),
    })
}
