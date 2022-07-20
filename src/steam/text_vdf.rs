use anyhow::Result;
use serde::Deserialize;

pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T> {
    let value = vdf_serde::from_str::<T>(input)?;

    Ok(value)
}
