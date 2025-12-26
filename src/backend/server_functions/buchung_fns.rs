use dioxus::prelude::*;
use uuid::Uuid;
use crate::backend::models::buchung::{Buchung, BuchungsIntervall, Art};
use chrono::NaiveDate;

#[cfg(feature = "server")]
use crate::backend::{db::get_db};

#[server]
pub async fn speichere_buchung(datum:NaiveDate, bezeichnung: String, betrag:f64, intervall: BuchungsIntervall, art: Art) -> Result<Uuid, ServerFnError> {
    let db = get_db().await;
    let new_id = Uuid::new_v4();

    // KORREKTUR: PostgreSQL nutzt $1, $2... und RETURNING id
    let id: Uuid = sqlx::query_scalar("INSERT INTO buchung (id, datum, bezeichnung, betrag, intervall, art) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(&new_id)
        .bind(&datum)
        .bind(&bezeichnung)
        .bind(&betrag)
        .bind(&intervall)
        .bind(&art)
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(id)
}

#[server]
pub async fn delete_buchung(id:Uuid) -> Result<(), ServerFnError> {
    let db = get_db().await;

    // KORREKTUR: Platzhalter $1 statt ?
    sqlx::query("DELETE FROM buchung WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}


#[server]
pub async fn liste_buchung() -> Result<Vec<Buchung>, ServerFnError> {
    let db = get_db().await;

    // Keine Parameter, Syntax ist ok
    let rows = sqlx::query_as::<_, Buchung>("SELECT id, datum, bezeichnung, betrag, intervall, art FROM buchung WHERE art = 'ausgaben'")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows)
}

#[server]
pub async fn liste_buchung_einahmen() -> Result<Vec<Buchung>, ServerFnError> {
    let db = get_db().await;

    // Keine Parameter, Syntax ist ok
    let rows = sqlx::query_as::<_, Buchung>("SELECT id, datum, bezeichnung, betrag, intervall, art FROM buchung WHERE art = 'einahmen'")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows)
}

#[server]
pub async fn total_buchung() -> Result<f64, ServerFnError> {
    let db = get_db().await;

    let summe: Option<f64> = sqlx::query_scalar("SELECT SUM(betrag) FROM buchung WHERE art = 'ausgaben'")
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(summe.unwrap_or(0.0))
}

#[server]
pub async fn total_buchung_einahmen() -> Result<f64, ServerFnError> {
    let db = get_db().await;

    let summe: Option<f64> = sqlx::query_scalar("SELECT SUM(betrag) FROM buchung WHERE art = 'einahmen'")
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(summe.unwrap_or(0.0))
}