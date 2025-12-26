use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "server")]
use sqlx::FromRow;

use chrono::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Buchung {
    pub id: Uuid,
    pub datum: NaiveDate,
    pub bezeichnung: String,
    pub betrag: f64,
    #[cfg_attr(feature = "server", sqlx(default))]
    pub intervall: Option<BuchungsIntervall>,
    #[cfg_attr(feature = "server", sqlx(default))]
    pub art: Option <Art>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
// WICHTIG: Diese Zeile muss hinzugefügt werden, damit Rust den SQL-Typ 'buchungs_intervall' findet
#[cfg_attr(feature = "server", sqlx(type_name = "intervall"))] 
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
pub enum BuchungsIntervall {
    Taeglich,
    Woechentlich,
    Monatlich,
    Jaehrlich,
    Einmalig, 
}

impl fmt::Display for BuchungsIntervall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuchungsIntervall::Taeglich => write!(f, "taeglich"),
            BuchungsIntervall::Woechentlich => write!(f, "woechentlich"),
            BuchungsIntervall::Monatlich => write!(f, "monatlich"),
            BuchungsIntervall::Jaehrlich => write!(f, "jaehrlich"),
            BuchungsIntervall::Einmalig => write!(f, "einmalig"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
// WICHTIG: Diese Zeile muss hinzugefügt werden, da der Typ in der DB 'art_enum' heißt
#[cfg_attr(feature = "server", sqlx(type_name = "art"))]
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
pub enum Art {
    Einahmen,
    Ausgaben,
}

impl fmt::Display for Art {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Art::Ausgaben => write!(f, "ausgaben"),
            Art::Einahmen => write!(f, "einahmen"),
        }
    }
}