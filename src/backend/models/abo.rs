use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Abo {
    id: Uuid,
    bezeichnung: String,
    beginn: NaiveDate,
    dauer: f64,
    knd_frist:f64,
}