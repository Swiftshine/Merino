pub struct NodeEditSettings {
    pub visible: bool,
    pub editable: bool,
}

impl Default for NodeEditSettings {
    fn default() -> Self {
        Self {
            visible: true,
            editable: true,
        }
    }
}
