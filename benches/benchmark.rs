use benchmark_simple::*;
use coarsetime::*;
use std::time;

fn main() {
    let options = &Options {
        iterations: 250_000,
        warmup_iterations: 25_000,
        min_samples: 5,
        max_samples: 10,
        max_rsd: 1.0,
        ..Default::default()
    };
    bench_coarsetime_now(options);
    bench_coarsetime_recent(options);
    bench_coarsetime_elapsed(options);
    bench_coarsetime_elapsed_since_recent(options);
    bench_stdlib_now(options);
    bench_stdlib_elapsed(options);
}

fn bench_coarsetime_now(options: &Options) {
    let b = Bench::new();
    Instant::update();
    let res = b.run(options, Instant::now);
    println!("coarsetime_now():          {}", res.throughput(1));
}

fn bench_coarsetime_recent(options: &Options) {
    let b = Bench::new();
    Instant::update();
    let res = b.run(options, Instant::recent);
    println!("coarsetime_recent():       {}", res.throughput(1));
}

fn bench_coarsetime_elapsed(options: &Options) {
    let b = Bench::new();
    let ts = Instant::now();
    let res = b.run(options, || ts.elapsed());
    println!("coarsetime_elapsed():      {}", res.throughput(1));
}

fn bench_coarsetime_elapsed_since_recent(options: &Options) {
    let b = Bench::new();
    let ts = Instant::now();
    let res = b.run(options, || ts.elapsed_since_recent());
    println!("coarsetime_since_recent(): {}", res.throughput(1));
}

fn bench_stdlib_now(options: &Options) {
    let b = Bench::new();
    let res = b.run(options, time::Instant::now);
    println!("stdlib_now():              {}", res.throughput(1));
}

fn bench_stdlib_elapsed(options: &Options) {
    let b = Bench::new();
    let ts = time::Instant::now();
    let res = b.run(options, || ts.elapsed());
    println!("stdlib_elapsed():          {}", res.throughput(1));
}
