use tacit::{JsonFormatter, SimpleConsoleOutput};

fn main() {
    let json_logger = tacit::Logger::<SimpleConsoleOutput, JsonFormatter>::default()
        .with_module_level_filter("mio".into(), log::LevelFilter::Off)
        .with_module_level_filter("tokio".into(), log::LevelFilter::Off)
        .with_module_level_filter("reqwest".into(), log::LevelFilter::Off)
        .with_module_level_filter("rustls".into(), log::LevelFilter::Off)
        .with_module_level_filter("want".into(), log::LevelFilter::Off)
        .with_level_filter(log::LevelFilter::Trace);

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
