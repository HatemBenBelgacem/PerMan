use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Füge das FromRow-Makro nur hinzu, wenn das "server"-Feature aktiv ist
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Benutzer {
    pub id: i64,
    pub benutzername: String,
    pub email: String,
    pub passwort: String,
}