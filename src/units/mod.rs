mod warrior;

use crate::prelude::*;
pub use warrior::WarriorPlugin;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnitPosition>()
            .register_type::<UnitSpeed>()
            .add_systems(Update, unit_movement)
            .add_plugins(WarriorPlugin);
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitPosition {
    pub current: Vec2,
    pub desired: Vec2,
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitSpeed(pub f32);

fn unit_movement(
    mut unit_q: Query<(&mut Transform, &mut UnitPosition, &UnitSpeed)>,
    mouse_pos: Res<MousePosition>,
    input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    for (mut transform, mut position, UnitSpeed(speed)) in &mut unit_q {
        transform.translation.x = position.current.x;
        transform.translation.y = position.current.y;
        let speed = speed * time.delta_secs();

        if input.just_pressed(MouseButton::Right) {
            position.desired.x = mouse_pos.x;
            position.desired.y = mouse_pos.y;
        }

        if position.current != position.desired {
            let current = position.current;
            let desired = position.desired;
            let calc = (desired - current).normalize() * speed;

            if desired.distance(current) > speed * 1.5 {
                position.current += calc;
            }
        }
    }
}
