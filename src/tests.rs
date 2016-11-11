use ::{Duration, Instant};
use std::thread::sleep;
use std::time;

#[test]
fn test_basic() {
    let ts = Instant::now();
    let d = Duration::from_secs(2);
    sleep(time::Duration::new(3, 0));
    assert!(ts.elapsed().as_secs() >= 2);
    assert!(ts.elapsed_since_recent() > d);

    let ts = Instant::now();
    sleep(time::Duration::new(1, 0));
    assert_eq!(Instant::recent(), ts);
    Instant::update();
    assert!(Instant::recent() > ts);
}
