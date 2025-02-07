use crate::prelude::*;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_position)
            .add_systems(Update, mouse_select)
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

fn mouse_select(
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
                if !selected.contains(entity) {
                    selected.clear();
                    selected.push(*entity);
                }
                return;
            }
        }

        selected.clear();
    }
}
