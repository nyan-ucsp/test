use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
pub fn connect(database_url: String) -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}
