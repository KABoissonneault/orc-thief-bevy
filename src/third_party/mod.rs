use bevy::prelude::*;

mod bevy_ecs_tiled;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tiled::plugin);
}