use bevy::input::common_conditions::input_just_pressed;

use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_warrior.run_if(input_just_pressed(KeyCode::Space)),
        );
    }
}

#[derive(Component)]
pub struct Warrior;

fn spawn_warrior(mut commands: Commands, assets: Res<AssetServer>) {
    let position = Vec2 {
        x: fastrand::i32(-200..200) as f32,
        y: fastrand::i32(-200..200) as f32,
    };

    commands.spawn((
        Name::new("Warrior"),
        Unit,
        Warrior,
        UnitPosition::new(position),
        UnitSpeed(250.),
        UnitState::Idle,
        AseSpriteAnimation {
            aseprite: assets.load("Warrior_Blue.aseprite"),
            animation: Animation::tag("Idle").with_repeat(AnimationRepeat::Loop),
        },
    ));
}
