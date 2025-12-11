use dioxus::prelude::*;
use crate::backend::models::periode::Periode;

#[cfg(feature = "server")]
use crate::backend::{db::get_db};

#[server]
pub async fn speichere_periode(bezeichnung:String) -> Result<i64, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query("INSERT INTO periode (bezeichnung) VALUES (?)")
        .bind(&bezeichnung)
        .execute(db)
        .await
        .unwrap();
    Ok(result.last_insert_rowid())
}

#[server]
pub async fn delete_periode(id:i64) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query("DELETE FROM periode WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}


#[server]
pub async fn liste_periode() -> Result<Vec<Periode>, ServerFnError> {
    let db = get_db().await;

    let rows = sqlx::query_as::<_, Periode>("SELECT id, bezeichnung FROM periode")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(rows)
}