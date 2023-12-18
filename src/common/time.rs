#[cfg(feature = "postgres")]
pub fn now() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_local()
}

#[cfg(feature = "sqlite")]
pub fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

pub fn unix() -> u64 {
    now().timestamp_millis() as u64
}
