use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TilemapAnchor};

#[derive(Component, Default)]
#[require(Transform)]
struct TileCoord(IVec2);

const TILE_COORD_TO_WORLD_POS: f32 = 8.0;
const TILE_ENTITY_LAYER: f32 = 1.0;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(TestEntityAssets::default());
    app.add_systems(Startup, (orc_thief_setup,
                              (load_title_entity_assets, spawn_player).chain()));
    app.add_systems(Update, move_player);
    app.add_systems(PostUpdate, project_tile_entities);
}

fn orc_thief_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = asset_server.load("maps/map1.tmx");
    commands.spawn((TiledMap(map), TilemapAnchor::Center));
}

#[derive(Resource, Default)]
struct TestEntityAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

const TEST_ENTITY_SHAPE: Circle = Circle::new(4.0);
const TEST_ENTITY_COLOR: Color = Color::srgb(0.6, 0.4, 0.4);

fn load_title_entity_assets(mut meshes: ResMut<Assets<Mesh>>
                            , mut materials: ResMut<Assets<ColorMaterial>>
                            , mut assets: ResMut<TestEntityAssets>)
{
    assets.mesh = meshes.add(TEST_ENTITY_SHAPE);
    assets.material = materials.add(TEST_ENTITY_COLOR);
}

#[derive(Component)]
#[require(TileCoord)]
struct Player;

fn spawn_player(mut commands: Commands
                     , assets: Res<TestEntityAssets>) {
    commands.spawn((Player, Mesh2d(assets.mesh.clone()), MeshMaterial2d(assets.material.clone())));
}

fn move_player(mut player: Single<&mut TileCoord, With<Player>>
    , input: Res<ButtonInput<KeyCode>>) {
    let increment = if input.pressed(KeyCode::ShiftLeft) { 3 } else { 1 };    
    
    if input.just_pressed(KeyCode::Numpad1) {
        player.0 += IVec2::new(-increment, -increment);
    }
    if input.just_pressed(KeyCode::Numpad2) {
        player.0 += IVec2::new(0, -increment);
    }
    if input.just_pressed(KeyCode::Numpad3) {
        player.0 += IVec2::new(increment, -increment);
    }
    if input.just_pressed(KeyCode::Numpad4) {
        player.0 += IVec2::new(-increment, 0);
    }
    if input.just_pressed(KeyCode::Numpad6) {
        player.0 += IVec2::new(increment, 0);
    }
    if input.just_pressed(KeyCode::Numpad7) {
        player.0 += IVec2::new(-increment, increment);
    }
    if input.just_pressed(KeyCode::Numpad8) {
        player.0 += IVec2::new(0, increment);
    }
    if input.just_pressed(KeyCode::Numpad9) {
        player.0 += IVec2::new(increment, increment);
    }
}

fn project_tile_entities(mut tile_coords: Query<(&TileCoord, &mut Transform)>) {
    for (coord, mut transform) in tile_coords.iter_mut() {
        transform.translation = ((coord.0.as_vec2() * TILE_COORD_TO_WORLD_POS) 
            + Vec2::new(TILE_COORD_TO_WORLD_POS/2.0, TILE_COORD_TO_WORLD_POS/2.0))
            .extend(TILE_ENTITY_LAYER);
    }
}