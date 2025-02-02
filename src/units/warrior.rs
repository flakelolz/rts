use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior);
    }
}

#[derive(Component)]
struct Warrior;

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
        UnitPosition::default(),
        UnitSpeed(250.),
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
