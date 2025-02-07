use crate::prelude::*;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationConfig>()
            .add_systems(Update, orientation)
            .add_systems(Update, animation);
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

fn animation(mut unit_q: Query<(&mut AseSpriteAnimation, &UnitState), With<Unit>>) {
    for (mut anim, state) in &mut unit_q {
        anim.animation.tag = Some(state.into());
    }
}

fn orientation(mut unit_q: Query<(&mut Sprite, &UnitPosition)>) {
    for (mut sprite, position) in &mut unit_q {
        if let Some(desired) = position.desired {
            if desired.x < position.current.x {
                sprite.flip_x = true;
            } else if desired.x > position.current.x {
                sprite.flip_x = false;
            }
        }
    }
}
