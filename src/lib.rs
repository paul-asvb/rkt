mod audio;
mod loading;
mod menu;
mod snake;

pub const TIME_STEP: f32 = 1.0 / 60.0;

// use crate::audio::InternalAudioPlugin;
// use crate::loading::LoadingPlugin;
// use crate::menu::MenuPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy::{app::App, time::FixedTimestep};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use snake::SnakePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_state(GameState::Loading)
        //     .add_plugin(LoadingPlugin)
        //     .add_plugin(MenuPlugin)
        //     .add_plugin(ActionsPlugin)
        //     .add_plugin(InternalAudioPlugin)
        app.add_system_set(
            SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        )
        .add_state(GameState::Playing)
        .add_plugin(SnakePlugin);

        #[cfg(debug_assertions)]
        {
            app.insert_resource(LogSettings {
                filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                level: bevy::log::Level::DEBUG,
            })
            //.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //  .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(DebugLinesPlugin::default());
        }
    }
}
