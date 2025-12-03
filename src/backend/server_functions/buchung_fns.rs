use dioxus::prelude::*;
use crate::backend::models::buchung::Buchung;
use chrono::NaiveDate;

#[cfg(feature = "server")]
use crate::backend::{db::get_db};

#[server]
pub async fn speichere_buchung(datum:NaiveDate, bezeichnung: String, betrag:f64) -> Result<i64, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query("INSERT INTO buchung (datum, bezeichnung, betrag) VALUES (?, ?, ?)")
        .bind(&datum)
        .bind(&bezeichnung)
        .bind(&betrag)
        .execute(db)
        .await
        .unwrap();
    Ok(result.last_insert_rowid())
}

#[server]
pub async fn delete_buchung(id:i64) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query("DELETE FROM buchung WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}


#[server]
pub async fn liste_buchung() -> Result<Vec<Buchung>, ServerFnError> {
    let db = get_db().await;

    let rows = sqlx::query_as::<_, Buchung>("SELECT id, datum, bezeichnung, betrag FROM buchung")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows)
}



#[server]
// 1. Rückgabetyp ändern: Wir erwarten eine Zahl (f64), keine Liste von Buchung
pub async fn total_buchung() -> Result<f64, ServerFnError> {
    let db = get_db().await;

    let summe: Option<f64> = sqlx::query_scalar("SELECT SUM(betrag) FROM buchung")
        .fetch_one(db) // fetch_one, weil wir nur ein Ergebnis erwarten
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;


    Ok(summe.unwrap_or(0.0))
}