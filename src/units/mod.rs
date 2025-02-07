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

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub enum UnitState {
    #[default]
    Idle,
    Run,
    Action,
}

impl std::fmt::Display for UnitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitState::Idle => write!(f, "Idle"),
            UnitState::Run => write!(f, "Run"),
            UnitState::Action => write!(f, "Action"),
        }
    }
}

impl From<&UnitState> for String {
    fn from(value: &UnitState) -> Self {
        format!("{value}")
    }
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

impl UnitPosition {
    pub fn new(current: Vec2) -> Self {
        Self {
            current,
            desired: None,
        }
    }
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
    mut unit_q: Query<(&mut UnitPosition, &mut UnitState), With<Unit>>,
    selected: ResMut<Selected>,
    mouse_pos: Res<MousePosition>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut iter = unit_q.iter_many_mut(&**selected);

    while let Some((mut position, mut state)) = iter.fetch_next() {
        if input.just_pressed(MouseButton::Right) {
            position.desired = Some(Vec2::from(*mouse_pos));
            *state = UnitState::Run;
        }
    }
}

fn update_unit_movement(
    mut unit_q: Query<(&mut UnitPosition, &UnitSpeed, &mut UnitState), With<Unit>>,
    time: Res<Time>,
) {
    for (mut position, speed, mut state) in &mut unit_q {
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
                *state = UnitState::Idle;
            }
        }
    }
}
