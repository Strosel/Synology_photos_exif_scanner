use crate::media::MediaType;
use anyhow::Result;
use std::{collections::HashMap, env, process::Command};

fn scan(ty: MediaType) -> Result<Vec<String>> {
    let stdout = Command::new(env::var("EXIFTOOL")?)
        .args(["-s", "FilePath"])
        .args(["-if", ty.exif_condition()])
        .args(["-r", "/Photos"])
        .output()?
        .stdout;

    let output = String::from_utf8(stdout)?
        .lines()
        .filter_map(|s| s.strip_prefix("======== "))
        .map(String::from)
        .collect();

    Ok(output)
}

pub fn scan_all() {
    let mut map = HashMap::new();
    for ty in [MediaType::Photo, MediaType::Video] {
        match scan(ty) {
            Ok(missing) => {
                map.insert(ty, missing);
            }
            Err(e) => {
                log::error!("{e:?}");
                return;
            }
        }
    }

    //TODO write map to file
}
