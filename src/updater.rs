use instant::*;
use std::io;
use std::sync::Arc;
use std::thread;
use std::time;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Updater {
    period: time::Duration,
    running: Arc<AtomicBool>,
    th: Option<thread::JoinHandle<()>>,
}

impl Updater {
    pub fn start(mut self) -> Result<Self, io::Error> {
        let period = self.period;
        let running = self.running.clone();
        running.store(true, Ordering::Relaxed);
        let th: thread::JoinHandle<()> =
            try!(thread::Builder::new().name("coarsetime_updater".to_string()).spawn(move || {
                while running.load(Ordering::Relaxed) != false {
                    thread::sleep(period);
                    Instant::update();
                }
            }));
        self.th = Some(th);
        Instant::update();
        Ok(self)
    }

    pub fn stop(mut self) -> Result<(), io::Error> {
        self.running.store(false, Ordering::Relaxed);
        self.th.take().expect("updater is not running").join().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "failed to properly stop the updater")
        })
    }

    pub fn new(period_millis: u64) -> Updater {
        Updater {
            period: time::Duration::from_millis(period_millis),
            running: Arc::new(AtomicBool::new(false)),
            th: None,
        }
    }
}
