use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Benutzer {
    pub id: i64,
    pub benutzername: String,
    pub email: String,
    pub passwort: String,
}