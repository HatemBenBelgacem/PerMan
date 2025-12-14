use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Buchung {
    pub id: i64,
    pub datum: NaiveDate,
    pub bezeichnung: String,
    pub betrag: f64,
    #[cfg_attr(feature = "server", sqlx(default))]
    pub intervall: Option<BuchungsIntervall>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Type))] // Macht es kompatibel mit der DB
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
pub enum BuchungsIntervall {
    Taeglich,
    Woechentlich,
    Monatlich,
    Jaehrlich,
    // Optional: Einmalig, falls das auch ein Typ ist
    Einmalig, 
}