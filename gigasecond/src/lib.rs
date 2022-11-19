use time::{PrimitiveDateTime as DateTime, Duration};

const GIGASECONDS: i64 = 10_i64.pow(9);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(Duration::seconds(GIGASECONDS))
}
