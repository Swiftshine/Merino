use strum::IntoEnumIterator;

use crate::merino::archive_viewer::level_editor::contexts::canvas_context::CanvasContext;
use crate::merino::archive_viewer::level_editor::contexts::canvas_context::CanvasTarget;
use crate::merino::archive_viewer::level_editor::contexts::message_context::MessageContext;
use crate::merino::archive_viewer::level_editor::editable::EditInfo;
use crate::merino::archive_viewer::level_editor::editable::Editable;
use crate::merino::archive_viewer::level_editor::params::ParameterObject;
use crate::merino::game::mapbin::MapDataNode;
use crate::merino::game::mapbin::NodeChildType;
use crate::merino::game::mapbin::NodePath;
use crate::merino::game::mapbin::NodeStep;

use crate::merino::{
    archive_viewer::level_editor::{LevelEditor, contexts::message_context::Command},
    game::mapbin::{MapNodeType, NodeData},
    util::emoji::EmojiMessage,
};

macro_rules! edit_fields {
    ($ui:expr, $( $field:expr => $label:expr ),* $(,)?) => {
        $(
            $field.edit_properties($ui, EditInfo::label($label));
        )*
    };
}

impl LevelEditor {
    pub fn show_object_properties(&mut self, ui: &mut egui::Ui) {
        if !self.canvas_context.can_edit_node_properties() {
            ui.centered_and_justified(|ui| {
                ui.label("Select exactly one node to edit its properties.");
            });

            return;
        }

        let path = self.canvas_context.selected_node_paths()[0].clone();

        let LevelEditor {
            mapdata,
            canvas_context,
            message_context,
            ..
        } = self;

        let messages = message_context;

        let Some(mapdata) = mapdata.as_mut() else {
            return;
        };

        let Some(node) = mapdata.get_node_at_path(&path) else {
            return;
        };

        // must have node data
        assert!(!matches!(node.node_data, NodeData::None));

        // properties heading
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Properties").strong());
            // - don't allow user to attempt to delete the root node
            // the root node doesn't have a parent

            if node.node_type != MapNodeType::MapSet {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(EmojiMessage::discard())
                        .on_hover_text("Delete node")
                        .clicked()
                    {
                        messages.push_command(Command::remove_node(path.clone()));
                    }

                    if ui
                        .button(EmojiMessage::target())
                        .on_hover_text("Go to parent")
                        .clicked()
                    {
                        messages.push_command(Command::select_parent_of(path.clone()));
                    }
                });
            }
        });

        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                node.edit_properties(ui, self.parameter_context.parameter_objects());
            });

        ui.add_space(4.0);
        node.edit_children(ui, canvas_context, messages, &path);
    }
}

impl MapDataNode {
    pub fn edit_properties(&mut self, ui: &mut egui::Ui, param_list: &[ParameterObject]) {
        match &mut self.node_data {
            NodeData::MapSet {
                unk1,
                bounds_start,
                bounds_end,
            } => {
                edit_fields!(ui,
                    unk1 => "Unk 1",
                    bounds_start => "Bounds Start",
                    bounds_end => "Bounds End"
                );
            }

            NodeData::MapPolySet {
                line,
                collision_type,
                unk3,
            } => {
                // not allowing user to edit collision normal directly
                // because that is to be auto-calculated
                edit_fields!(ui,
                    line => "Line",
                    collision_type => "Collision Type",
                    unk3 => "Unk 3"
                );
            }

            NodeData::MapObjSet {
                name,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
                unk14,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    position => "Position",
                    unk3 => "Unk 3",
                    unk4 => "Unk 4",
                    unk5 => "Unk 5",
                    unk6 => "Unk 6",
                    unk7 => "Unk 7",
                    unk8 => "Unk 8",
                    unk9 => "Unk 9",
                    unk10 => "Unk 10",
                    unk11 => "Unk 11",
                    unk12 => "Unk 12",
                    unk13 => "Unk 13",
                    unk14 => "Unk 14"
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapItemSet {
                name,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    position => "Position",
                    unk3 => "Unk 3",
                    unk4 => "Unk 4",
                    unk5 => "Unk 5",
                    unk6 => "Unk 6",
                    unk7 => "Unk 7",
                    unk8 => "Unk 8",
                    unk9 => "Unk 9",
                    unk10 => "Unk 10",
                    unk11 => "Unk 11",
                    unk12 => "Unk 12",
                    unk13 => "Unk 13",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapEnemySet {
                name,
                direction,
                orientation,
                position,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                unk14,
                unk15,
                unk16,
                unk17,
                unk18,
                unk19,
                unk20,
                unk21,
                unk22,
                unk23,
                unk24,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    direction => "Direction",
                    orientation => "Orientation",
                    position => "Position",
                    unk7 => "Unk 7",
                    unk8 => "Unk 8",
                    unk9 => "Unk 9",
                    unk10 => "Unk 10",
                    unk11 => "Unk 11",
                    unk12 => "Unk 12",
                    unk13 => "Unk 13",
                    unk14 => "Unk 14",
                    unk15 => "Unk 15",
                    unk16 => "Unk 16",
                    unk17 => "Unk 17",
                    unk18 => "Unk 18",
                    unk19 => "Unk 19",
                    unk20 => "Unk 20",
                    unk21 => "Unk 21",
                    unk22 => "Unk 22",
                    unk23 => "Unk 23",
                    unk24 => "Unk 24",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapLocator {
                name,
                position,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    position => "Position",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapPath {
                name,
                points,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    points => "Points",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapRect {
                name,
                bounds_start,
                bounds_end,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    bounds_start => "Bounds Start",
                    bounds_end => "Bounds End",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapCircle {
                name,
                position,
                radius,
                params,
            } => {
                edit_fields!(ui,
                    name => "Name",
                    position => "Position",
                    radius => "Radius",
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, name.as_str()),
                );
            }

            NodeData::MapTerrain {
                collision_type,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                lines,
                params,
                unk15,
            } => {
                edit_fields!(ui,
                    collision_type => "Collision Type",
                    position => "Position",
                    unk3 => "Unk 3",
                    unk4 => "Unk 4",
                    unk5 => "Unk 5",
                    unk6 => "Unk 6",
                    unk7 => "Unk 7",
                    unk8 => "Unk 8",
                    unk9 => "Unk 9",
                    unk10 => "Unk 10",
                    unk11 => "Unk 11",
                    unk12 => "Unk 12",
                    lines => "Lines",
                    unk15 => "Unk 15"
                );

                params.edit_properties(
                    ui,
                    EditInfo::search_param(param_list, self.node_type, collision_type.as_str()),
                );
            }

            _ => unreachable!(),
        }
    }

    fn edit_children(
        &mut self,
        ui: &mut egui::Ui,
        canvas_context: &mut CanvasContext,
        messages: &mut MessageContext,
        node_path: &NodePath,
    ) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Children").strong().underline())
                .on_hover_text(
                    "The parentheses indicate how many children of that type are present.",
                );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(EmojiMessage::target())
                    .on_hover_text("Make an existing node a child of this node")
                    .clicked()
                {
                    canvas_context.set_target(Some(CanvasTarget::search(node_path.clone())));
                }
            });
        });

        for child_type in NodeChildType::iter() {
            let children = self.children_vec_mut(child_type);
            let child_count = children.as_ref().map(|v| v.len()).unwrap_or(0);

            let header = format!("({}) {}", child_count, child_type);

            egui::CollapsingHeader::new(header)
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        match children {
                            None => {}

                            Some(children) => {
                                ui.indent(ui.id().with(child_type), |ui| {
                                    for (index, _) in children.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Index {}", index));

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui
                                                        .button(EmojiMessage::discard())
                                                        .on_hover_text("Delete child")
                                                        .clicked()
                                                    {
                                                        let child_path = node_path.with_step(
                                                            NodeStep::new(child_type, index),
                                                        );
                                                        messages.push_command(
                                                            Command::remove_node(child_path),
                                                        );
                                                    }

                                                    if ui
                                                        .button(EmojiMessage::cross())
                                                        .on_hover_text("Detach child")
                                                        .clicked()
                                                    {
                                                        let child_path = node_path.with_step(
                                                            NodeStep::new(child_type, index),
                                                        );
                                                        messages.push_command(
                                                            Command::make_child_of_root(child_path),
                                                        );
                                                    }

                                                    if ui
                                                        .button(EmojiMessage::target())
                                                        .on_hover_text("Go to child")
                                                        .clicked()
                                                    {
                                                        let child_path = node_path.with_step(
                                                            NodeStep::new(child_type, index),
                                                        );
                                                        messages.push_command(
                                                            Command::select_node(child_path),
                                                        );
                                                    }
                                                },
                                            );
                                        });
                                    }
                                });
                            }
                        }

                        ui.horizontal(|ui| {
                            if ui
                                .button(EmojiMessage::add_msg("New Child"))
                                .on_hover_text("Create a new node of this type.")
                                .clicked()
                            {
                                canvas_context.set_target(Some(CanvasTarget::new_to_node(
                                    child_type,
                                    node_path.clone(),
                                )));
                            }
                        });
                    });
                });
        }
    }
}
