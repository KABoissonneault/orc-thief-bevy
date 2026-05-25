use std::process::ExitCode;
use bevy::prelude::*;
use orc_thief_bevy::AppPlugin;

fn main() -> ExitCode {
    match App::new().add_plugins(AppPlugin).run() {
        AppExit::Success => ExitCode::SUCCESS,
        AppExit::Error(err) => ExitCode::from(err.get()),
    }
}
