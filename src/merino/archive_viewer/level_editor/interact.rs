use crate::merino::{archive_viewer::level_editor::{LevelEditor, contexts::{canvas_context::CanvasContext, message_context::{Command, MessageContext}}}, game::mapbin::{MapDataNode, MapNodeType, NodeData, NodePath, types::{AnyParams, Vec2Like, Vec2f}}};

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
                MapNodeType::MapObjSet
                | MapNodeType::MapItemSet
                | MapNodeType::MapEnemySet
                | MapNodeType::MapLocator
                | MapNodeType::MapTerrain => {
                    self.interact_basic(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit
                    );
                },
                MapNodeType::MapPath => {},
                MapNodeType::MapRect => {},
                MapNodeType::MapCircle => {},
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

    /// "basic node" as in a node that contains position and parameter data
    fn interact_basic(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &mut NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        can_edit: bool,
    ) {
        let (name, position, _params, color) = match &mut self.node_data {
            NodeData::MapObjSet { name, position, params, .. } => {
                (name.as_str(), position, AnyParams::from(&*params), egui::Color32::WHITE)
            }
            
            NodeData::MapItemSet { name, position, params, .. } => {
                (name.as_str(), position, AnyParams::from(&*params), egui::Color32::GOLD)
            }

            NodeData::MapLocator { name, position, params, .. } => {
                (name.as_str(), position, AnyParams::from(&*params), egui::Color32::LIGHT_BLUE)
            }

            NodeData::MapEnemySet { name, position, params, .. } => {
                (name.as_str(), position, AnyParams::from(&*params), egui::Color32::RED)
            }

            NodeData::MapTerrain { collision_type, position, params, .. } => {
                (collision_type.as_str(), position, AnyParams::from(&*params), egui::Color32::LIGHT_GREEN)
            }
            _ => return,
        };

        let selection_highlight = egui::Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 0x10);
        let square_size = 0.7;

        let draw_pos = canvas_rect.min + canvas_context.convert_to_camera(Vec2f::from(*position).into());
        let mut positions = [position];
        let (_, rects, responses) = handle_drag_and_selections(
            ui,
            &[draw_pos],
            &mut positions,
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            square_size,
            false,
            color,
            can_edit,
        );

        // draw name
        let painter = ui.painter_at(canvas_rect);

        let rect = rects[0];
        let response = &responses[0];

        let selected = canvas_context.is_node_selected(current_path);

        if response.hovered() || selected {
            let text_pos = rect.center_top() - egui::Vec2::new(0.0, 5.0);

            let label = if !name.is_empty() { name.to_string() } else { String::from("<unnamed>") };
            let galley = painter.layout_no_wrap(label, egui::FontId::monospace(12.0), color);

            let text_rect = egui::Align2::CENTER_BOTTOM.anchor_size(text_pos, galley.size());

            painter.rect_filled(
                text_rect.expand(2.0),
                2.0,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100)
            );

            painter.galley(text_rect.min, galley, color);
        }

        if selected {
            painter.rect_filled(rect, 0.0, selection_highlight);
        }
    }

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

        let points = [draw_start, draw_end];

        let (changed, _, _) = handle_drag_and_selections(
            ui,
            &points,
            &mut [start, end],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            0.5,
            true,
            color,
            can_edit
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
/// Returns if any changes were made, as well as any rects and responses that were created.
fn handle_drag_and_selections<T: Vec2Like>(
    ui: &mut egui::Ui,
    draw_points: &[egui::Pos2],
    positions: &mut [&mut T],
    canvas_rect: egui::Rect,
    current_path: &NodePath,
    canvas_context: &mut CanvasContext,
    messages: &mut MessageContext,
    size: f32,
    fill_in: bool,
    mut color: egui::Color32,
    can_drag: bool,
) -> (bool, Vec<egui::Rect>, Vec<egui::Response>) {
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
        if fill_in {
            painter.rect_filled(*rect, 0.0, color);
        } else {
            painter.rect_stroke(*rect, 0.0, egui::Stroke::new(1.0, color), egui::StrokeKind::Middle);
        }
    }

    // handle inputs

    let responses: Vec<egui::Response> = rects.iter().enumerate().map(|(index, r)|{
        make_handle_response(ui, canvas_rect, *r, current_path, index)
    }).collect();

    let shift_held = ui.input(|i| i.modifiers.shift);

    let changed= responses.iter().enumerate().any(|(index, resp)|{
        if resp.clicked_by(egui::PointerButton::Primary) {
            let command = if shift_held {
                Command::add_to_selection(current_path.clone())
            } else {
                Command::select_node(current_path.clone())
            };

            messages.push_command(command);
        }

        if resp.dragged_by(egui::PointerButton::Primary) && can_drag {
            drag_position(positions[index], resp, canvas_context);
            true
        } else {
            false
        }
    });

    (changed, rects, responses)
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

fn drag_world_delta(response: &egui::Response, canvas_context: &CanvasContext) -> egui::Vec2 {
    response.drag_delta() / canvas_context.camera_zoom()
}

fn drag_position<T: Vec2Like>(
    value: &mut T,
    response: &egui::Response,
    canvas_context: &CanvasContext,
) {
    let world_delta = drag_world_delta(response, canvas_context);

    *value.x_mut() += world_delta.x;
    *value.y_mut() -= world_delta.y;
}
