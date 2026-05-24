use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TiledPlugin, TilemapAnchor};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

struct OrcThiefPlugin;

impl Plugin for OrcThiefPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default());
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.1, 0.08)));
        app.add_systems(Startup, orc_thief_setup);
    }
}

fn orc_thief_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let map = asset_server.load("maps/map1.tmx");
    commands.spawn((TiledMap(map), TilemapAnchor::Center));
}

#[cfg(feature = "debug")]
#[derive(Resource, PartialEq, Eq)]
struct WorldInspectorRun(bool);

#[cfg(feature = "debug")]
fn toggle_debug(input: Res<ButtonInput<KeyCode>>, mut world_inspector_run: ResMut<WorldInspectorRun>) {
    if input.just_pressed(KeyCode::F1) {
        world_inspector_run.0 = !world_inspector_run.0;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Orc Thief".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(OrcThiefPlugin);

    #[cfg(feature = "debug")]
    {
        app
            .insert_resource(WorldInspectorRun(false))
            .add_plugins((
                EguiPlugin::default(),
                WorldInspectorPlugin::default().run_if(resource_equals(WorldInspectorRun(true))),
            ))
            .add_systems(Update, toggle_debug);
    }

    app.run();

    Ok(())
}
