use crate::merino::{archive_viewer::level_editor::{LevelEditor, contexts::{canvas_context::CanvasContext, message_context::{Command, MessageContext}}}, game::mapbin::{MapDataNode, MapNodeType, NodeData, NodePath, types::Vec2f}};

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
        current_path: &NodePath,
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

        if !can_edit {
            return;
        }

        let points = [draw_start, draw_end];

        let changed = handle_drag_and_selections(
            ui,
            &points,
            &mut [start, end],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            0.5,
            color
        );


        if changed {
            // update collision normals
            let direction = (end.x - start.x, end.y - start.y);

            let magnitude = f32::sqrt(direction.0.powf(2.0) + direction.1.powf(2.0));

            let normalized = (direction.0 / magnitude, direction.1 / magnitude);

            collision_normal.x = -normalized.1;
            collision_normal.y = normalized.0;
        }
    }
}

/* helpers */

/// Handles mouse inputs (dragging, clicking) for any number of points.
/// Returns if any changes were made.
fn handle_drag_and_selections(
    ui: &mut egui::Ui,
    draw_points: &[egui::Pos2],
    positions: &mut [&mut Vec2f],
    canvas_rect: egui::Rect,
    current_path: &NodePath,
    canvas_context: &mut CanvasContext,
    messages: &mut MessageContext,
    size: f32,
    mut color: egui::Color32,
) -> bool {
    assert_eq!(draw_points.len(), positions.len());

    let painter = ui.painter_at(canvas_rect);
    let rects: Vec<egui::Rect> = draw_points.iter().map(|p|{
        make_handle_rect(*p, size, canvas_context)
    }).collect();

    // paint rects

    if canvas_context.is_node_selected(current_path) {
        color = egui::Color32::LIGHT_RED;
    }

    for rect in rects.iter() {
        painter.rect_filled(*rect, 0.0, color);
    }

    // handle inputs

    let responses: Vec<egui::Response> = rects.iter().enumerate().map(|(index, r)|{
        make_handle_response(ui, canvas_rect, *r, current_path, index)
    }).collect();

    let shift_held = ui.input(|i| i.modifiers.shift);

    let changed= responses.into_iter().enumerate().any(|(index, resp)|{
        if resp.clicked_by(egui::PointerButton::Primary) {
            let command = if shift_held {
                Command::add_to_selection(current_path.clone())
            } else {
                Command::select_node(current_path.clone())
            };

            messages.push_command(command);
        }

        if resp.dragged_by(egui::PointerButton::Primary) {
            drag_position(positions[index], resp, canvas_context);
            true
        } else {
            false
        }
    });

    changed
}

fn make_handle_rect(
    center: egui::Pos2,
    size: f32,
    canvas_context: &CanvasContext
) -> egui::Rect {
    egui::Rect::from_center_size(center, egui::Vec2::splat(size * canvas_context.camera_zoom()))
}

fn make_handle_response(
    ui: &mut egui::Ui,
    canvas_rect: egui::Rect,
    target_rect: egui::Rect,
    current_path: &NodePath,
    index: usize, // for hashing
) -> egui::Response {
    ui.interact(
        canvas_rect.intersect(target_rect),
        egui::Id::new(current_path).with(index),
        egui::Sense::click_and_drag()
    )
}

fn drag_world_delta(response: egui::Response, canvas_context: &CanvasContext) -> egui::Vec2 {
    response.drag_delta() / canvas_context.camera_zoom()
}

fn drag_position(value: &mut Vec2f, response: egui::Response, canvas_context: &CanvasContext) {
    let world_delta = drag_world_delta(response, canvas_context);

    value.x += world_delta.x;
    value.y -= world_delta.y;
}
