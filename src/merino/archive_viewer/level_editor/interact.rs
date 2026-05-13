use crate::merino::{archive_viewer::level_editor::{LevelEditor, contexts::{canvas_context::CanvasContext, message_context::MessageContext}}, game::mapbin::{MapDataNode, MapNodeType, NodeData, NodePath}};

impl LevelEditor {
    pub fn interact_with_all_nodes(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        let Self {
            mapdata,
            canvas_context,
            message_context,
            ..
        } = self;

        let root = &mut mapdata.as_mut().unwrap().root;
        let mut node_path = NodePath::root();

        root.interact(
            ui,
            canvas_rect,
            &mut node_path,
            canvas_context,
            message_context
        );
    }
}


impl MapDataNode {
    fn interact(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
    ) {
        // process self if we're allowed to do that
        if canvas_context.can_view(self.node_type) {
            let can_edit = canvas_context.can_edit(self.node_type);

            match self.node_type {
                MapNodeType::MapSet => {},
                MapNodeType::MapPolySet => {
                    self.interact_mappolyset(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        egui::Color32::WHITE
                    );
                },
                MapNodeType::MapObjSet => {},
                MapNodeType::MapItemSet => {},
                MapNodeType::MapEnemySet => {},
                MapNodeType::MapLocator => {},
                MapNodeType::MapPath => {},
                MapNodeType::MapRect => {},
                MapNodeType::MapCircle => {},
                MapNodeType::MapTerrain => {},
            }
        }

        // process children
        for (step, child) in self.iter_mut() {
            current_path.push(step);
            child.interact(ui, canvas_rect, current_path, canvas_context, messages);
            current_path.pop();
        }
    }

    /* generic editors */

    // /// "basic node" as in a node that contains position and parameter data
    // fn interact_basic(
    //     &mut self,
    //     ui: &mut egui::Ui,
    //     current_path: &mut NodePath,
    //     canvas_context: &mut CanvasContext,
    //     messages: &mut MessageContext,
    //     can_edit: bool,
    //     color: egui::Color32,
    // ) {

    // }

    /* specific editors */

    fn interact_mappolyset(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        can_edit: bool,
        color: egui::Color32,
    ) {
        let NodeData::MapPolySet {
            start,
            end,
            collision_normal,
            ..
        } = &mut self.node_data else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let draw_start = canvas_rect.min + canvas_context.convert_to_camera(start.into());
        let draw_end = canvas_rect.min + canvas_context.convert_to_camera(end.into());

        painter.line_segment([draw_start, draw_end], egui::Stroke::new(0.5, color));
    }

    /* helpers */
}
