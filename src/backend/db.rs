#[cfg(feature = "server")]
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions, Executor}; // SqlitePoolOptions importieren
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB : OnceCell<Pool<Sqlite>> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn db() -> Pool<Sqlite> {
  // KORREKTUR: create_if_missing(true) hinzufügen!
  // Wir nutzen SqliteConnectOptions um sicherzustellen, dass die Datei erstellt wird.
  let options = sqlx::sqlite::SqliteConnectOptions::new()
      .filename("Perman.db")
      .create_if_missing(true);

  let pool = SqlitePoolOptions::new()
      .connect_with(options)
      .await
      .expect("Konnte Datenbank nicht verbinden");

  // Tabelle erstellen
  sqlx::migrate!("./migrations")
      .run(&pool)
      .await
      .expect("Migration fehlgeschlagen");

  pool
}

#[cfg(feature = "server")]
pub async fn get_db() -> &'static Pool<Sqlite> {
  DB.get_or_init(db).await
}