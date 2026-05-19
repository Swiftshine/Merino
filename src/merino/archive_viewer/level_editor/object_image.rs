use anyhow::{Result, anyhow};
use std::{collections::HashMap, fs, str::FromStr};

use crate::merino::{
    archive_viewer::level_editor::{LevelEditor, params::ParameterDataType},
    game::mapbin::{MapNodeType, types::AnyParams},
    util::res_folder::get_merino_folder,
};

const IMAGEDATA_FILE: &str = "imagedata.json";

#[derive(Debug, Clone)]
pub struct VariantCondition {
    pub data_type: ParameterDataType,
    pub slot: usize,
    pub expected_value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct CopySource {
    pub data_type: ParameterDataType,
    pub slot: usize,
}

#[derive(Debug, Clone)]
pub enum VariantAction {
    SwapImage { display_image: String },

    RotateFromParam { source: CopySource },
}

#[derive(Debug, Clone)]
pub struct ImageVariant {
    pub when: Option<VariantCondition>,
    pub action: VariantAction,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ImageAnchor {
    #[default]
    Center,
    TopLeft,
    TopCenter,
    TopRight,
    LeftCenter,
    RightCenter,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl ImageAnchor {
    pub fn from_str(value: &str) -> Result<Self> {
        Ok(match value {
            "center" => Self::Center,
            "top_left" => Self::TopLeft,
            "top_center" => Self::TopCenter,
            "top_right" => Self::TopRight,
            "left_center" => Self::LeftCenter,
            "right_center" => Self::RightCenter,
            "bottom_left" => Self::BottomLeft,
            "bottom_center" => Self::BottomCenter,
            "bottom_right" => Self::BottomRight,

            _ => {
                return Err(anyhow!("invalid image anchor `{}`", value));
            }
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImageDefinition {
    pub display_image: Option<String>,
    pub variants: Vec<ImageVariant>,
    pub anchor: ImageAnchor,
}

pub struct ResolvedImage {
    pub image_path: String,
    pub rotation_degrees: f32,
    pub anchor: ImageAnchor,
}

#[derive(Default)]
pub struct ImageBank {
    pub image_objects: HashMap<(MapNodeType, String), ImageDefinition>,
    textures: HashMap<String, egui::TextureHandle>,
}

impl ImageBank {
    pub fn load_texture(
        &mut self,
        ctx: &egui::Context,
        asset_id: &str,
        file_path: &str,
    ) -> Result<()> {
        if self.textures.contains_key(asset_id) {
            return Ok(());
        }

        let image = image::open(file_path)?.to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

        let texture = ctx.load_texture(asset_id, color_image, egui::TextureOptions::LINEAR);

        self.textures.insert(asset_id.to_string(), texture);

        Ok(())
    }

    pub fn resolve_image_for_node(
        &mut self,
        ctx: &egui::Context,
        node_type: MapNodeType,
        name: &str,
        params: &AnyParams<'_>, // Changed from &Params<N>
    ) -> Option<(egui::TextureHandle, f32, ImageAnchor)> {
        let def = self.image_objects.get(&(node_type, name.to_string()))?;

        let resolved = def.resolve(params)?;

        let asset_id = format!("{}/{}/{}", node_type, name, resolved.image_path);
        let base_path = get_merino_folder().ok()?;
        let file_path = base_path
            .join("image")
            .join(node_type.to_string())
            .join(&resolved.image_path);

        let file_path = file_path.to_string_lossy();
        let _ = self.load_texture(ctx, &asset_id, &file_path);
        let texture = self.textures.get(&asset_id)?;

        Some((texture.clone(), resolved.rotation_degrees, resolved.anchor))
    }

    fn clear_image_objects(&mut self) {
        self.image_objects.clear()
    }

    fn clear_textures(&mut self) {
        self.textures.clear()
    }

    fn clear_all(&mut self) {
        self.clear_image_objects();
        self.clear_textures();
    }

    pub fn insert_image_object(
        &mut self,
        k: (MapNodeType, String),
        v: ImageDefinition,
    ) -> Option<ImageDefinition> {
        self.image_objects.insert(k, v)
    }
}

impl ImageDefinition {
    pub fn resolve(&self, params: &AnyParams<'_>) -> Option<ResolvedImage> {
        let mut resolved = ResolvedImage {
            image_path: self.display_image.clone()?,
            rotation_degrees: 0.0,
            anchor: self.anchor,
        };

        for variant in &self.variants {
            let matched = match &variant.when {
                Some(condition) => evaluate_condition(condition, params),
                None => true,
            };

            if !matched {
                continue;
            }

            apply_variant_action(&variant.action, params, &mut resolved);
        }

        Some(resolved)
    }
}

impl ParameterDataType {
    pub fn matches_json_value(
        &self,
        expected: &serde_json::Value,
        params: &AnyParams<'_>,
        slot: usize,
    ) -> bool {
        match self {
            Self::Int | Self::DropdownInt => params
                .int_params()
                .get(slot)
                .map(|&val| expected.as_i64() == Some(val as i64))
                .unwrap_or(false),

            Self::Float => params
                .float_params()
                .get(slot)
                .map(|&val| expected.as_f64() == Some(val as f64))
                .unwrap_or(false),

            Self::String => params
                .string_params()
                .get(slot)
                .map(|val| expected.as_str() == Some(val.as_str()))
                .unwrap_or(false),

            Self::Bool => params
                .int_params()
                .get(slot)
                .map(|&val| expected.as_bool() == Some(val != 0))
                .unwrap_or(false),

            Self::None => false,
        }
    }

    pub fn extract_as_f32(&self, params: &AnyParams<'_>, slot: usize) -> Option<f32> {
        match self {
            Self::Int | Self::DropdownInt => params.int_params().get(slot).map(|&val| val as f32),

            Self::Float => params.float_params().get(slot).copied(),

            _ => None,
        }
    }
}

fn apply_variant_action(
    action: &VariantAction,
    params: &AnyParams<'_>,
    resolved: &mut ResolvedImage,
) {
    match action {
        VariantAction::SwapImage { display_image } => {
            resolved.image_path = display_image.clone();
        }

        VariantAction::RotateFromParam { source } => {
            if let Some(rotation) = source.data_type.extract_as_f32(params, source.slot) {
                resolved.rotation_degrees = rotation;
            }
        }
    }
}

fn evaluate_condition(condition: &VariantCondition, params: &AnyParams<'_>) -> bool {
    condition
        .data_type
        .matches_json_value(&condition.expected_value, params, condition.slot)
}

impl LevelEditor {
    pub fn load_image_data() -> Result<String> {
        let path = get_merino_folder()?.join(IMAGEDATA_FILE);
        let string = fs::read_to_string(path)?;
        Ok(string)
    }

    pub fn parse_image_data(&mut self, json: String) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(&json)?;

        let set_names = [
            "MapObjSet",
            "MapItemSet",
            "MapEnemySet",
            "MapLocator",
            "MapTerrain",
        ];

        self.canvas_context.image_bank_mut().clear_all();

        for set_name in set_names {
            let set_object = match json.get(set_name).and_then(|v| v.as_object()) {
                Some(v) => v,
                None => continue,
            };

            let set_type = MapNodeType::from_str(set_name)?;

            for (obj_name, obj_data) in set_object {
                let mut image_def = ImageDefinition::default();

                image_def.display_image = obj_data
                    .get("display_image")
                    .and_then(|v| v.as_str())
                    .map(String::from);

                image_def.anchor = obj_data
                    .get("anchor")
                    .and_then(|v| v.as_str())
                    .map(ImageAnchor::from_str)
                    .transpose()?
                    .unwrap_or_default();

                if let Some(variants) = obj_data.get("variants").and_then(|v| v.as_array()) {
                    for variant in variants {
                        let when = variant
                            .get("when")
                            .map(|when_obj| {
                                Ok::<VariantCondition, anyhow::Error>(VariantCondition {
                                    data_type: ParameterDataType::from_string(
                                        when_obj["data_type"]
                                            .as_str()
                                            .ok_or_else(|| anyhow!("missing when.data_type"))?,
                                    )?,

                                    slot: when_obj["slot"]
                                        .as_u64()
                                        .ok_or_else(|| anyhow!("missing when.slot"))?
                                        as usize,

                                    expected_value: when_obj["expected_value"].clone(),
                                })
                            })
                            .transpose()?;

                        let action = if let Some(then_obj) = variant.get("then") {
                            VariantAction::SwapImage {
                                display_image: then_obj["display_image"]
                                    .as_str()
                                    .ok_or_else(|| anyhow!("missing then.display_image"))?
                                    .to_string(),
                            }
                        } else if let Some(copy_obj) = variant.get("copy") {
                            let source = CopySource {
                                data_type: ParameterDataType::from_string(
                                    copy_obj["data_type"]
                                        .as_str()
                                        .ok_or_else(|| anyhow!("missing copy.data_type"))?,
                                )?,

                                slot: copy_obj["slot"]
                                    .as_u64()
                                    .ok_or_else(|| anyhow!("missing copy.slot"))?
                                    as usize,
                            };

                            let to = variant["to"]
                                .as_str()
                                .ok_or_else(|| anyhow!("missing to"))?;

                            match to {
                                "rotation" => VariantAction::RotateFromParam { source },

                                _ => {
                                    return Err(anyhow!("unsupported manipulation type"));
                                }
                            }
                        } else {
                            return Err(anyhow!(
                                "variant must contain either \
                                     `then` or `copy`"
                            ));
                        };

                        image_def.variants.push(ImageVariant { when, action });
                    }
                }

                self.canvas_context
                    .image_bank_mut()
                    .insert_image_object((set_type, obj_name.clone()), image_def);
            }
        }

        Ok(())
    }
}
