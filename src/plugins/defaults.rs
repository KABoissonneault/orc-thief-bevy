use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.08, 0.1, 0.08)));
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Orc Thief".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
}
