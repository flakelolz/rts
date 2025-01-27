use crate::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .register_type::<Grid>();
    }
}

pub const GRID_SIZE: usize = 20;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Grid {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}
