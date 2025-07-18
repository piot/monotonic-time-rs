pub use num_traits::{Bounded, ToPrimitive};
use crate::MillisDuration;
impl Bounded for MillisDuration {
    fn min_value() -> Self {
        Self(0)
    }

    fn max_value() -> Self {
        Self(u64::MAX)
    }
}

impl ToPrimitive for MillisDuration {
    fn to_i64(&self) -> Option<i64> {
        i64::try_from(self.0).ok()
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.0)
    }
}
