use crate::merino::game::mapbin::types::{Vec2f, Vec3f};

// to distinguish it from a potential 3D camera
pub struct CanvasCamera {
    pub position: egui::Vec2,
    pub zoom: f32,
    center_attempted: bool,
}

impl Default for CanvasCamera {
    fn default() -> Self {
        Self {
            position: egui::Vec2::ZERO,
            zoom: 1.0,
            center_attempted: false,
        }
    }
}

impl CanvasCamera {
    pub fn update(&mut self, ctx: &egui::Context, canvas_response: &egui::Response) {
        let zoom_min = 0.5;
        let zoom_max = 150.0;

        let hover_pos = ctx.input(|i| i.pointer.hover_pos());
        let is_mouse_over_canvas = if let Some(pos) = hover_pos {
            let is_over_ui = ctx.is_pointer_over_area() || ctx.wants_pointer_input();
            canvas_response.rect.contains(pos) && !is_over_ui
        } else {
            false
        };

        // zoom handling
        let zoom_delta = ctx.input(|i| i.zoom_delta());

        if zoom_delta != 1.0 && is_mouse_over_canvas {
            let mouse_pos =
                ctx.input(|i| i.pointer.hover_pos().unwrap_or(egui::Pos2::ZERO).to_vec2());

            let world_before = self.convert_from_camera(mouse_pos);

            self.zoom = (self.zoom * zoom_delta).clamp(zoom_min, zoom_max);

            let world_after = self.convert_from_camera(mouse_pos);

            // keep mouse anchored
            self.position += world_before - world_after;
        }

        if self.center_attempted {
            let screen_center = canvas_response.rect.center().to_vec2();
            self.position.x -= screen_center.x / self.zoom;
            self.position.y = -self.position.y - (screen_center.y / self.zoom);
            self.center_attempted = false;
        }
    }

    pub fn pan(&mut self, delta: egui::Vec2) {
        self.position -= delta;
    }

    pub fn convert_to_camera(&self, pos: egui::Vec2) -> egui::Vec2 {
        egui::Vec2 {
            x: (pos.x - self.position.x) * self.zoom,
            y: (-pos.y - self.position.y) * self.zoom,
        }
    }

    pub fn convert_from_camera(&self, pos: egui::Vec2) -> egui::Vec2 {
        egui::Vec2 {
            x: (pos.x / self.zoom) + self.position.x,
            y: (-pos.y / self.zoom) - self.position.y,
        }
    }

    /// Schedules a centering.
    // pub fn center(&mut self, pos: egui::Vec2) {
    //     self.center_attempted = true;
    //     self.position = pos;
    // }

    pub fn reset(&mut self) {
        self.position = Default::default();
        self.zoom = 1.0;
    }
}

impl From<Vec3f> for Vec2f {
    fn from(v: Vec3f) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<&Vec3f> for Vec2f {
    fn from(v: &Vec3f) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<&mut Vec3f> for Vec2f {
    fn from(v: &mut Vec3f) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vec2f> for egui::Vec2 {
    fn from(v: Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<&Vec2f> for egui::Vec2 {
    fn from(v: &Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<&mut Vec2f> for egui::Vec2 {
    fn from(v: &mut Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Vec2f> for egui::Pos2 {
    fn from(v: Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<&Vec2f> for egui::Pos2 {
    fn from(v: &Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<&mut Vec2f> for egui::Pos2 {
    fn from(v: &mut Vec2f) -> Self {
        Self::new(v.x, v.y)
    }
}
