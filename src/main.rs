use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

struct OrcThiefPlugin;

impl Plugin for OrcThiefPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)));
        app.add_systems(Startup, orc_thief_setup);
    }
}

fn orc_thief_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, PartialEq, Eq)]
struct WorldInspectorRun(bool);

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
        )
        .add_plugins(OrcThiefPlugin);

    app
        .insert_resource(WorldInspectorRun(false))
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(resource_equals(WorldInspectorRun(true))),
        ))
        .add_systems(Update, toggle_debug);
    
    app.run();

    Ok(())
}
