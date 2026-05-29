use std::process::ExitCode;
use bevy::prelude::*;
use orc_thief_bevy::AppPlugin;

fn main() -> ExitCode {
    let mut app = App::new();
    app.add_plugins(AppPlugin);
        
    match app.run() {
        AppExit::Success => ExitCode::SUCCESS,
        AppExit::Error(err) => ExitCode::from(err.get()),
    }
}
