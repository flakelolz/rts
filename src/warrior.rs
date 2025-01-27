use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior)
            .register_type::<AnimationConfig>()
            .add_systems(Update, execute_animations)
            .init_resource::<MousePosition>()
            .register_type::<MousePosition>()
            .register_type::<Position>()
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, movement);
    }
}

#[derive(Component)]
struct Warrior;

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct Position {
    current: Vec2,
    desired: Vec2,
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct AnimationConfig {
    first: usize,
    last: usize,
    fps: u8,
    frame_timer: Timer,
    looping: bool,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8, looping: bool) -> Self {
        Self {
            first,
            last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            looping,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn spawn_warrior(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = assets.load("Warrior_Blue.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(192), 6, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let anim_config = AnimationConfig::new(0, 5, 10, true);

    commands.spawn((
        Warrior,
        Name::new("Warrior"),
        Position::default(),
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: anim_config.first,
            }),
            ..default()
        },
        anim_config,
    ));
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last {
                    atlas.index = config.first;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }

                if config.looping {
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

fn movement(
    mut warrior_q: Query<(&mut Transform, &mut Position), With<Warrior>>,
    mouse_pos: Res<MousePosition>,
    click: Res<ButtonInput<MouseButton>>,
) {
    for (mut transform, mut position) in &mut warrior_q {
        transform.translation.x = position.current.x;
        transform.translation.y = position.current.y;

        if click.just_pressed(MouseButton::Right) {
            position.desired.x = mouse_pos.x;
            position.desired.y = mouse_pos.y;
        }

        if position.current != position.desired {
            let current = position.current;
            let desired = position.desired;
            let speed = Vec2::splat(2.);
            let calc = (desired - current).normalize() * speed;

            if desired.distance(current) > 10. {
                position.current += calc;
            }
        }
    }
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MousePosition {
    x: f32,
    y: f32,
}

fn my_cursor_system(
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
