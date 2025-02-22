use crate::prelude::*;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_position)
            .add_systems(Update, (group_select, click_select).chain())
            .init_resource::<MousePosition>()
            .register_type::<MousePosition>()
            .init_resource::<Selected>()
            .register_type::<Selected>();
    }
}

#[derive(Resource, Default, Clone, Copy, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
    pub clicked: Option<Vec2>,
}

impl From<MousePosition> for Vec2 {
    fn from(value: MousePosition) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vec2> for MousePosition {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            clicked: None,
        }
    }
}

fn mouse_position(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_pos: ResMut<MousePosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_pos.x = world_position.unwrap().x;
        mouse_pos.y = world_position.unwrap().y;
    }
}

#[derive(Resource, Default, Clone, Deref, DerefMut, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Selected(pub Vec<Entity>);

fn click_select(
    click: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    pointers: Query<&bevy::picking::pointer::PointerInteraction>,
    mut selected: ResMut<Selected>,
) {
    // Select multiple with left click when holding left control
    if click.just_pressed(MouseButton::Left) && key.pressed(KeyCode::ControlLeft) {
        for interaction in &pointers {
            if let Some((entity, _)) = interaction.get_nearest_hit() {
                if !selected.contains(entity) {
                    selected.push(*entity);
                }
                return;
            }
        }
    }

    // Select one with left click and erase selections if nothing is selected
    if click.just_pressed(MouseButton::Left) {
        for interaction in &pointers {
            if let Some((entity, _)) = interaction.get_nearest_hit() {
                // Since left control wasn't being held, clear selected for new single selection
                selected.clear();

                if !selected.contains(entity) {
                    selected.push(*entity);
                }
                return;
            }
        }

        selected.clear();
    }
}

fn group_select(
    click: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<Selected>,
    mut mouse: ResMut<MousePosition>,
    unit_q: Query<(Entity, &Transform), With<Unit>>,
) {
    if click.just_pressed(MouseButton::Left) {
        mouse.clicked = Some(Vec2::new(mouse.x, mouse.y));
    }

    if click.just_released(MouseButton::Left) {
        let Some(clicked) = mouse.clicked else {
            return;
        };

        let x_min = f32::min(clicked.x, mouse.x);
        let x_max = f32::max(clicked.x, mouse.x);
        let y_min = f32::min(clicked.y, mouse.y);
        let y_max = f32::max(clicked.y, mouse.y);

        for (entity, transform) in &unit_q {
            let pos = &transform.translation;

            if x_min <= pos.x && pos.x <= x_max && y_min <= pos.y && pos.y <= y_max {
                selected.push(entity);
            }
        }

        mouse.clicked = None;
    }
}
