mod grid;
mod inspector;
mod warrior;

pub mod prelude {
    pub use crate::grid::GridPlugin;
    pub use crate::inspector::InspectorPlugin;
    pub use crate::warrior::WarriorPlugin;
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;
    pub use bevy_inspector_egui::prelude::*;
    pub use bevy_inspector_egui::{
        DefaultInspectorConfigPlugin,
        bevy_egui::{EguiContext, EguiPlugin},
        bevy_inspector, egui,
    };
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
    app.add_plugins(InspectorPlugin);
    app.add_plugins((GridPlugin, WarriorPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
