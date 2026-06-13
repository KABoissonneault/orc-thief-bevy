mod tile_entity_transform;

use std::ops::Deref;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{MapCreated, TilePos, TiledEvent, TiledMap, TiledMapStorage, TiledObject, TilemapAnchor};

const TILE_COORD_TO_WORLD_POS: f32 = 24.0;

fn get_tile_center_world_coord(tile_coord: IVec2) -> Vec2 {
    (tile_coord.as_vec2() * TILE_COORD_TO_WORLD_POS)
        + Vec2::new(TILE_COORD_TO_WORLD_POS/2.0, TILE_COORD_TO_WORLD_POS/2.0)
}

#[derive(Component, Default, Clone, Copy, PartialEq, Debug)]
struct TileCoord(IVec2);

impl TileCoord {
    fn center_world_coord(&self) -> Vec2 {
        get_tile_center_world_coord(self.0)
    }

    fn from_world_coord(world_coord: Vec2) -> TileCoord {
        TileCoord((world_coord / TILE_COORD_TO_WORLD_POS).as_ivec2())
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
enum SpawnPoint {
    #[default]
    Unknown,
    Player,
}

#[derive(Resource, Default)]
struct SpawnPoints {
    player_points: Vec<Entity>,
    player_point_coords: Vec<TileCoord>,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(tile_entity_transform::plugin);

    app.insert_resource(TestEntityAssets::default());
    app.insert_resource(SpawnPoints::default());
    app.add_systems(Startup, (load_test_entity_assets, orc_thief_setup).chain());
    app.add_systems(Update, move_player);

    app.register_type::<SpawnPoint>();
    app.add_observer(on_add_spawn);
}

fn orc_thief_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = asset_server.load("maps/map1.tmx");
    commands.spawn((TiledMap(map), TilemapAnchor::Center))
        .observe(on_map_created);
}

fn on_add_spawn(add_spawn: On<Add, SpawnPoint>, spawn_query: Query<(&SpawnPoint, &GlobalTransform)>, mut spawn_points: ResMut<SpawnPoints>) {
    let spawn_entity = add_spawn.event().entity;

    let (spawn_type, spawn_world_transform) = spawn_query.get(spawn_entity).unwrap();

    match spawn_type {
        SpawnPoint::Player => {
            spawn_points.player_points.push(spawn_entity);
            spawn_points.player_point_coords.push(TileCoord::from_world_coord(spawn_world_transform.translation().truncate()));
        },
        SpawnPoint::Unknown => {
            warn!("Unknown spawn type");
        }
    }
}

fn on_map_created(_map_created: On<TiledEvent<MapCreated>>
                  , spawn_points: Res<SpawnPoints>
                  , mut commands: Commands
                  , assets: Res<TestEntityAssets>)
{
    if let Some(spawn_point) = spawn_points.player_point_coords.first() {
        commands.spawn((Player, *spawn_point, Mesh2d(assets.mesh.clone()), MeshMaterial2d(assets.material.clone())));
    } else {
        warn!("No player spawn point found");
    }
}

#[derive(Resource, Default)]
struct TestEntityAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

const TEST_ENTITY_SHAPE: Circle = Circle::new(12.0);
const TEST_ENTITY_COLOR: Color = Color::srgb(0.6, 0.4, 0.4);

fn load_test_entity_assets(mut meshes: ResMut<Assets<Mesh>>
                            , mut materials: ResMut<Assets<ColorMaterial>>
                            , mut assets: ResMut<TestEntityAssets>)
{
    assets.mesh = meshes.add(TEST_ENTITY_SHAPE);
    assets.material = materials.add(TEST_ENTITY_COLOR);
}

#[derive(Component, Default)]
#[require(TileCoord)]
#[require(Transform)] // Transform is for the visuals, gameplay usually uses TileCoord
struct Character;

#[derive(Component, Default)]
#[require(Character)]
struct Player;

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
