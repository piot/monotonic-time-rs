pub use num_traits::{Bounded, ToPrimitive};
use crate::MillisDuration;
impl Bounded for MillisDuration {
    fn min_value() -> Self {
        MillisDuration(0)
    }

    fn max_value() -> Self {
        MillisDuration(u64::MAX)
    }
}

impl ToPrimitive for MillisDuration {
    fn to_i64(&self) -> Option<i64> {
        match i64::try_from(self.0) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.0)
    }
}
