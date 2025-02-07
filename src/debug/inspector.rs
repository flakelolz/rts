use crate::prelude::*;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultInspectorConfigPlugin, EguiPlugin))
            .add_systems(Update, world_inspector_ui)
            .add_systems(Update, selected_ui);
    }
}

fn world_inspector_ui(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector")
        .default_pos((0., 0.))
        .default_size((320., 360.))
        .default_open(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}

fn selected_ui(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Selected")
        .default_pos((0., 50.))
        .default_size((200., 150.))
        .default_open(true)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_resource::<Selected>(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
