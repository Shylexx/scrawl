use chrono::{DateTime, Utc};

pub struct Entry {
    title: String,
    content: String,
    complete: bool,
    created_at: DateTime<Utc>,
}
