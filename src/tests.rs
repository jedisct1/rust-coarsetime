#[cfg(all(feature = "nightly", test))]
extern crate test;

use std::thread::sleep;
use std::time;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use super::Updater;
use super::{Clock, Duration, Instant};

#[test]
fn tests() {
    let ts = Instant::now();
    let d = Duration::from_secs(2);
    sleep(time::Duration::new(3, 0));
    let elapsed = ts.elapsed().as_secs();
    println!("Elapsed: {elapsed} secs");
    assert!(elapsed >= 2);
    assert!(elapsed < 100);
    assert!(ts.elapsed_since_recent() > d);

    let ts = Instant::now();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Instant::recent(), ts);
    Instant::update();
    assert!(Instant::recent() > ts);

    let clock_now = Clock::recent_since_epoch();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Clock::recent_since_epoch(), clock_now);
    assert!(Clock::now_since_epoch() > clock_now);

    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    tests_updater();
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn tests_updater() {
    let updater = Updater::new(250)
        .start()
        .expect("Unable to start a background updater");
    let ts = Instant::recent();
    let clock_recent = Clock::recent_since_epoch();
    sleep(time::Duration::new(2, 0));
    assert!(Clock::recent_since_epoch() > clock_recent);
    assert!(Instant::recent() != ts);
    updater.stop().unwrap();
    let ts = Instant::recent();
    let clock_recent = Clock::recent_since_epoch();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Instant::recent(), ts);
    assert_eq!(Clock::recent_since_epoch(), clock_recent);
}
