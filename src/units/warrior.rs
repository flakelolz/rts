use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior)
            .add_systems(Update, warr_movement)
            .add_systems(Update, warr_animation);
    }
}

#[derive(Component)]
pub struct Warrior;

fn spawn_warrior(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Name::new("Warrior"),
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

fn warr_movement(
    mut warrior_q: Query<(&mut Transform, &mut UnitPosition, &UnitSpeed), With<Warrior>>,
    mouse_pos: Res<MousePosition>,
    input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    for (mut transform, mut position, UnitSpeed(speed)) in &mut warrior_q {
        transform.translation.x = position.current.x;
        transform.translation.y = position.current.y;
        let speed = speed * time.delta_secs();

        if input.just_pressed(MouseButton::Right) {
            position.desired = Some(Vec2::from(*mouse_pos));
        }

        if let Some(desired) = position.desired {
            if position.current != desired {
                let calc = (desired - position.current).normalize() * speed;

                if desired.distance(position.current) < speed * 1.5 {
                    position.current = desired;
                } else {
                    position.current += calc;
                }
            } else {
                position.desired = None;
            }
        }
    }
}
