use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn handle_mouse_inputs(&mut self, ui: &mut egui::Ui) {
        let secondary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

        // camera pan
        if secondary_down {
            let delta = ui.input(|i| i.pointer.delta());
            if delta != egui::Vec2::ZERO {
                self.canvas_context.camera_pan(delta);
            }
        }
    }

    pub fn handle_keyboard_inputs(&mut self, ui: &mut egui::Ui) {
        let secondary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Secondary));

        // pan reset handling
        if secondary_down && ui.input(|i| i.key_pressed(egui::Key::R)) {
            self.canvas_context.camera_mut().reset();
        }

        // clear selections
        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));

        if escape_pressed {
            self.canvas_context.clear_selections();
        }
    }
}
