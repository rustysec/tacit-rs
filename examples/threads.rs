use std::{
    thread::{sleep, spawn},
    time::Duration,
};
use tacit::{JsonFormatter, SimpleConsoleOutput};

fn main() {
    let json_logger = tacit::Logger::<SimpleConsoleOutput, JsonFormatter>::default()
        .with_module_level_filter("threads".into(), log::LevelFilter::Trace)
        .with_explicit_logging();

    tacit::new().with_logger(json_logger).log().unwrap();
    log::info!("logging a thing");
    log::debug!("something to debug");
    log::trace!("something to trace");

    let mut threads = Vec::new();

    for n in 0..10 {
        threads.push(spawn(move || {
            log::info!("Thread #{} has started", n);

            for i in 0..5 {
                sleep(Duration::from_millis(100 * n));
                log::info!("Thread #{}, check in #{}", n, i);
            }
        }));
    }

    log::info!("fetching web data...");
    match reqwest::blocking::get("https://www.rust-lang.org").and_then(|resp| resp.text()) {
        Ok(_) => log::info!("success!"),
        Err(err) => log::error!("error: {}", err),
    }

    log::info!("fetching local web data...");
    match reqwest::blocking::get("https://localhost").and_then(|resp| resp.text()) {
        Ok(_) => log::info!("success!"),
        Err(_err) => log::error!("error: could not connect to localhost"),
    }

    for thread in threads {
        thread.join().unwrap();
    }

    log::info!("well that is all folks")
}
