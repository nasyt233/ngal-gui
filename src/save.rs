use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct DialogueState {
    pub scene_id: String,
    pub cmd_index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EngineState {
    InDialogue(DialogueState),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageParams {
    pub filename: String,
    pub position: u8,
    pub scale: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub state: EngineState,
    pub menu_selected: usize,
    pub variables: HashMap<String, String>,
    pub current_image: Option<String>,
    pub timestamp: String,
    pub current_file: String,
    pub background: Option<String>,
    pub bgm: Option<String>,
    pub image_params: Option<ImageParams>,
    
    #[serde(default)]
    pub preview_text: String,
    #[serde(default)]
    pub background_path: Option<String>,
}

impl SaveData {
    pub fn save_to_slot(slot: usize, data: &SaveData) -> Result<(), std::io::Error> {
        let path = format!("save/slot_{}.json", slot);
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(path, json)
    }

    pub fn load_from_slot(slot: usize) -> Option<SaveData> {
        let path = format!("save/slot_{}.json", slot);
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
}
