use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PetStateResponse {
    pub skin: String,
    pub mood: String,
}

#[tauri::command]
pub fn get_pet_state() -> PetStateResponse {
    PetStateResponse {
        skin: "default".to_string(),
        mood: "happy".to_string(),
    }
}
