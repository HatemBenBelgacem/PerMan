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
    #[cfg_attr(feature = "server", sqlx(default))]
    pub art: Option <Art>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))] 
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
#[cfg_attr(feature = "server", sqlx(type_name = "ntervall"))]
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
            BuchungsIntervall::Jaehrlich => write!(f, "Jaehrlich"),
            BuchungsIntervall::Einmalig => write!(f, "einmalig"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))] 
#[cfg_attr(feature = "server", sqlx(rename_all = "lowercase"))]
#[cfg_attr(feature = "server", sqlx(type_name = "art"))]
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