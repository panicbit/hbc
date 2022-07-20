use std::fs::{File, self};
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Result, Context};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use steam::id::AppId;

mod steam;
mod serde_utils;

fn main() -> Result<()> {
    let file = fs::read(get_shortcuts_vdf_path())?;

    let shortcuts = steam::binary_vdf::from_bytes::<ShortcutsVdf>(&file)?;
    println!("{:#?}", shortcuts);

    let bytes = steam::binary_vdf::to_bytes(&shortcuts)
        .context("failed to serialize bvdf")?;


    std::fs::write("/tmp/foo.bvdf", &bytes)?;

    let shortcuts2 = steam::binary_vdf::from_bytes::<ShortcutsVdf>(&bytes)?;

    // println!("{:#?}", shortcuts2);

    // println!("{}", serde_json::to_string_pretty(&shortcuts).unwrap());

    // for shortcut in shortcuts.get("shortcuts").unwrap().as_object().unwrap().values() {
    //     let name = shortcut.get("AppName").unwrap();
    //     println!("gameName: {}", name);
    // }

    Ok(())
}

fn get_shortcuts_vdf_path() -> PathBuf {
    // TODO
    todo!()
}

#[derive(Deserialize, Serialize, Debug)]
struct ShortcutsVdf {
    shortcuts: Vec<Shortcut>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Shortcut {
    #[serde(rename = "appid")]
    app_id: AppId,
    app_name: String,
    exe: String,
    start_dir: String,
    #[serde(rename = "icon")]
    icon: String,
    shortcut_path: String,
    launch_options: String,
    is_hidden: bool,
    allow_desktop_config: bool,
    allow_overlay: bool,
    #[serde(rename = "openvr")]
    open_vr: u32,
    devkit: u32,
    #[serde(rename = "DevkitGameID")]
    devkit_game_id: String,
    #[serde(rename = "DevkitOverrideAppID")]
    devkit_override_app_id: u32,
    last_play_time: u32,
    #[serde(rename = "FlatpakAppID")]
    flatpak_app_id: String,
    #[serde(rename = "tags")]
    tags: Vec<String>,
    #[serde(flatten)]
    rest: IndexMap<String, serde_value::Value>,
}
