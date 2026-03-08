use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GreetRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GreetResponse {
    pub message: String,
}

#[tauri::command]
pub fn greet(request: GreetRequest) -> GreetResponse {
    GreetResponse {
        message: format!("Hello, {}! Welcome to o my claw!", request.name),
    }
}
