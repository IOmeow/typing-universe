use crate::state::AppState;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_store::StoreExt;

#[derive(serde::Deserialize, Debug)]
pub struct PartialSettings {
    #[serde(flatten)]
    pub inner: HashMap<String, serde_json::Value>,
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> crate::state::VisualSettings {
    let settings = state.settings.lock().unwrap();
    settings.clone()
}

#[tauri::command]
pub fn update_settings(app: AppHandle, new_settings: PartialSettings, state: State<'_, AppState>) {
    let mut settings = state.settings.lock().unwrap();

    settings.update_from_partial(&new_settings);

    if let Ok(store) = app.store("settings.json") {
        for (k, v) in serde_json::to_value(&*settings)
            .unwrap()
            .as_object()
            .unwrap()
        {
            store.set(k, v.clone());
        }
    }

    app.emit("settings-updated", &*settings).unwrap();
}

pub fn load_store(app: &AppHandle, state: &State<'_, AppState>) {
    if let Ok(store) = app.store("settings.json") {
        let mut settings = state.settings.lock().unwrap();

        let mut current = serde_json::to_value(&*settings).unwrap();

        let keys: Vec<String> = current.as_object().unwrap().keys().cloned().collect();

        for key in keys {
            if let Some(v) = store.get(&key) {
                current[&key] = v.clone();
            }
        }

        *settings = serde_json::from_value(current).unwrap();
    }
}
