use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::db::get_db;

// Prüft, ob überhaupt Benutzer existieren
#[server]
pub async fn existiert_benutzer() -> Result<bool, ServerFnError> {
    let db = get_db().await;
    // Zähle alle Einträge in der Benutzer-Tabelle
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM benutzer")
        .fetch_one(db)
        .await
        .unwrap_or(0);

    Ok(count > 0)
}

#[server]
pub async fn speichere_benutzer(benutzername: String, email: String, passwort:String) -> Result<i64, ServerFnError> {
    let db = get_db().await;

    // KORREKTUR: 'betrag' durch 'passwort' ersetzt, da die Tabelle 'benutzer' heißt
    // Hinweis: In einer echten App das Passwort hier unbedingt hashen (z.B. mit argon2)!
    let result = sqlx::query("INSERT INTO benutzer (benutzername, email, passwort) VALUES (?,?,?)")
        .bind(&benutzername)
        .bind(&email)
        .bind(&passwort)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?; // Fehlerbehandlung verbessert
        
    Ok(result.last_insert_rowid())
}