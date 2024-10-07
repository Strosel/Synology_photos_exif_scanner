use anyhow::Result;
use clokwerk::{Job, Scheduler, TimeUnits};
use std::{thread, time::Duration};

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
        .chain(std::io::stderr())
        .chain(fern::log_file("/log/scan.log").unwrap())
        .apply()?;
    Ok(())
}

fn main() -> Result<()> {
    setup_log()?;

    log::info!("Running initial scan");
    scan::scan_all()?;

    let mut scheduler = Scheduler::new();

    scheduler.every(1.days()).at("00:00").run(|| {
        if let Err(e) = scan::scan_all() {
            log::error!("{e:?}");
        }
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}
