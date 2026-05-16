use crate::merino::{
    archive_viewer::level_editor::{
        LevelEditor,
        contexts::{
            canvas_context::CanvasContext,
            message_context::{Command, MessageContext},
        },
    },
    game::mapbin::{
        MapDataNode, MapNodeType, NodeData, NodePath,
        types::{AnyParams, Vec2Like, Vec2f},
    },
};

pub const SELECTION_HIGHLIGHT: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(0xFF, 0xFF, 0xFF, 0x10);

impl LevelEditor {
    pub fn interact_with_all_nodes(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect, canvas_response: &egui::Response) {
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
            message_context,
            canvas_response
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
        canvas_response: &egui::Response,
    ) {
        // process self if we're allowed to do that
        if canvas_context.can_view(self.node_type) {
            let can_edit = canvas_context.can_edit(self.node_type);

            match self.node_type {
                MapNodeType::MapSet | MapNodeType::MapRect => {
                    self.interact_rect(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        canvas_response
                    );
                }
                MapNodeType::MapPolySet => {
                    self.interact_mappolyset(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        egui::Color32::WHITE,
                        canvas_response
                    );
                }
                MapNodeType::MapObjSet
                | MapNodeType::MapItemSet
                | MapNodeType::MapEnemySet
                | MapNodeType::MapLocator => {
                    self.interact_basic(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        canvas_response
                    );
                }
                MapNodeType::MapPath => {
                    self.interact_mappath(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        egui::Color32::from_rgb(0x31, 0x5C, 0x2B),
                        canvas_response
                    );
                }
                MapNodeType::MapCircle => {
                    self.interact_mapcircle(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        egui::Color32::PURPLE,
                        canvas_response
                    );
                }
                MapNodeType::MapTerrain => {
                    self.interact_mapterrain(
                        ui,
                        canvas_rect,
                        current_path,
                        canvas_context,
                        messages,
                        can_edit,
                        egui::Color32::LIGHT_GREEN,
                        canvas_response
                    );
                }
            }
        }

        // process children
        for (step, child) in self.iter_mut() {
            current_path.push(step);
            child.interact(ui, canvas_rect, current_path, canvas_context, messages, canvas_response);
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
        canvas_response: &egui::Response,
    ) {
        let (name, position, _params, color) = match &mut self.node_data {
            NodeData::MapObjSet {
                name,
                position,
                params,
                ..
            } => (
                name.as_str(),
                position,
                AnyParams::from(&*params),
                egui::Color32::WHITE,
            ),

            NodeData::MapItemSet {
                name,
                position,
                params,
                ..
            } => (
                name.as_str(),
                position,
                AnyParams::from(&*params),
                egui::Color32::GOLD,
            ),

            NodeData::MapLocator {
                name,
                position,
                params,
                ..
            } => (
                name.as_str(),
                position,
                AnyParams::from(&*params),
                egui::Color32::LIGHT_BLUE,
            ),

            NodeData::MapEnemySet {
                name,
                position,
                params,
                ..
            } => (
                name.as_str(),
                position,
                AnyParams::from(&*params),
                egui::Color32::RED,
            ),

            NodeData::MapTerrain {
                collision_type,
                position,
                params,
                ..
            } => (
                collision_type.as_str(),
                position,
                AnyParams::from(&*params),
                egui::Color32::LIGHT_GREEN,
            ),
            _ => return,
        };

        let square_size = 1.0;

        let draw_pos =
            canvas_rect.min + canvas_context.convert_to_camera(Vec2f::from(*position).into());
        let (_, rects, responses) = handle_drag_and_selections(
            ui,
            &[draw_pos],
            &mut [position],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            square_size,
            false,
            color,
            can_edit,
            &current_path,
            canvas_response
        );

        // draw name
        let painter = ui.painter_at(canvas_rect);

        let rect = rects[0];
        let response = &responses[0];

        let selected = canvas_context.is_node_selected(current_path);

        if response.hovered() || selected {
            draw_text_above_point(ui, canvas_rect, rect, name, color);
        }

        if selected {
            painter.rect_filled(rect, 0.0, SELECTION_HIGHLIGHT);
        }
    }

    fn interact_rect(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        do_edit: bool,
        canvas_response: &egui::Response,
    ) {
        let (start, end, color) = {
            match &mut self.node_data {
                NodeData::MapSet {
                    bounds_start,
                    bounds_end,
                    ..
                } => (bounds_start, bounds_end, egui::Color32::LIGHT_GRAY),

                NodeData::MapRect {
                    bounds_start,
                    bounds_end,
                    ..
                } => (
                    bounds_start,
                    bounds_end,
                    egui::Color32::from_rgb(0x6E, 0x7D, 0xAB),
                ),

                _ => return,
            }
        };

        let painter = ui.painter_at(canvas_rect);
        let draw_bounds_start = canvas_rect.min + canvas_context.convert_to_camera(start.into());
        let draw_bounds_end = canvas_rect.min + canvas_context.convert_to_camera(end.into());

        let rect = egui::Rect::from_two_pos(draw_bounds_start, draw_bounds_end);

        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.0, color),
            egui::StrokeKind::Middle,
        );

        handle_drag_and_selections(
            ui,
            &[draw_bounds_start, draw_bounds_end],
            &mut [start, end],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            0.3,
            true,
            color,
            do_edit,
            &current_path,
            canvas_response
        );
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
        canvas_response: &egui::Response,
    ) {
        let NodeData::MapPolySet { line, .. } = &mut self.node_data else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);

        let draw_start = canvas_rect.min + canvas_context.convert_to_camera(line.start.into());
        let draw_end = canvas_rect.min + canvas_context.convert_to_camera(line.end.into());

        painter.line_segment([draw_start, draw_end], egui::Stroke::new(0.5, color));

        let points = [draw_start, draw_end];

        let (changed, _, _) = handle_drag_and_selections(
            ui,
            &points,
            &mut [&mut line.start, &mut line.end],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            0.5,
            true,
            color,
            can_edit,
            &current_path,
            canvas_response
        );

        if changed {
            // update collision normals
            line.calculate_collision_normal();
        }
    }

    fn interact_mappath(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        can_edit: bool,
        color: egui::Color32,
        canvas_response: &egui::Response,
    ) {
        let NodeData::MapPath { name, points, .. } = &mut self.node_data else {
            return;
        };

        assert!(points.len() >= 2);

        let painter = ui.painter_at(canvas_rect);

        let draw_points: Vec<egui::Pos2> = points
            .iter()
            .map(|point| canvas_rect.min + canvas_context.convert_to_camera(point.into()))
            .collect();

        // draw lines in between points
        for window in draw_points.windows(2) {
            painter.line_segment([window[0], window[1]], egui::Stroke::new(1.0, color));
        }

        let square_size = 0.5;
        let mut positions: Vec<&mut Vec2f> = points.iter_mut().collect();

        let (_, rects, responses) = handle_drag_and_selections(
            ui,
            &draw_points,
            positions.as_mut_slice(),
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            square_size,
            true,
            color,
            can_edit,
            &current_path,
            canvas_response
        );

        assert_eq!(rects.len(), responses.len());

        let selected = canvas_context.is_node_selected(current_path);

        if selected {
            // pick the first one for simplicity's sake
            let rect = rects[0];
            draw_text_above_point(ui, canvas_rect, rect, name.as_str(), color);
        } else {
            for (index, resp) in responses.iter().enumerate() {
                if resp.hovered() {
                    let rect = rects[index];
                    draw_text_above_point(ui, canvas_rect, rect, name.as_str(), color);
                    break; // don't render the name more than once
                }
            }
        }
    }

    fn interact_mapcircle(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        do_edit: bool,
        color: egui::Color32,
        canvas_response: &egui::Response,
    ) {
        let NodeData::MapCircle {
            name,
            position,
            radius,
            ..
        } = &mut self.node_data
        else {
            return;
        };

        let painter = ui.painter_at(canvas_rect);
        let draw_center = canvas_rect.min + canvas_context.convert_to_camera(position.into());
        let draw_radius = *radius * canvas_context.camera_zoom();

        painter.circle_stroke(draw_center, draw_radius, egui::Stroke::new(1.0, color));

        let square_size = 0.5;

        // center handle
        let (_, rects, responses) = handle_drag_and_selections(
            ui,
            &[draw_center],
            &mut [position],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            square_size,
            true,
            color,
            do_edit,
            &current_path,
            canvas_response
        );

        // radius handle
        let draw_radius_pos = egui::Pos2::new(draw_center.x + draw_radius, draw_center.y);

        let radius_rect = egui::Rect::from_center_size(
            draw_radius_pos,
            egui::Vec2::splat(square_size * canvas_context.camera_zoom()),
        );

        painter.rect_filled(radius_rect, 0.0, color);

        let radius_resp = ui.interact(
            canvas_rect.intersect(radius_rect),
            egui::Id::new(&current_path).with("radius"),
            egui::Sense::click_and_drag(),
        );

        if radius_resp.clicked_by(egui::PointerButton::Primary) {
            let shift_held = ui.input(|i| i.modifiers.shift);

            let command = if shift_held {
                Command::add_to_selection(current_path.clone())
            } else {
                Command::select_node(current_path.clone())
            };

            messages.push_command(command);
        }

        if radius_resp.dragged_by(egui::PointerButton::Primary) {
            let pointer = ui.input(|i| i.pointer.hover_pos());

            if let Some(pointer) = pointer {
                let dx = pointer.x - draw_center.x;
                let dy = pointer.y - draw_center.y;

                *radius = (dx * dx + dy * dy).sqrt() / canvas_context.camera_zoom();
            }
        }

        // draw text

        let center_rect = rects[0];
        let center_resp = &responses[0];
        let selected = canvas_context.is_node_selected(current_path);

        if center_resp.hovered() || radius_resp.hovered() || selected {
            draw_text_above_point(ui, canvas_rect, center_rect, name.as_str(), color);
        }

        if canvas_context.is_node_selected(current_path) {
            painter.circle_filled(draw_center, draw_radius, SELECTION_HIGHLIGHT);
            painter.rect_filled(radius_rect, 0.0, egui::Color32::LIGHT_RED);
        }
    }

    fn interact_mapterrain(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        current_path: &NodePath,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        can_edit: bool,
        color: egui::Color32,
        canvas_response: &egui::Response,
    ) {
        let NodeData::MapTerrain {
            collision_type,
            position,
            lines,
            ..
        } = &mut self.node_data
        else {
            return;
        };

        let draw_pos =
            canvas_rect.min + canvas_context.convert_to_camera(Vec2f::from(*position).into());

        let (_, rects, responses) = handle_drag_and_selections(
            ui,
            &[draw_pos],
            &mut [position],
            canvas_rect,
            current_path,
            canvas_context,
            messages,
            1.0,
            false,
            color,
            can_edit,
            &current_path,
            canvas_response
        );

        let position_rect = rects[0];
        let position_resp = &responses[0];

        if position_resp.hovered() || canvas_context.is_node_selected(current_path) {
            draw_text_above_point(
                ui,
                canvas_rect,
                position_rect,
                collision_type.as_str(),
                color,
            );
        }

        // render + edit lines

        let painter = ui.painter_at(canvas_rect);

        for (line_index, line) in lines.iter_mut().enumerate() {
            let draw_start = canvas_rect.min + canvas_context.convert_to_camera(line.start.into());

            let draw_end = canvas_rect.min + canvas_context.convert_to_camera(line.end.into());

            painter.line_segment([draw_start, draw_end], egui::Stroke::new(1.0, color));

            let points = [draw_start, draw_end];

            let (changed, _, _) = handle_drag_and_selections(
                ui,
                &points,
                &mut [&mut line.start, &mut line.end],
                canvas_rect,
                current_path,
                canvas_context,
                messages,
                0.5,
                true,
                color,
                can_edit,
                &(current_path, line_index),
                canvas_response
            );

            if changed {
                line.calculate_collision_normal();
            }
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
    id_source: &impl std::hash::Hash,
    canvas_response: &egui::Response,
) -> (bool, Vec<egui::Rect>, Vec<egui::Response>) {
    assert_eq!(draw_points.len(), positions.len());

    let painter = ui.painter_at(canvas_rect);
    let rects: Vec<egui::Rect> = draw_points
        .iter()
        .map(|p| make_handle_rect(*p, size, canvas_context))
        .collect();

    // paint rects

    if canvas_context.is_node_selected(current_path) {
        color = egui::Color32::LIGHT_RED;
    }

    for rect in rects.iter() {
        if fill_in {
            painter.rect_filled(*rect, 0.0, color);
        } else {
            painter.rect_stroke(
                *rect,
                0.0,
                egui::Stroke::new(1.0, color),
                egui::StrokeKind::Middle,
            );
        }
    }

    // handle inputs

    let responses: Vec<egui::Response> = rects
        .iter()
        .enumerate()
        .map(|(index, r)| {
            make_handle_response(ui, canvas_rect, *r, current_path, (id_source, index))
        })
        .collect();

    let shift_held = ui.input(|i| i.modifiers.shift);

    let changed = responses.iter().enumerate().any(|(index, resp)| {
        if resp.clicked_by(egui::PointerButton::Primary) {
            let command = if shift_held {
                Command::add_to_selection(current_path.clone())
            } else {
                Command::select_node(current_path.clone())
            };

            messages.push_command(command);
        }

        if resp.dragged_by(egui::PointerButton::Primary) && can_drag {
            drag_position(positions[index], resp, canvas_context, canvas_response);
            true
        } else {
            false
        }
    });

    (changed, rects, responses)
}

fn make_handle_rect(center: egui::Pos2, size: f32, canvas_context: &CanvasContext) -> egui::Rect {
    egui::Rect::from_center_size(
        center,
        egui::Vec2::splat(size * canvas_context.camera_zoom()),
    )
}

fn make_handle_response(
    ui: &mut egui::Ui,
    canvas_rect: egui::Rect,
    target_rect: egui::Rect,
    current_path: &NodePath,
    id_source: impl std::hash::Hash, // for hashing
) -> egui::Response {
    ui.interact(
        canvas_rect.intersect(target_rect),
        egui::Id::new(current_path).with(id_source),
        egui::Sense::click_and_drag(),
    )
}

fn drag_position<T: Vec2Like>(
    value: &mut T,
    response: &egui::Response,
    canvas_context: &CanvasContext,
    canvas_response: &egui::Response,
) {
    let grid_size = 0.5;

    let pointer = response.interact_pointer_pos().unwrap();
    let local_pos = pointer - canvas_response.rect.min;

    let world = canvas_context
        .convert_from_camera(local_pos)
        .to_pos2();

    let snapped = if canvas_context.snap_to_grid() {
        egui::pos2(snap(world.x, grid_size), snap(world.y, grid_size))
    } else {
        world
    };

    *value.x_mut() = snapped.x;
    *value.y_mut() = snapped.y;
}

fn snap(value: f32, grid: f32) -> f32 {
    (value / grid).round() * grid
}

fn draw_text_above_point(
    ui: &mut egui::Ui,
    canvas_rect: egui::Rect,
    target_rect: egui::Rect,
    text: &str,
    color: egui::Color32,
) {
    let painter = ui.painter_at(canvas_rect);
    let text_pos = target_rect.center_top() - egui::Vec2::new(0.0, 5.0);
    let label = if !text.is_empty() {
        text.to_string()
    } else {
        String::from("<unnamed>")
    };
    let galley = painter.layout_no_wrap(label, egui::FontId::monospace(12.0), color);
    let text_rect = egui::Align2::CENTER_BOTTOM.anchor_size(text_pos, galley.size());

    painter.rect_filled(
        text_rect.expand(2.0),
        2.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
    );

    painter.galley(text_rect.min, galley, color);
}
