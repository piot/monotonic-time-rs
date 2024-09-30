/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/monotonic-time-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use monotonic_time_rs::{InstantMonotonicClock, MonotonicClock};
use std::{thread::sleep, time::Duration};

#[test_log::test]
fn realtime_instant() {
    let t = InstantMonotonicClock::new();

    let start_time = t.now();
    sleep(Duration::from_millis(1000));
    let end_time = t.now();
    let duration = end_time - start_time;
    assert!(duration > 900.into() && duration < 1100.into());

    let lower = end_time.to_lower();
    let full = end_time.from_lower(lower).expect("expect to work");
    assert_eq!(full, end_time);
}
