use bevy::prelude::*;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Upgrades>();
    }
}

#[derive(Debug, Default)]
pub struct Upgrades(u64);

impl Upgrades {
    pub const DOUBLE_SPEED: u64 = 1 << 0;
    pub const SHRINK: u64 = 1 << 1;

    pub fn has_upgrade(&self, upgrade: u64) -> bool {
        self.0 & upgrade == upgrade
    }
}
