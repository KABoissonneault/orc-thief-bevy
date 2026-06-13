use std::path::PathBuf;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let mut export_path : PathBuf = PathBuf::new();
    export_path.push("exports");
    export_path.push("orc_thief_types.json");
    app.add_plugins(TiledPlugin(TiledPluginConfig {
        tiled_types_export_file: Some(export_path),
        tiled_types_filter: TiledFilter::from(
            regex::RegexSet::new([
                r"^orc_thief_bevy::.*",
                r"^bevy_sprite::text2d::Text2d$",
                r"^bevy_text::text::TextColor$",
                r"^bevy_ecs::name::Name$",
            ]).unwrap(),
        ),
    }));
}
