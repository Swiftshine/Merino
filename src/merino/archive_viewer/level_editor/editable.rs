use crate::merino::{
    archive_viewer::level_editor::params::{ParameterDataType, ParameterObject},
    game::mapbin::{
        CollisionLine, MapNodeType,
        types::{LimitedString, Params, Vec2f, Vec3f},
    },
    util::emoji::EmojiMessage,
};

/// A trait to simplify property parsing.
pub trait Editable {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool;
}

pub enum EditInfo<'a> {
    Label(&'a str),
    Params(&'a ParameterObject),
}

impl<'a> EditInfo<'a> {
    pub fn label(label: &'a str) -> Option<Self> {
        Some(Self::Label(label))
    }

    pub fn search_param(
        list: &'a [ParameterObject],
        node_type: MapNodeType,
        name: &'a str,
    ) -> Option<Self> {
        list.iter()
            .find(|obj| obj.set_type == node_type && obj.name == name)
            .map(Self::Params)
    }
}

// actual trait implementations

impl<T> Editable for Option<T>
where
    T: Editable,
{
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        if let Some(val) = self {
            val.edit_properties(ui, info)
        } else {
            false
        }
    }
}

macro_rules! impl_editable_numeric {
    ($($t:ty),*) => {
        $(
            impl Editable for $t {
                fn edit_properties(
                    &mut self,
                    ui: &mut egui::Ui,
                    info: Option<EditInfo>,
                ) -> bool {
                    let mut changed = false;

                    let render = |ui: &mut egui::Ui,
                                  value: &mut $t,
                                  changed: &mut bool| {
                        *changed |= ui.add(
                            egui::DragValue::new(value)
                                .speed(1.0)
                                .range(<$t>::MIN..=<$t>::MAX)
                        ).changed();
                    };

                    if let Some(EditInfo::Label(label)) = info {
                        ui.collapsing(label, |ui| {
                            ui.horizontal(|ui| {
                                render(ui, self, &mut changed);
                            });
                        });
                    } else {
                        render(ui, self, &mut changed);
                    }

                    changed
                }
            }
        )*
    };
}

impl_editable_numeric!(u32, i32, f32);

macro_rules! impl_editable_vec {
    ($t:ty, [$($field:ident),*]) => {
        impl Editable for $t {
            fn edit_properties(
                &mut self,
                ui: &mut egui::Ui,
                info: Option<EditInfo>,
            ) -> bool {
                let mut changed = false;

                let render = |ui: &mut egui::Ui,
                              value: &mut $t,
                              changed: &mut bool| {
                    ui.horizontal(|ui| {
                        $(
                            ui.label(stringify!($field).to_uppercase());

                            *changed |= ui.add(
                                egui::DragValue::new(&mut value.$field)
                                    .speed(0.5)
                                    .range(f32::MIN..=f32::MAX)
                            ).changed();
                        )*
                    });
                };

                if let Some(EditInfo::Label(label)) = info {
                    ui.collapsing(label, |ui| {
                        render(ui, self, &mut changed);
                    });
                } else {
                    render(ui, self, &mut changed);
                }

                changed
            }
        }
    };
}

impl_editable_vec!(Vec2f, [x, y]);
impl_editable_vec!(Vec3f, [x, y, z]);

impl<const N: usize> Editable for LimitedString<N> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        let render = |ui: &mut egui::Ui, value: &mut LimitedString<N>, changed: &mut bool| {
            *changed |= ui
                .add(egui::TextEdit::singleline(&mut value.0).char_limit(N))
                .changed();
        };

        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                render(ui, self, &mut changed);
            });
        } else {
            render(ui, self, &mut changed);
        }

        changed
    }
}

impl<const N: usize> Editable for Params<N> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        if let Some(EditInfo::Params(param_object)) = info {
            ui.collapsing("Parameters", |ui| {
                for param in param_object.parameters.iter() {
                    let mut resp = ui.collapsing(&param.name, |ui| {
                        if let Some(desc) = &param.description
                            && !desc.is_empty()
                        {
                            ui.label(desc);
                        }

                        match &param.data_type {
                            ParameterDataType::Int => {
                                if let Some(val) = self.int_values.get_mut(param.slot) {
                                    changed |= val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::Float => {
                                if let Some(val) = self.float_values.get_mut(param.slot) {
                                    changed |= val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::String => {
                                if let Some(val) = self.string_values.get_mut(param.slot) {
                                    changed |= val.edit_properties(ui, None);
                                }
                            }

                            ParameterDataType::Bool => {
                                if let Some(val) = self.int_values.get_mut(param.slot) {
                                    let mut bool_value = *val != 0;

                                    if ui.checkbox(&mut bool_value, "Value").changed() {
                                        *val = if bool_value { 1 } else { 0 };
                                        changed = true;
                                    }
                                }
                            }

                            ParameterDataType::DropdownInt => {
                                if let Some(options) = &param.dropdown_options
                                    && let Some(val) = self.int_values.get_mut(param.slot)
                                {
                                    let selected_text = options
                                        .iter()
                                        .find(|o| o.value == *val)
                                        .map(|o| format!("({}) {}", o.value, o.key))
                                        .unwrap_or_else(|| format!("Unknown ({})", *val));

                                    egui::ComboBox::from_label("Value")
                                        .selected_text(selected_text)
                                        .show_ui(ui, |ui| {
                                            for option in options.iter() {
                                                let label =
                                                    format!("({}) {}", option.value, option.key);

                                                changed |= ui
                                                    .selectable_value(val, option.value, label)
                                                    .changed();
                                            }
                                        });
                                }
                            }

                            _ => {}
                        }
                    });

                    if let Some(notes) = &param.notes
                        && !notes.is_empty()
                    {
                        let tooltip = if notes.len() == 1 {
                            notes[0].clone()
                        } else {
                            // bullet points
                            notes
                                .iter()
                                .map(|n| format!("• {n}"))
                                .collect::<Vec<_>>()
                                .join("\n")
                        };

                        resp.header_response = resp.header_response.on_hover_text(tooltip);
                    }
                }
            });
        }

        ui.collapsing("Raw Parameters", |ui| {
            ui.label("Int Params");

            ui.horizontal(|ui| {
                for val in self.int_values.iter_mut() {
                    changed |= val.edit_properties(ui, None);
                }
            });

            ui.label("Float Params");

            ui.horizontal(|ui| {
                for val in self.float_values.iter_mut() {
                    changed |= val.edit_properties(ui, None);
                }
            });

            ui.label("String Params");

            for val in self.string_values.iter_mut() {
                changed |= val.edit_properties(ui, None);
            }
        });

        changed
    }
}

impl<T, const N: usize> Editable for [T; N]
where
    T: Editable,
{
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        let render = |ui: &mut egui::Ui, values: &mut [T; N], changed: &mut bool| {
            for item in values.iter_mut() {
                *changed |= item.edit_properties(ui, None);
            }
        };

        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                render(ui, self, &mut changed);
            });
        } else {
            render(ui, self, &mut changed);
        }

        changed
    }
}

impl Editable for Vec<Vec2f> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        let render = |ui: &mut egui::Ui, values: &mut Vec<Vec2f>, changed: &mut bool| {
            let mut remove_index = None;
            let mut insert_index = None;

            let can_remove = values.len() > 2;

            for (index, value) in values.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("[{}]", index));

                    *changed |= value.edit_properties(ui, None);

                    if ui
                        .button(EmojiMessage::add())
                        .on_hover_text("Insert after")
                        .clicked()
                    {
                        insert_index = Some(index + 1);
                    }

                    if ui
                        .add_enabled(can_remove, egui::Button::new(EmojiMessage::cross()))
                        .on_disabled_hover_text("Remove")
                        .on_hover_text("Remove")
                        .clicked()
                    {
                        remove_index = Some(index);
                    }
                });
            }

            // removal
            if let Some(index) = remove_index {
                values.remove(index);
                *changed = true;
            }

            // insertion
            if let Some(index) = insert_index {
                let new_point = values
                    .get(index.saturating_sub(1))
                    .or_else(|| values.last())
                    .map(|prev| Vec2f {
                        x: prev.x + 4.0,
                        y: prev.y,
                    })
                    .unwrap_or(Vec2f { x: 0.0, y: 0.0 });

                values.insert(index, new_point);
                *changed = true;
            }

            ui.separator();

            // append
            if ui.button(EmojiMessage::add_msg("Add Point")).clicked() {
                let new_point = values
                    .last()
                    .map(|last| Vec2f {
                        x: last.x + 4.0,
                        y: last.y,
                    })
                    .unwrap_or(Vec2f { x: 0.0, y: 0.0 });

                values.push(new_point);
                *changed = true;
            }
        };

        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                render(ui, self, &mut changed);
            });
        } else {
            render(ui, self, &mut changed);
        }

        changed
    }
}

impl Editable for CollisionLine {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        let render = |ui: &mut egui::Ui, value: &mut CollisionLine, changed: &mut bool| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Start");
                    *changed |= value.start.edit_properties(ui, None);
                });

                ui.horizontal(|ui| {
                    ui.label("End  "); // extra two spaces for padding reasons
                    *changed |= value.end.edit_properties(ui, None);
                });
            });
        };

        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                render(ui, self, &mut changed);
            });
        } else {
            render(ui, self, &mut changed);
        }

        if changed {
            // don't allow user to modify manually
            self.calculate_collision_normal();
        }

        changed
    }
}

impl Editable for Vec<CollisionLine> {
    fn edit_properties(&mut self, ui: &mut egui::Ui, info: Option<EditInfo>) -> bool {
        let mut changed = false;

        let render = |ui: &mut egui::Ui, values: &mut Vec<CollisionLine>, changed: &mut bool| {
            let mut remove_index = None;
            let mut insert_index = None;

            let can_remove = values.len() > 1;

            for (index, value) in values.iter_mut().enumerate() {
                ui.push_id(index, |ui| {
                    let id = ui.id().with("line");

                    let mut open = ui
                        .ctx()
                        .data_mut(|d| d.get_persisted::<bool>(id))
                        .unwrap_or(false);

                    ui.horizontal(|ui| {
                        let arrow = if open { "⏷" } else { "⏵" };

                        if ui.button(arrow).clicked() {
                            open = !open;

                            ui.ctx().data_mut(|d| {
                                d.insert_persisted(id, open);
                            });
                        }

                        ui.label(format!("Line {}", index));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add_enabled(can_remove, egui::Button::new(EmojiMessage::cross()))
                                .on_hover_text("Remove")
                                .clicked()
                            {
                                remove_index = Some(index);
                            }

                            if ui
                                .button(EmojiMessage::add())
                                .on_hover_text("Insert after")
                                .clicked()
                            {
                                insert_index = Some(index + 1);
                            }
                        });
                    });

                    if open {
                        ui.indent("line_contents", |ui| {
                            *changed |= value.edit_properties(ui, None);
                        });
                    }

                    ui.separator();
                });
            }

            // remove
            if let Some(index) = remove_index {
                values.remove(index);
                *changed = true;
            }

            // insert
            if let Some(index) = insert_index {
                let new_line = values
                    .get(index.saturating_sub(1))
                    .map(|prev| CollisionLine {
                        start: prev.end,
                        end: Vec2f {
                            x: prev.end.x + 4.0,
                            y: prev.end.y,
                        },
                        ..Default::default()
                    })
                    .unwrap_or_default();

                values.insert(index, new_line);
                *changed = true;
            }

            // append
            if ui.button(EmojiMessage::add_msg("Add Line")).clicked() {
                let new_line = values
                    .last()
                    .map(|last| CollisionLine {
                        start: last.end,
                        end: Vec2f {
                            x: last.end.x + 4.0,
                            y: last.end.y,
                        },
                        ..Default::default()
                    })
                    .unwrap_or_default();

                values.push(new_line);
                *changed = true;
            }
        };

        if let Some(EditInfo::Label(label)) = info {
            ui.collapsing(label, |ui| {
                render(ui, self, &mut changed);
            });
        } else {
            render(ui, self, &mut changed);
        }

        changed
    }
}
