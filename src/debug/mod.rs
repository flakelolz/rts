mod inspector;
use inspector::InspectorPlugin;

use crate::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorPlugin);
    }
}
