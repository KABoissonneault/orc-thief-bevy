use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TilemapAnchor};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, orc_thief_setup);
}

fn orc_thief_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = asset_server.load("maps/map1.tmx");
    commands.spawn((TiledMap(map), TilemapAnchor::Center));
}
