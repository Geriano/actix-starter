#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub use sqlite::*;
#[cfg(feature = "sqlite")]
pub type Id = String;
#[cfg(feature = "sqlite")]
pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
pub use postgres::*;
#[cfg(feature = "postgres")]
pub type Id = uuid::Uuid;
#[cfg(feature = "postgres")]
pub type Timestamp = chrono::NaiveDateTime;
