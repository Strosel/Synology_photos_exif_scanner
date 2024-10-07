use crate::media::MediaType;
use anyhow::Result;
use std::{collections::HashMap, env, fs::File, process::Command};

fn scan(ty: MediaType) -> Result<Vec<String>> {
    let stdout = Command::new(env::var("EXIFTOOL")?)
        .args(["-s", "-FilePath"])
        .args(["-if", ty.exif_condition()])
        .args(["-r", "/Photos"])
        .args(["-i", "@eaDir"])
        .output()?
        .stdout;

    let output = String::from_utf8(stdout)?
        .lines()
        .filter_map(|s| s.strip_prefix("======== "))
        .map(String::from)
        .collect();

    Ok(output)
}

pub fn scan_all() -> Result<()> {
    let mut map = HashMap::new();
    for ty in [MediaType::Photo, MediaType::Video, MediaType::Gif] {
        map.insert(ty, scan(ty)?);
    }

    let file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open("/log/scan_result.yml")?;

    serde_yaml::to_writer(file, &map)?;

    Ok(())
}
