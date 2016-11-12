use instant::*;
use std::io;
use std::sync::Arc;
use std::thread;
use std::time;

#[cfg(feature = "nightly")]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(feature = "nightly"))]
use std::sync::Mutex;

#[cfg(feature = "nightly")]
type Running = AtomicBool;

#[cfg(not(feature = "nightly"))]
type Running = Mutex<bool>;

pub struct Updater {
    period: time::Duration,
    running: Arc<Running>,
    th: Option<thread::JoinHandle<()>>,
}

impl Updater {
    pub fn start(mut self) -> Result<Self, io::Error> {
        let period = self.period;
        let running = self.running.clone();
        _running_store(&running, true);
        let th: thread::JoinHandle<()> =
            try!(thread::Builder::new().name("coarsetime_updater".to_string()).spawn(move || {
                while _running_load(&running) != false {
                    thread::sleep(period);
                    Instant::update();
                }
                ()
            }));
        self.th = Some(th);
        Instant::update();
        Ok(self)
    }

    pub fn stop(mut self) -> Result<(), io::Error> {
        _running_store(&self.running, false);
        self.th.take().expect("updater is not running").join().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "failed to properly stop the updater")
        })
    }

    pub fn new(period_millis: u64) -> Updater {
        Updater {
            period: time::Duration::from_millis(period_millis),
            running: Arc::new(_running_new(false)),
            th: None,
        }
    }
}

#[cfg(feature = "nightly")]
fn _running_new(v: bool) -> Running {
    AtomicBool::new(v)
}

#[cfg(feature = "nightly")]
fn _running_store(running: &Running, v: bool) {
    running.store(v, Ordering::Relaxed)
}

#[cfg(feature = "nightly")]
#[inline]
fn _running_load(running: &Running) -> bool {
    running.load(Ordering::Relaxed)
}

#[cfg(not(feature = "nightly"))]
fn _running_new(v: bool) -> Running {
    Mutex::new(v)
}

#[cfg(not(feature = "nightly"))]
fn _running_store(running: &Running, v: bool) {
    *running.lock().unwrap() = v;
}

#[cfg(not(feature = "nightly"))]
#[inline]
fn _running_load(running: &Running) -> bool {
    *running.lock().unwrap()
}
