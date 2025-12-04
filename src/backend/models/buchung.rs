use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Füge das FromRow-Makro nur hinzu, wenn das "server"-Feature aktiv ist
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Buchung {
    pub id: i64,
    pub datum: NaiveDate,
    pub bezeichnung: String,
    pub betrag: f64,
    pub periode: Option<String>,
}