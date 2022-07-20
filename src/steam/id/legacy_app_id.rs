use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use super::AppId;

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
