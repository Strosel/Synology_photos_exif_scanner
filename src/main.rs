use anyhow::Result;
use std::process::Command;

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

    match Command::new("exiftool").output() {
        Ok(out) => log::warn!("{out:?}"),
        Err(e) => log::error!("{e:?}"),
    }

    Ok(())
}
