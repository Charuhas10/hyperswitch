#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR" ), "/", "README.md"))]

pub mod consts;
pub mod crypto;
pub mod custom_serde;
pub mod errors;
pub mod ext_traits;
pub mod fp_utils;
pub mod pii;
pub mod validation;

/// Date-time utilities.
pub mod date_time {
    use time::{Instant, OffsetDateTime, PrimitiveDateTime};
    /// Struct to represent milliseconds in time sensitive data fields
    #[derive(Debug)]
    pub struct Milliseconds(i32);

    /// Create a new [`PrimitiveDateTime`] with the current date and time in UTC.
    pub fn now() -> PrimitiveDateTime {
        let utc_date_time = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
    }

    /// Convert from OffsetDateTime to PrimitiveDateTime
    pub fn convert_to_pdt(offset_time: OffsetDateTime) -> PrimitiveDateTime {
        PrimitiveDateTime::new(offset_time.date(), offset_time.time())
    }

    /// Return the UNIX timestamp of the current date and time in UTC
    pub fn now_unix_timestamp() -> i64 {
        OffsetDateTime::now_utc().unix_timestamp()
    }

    /// Calculate execution time for a async block in milliseconds
    pub async fn time_it<T, Fut: futures::Future<Output = T>, F: FnOnce() -> Fut>(
        block: F,
    ) -> (T, f64) {
        let start = Instant::now();
        let result = block().await;
        (result, start.elapsed().as_seconds_f64() * 1000f64)
    }
}

/// Generate a nanoid with the given prefix and length
#[inline]
pub fn generate_id(length: usize, prefix: &str) -> String {
    format!("{}_{}", prefix, nanoid::nanoid!(length, &consts::ALPHABETS))
}

/// Generate a nanoid with the given prefix and a default length
#[inline]
pub fn generate_id_with_default_len(prefix: &str) -> String {
    let len = consts::ID_LENGTH;
    format!("{}_{}", prefix, nanoid::nanoid!(len, &consts::ALPHABETS))
}
