use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use crc::{Crc, CRC_32_ISO_HDLC};

mod steam;

fn main() -> Result<()> {
    let file = File::open(get_shortcuts_vdf_path())?;
    let mut file = BufReader::new(file);

    let shortcuts = steam::binary_vdf::from_reader::<serde_value::Value, _>(&mut file)?;

    println!("{:#?}", shortcuts);

    let name = "foo";
    let path = "bar.exe";
    let legacy_app_id = generate_legacy_app_id(name, path);
    dbg!(legacy_app_id);
    let shortcut_id = generate_app_id(name, path);
    dbg!(shortcut_id);
    Ok(())
}

fn generate_legacy_app_id(name: &str, path: &str) -> u64 {
    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = crc.digest();

    digest.update(path.as_bytes());
    digest.update(name.as_bytes());

    let mut id = digest.finalize() as u64;
    id |= 0x80000000;
    id <<= 32;
    id |= 0x02000000;

    id
}

fn generate_app_id(name: &str, path: &str) -> u32 {
    (generate_legacy_app_id(name, path) >> 32) as u32
}

fn get_shortcuts_vdf_path() -> PathBuf {
    // TODO
    todo!()
}
