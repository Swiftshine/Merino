use crate::merino::archive_viewer::level_editor::params::ParameterObject;

pub struct ParameterContext {
    parameter_objects: Vec<ParameterObject>,
}

impl ParameterContext {
    pub fn new() -> Self {
        Self {
            parameter_objects: Vec::new(),
        }
    }

    pub fn parameter_objects(&self) -> &[ParameterObject] {
        &self.parameter_objects
    }

    pub fn set_parameter_objects(&mut self, parameter_objects: Vec<ParameterObject>) {
        self.parameter_objects = parameter_objects;
    }
}
