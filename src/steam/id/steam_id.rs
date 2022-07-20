use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
    Debug,
)]
pub struct SteamId(u64);

impl SteamId {
    pub fn to_path_id(self) -> u32 {
        self.0 as u32
    }

    pub fn from_path_id(id: u32) -> Self {
        Self(id as u64 | 0x01100001)
    }
}

impl From<SteamId> for u64 {
    fn from(steam_id: SteamId) -> u64 {
        steam_id.0
    }
}

impl From<u64> for SteamId {
    fn from(steam_id: u64) -> SteamId {
        SteamId(steam_id)
    }
}
