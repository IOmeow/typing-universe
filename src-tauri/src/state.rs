use std::sync::Mutex;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct VisualSettings {
    pub particle_size: f32,
    pub gravity_strength: f32,
    pub hue_offset: f32,
    pub life_decay: f32,
    pub rotate_speed: f32,
}

impl VisualSettings {
    pub fn update_from_partial(&mut self, partial: &crate::control::PartialSettings) {
        let mut current = serde_json::to_value(&self).unwrap();
        for (k, v) in &partial.inner {
            if current.get_mut(k).is_some() {
                current[k] = v.clone();
            }
        }
        *self = serde_json::from_value(current).unwrap();
    }
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
                life_decay: 0.1,
                rotate_speed: 50.0,
            }),
        }
    }
}
