use std::time::Duration;

use crate::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_warrior)
            .add_systems(Update, execute_animations);
    }
}

#[derive(Component)]
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
        Name::new("Warrior"),
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
