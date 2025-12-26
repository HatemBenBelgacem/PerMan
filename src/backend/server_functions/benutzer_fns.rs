use dioxus::prelude::*;
use uuid::Uuid;
#[cfg(feature = "server")]
use crate::backend::db::get_db;

#[server]
pub async fn existiert_benutzer() -> Result<bool, ServerFnError> {
    let db = get_db().await;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM benutzer")
        .fetch_one(db)
        .await
        .unwrap_or(0);

    Ok(count > 0)
}

#[server]
pub async fn speichere_benutzer(benutzername: String, email: String, passwort:String) -> Result<Uuid, ServerFnError> {
    let db = get_db().await;
    let new_id = Uuid::new_v4();

    // KORREKTUR: $1, $2, $3 und RETURNING id
    let id: Uuid = sqlx::query_scalar("INSERT INTO benutzer (id, benutzername, email, passwort) VALUES ($1, $2, $3, $4) RETURNING id")
        .bind(&new_id)
        .bind(&benutzername)
        .bind(&email)
        .bind(&passwort)
        .fetch_one(db) // fetch_one statt execute
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?; 
        
    Ok(id)
}


#[server]
pub async fn login_check(benutzername: String, passwort: String) -> Result<bool, ServerFnError> {
    let db = get_db().await;
    
    // KORREKTUR: Platzhalter $1 und $2
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM benutzer WHERE benutzername = $1 AND passwort = $2")
        .bind(benutzername)
        .bind(passwort)
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(count > 0)
}