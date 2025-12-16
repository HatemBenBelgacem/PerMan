#[cfg(feature = "server")]
use sqlx::{Pool, Postgres, postgres::PgPoolOptions}; // Postgres statt Sqlite
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB : OnceCell<Pool<Postgres>> = OnceCell::const_new(); // Typ auf Postgres ändern

#[cfg(feature = "server")]
async fn db() -> Pool<Postgres> {
  // Verbindung zur Postgres-Datenbank herstellen.
  // Es ist Best Practice, die URL aus einer Umgebungsvariable zu laden.
  // Beispiel URL: "postgres://user:password@localhost/perman_db"
  let database_url = std::env::var("DATABASE_URL")
      .expect("DATABASE_URL muss gesetzt sein");

  let pool = PgPoolOptions::new()
      .max_connections(5) // Optional: Verbindungen begrenzen
      .connect(&database_url)
      .await
      .expect("Konnte Datenbank nicht verbinden");

  // Migrationen ausführen
  sqlx::migrate!("./migrations")
      .run(&pool)
      .await
      .expect("Migration fehlgeschlagen");

  pool
}

#[cfg(feature = "server")]
pub async fn get_db() -> &'static Pool<Postgres> {
  DB.get_or_init(db).await
}