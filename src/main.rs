use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use steam::id::AppId;

use crate::serde_utils::deserialize_map_values;

mod steam;
mod serde_utils;

fn main() -> Result<()> {
    let file = File::open(get_shortcuts_vdf_path())?;
    let mut file = BufReader::new(file);

    let shortcuts = steam::binary_vdf::from_reader::<ShortcutsVdf, _>(&mut file)?;

    println!("{:#?}", shortcuts);

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

#[derive(Deserialize, Debug)]
struct ShortcutsVdf {
    #[serde(deserialize_with = "deserialize_map_values")]
    shortcuts: Vec<Shortcut>,
}

#[derive(Deserialize, Debug)]
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
    #[serde(rename = "tags", deserialize_with = "deserialize_map_values")]
    tags: Vec<String>,
    #[serde(flatten)]
    rest: IndexMap<String, serde_value::Value>,
}

fn deserialize_map_as_vec<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let map = IndexMap::<String, T>::deserialize(de).map_err(serde::de::Error::custom)?;
    let vec = map.into_values().collect::<Vec<_>>();

    Ok(vec)
}
