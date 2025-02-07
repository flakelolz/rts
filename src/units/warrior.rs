use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior)
            .add_systems(Update, warr_animation);
    }
}

#[derive(Component)]
pub struct Warrior;

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub enum WarrStates {
    #[default]
    Idle,
    Run,
    Attack,
}

fn spawn_warrior(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Name::new("Warrior"),
        Unit,
        Warrior,
        UnitPosition::default(),
        UnitSpeed(250.),
        AseSpriteAnimation {
            aseprite: assets.load("Warrior_Blue.aseprite"),
            animation: Animation::tag("Idle").with_repeat(AnimationRepeat::Loop),
        },
    ));
}

fn warr_animation(
    mut warrior_q: Query<(&mut AseSpriteAnimation, &mut Sprite, &UnitPosition), With<Warrior>>,
) {
    for (mut animation, mut sprite, position) in &mut warrior_q {
        if let Some(desired) = position.desired {
            if desired.x < position.current.x {
                sprite.flip_x = true;
            } else if desired.x > position.current.x {
                sprite.flip_x = false;
            }
        }

        if position.desired.is_some() {
            animation.animation.tag = Some("Run".into());
        } else {
            animation.animation.tag = Some("Idle".into());
        }
    }
}
