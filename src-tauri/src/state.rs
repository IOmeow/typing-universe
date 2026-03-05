use std::sync::Mutex;

#[derive(Default)]
pub struct VisualSettings {
    pub particle_size: f32,
    pub gravity_strength: f32,
    pub hue_offset: f32,
}

pub struct AppState {
    pub settings: Mutex<VisualSettings>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Mutex::new(VisualSettings {
                particle_size: 20.0,
                gravity_strength: 1.0,
                hue_offset: 0.0,
            }),
        }
    }
}