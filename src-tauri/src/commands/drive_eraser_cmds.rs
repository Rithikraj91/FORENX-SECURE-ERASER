use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

// Payload format to send progress updates to React
#[derive(Clone, Serialize, Deserialize)]
pub struct DriveProgressPayload {
    pub step: String,
    pub progress_percent: f32,
    pub message: String,
}

// Result structure returned to React after mock execution
#[derive(Serialize, Deserialize)]
pub struct DriveErasureResponse {
    pub success: bool,
    pub drive_id: String,
    pub method_used: String,
    pub verification_passed: bool,
    pub summary: String,
}

#[tauri::command]
pub async fn run_mock_drive_erasure(
    app: AppHandle,
    drive_id: String,
    method: Option<String>,
) -> Result<DriveErasureResponse, String> {
    // 1. Emit start event
    let _ = app.emit(
        "drive-erasure-progress",
        DriveProgressPayload {
            step: "validation".to_string(),
            progress_percent: 10.0,
            message: format!("Validating safety for drive {}...", drive_id),
        },
    );

    // 2. Emit mock sanitization progress
    let _ = app.emit(
        "drive-erasure-progress",
        DriveProgressPayload {
            step: "erasure_simulation".to_string(),
            progress_percent: 50.0,
            message: "Running mock overwrite passes... (Simulation Only)".to_string(),
        },
    );

    // 3. Emit mock verification progress
    let _ = app.emit(
        "drive-erasure-progress",
        DriveProgressPayload {
            step: "verification".to_string(),
            progress_percent: 90.0,
            message: "Verifying mock erasure patterns...".to_string(),
        },
    );

    // 4. Final completion event
    let _ = app.emit(
        "drive-erasure-progress",
        DriveProgressPayload {
            step: "completed".to_string(),
            progress_percent: 100.0,
            message: "Mock drive erasure and verification completed successfully.".to_string(),
        },
    );

    Ok(DriveErasureResponse {
        success: true,
        drive_id,
        method_used: method.unwrap_or_else(|| "Mock NIST 800-88 Clear".to_string()),
        verification_passed: true,
        summary: "Mock drive sanitization completed with verification status: PASS (Safe Simulation)".to_string(),
    })
}