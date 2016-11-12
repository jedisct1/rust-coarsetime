#[cfg(all(feature = "nightly", test))]
extern crate test;

use ::{Duration, Instant, Updater};
use std::thread::sleep;
use std::time;

#[cfg(feature = "nightly")]
use self::test::Bencher;

#[test]
fn tests() {
    let ts = Instant::now();
    let d = Duration::from_secs(2);
    sleep(time::Duration::new(3, 0));
    let elapsed = ts.elapsed().as_secs();
    println!("Elapsed: {} secs", elapsed);
    assert!(elapsed >= 2);
    assert!(elapsed < 100);
    assert!(ts.elapsed_since_recent() > d);

    let ts = Instant::now();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Instant::recent(), ts);
    Instant::update();
    assert!(Instant::recent() > ts);

    let updater = Updater::new(250).start().unwrap();
    let ts = Instant::recent();
    sleep(time::Duration::new(1, 0));
    assert!(Instant::recent() != ts);
    updater.stop().unwrap();
    let ts = Instant::recent();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Instant::recent(), ts);
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_coarsetime_now(b: &mut Bencher) {
    Instant::update();
    b.iter(|| Instant::now())
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_coarsetime_recent(b: &mut Bencher) {
    Instant::update();
    b.iter(|| Instant::recent())
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_coarsetime_elapsed(b: &mut Bencher) {
    let ts = Instant::now();
    b.iter(|| ts.elapsed())
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_coarsetime_elapsed_since_recent(b: &mut Bencher) {
    let ts = Instant::now();
    b.iter(|| ts.elapsed_since_recent())
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_stdlib_now(b: &mut Bencher) {
    b.iter(|| time::Instant::now())
}

#[cfg(feature = "nightly")]
#[bench]
fn bench_stdlib_elapsed(b: &mut Bencher) {
    let ts = time::Instant::now();
    b.iter(|| ts.elapsed())
}
