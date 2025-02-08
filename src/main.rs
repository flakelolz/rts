mod animations;
mod debug;
mod pointer;
mod units;

pub mod prelude {
    pub use crate::MainCamera;
    pub use crate::animations::*;
    pub use crate::debug::*;
    pub use crate::pointer::*;
    pub use crate::units::*;
    pub use bevy::picking::*;
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;
    pub use bevy_aseprite_ultra::AsepriteUltraPlugin;
    pub use bevy_aseprite_ultra::prelude::*;
    pub use bevy_inspector_egui::prelude::*;
    pub use bevy_inspector_egui::quick::ResourceInspectorPlugin;
    pub use bevy_inspector_egui::{
        DefaultInspectorConfigPlugin,
        bevy_egui::{EguiContext, EguiPlugin},
        bevy_inspector, egui,
    };
    pub use fastrand::i32;
    pub use std::time::Duration;
}

use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RTS".to_string(),
                    // resolution: (640., 360.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(DebugPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins((MousePlugin, AnimationsPlugin, UnitsPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}
