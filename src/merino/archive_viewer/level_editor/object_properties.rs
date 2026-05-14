use crate::merino::archive_viewer::level_editor::editable::EditInfo;
use crate::merino::archive_viewer::level_editor::editable::Editable;
use crate::merino::game::mapbin::MapDataNode;
use crate::merino::game::mapbin::recalculate_collision_normal;
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
                node.edit_propertes(ui);
            });

        // todo! show children
    }
}

impl MapDataNode {
    pub fn edit_propertes(&mut self, ui: &mut egui::Ui) {
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
                start,
                end,
                collision_normal,
                collision_type,
                unk3,
            } => {
                // not allowing user to edit collision normal directly
                // because that is to be auto-calculated

                let mut changed = false;

                changed |= start.edit_properties(ui, EditInfo::label("Start"));
                changed |= end.edit_properties(ui, EditInfo::label("End"));

                if changed {
                    recalculate_collision_normal(collision_normal, *start, *end);
                }

                edit_fields!(ui,
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
                    params => "Params",
                    unk14 => "Unk 14"
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
                    params => "Params"
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
                    params => "Params"
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
                    params => "Params"
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
                    params => "Params"
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
                    params => "Params"
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
                    params => "Params"
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
                unk13,
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
                    unk13 => "Unk 13",
                    params => "Params",
                    unk15 => "Unk 15"
                );
            }

            _ => unreachable!(),
        }
    }
}
