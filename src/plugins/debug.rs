use bevy::diagnostic::{Diagnostic, DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPrimaryContextPass};

#[derive(Resource, PartialEq, Eq)]
struct DebugInspectToggle(bool);

fn toggle_debug(
    input: Res<ButtonInput<KeyCode>>,
    mut debug_inspect_toggle: ResMut<DebugInspectToggle>,
) {
    if input.just_pressed(KeyCode::F1) {
        debug_inspect_toggle.0 = !debug_inspect_toggle.0;
    }
}

struct DiagnosticsEguiPlugin;

impl Plugin for DiagnosticsEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, draw_diagnostic_ui.run_if(resource_equals(DebugInspectToggle(true))));
    }
}

fn draw_diagnostic_ui(diagnostics: Res<DiagnosticsStore>, mut contexts: EguiContexts) {
    let context_result = contexts.ctx_mut();
    match context_result {
        Ok(context) => {
            egui::Window::new("Diagnostics").show(context, |ui| {
                for diagnostic in diagnostics.iter() {
                    let Some(value) = get_diagnostic_value(diagnostic) else {
                        continue;
                    };
                    ui.label(format!(
                        "{}: {}{}",
                        diagnostic.path(),
                        format_value(value),
                        diagnostic.suffix,
                    ));
                }
            });
        },
        Err(query_error) => {
            warn_once!("draw_diagnostic_ui query_error: {:?}", query_error);
        }
    };

}

fn get_diagnostic_value(diagnostic: &Diagnostic) -> Option<f64> {
    if !diagnostic.is_enabled {
        return None;
    }
    diagnostic.smoothed()
}

fn format_value(value: f64) -> String {
    format!("{:.2}", value)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(DebugInspectToggle(false))
        .add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(resource_equals(DebugInspectToggle(true))),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
            DiagnosticsEguiPlugin,
        ))
        .add_systems(Update, toggle_debug);
}
