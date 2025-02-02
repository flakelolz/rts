use crate::prelude::*;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationConfig>()
            // .add_systems(Update, execute_animations)
        ;
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct AnimationConfig {
    pub first: usize,
    pub last: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub looping: bool,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, looping: bool) -> Self {
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

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(mut query: Query<(&mut AnimationConfig, &mut Sprite)>, time: Res<Time>) {
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

