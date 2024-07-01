use serde::{Deserialize, Serialize};

/// Trait for determining the age of messages.
pub trait Age {
    fn age(&self) -> Seconds;
}

impl<T> Age for T
where
    T: AsRef<Timestamp>,
{
    fn age(&self) -> Seconds {
        self.as_ref().age()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Timestamp(chrono::DateTime<chrono::Utc>);

impl Timestamp {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the [`Seconds`] since this [`Timestamp`] was created.
    pub fn age(&self) -> Seconds {
        let seconds = chrono::Utc::now()
            .signed_duration_since(self.0)
            .num_seconds();
        Seconds(u64::try_from(seconds).unwrap_or(0))
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(chrono::Utc::now())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Seconds(u64);

impl Seconds {
    pub const fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<Seconds> for u64 {
    fn from(Seconds(val): Seconds) -> Self {
        val
    }
}
