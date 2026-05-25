#![allow(unused_imports)]

use bevy::prelude::*;

mod third_party;
mod plugins;

mod prelude {
    pub use super::*;
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            plugins::defaults::plugin,
            third_party::plugin,
            plugins::camera::plugin,
            plugins::game::plugin,
        ));
        
        #[cfg(feature = "dev")]
        app.add_plugins(plugins::debug::plugin);
    }
}