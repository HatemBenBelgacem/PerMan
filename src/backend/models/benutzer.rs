use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Benutzer {
    pub id: Uuid,
    pub benutzername: String,
    pub email: String,
    pub passwort: String,
}