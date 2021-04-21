use tacit::Logger;

fn main() {
    let json_logger = tacit::JsonLogger::default()
        .with_module_level_filter("explicit".into(), log::LevelFilter::Trace)
        .with_explicit_logging();

    tacit::new().with_logger(json_logger).log().unwrap();
    log::info!("logging a thing");
    log::debug!("something to debug");
    log::trace!("something to trace");

    log::info!("fetching web data...");
    match reqwest::blocking::get("https://www.rust-lang.org").and_then(|resp| resp.text()) {
        Ok(_) => log::info!("success!"),
        Err(err) => log::error!("error: {}", err),
    }

    log::info!("fetching local web data...");
    match reqwest::blocking::get("https://localhost").and_then(|resp| resp.text()) {
        Ok(_) => log::info!("success!"),
        Err(err) => log::error!("error: {}", err),
    }

    log::info!("well that is all folks")
}
