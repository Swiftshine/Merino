use strum::IntoEnumIterator;

use crate::merino::{
    archive_viewer::level_editor::{
        LevelEditor,
        contexts::{
            canvas_context::{CanvasContext, CanvasTarget},
            parameter_context::AddObjectMode,
        },
    },
    game::mapbin::{
        CollisionLine, MapDataNode, NodeChildType, NodeData,
        types::{String16, String32, Vec2f, Vec3f},
    },
};

impl LevelEditor {
    fn show_database_search_ui(&mut self, ui: &mut egui::Ui) {
        if self.parameter_context.parameter_objects().is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.label(egui::RichText::new("Object database not loaded.").strong());

                ui.label("Download or load objectdata.json to enable searchable object placement.");
            });

            return;
        }

        ui.add(
            egui::TextEdit::singleline(self.parameter_context.search_query_mut())
                .hint_text("Search for an object"),
        );

        ui.separator();

        let search = self.parameter_context.search_query().trim().to_lowercase();

        let mut found_any = false;
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                egui::Grid::new("object_search_grid")
                    .striped(true)
                    .spacing([16.0, 4.0])
                    .min_col_width(120.0)
                    .show(ui, |ui| {
                        ui.strong("Display Name");
                        ui.strong("Raw Name");
                        ui.strong("Type");
                        ui.end_row();

                        for param_obj in self.parameter_context.parameter_objects() {
                            let canon_name = param_obj.name.to_lowercase();

                            let display_name = param_obj
                                .display_name
                                .as_deref()
                                .unwrap_or("")
                                .to_lowercase();

                            let matches = search.is_empty()
                                || canon_name.contains(&search)
                                || display_name.contains(&search);

                            if !matches {
                                continue;
                            }

                            found_any = true;

                            let display =
                                param_obj.display_name.as_deref().unwrap_or(&param_obj.name);

                            let clicked = ui.selectable_label(false, display).clicked();

                            ui.label(&param_obj.name);

                            ui.label(param_obj.set_type.to_string());

                            ui.end_row();

                            if clicked {
                                let Ok(child_type) = NodeChildType::try_from(param_obj.set_type)
                                else {
                                    continue;
                                };

                                self.canvas_context.set_target(Some(
                                    CanvasTarget::new_named_to_root(child_type, canon_name),
                                ));
                            }
                        }
                    });
            });

        if !found_any {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("No results.").weak());
            });
        }
    }

    fn show_blank_object_ui(&mut self, ui: &mut egui::Ui) {
        for child_type in NodeChildType::iter() {
            if ui
                .add_sized(
                    [ui.available_width(), 30.0],
                    egui::Button::new(child_type.to_string()),
                )
                .clicked()
            {
                self.canvas_context
                    .set_target(Some(CanvasTarget::new_to_root(child_type)));
            }
        }
    }

    pub fn show_add_object_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                self.parameter_context.add_object_mode_mut(),
                AddObjectMode::CreateBlank,
                "Create Blank",
            );

            ui.selectable_value(
                self.parameter_context.add_object_mode_mut(),
                AddObjectMode::SearchDatabase,
                "Search Database",
            );
        });

        ui.separator();

        match self.parameter_context.add_object_mode() {
            AddObjectMode::CreateBlank => {
                self.show_blank_object_ui(ui);
            }

            AddObjectMode::SearchDatabase => {
                self.show_database_search_ui(ui);
            }
        }
    }

    pub fn add_object(
        &mut self,
        ui: &mut egui::Ui,
        canvas_rect: egui::Rect,
        response: &egui::Response,
    ) {
        // take the target
        let target = self.canvas_context.take_target().unwrap();
        let mut placed = false;

        let painter = ui.painter_at(canvas_rect);

        if let Some(pointer_pos) = response.hover_pos() {
            // draw text
            let text_pos = pointer_pos - egui::Vec2::new(0.0, 8.0);
            let label = target.to_string();
            let galley =
                painter.layout_no_wrap(label, egui::FontId::monospace(12.0), egui::Color32::WHITE);
            let text_rect = egui::Align2::CENTER_BOTTOM.anchor_size(text_pos, galley.size());

            painter.rect_filled(
                text_rect.expand(2.0),
                2.0,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            );

            painter.galley(text_rect.min, galley, egui::Color32::WHITE);

            // draw crosshair
            draw_crosshair(painter, response);
        }

        // we already know the response is hovered
        if ui.ctx().input(|i| i.pointer.primary_released())
            && let Some(pointer_pos) = response.hover_pos()
        {
            // offset to make it align with the mouse
            let local_pos = (pointer_pos - response.rect.min).to_pos2();
            let version = self.mapdata.as_ref().unwrap().version;

            match &target {
                CanvasTarget::NewNode(child_type, new_parent) => {
                    let parent = self
                        .mapdata
                        .as_mut()
                        .unwrap()
                        .get_node_at_path_mut(new_parent)
                        .unwrap();
                    parent.add_new_child(
                        version,
                        *child_type,
                        &mut self.canvas_context,
                        local_pos,
                        None,
                    );
                    placed = true;
                }

                CanvasTarget::NewNamedNode(child_type, name, new_parent) => {
                    let parent = self
                        .mapdata
                        .as_mut()
                        .unwrap()
                        .get_node_at_path_mut(new_parent)
                        .unwrap();

                    parent.add_new_child(
                        version,
                        *child_type,
                        &mut self.canvas_context,
                        local_pos,
                        Some(name.to_owned()),
                    );

                    placed = true;
                }

                _ => unreachable!(),
            }
        }

        // put it back if still needed
        if !placed {
            self.canvas_context.set_target(Some(target));
        }
    }
}

impl MapDataNode {
    fn add_new_child(
        &mut self,
        version: f32,
        child_type: NodeChildType,
        canvas_context: &mut CanvasContext,
        pointer_pos: egui::Pos2,
        object_name: Option<String>,
    ) {
        let pos = canvas_context.camera_to_world(pointer_pos.to_vec2());

        let mut node_data = match child_type {
            NodeChildType::MapPolySet => {
                let mut node_data = NodeData::default_mappolyset();

                let len = egui::Vec2::new(4.0, 0.0);

                if let NodeData::MapPolySet { line, .. } = &mut node_data {
                    line.start = pos.into();
                    line.end = (pos + len).into();
                }

                node_data
            }

            NodeChildType::MapObjSet
            | NodeChildType::MapItemSet
            | NodeChildType::MapEnemySet
            | NodeChildType::MapLocator => {
                let mut node_data = match child_type {
                    NodeChildType::MapObjSet => NodeData::default_mapobjset(version),
                    NodeChildType::MapItemSet => NodeData::default_mapitemset(version),
                    NodeChildType::MapEnemySet => NodeData::default_mapenemyset(version),
                    NodeChildType::MapLocator => NodeData::default_maplocator(),
                    _ => return,
                };

                let position = match &mut node_data {
                    NodeData::MapObjSet { position, .. } => position,
                    NodeData::MapItemSet { position, .. } => position,
                    NodeData::MapEnemySet { position, .. } => position,
                    NodeData::MapLocator { position, .. } => position,
                    _ => return,
                };

                *position = pos.into();

                node_data
            }

            NodeChildType::MapPath => {
                let mut node_data = NodeData::default_mappath();

                let len = egui::Vec2::new(4.0, 0.0);

                if let NodeData::MapPath { points, .. } = &mut node_data {
                    points.push(pos.into());
                    points.push((pos + len).into())
                }

                node_data
            }

            NodeChildType::MapRect => {
                let mut node_data = NodeData::default_maprect();

                let size = egui::Vec2::splat(4.0);

                if let NodeData::MapRect {
                    bounds_start,
                    bounds_end,
                    ..
                } = &mut node_data
                {
                    *bounds_start = pos.into();
                    *bounds_end = (pos + size).into();
                }

                node_data
            }

            NodeChildType::MapCircle => {
                let mut node_data = NodeData::default_mapcircle();

                let default_radius = 4.0;

                if let NodeData::MapCircle {
                    position, radius, ..
                } = &mut node_data
                {
                    *position = pos.into();
                    *radius = default_radius;
                }

                node_data
            }

            NodeChildType::MapTerrain => {
                let mut node_data = NodeData::default_mapterrain(version);

                if let NodeData::MapTerrain {
                    position, lines, ..
                } = &mut node_data
                {
                    *position = pos.into();

                    // make the default line below the position of the central node
                    let mut line = CollisionLine::default();

                    let line_length = 4.0;
                    let half = line_length * 0.5;
                    let base = egui::Vec2::new(pos.x, pos.y - 4.0);
                    let offset = egui::Vec2::new(half, 0.0);

                    line.start = (base - offset).into();
                    line.end = (base + offset).into();

                    line.calculate_collision_normal();

                    lines.push(line);
                }

                node_data
            }
        };

        // optionally apply name
        if let Some(object_name) = object_name {
            match &mut node_data {
                NodeData::MapObjSet { name, .. } => *name = object_name.into(),
                NodeData::MapItemSet { name, .. } => *name = object_name.into(),
                NodeData::MapEnemySet { name, .. } => *name = object_name.into(),
                NodeData::MapLocator { name, .. } => *name = object_name.into(),
                NodeData::MapPath { name, .. } => *name = object_name.into(),
                NodeData::MapRect { name, .. } => *name = object_name.into(),
                NodeData::MapCircle { name, .. } => *name = object_name.into(),
                _ => {}
            }
        }

        self.push_child_node(child_type, node_data);
    }

    fn push_child_node(&mut self, child_type: NodeChildType, node_data: NodeData) {
        let child = Self {
            node_type: child_type.into(),
            node_data,
            ..Default::default()
        };

        self.children_vec_option_mut(child_type)
            .get_or_insert_with(Vec::new)
            .push(child);
    }
}

impl NodeData {
    pub fn default_mappolyset() -> Self {
        Self::MapPolySet {
            line: CollisionLine::default(),
            collision_type: Default::default(),
            unk3: 0,
        }
    }

    pub fn default_mapobjset(version: f32) -> Self {
        Self::MapObjSet {
            name: Default::default(),
            position: Default::default(),
            unk3: 0.0,
            unk4: Default::default(),
            unk5: Default::default(),
            unk6: (version >= 4.43).then_some(0),
            unk7: (version >= 4.44).then(String32::default),
            unk8: Default::default(),
            unk9: Default::default(),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            unk13: (version >= 4.71).then_some(0),
            params: Default::default(),
            unk14: (version >= 4.50).then(Default::default),
        }
    }

    pub fn default_mapitemset(version: f32) -> Self {
        Self::MapItemSet {
            name: Default::default(),
            position: Default::default(),
            unk3: 0.0,
            unk4: Default::default(),
            unk5: Default::default(),
            unk6: (version >= 4.43).then_some(0),
            unk7: (version >= 4.44).then(String32::default),
            unk8: Default::default(),
            unk9: Default::default(),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            unk13: (version >= 4.71).then_some(0),
            params: Default::default(),
        }
    }

    pub fn default_mapenemyset(version: f32) -> Self {
        Self::MapEnemySet {
            name: Default::default(),
            direction: Default::default(),
            orientation: Default::default(),
            position: Default::default(),
            unk7: (version >= 4.45).then(String32::default),
            unk8: (version < 4.43).then(String16::default),
            unk9: (version < 4.43).then(String16::default),
            unk10: (version < 4.43).then(String32::default),
            unk11: (version < 4.43).then_some(0),
            unk12: (version < 4.43).then_some(0),
            unk13: 0,
            unk14: (version >= 4.42).then_some(0),
            unk15: (version >= 4.44).then(String32::default),
            unk16: 0.0,
            unk17: 0.0,
            unk18: 0.0,
            unk19: 0.0,
            unk20: (version >= 4.71).then_some(0),
            unk21: (version >= 4.71).then_some(0),
            unk22: (version >= 4.71).then_some(0),
            unk23: (version >= 4.71).then_some(0),
            unk24: (version >= 4.72).then_some(0),
            params: Default::default(),
        }
    }

    pub fn default_maplocator() -> Self {
        Self::MapLocator {
            name: Default::default(),
            position: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_mappath() -> Self {
        Self::MapPath {
            name: Default::default(),
            points: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_maprect() -> Self {
        Self::MapRect {
            name: Default::default(),
            bounds_start: Default::default(),
            bounds_end: Default::default(),
            params: Default::default(),
        }
    }

    pub fn default_mapcircle() -> Self {
        Self::MapCircle {
            name: Default::default(),
            position: Default::default(),
            radius: 0.0,
            params: Default::default(),
        }
    }

    pub fn default_mapterrain(version: f32) -> Self {
        Self::MapTerrain {
            collision_type: Default::default(),
            position: Default::default(),
            unk3: (version >= 4.43).then_some(0),
            unk4: (version >= 4.44).then(String32::default),
            unk5: 0.0,
            unk6: 0.0,
            unk7: 0.0,
            unk8: 0.0,
            unk9: (version >= 4.71).then_some(0),
            unk10: (version >= 4.71).then_some(0),
            unk11: (version >= 4.71).then_some(0),
            unk12: (version >= 4.71).then_some(0),
            lines: Default::default(),
            params: Default::default(),
            unk15: (version >= 4.6).then(Default::default),
        }
    }
}

// helpers

pub fn draw_crosshair(painter: egui::Painter, response: &egui::Response) {
    if let Some(pointer_pos) = response.hover_pos() {
        // circle
        painter.circle_filled(pointer_pos, 1.0, egui::Color32::GRAY);
        let crosshair_size = 10.0;

        // horizontal line
        painter.line_segment(
            [
                pointer_pos - egui::vec2(crosshair_size, 0.0),
                pointer_pos + egui::vec2(crosshair_size, 0.0),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );

        // vertical line
        painter.line_segment(
            [
                pointer_pos - egui::vec2(0.0, crosshair_size),
                pointer_pos + egui::vec2(0.0, crosshair_size),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }
}

impl From<egui::Vec2> for Vec3f {
    fn from(value: egui::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0.0,
        }
    }
}

impl From<egui::Vec2> for Vec2f {
    fn from(value: egui::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
