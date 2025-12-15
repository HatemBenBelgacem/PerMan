use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Abo {
    pub id: Uuid,
    pub bezeichnung: String,
    pub beginn: NaiveDate,
    pub dauer: f64,
    pub knd_frist:f64,
}