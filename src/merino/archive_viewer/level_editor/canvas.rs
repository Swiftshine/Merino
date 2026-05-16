use crate::merino::archive_viewer::level_editor::LevelEditor;

impl LevelEditor {
    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let desired_canvas_size = ui.available_size();
        let (_, response) =
            ui.allocate_exact_size(desired_canvas_size, egui::Sense::click_and_drag());

        let rect = response.rect;

        // update camera
        self.canvas_context.camera_mut().update(ui.ctx(), &response);

        // draw black
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);

        // draw grid
        // todo! make this toggleable
        self.canvas_context
            .draw_grid(&painter, rect, 1.0, egui::Color32::from_gray(30));

        // interact with objects
        self.interact_with_all_nodes(ui, rect, &response);

        // process inputs
        if response.hovered() {
            // deal with any targets
            if self.canvas_context.is_target_new() {
                self.add_object(ui, rect, &response);
            }

            self.handle_mouse_inputs(ui);
            self.handle_keyboard_inputs(ui);
        }
    }
}
