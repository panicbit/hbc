use crc::{Crc, CRC_32_ISO_HDLC};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

const BIT_31: u32 = 0x80000000;
const LEGACY_APP_ID_LOW: u64 = 0x02000000;

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
#[serde(transparent)]
pub struct AppId(u32);

impl AppId {
    pub fn from_shortcut_name_and_target(name: &str, target: &str) -> Self {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();

        digest.update(target.as_bytes());
        digest.update(name.as_bytes());

        let mut app_id = digest.finalize();
        app_id |= BIT_31;

        Self(app_id)
    }
}

impl From<AppId> for u32 {
    fn from(app_id: AppId) -> u32 {
        app_id.0
    }
}

impl From<LegacyAppId> for AppId {
    fn from(legacy_app_id: LegacyAppId) -> AppId {
        let legacy_app_id = u64::from(legacy_app_id);
        let app_id = (legacy_app_id >> 32) as u32;

        Self(app_id)
    }
}

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
pub struct LegacyAppId(u64);

impl LegacyAppId {
    pub fn from_shortcut_name_and_target(name: &str, target: &str) -> Self {
        let app_id = AppId::from_shortcut_name_and_target(name, target);

        Self::from(app_id)
    }
}

impl From<LegacyAppId> for u64 {
    fn from(legacy_app_id: LegacyAppId) -> u64 {
        legacy_app_id.0
    }
}

impl From<AppId> for LegacyAppId {
    fn from(app_id: AppId) -> LegacyAppId {
        let legacy_id = u32::from(app_id);
        let mut legacy_id = u64::from(legacy_id);
        legacy_id <<= 32;
        legacy_id |= LEGACY_APP_ID_LOW;

        Self(legacy_id)
    }
}
