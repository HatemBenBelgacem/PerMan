use dioxus::prelude::*;
use chrono::prelude::*;
use crate::backend::models::abo::Abo;

#[cfg(feature = "server")]
use crate::backend::db::get_db;

#[server]
pub async fn speichere_abo(bezeichnung: String, beginn: NaiveDate, dauer: f64, knd_frist:f64) -> Result<i64, ServerFnError>{
    let db = get_db().await;

    let result = sqlx::query("INSERT INTO abo (bezeichnung, beginn, dauer, knd_frist) VALUES (?, ?, ?, ?)")
        .bind(&bezeichnung)
        .bind(&beginn)
        .bind(&dauer)
        .bind(&knd_frist)
        .execute(db)
        .await
        .unwrap();
    Ok(result.last_insert_rowid())
}

#[server]
pub async fn loesche_abo(id:i64) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query("DELETE FROM abo WHERE id= ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn list_abo() -> Result<Vec<Abo>, ServerFnError> {
    let db = get_db().await;

    let rows = sqlx::query_as::<_, Abo>("SELECT id, bezeichnung, beginn, dauer, knd_frist FROM abo")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(rows)
}