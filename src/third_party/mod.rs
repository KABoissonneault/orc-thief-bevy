use bevy::prelude::*;
use bevy_tween::DefaultTweenPlugins;

mod bevy_ecs_tiled;

pub(super) fn plugin(app: &mut App) {    
    app.add_plugins(bevy_ecs_tiled::plugin);
    app.add_plugins(DefaultTweenPlugins::default());
}