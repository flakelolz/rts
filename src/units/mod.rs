mod warrior;

use crate::prelude::*;
pub use warrior::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnitPosition>()
            .register_type::<UnitSpeed>()
            .add_plugins(WarriorPlugin);
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitPosition {
    pub current: Vec2,
    pub desired: Option<Vec2>,
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct UnitSpeed(pub f32);

