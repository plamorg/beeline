use bevy::prelude::*;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Upgrades>();
    }
}

#[derive(Debug, Default)]
pub struct Upgrades(u64);

macro_rules! upgrade_definitions {
    ($n:expr ;) => {
        pub fn number_of_upgrades() -> u64 {
            $n
        }
    };
    ($n:expr ; $t:ident $(, $rest:tt)*) => {
        pub const $t: u64 = 1 << $n;
        upgrade_definitions!($n + 1; $($rest),*);
    };
    ($($upgrades:tt),+) => { upgrade_definitions!(0; $($upgrades),*); };
}

impl Upgrades {
    upgrade_definitions!(DOUBLE_SPEED, SHRINK, TELEPORT);
    const UPGRADE_NAMES: &'static [&'static str] = &["Double Speed", "Shrink", "Teleport"];

    pub fn set_upgrade(&mut self, upgrade: u64) {
        self.0 |= upgrade;
    }

    pub fn unset_upgrade(&mut self, upgrade: u64) {
        self.0 &= !upgrade;
    }

    pub fn has_upgrade(&self, upgrade: u64) -> bool {
        self.0 & upgrade == upgrade
    }

    pub fn get_name(upgrade: u64) -> String {
        Self::UPGRADE_NAMES[(upgrade as f64).log2() as usize].to_string()
    }
}
