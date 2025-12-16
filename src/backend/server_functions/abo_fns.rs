use dioxus::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;
use crate::backend::models::abo::Abo;

#[cfg(feature = "server")]
use crate::backend::db::get_db;

#[server]
pub async fn speichere_abo(bezeichnung: String, beginn: NaiveDate, dauer: f64, knd_frist:f64) -> Result<Uuid, ServerFnError>{
    let db = get_db().await;
    let new_id = Uuid::new_v4();

    // KORREKTUR: Platzhalter $1, $2, $3, $4, $5 für PostgreSQL
    let result = sqlx::query("INSERT INTO abo (id, bezeichnung, beginn, dauer, knd_frist) VALUES ($1, $2, $3, $4, $5)")
        .bind(&new_id)
        .bind(&bezeichnung)
        .bind(&beginn)
        .bind(&dauer)
        .bind(&knd_frist)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?; // .unwrap() durch Fehlerbehandlung ersetzt
    
    Ok(new_id)
}

#[server]
// KORREKTUR: Typ von 'i64' auf 'Uuid' geändert, da Abos UUIDs als IDs haben
pub async fn loesche_abo(id: Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;

    // KORREKTUR: Platzhalter $1
    sqlx::query("DELETE FROM abo WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn list_abo() -> Result<Vec<Abo>, ServerFnError> {
    let db = get_db().await;

    // Query hat keine Parameter, ist also kompatibel
    let rows = sqlx::query_as::<_, Abo>("SELECT id, bezeichnung, beginn, dauer, knd_frist FROM abo")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(rows)
}