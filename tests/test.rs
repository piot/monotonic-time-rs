/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/monotonic-time-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use monotonic_time_rs::{InstantMonotonicClock, Millis, MillisDuration, MonotonicClock};
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

#[test_log::test]
fn add() {
    let mut now = Millis::new(0);
    now += MillisDuration::from_secs(2.0).unwrap();

    assert_eq!(now.absolute_milliseconds(), 2000);
}

#[test_log::test]
#[should_panic(expected = "attempt to add with overflow")]
fn illegal_assign_add() {
    let mut now = Millis::new(u64::MAX);
    now += MillisDuration::from_millis(1);
}

#[test_log::test]
fn assign_sub() {
    let mut now = Millis::new(5000);
    now -= MillisDuration::from_secs(2.0).unwrap();

    assert_eq!(now.absolute_milliseconds(), 3000);
}

#[test_log::test]
fn sub() {
    let now = Millis::new(5000);
    let answer = now - MillisDuration::from_secs(2.0).unwrap();

    assert_eq!(answer.absolute_milliseconds(), 3000);
}

#[test_log::test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn illegal_assign_sub() {
    let mut now = Millis::new(0);
    now -= MillisDuration::from_secs(2.0).unwrap();
}
