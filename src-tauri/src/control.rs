use serde_json::json;
use tauri::{AppHandle, State};
use tauri::Emitter;
use tauri_plugin_store::StoreExt;
use crate::state::{AppState};

#[derive(serde::Deserialize, Debug)]
pub struct PartialSettings {
    pub particle_size: Option<f32>,
    pub gravity_strength: Option<f32>,
    pub hue_offset: Option<f32>,
}

#[tauri::command]
pub fn update_settings(app: tauri::AppHandle, new_settings: PartialSettings, state: tauri::State<'_, AppState>,) {
    let payload = {
        let mut settings = state.settings.lock().unwrap();
        if let Some(size) = new_settings.particle_size {
            settings.particle_size = size;
        }
        if let Some(gravity) = new_settings.gravity_strength {
            settings.gravity_strength = gravity;
        }
        if let Some(hue) = new_settings.hue_offset {
            settings.hue_offset = hue;
        }

        SettingsResponse {
            particle_size: settings.particle_size,
            gravity_strength: settings.gravity_strength,
            hue_offset: settings.hue_offset,
        }
    };
    let store = app.store("settings.json").unwrap();
    store.set("particle_size", json!(payload.particle_size));
    store.set("gravity_strength", json!(payload.gravity_strength));
    store.set("hue_offset", json!(payload.hue_offset));

    app.emit("settings-updated", payload).unwrap();
    // println!("update_settings called: {:?}", new_settings);
}

#[derive(serde::Serialize, Clone)]
pub struct SettingsResponse {
    pub particle_size: f32,
    pub gravity_strength: f32,
    pub hue_offset: f32,
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> SettingsResponse {
    let settings = state.settings.lock().unwrap();

    SettingsResponse {
        particle_size: settings.particle_size,
        gravity_strength: settings.gravity_strength,
        hue_offset: settings.hue_offset,
    }
}

pub fn load_store(app: &AppHandle, state: &State<'_, AppState>) {
    let store = app.store("settings.json").unwrap();
    let particle_size: f32 = store
        .get("particle_size")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
        .unwrap_or(20.0);
    let gravity_strength: f32 = store
        .get("gravity_strength")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
        .unwrap_or(1.0);
    let hue_offset: f32 = store
        .get("hue_offset")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
        .unwrap_or(0.0);

    let mut settings = state.settings.lock().unwrap();
    settings.particle_size = particle_size;
    settings.gravity_strength = gravity_strength;
    settings.hue_offset = hue_offset;
}