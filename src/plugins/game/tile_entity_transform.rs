use bevy::ecs::system::IntoResult;
use bevy::prelude::*;
use bevy_tween::interpolate::translation;
use bevy_tween::prelude::*;
use bevy_tween::tween::{AnimationTarget, TargetComponent};
use crate::plugins::game::{Character, TileCoord};

const TILE_ENTITY_LAYER: f32 = 1.0;

pub fn plugin(app: &mut App) {

    app.add_observer(character_hook);
    app.add_systems(Update, start_tween);
    app.add_systems(Update, project_tile_entities);
}

fn character_hook(event: On<Add, Character>, tile_coords: Query<&TileCoord, With<Character>>, mut commands: Commands) {
    let entity = event.entity;

    let mut entity_commands = commands.entity(event.entity);

    let tile_coord = tile_coords.get(entity).unwrap();
    let tile_pos = tile_coord.center_world_coord();
    entity_commands.entry::<Transform>()
        .and_modify(move |mut t| {
            t.translation = tile_pos.extend(TILE_ENTITY_LAYER);
        });

    entity_commands.insert(AnimationTarget);
}

fn start_tween(changed_characters: Query<(Entity, &Transform, &TileCoord), (With<Character>, Changed<TileCoord>)>, mut commands: Commands) {
    let target = AnimationTarget.into_target();
    for (changed_entity, transform, tile_coord) in changed_characters.iter() {
        commands.entity(changed_entity).animation().insert_tween_here(
            Duration::from_millis(100),
            EaseKind::SmoothStepOut,
            target.with(translation(transform.translation, tile_coord.center_world_coord().extend(TILE_ENTITY_LAYER)))
        );
    }
}

fn project_tile_entities(mut tile_coords: Query<(&TileCoord, &mut Transform), (Changed<TileCoord>, Without<Character>)>) {
    for (coord, mut transform) in tile_coords.iter_mut() {
        transform.translation = coord.center_world_coord().extend(TILE_ENTITY_LAYER);
    }
}