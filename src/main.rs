use anyhow::Result;

mod media;
mod scan;

fn setup_log() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("/log/scan.log").unwrap())
        .apply()?;
    Ok(())
}

fn main() -> Result<()> {
    setup_log()?;

    log::info!("Running initial scan");
    scan::scan_all()?;

    Ok(())
}
