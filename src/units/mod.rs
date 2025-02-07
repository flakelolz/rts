mod warrior;

use crate::prelude::*;
pub use warrior::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnitPosition>()
            .register_type::<UnitSpeed>()
            .add_systems(Update, update_unit_transform)
            .add_systems(Update, assign_unit_movement)
            .add_systems(Update, update_unit_movement)
            .add_plugins(WarriorPlugin);
    }
}

pub enum UnitState {
    Idle,
    Run,
    Action,
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Unit;

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitPosition {
    pub current: Vec2,
    pub desired: Option<Vec2>,
}

#[derive(Component, Default, Deref, DerefMut, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitSpeed(pub f32);

fn update_unit_transform(mut query: Query<(&mut Transform, &mut UnitPosition), With<Unit>>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.current.x;
        transform.translation.y = position.current.y;
    }
}

fn assign_unit_movement(
    mut warrior_q: Query<&mut UnitPosition, With<Unit>>,
    selected: ResMut<Selected>,
    mouse_pos: Res<MousePosition>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut iter = warrior_q.iter_many_mut(&**selected);

    while let Some(mut position) = iter.fetch_next() {
        if input.just_pressed(MouseButton::Right) {
            position.desired = Some(Vec2::from(*mouse_pos));
        }
    }
}

fn update_unit_movement(
    mut warrior_q: Query<(&mut UnitPosition, &UnitSpeed), With<Unit>>,
    time: Res<Time>,
) {
    for (mut position, speed) in &mut warrior_q {
        let speed = **speed * time.delta_secs();

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
