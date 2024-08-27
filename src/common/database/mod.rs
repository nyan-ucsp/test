use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

pub mod sqlite_connection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
