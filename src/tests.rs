use ::{Duration, Instant, Updater};
use std::thread::sleep;
use std::time;

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
