use crate::merino::archive_viewer::level_editor::params::ParameterObject;

#[derive(PartialEq, Eq)]
pub enum AddObjectMode {
    SearchDatabase,
    CreateBlank,
}

pub struct ParameterContext {
    parameter_objects: Vec<ParameterObject>,
    search_query: String,
    add_object_mode: AddObjectMode,
}

impl ParameterContext {
    pub fn new() -> Self {
        Self {
            parameter_objects: Vec::new(),
            search_query: String::new(),
            add_object_mode: AddObjectMode::CreateBlank,
        }
    }

    pub fn parameter_objects(&self) -> &[ParameterObject] {
        &self.parameter_objects
    }

    pub fn set_parameter_objects(&mut self, parameter_objects: Vec<ParameterObject>) {
        self.parameter_objects = parameter_objects;
    }

    pub fn search_query_mut(&mut self) -> &mut String {
        &mut self.search_query
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn add_object_mode_mut(&mut self) -> &mut AddObjectMode {
        &mut self.add_object_mode
    }

    pub fn add_object_mode(&self) -> &AddObjectMode {
        &self.add_object_mode
    }
}
