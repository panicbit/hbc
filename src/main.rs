use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::steam::id::LegacyAppId;

mod steam;

fn main() -> Result<()> {
    let file = File::open(get_shortcuts_vdf_path())?;
    let mut file = BufReader::new(file);

    let shortcuts = steam::binary_vdf::from_reader::<serde_value::Value, _>(&mut file)?;

    println!("{:#?}", shortcuts);

    let name = "foo";
    let target = "bar.exe";
    let legacy_app_id = LegacyAppId::from_shortcut_name_and_target(name, target);

    Ok(())
}

fn get_shortcuts_vdf_path() -> PathBuf {
    // TODO
    todo!()
}
