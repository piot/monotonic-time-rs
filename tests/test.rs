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

#[test_log::test]
fn from_lower() {
    let now = Millis::new(0x12345678);
    let lower = now.to_lower();
    let reconstructed = now.from_lower(lower).unwrap();
    assert_eq!(reconstructed, now);
}

#[test_log::test]
fn multiply_duration() {
    let duration = MillisDuration::from_millis(800);

    let scaled_duration = duration * 1.5;

    assert_eq!(scaled_duration, MillisDuration::from_millis(1200));
}

#[test_log::test]
fn multiply_duration_after() {
    let duration = MillisDuration::from_millis(800);

    let scaled_duration = 1.5 * duration;

    assert_eq!(scaled_duration, MillisDuration::from_millis(1200));
}

#[test_log::test]
fn multiply_int_duration() {
    let duration = MillisDuration::from_millis(800);

    let scaled_duration = duration * 2;

    assert_eq!(scaled_duration, MillisDuration::from_millis(1600));
}

#[test_log::test]
fn multiply_duration_int_after() {
    let duration = MillisDuration::from_millis(800);

    let scaled_duration = 4 * duration;

    assert_eq!(scaled_duration, MillisDuration::from_millis(3200));
}

#[test_log::test]
fn diff_duration() {
    let duration = MillisDuration::from_millis(1500);
    let duration_greater = MillisDuration::from_millis(2000);

    let diff = duration_greater - duration;

    assert_eq!(diff, MillisDuration::from_millis(500));
}

#[test_log::test]
fn div_duration() {
    let duration_greater = MillisDuration::from_millis(3000);

    let diff = duration_greater / 30;

    assert_eq!(diff, MillisDuration::from_millis(100));
}

#[test_log::test]
fn sub_assign() {
    let mut duration = MillisDuration::from_millis(3000);
    duration -= MillisDuration::from_millis(100);

    assert_eq!(duration, MillisDuration::from_millis(2900));
}

#[test_log::test]
fn add_durations() {
    let duration = MillisDuration::from_millis(3000);
    let delta = MillisDuration::from_millis(100);

    assert_eq!(duration + delta, MillisDuration::from_millis(3100));
}

#[test_log::test]
fn add_assign_durations() {
    let mut duration = MillisDuration::from_millis(3000);
    let delta = MillisDuration::from_millis(100);
    duration += delta;

    assert_eq!(duration, MillisDuration::from_millis(3100));
}
