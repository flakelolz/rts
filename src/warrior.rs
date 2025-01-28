use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior)
            .register_type::<Position>()
            .add_systems(Update, unit_movement);
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

#[derive(Component)]
struct Speed(f32);

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
        Speed(2.),
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

fn unit_movement(
    mut unit_q: Query<(&mut Transform, &mut Position, &Speed)>,
    mouse_pos: Res<MousePosition>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for (mut transform, mut position, Speed(speed)) in &mut unit_q {
        transform.translation.x = position.current.x;
        transform.translation.y = position.current.y;

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
