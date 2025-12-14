use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

use chrono::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Buchung {
    pub id: i64,
    pub datum: NaiveDate,
    pub bezeichnung: String,
    pub betrag: f64,
    #[cfg_attr(feature = "server", sqlx(default))]
    pub intervall: Option<BuchungsIntervall>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))] 
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
pub enum BuchungsIntervall {
    Taeglich,
    Woechentlich,
    Monatlich,
    Jaehrlich,
    // Optional: Einmalig, falls das auch ein Typ ist
    Einmalig, 
}

impl fmt::Display for BuchungsIntervall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuchungsIntervall::Taeglich => write!(f, "Täglich"),
            BuchungsIntervall::Woechentlich => write!(f, "Wöchentlich"),
            BuchungsIntervall::Monatlich => write!(f, "Monatlich"),
            BuchungsIntervall::Jaehrlich => write!(f, "Jährlich"),
            BuchungsIntervall::Einmalig => write!(f, "Einmalig"),
        }
    }
}