use std::{fs, str::FromStr};

use anyhow::{Result, anyhow};

use crate::merino::{
    archive_viewer::level_editor::LevelEditor, game::mapbin::MapNodeType,
    util::res_folder::get_merino_folder,
};

const OBJECTDATA_FILE: &str = "objectdata.json";

#[derive(Default, Debug, Clone)]
pub enum ParameterDataType {
    #[default]
    None,
    Int,
    Float,
    String,
    Bool,
    DropdownInt,
}

impl ParameterDataType {
    pub fn from_string(string: &str) -> Result<Self> {
        match string {
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "bool" => Ok(Self::Bool),
            "dropdown_int" => Ok(Self::DropdownInt),
            _ => Err(anyhow!("Invalid string type (found {string}")),
        }
    }
}

#[derive(Default, Debug)]
pub struct DropdownOption {
    pub key: String,
    pub value: i32,
}

impl DropdownOption {
    fn new(key: String, value: i32) -> Self {
        Self { key, value }
    }
}

#[derive(Default, Debug)]
pub struct Parameter {
    pub name: String,
    pub data_type: ParameterDataType,
    pub slot: usize,
    pub description: Option<String>,
    pub notes: Option<Vec<String>>,
    pub dropdown_options: Option<Vec<DropdownOption>>,
}

#[derive(Default, Debug)]
pub struct ParameterObject {
    pub set_type: MapNodeType,
    pub name: String,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub parameters: Vec<Parameter>,
}

impl LevelEditor {
    pub fn load_params() -> Result<String> {
        let path = get_merino_folder()?.join(OBJECTDATA_FILE);
        let string = fs::read_to_string(path)?;
        Ok(string)
    }

    pub fn parse_params(&mut self, json: String) -> Result<()> {
        let json: serde_json::Value = serde_json::from_str(&json).expect("failed to parse json");

        let mut parameter_objects = Vec::new();

        let set_names = [
            "MapObjSet",
            "MapItemSet",
            "MapEnemySet",
            "MapLocator",
            "MapPath",
            "MapRect",
            "MapCircle",
            "MapTerrain",
        ];

        for set_name in set_names {
            let set_object = match json.get(set_name).and_then(|v| v.as_object()) {
                Some(obj) => obj,
                None => continue,
            };

            let set_type = MapNodeType::from_str(set_name)?;

            let objects = set_object.iter().map(|(obj_name, props)| {
                let mut param_object = ParameterObject::default();
                param_object.set_type = set_type;
                param_object.name = obj_name.to_owned();
                param_object.description = props["description"].as_str().map(String::from);
                param_object.display_name = props["display_name"].as_str().map(String::from);
                param_object.note = props["note"].as_str().map(String::from);

                if let Some(params) = props.get("parameters").and_then(|v| v.as_object()) {
                    param_object.parameters = params
                        .iter()
                        .filter_map(|(p_name, p_val)| {
                            let data_type_str = p_val.get("data_type")?.as_str()?;
                            let data_type = ParameterDataType::from_string(data_type_str).ok()?;

                            let slot = p_val.get("slot")?.as_u64()? as usize;

                            let description = p_val
                                .get("description")
                                .and_then(|v| v.as_str())
                                .map(String::from);

                            let notes = if let Some(arr) =
                                p_val.get("notes").and_then(|v| v.as_array())
                            {
                                Some(
                                    arr.iter()
                                        .filter_map(|v| v.as_str().map(String::from))
                                        .collect::<Vec<String>>(),
                                )
                            } else if let Some(note) = p_val.get("note").and_then(|v| v.as_str()) {
                                Some(vec![note.to_string()])
                            } else {
                                None
                            };

                            // dropdown logic
                            let dropdown_options = match &data_type {
                                ParameterDataType::DropdownInt => p_val
                                    .get("values")
                                    .and_then(|v| v.as_object())
                                    .and_then(|obj| {
                                        obj.iter()
                                            .map(|(k, v)| {
                                                Some(DropdownOption::new(
                                                    k.clone(),
                                                    v.as_i64()? as i32,
                                                ))
                                            })
                                            .collect::<Option<Vec<_>>>()
                                    }),
                                _ => None,
                            };

                            Some(Parameter {
                                name: p_name.to_owned(),
                                data_type,
                                slot,
                                description,
                                notes,
                                dropdown_options,
                            })
                        })
                        .collect();
                }
                param_object
            });

            parameter_objects.extend(objects);
        }

        self.parameter_context
            .set_parameter_objects(parameter_objects);

        Ok(())
    }
}
