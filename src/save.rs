use serde::{Deserialize, Serialize};

pub const SAVE_DIR: &str = "save";

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub timestamp: String,
    pub speaker: String,
    pub preview_text: String,
    pub line_index: usize,
    pub background_path: Option<String>,
    pub portrait_path: Option<String>,
}

impl SaveData {
    pub fn save_to_slot(slot: usize, data: &SaveData) -> Result<(), std::io::Error> {
        let _ = std::fs::create_dir_all(SAVE_DIR);
        let path = format!("{}/slot_{}.json", SAVE_DIR, slot);
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(path, json)
    }

    pub fn load_from_slot(slot: usize) -> Option<SaveData> {
        let path = format!("{}/slot_{}.json", SAVE_DIR, slot);
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
}
