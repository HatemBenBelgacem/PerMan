use dioxus::prelude::*;



#[cfg(feature = "server")]
use crate::backend::{db::get_db};

#[server]
pub async fn speichere_benutzer(benutzername: String, email: String, passwort:String) -> Result<i64, ServerFnError> {
    let db = get_db().await;

    let rusult = sqlx::query("INSERT INTO benutzer (benutzername, email, betrag) VALUES (?,?,?)")
        .bind(&benutzername)
        .bind(&email)
        .bind(&passwort)
        .execute(db)
        .await
        .unwrap();
    Ok(rusult.last_insert_rowid())
}