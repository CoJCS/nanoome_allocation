mod allocation;
mod config_store;
mod hungarian;
mod models;

use allocation::{generate_mock, run_allocation, validate_config, validate_volunteers};
use config_store::{load_config, save_config};
use models::{AllocationOutput, AppConfig, VolunteerInput};
use tauri::AppHandle;

#[tauri::command]
fn get_config(app: AppHandle) -> Result<AppConfig, String> {
    load_config(&app)
}

#[tauri::command]
fn set_config(app: AppHandle, config: AppConfig) -> Result<AppConfig, String> {
    let errors = validate_config(&config);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    save_config(&app, &config)?;
    Ok(config)
}

#[tauri::command]
fn validate_input(
    config: AppConfig,
    volunteers: Vec<VolunteerInput>,
) -> Result<(), String> {
    let errors = validate_volunteers(&config, &volunteers);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

#[tauri::command]
fn allocate(
    config: AppConfig,
    volunteers: Vec<VolunteerInput>,
) -> Result<AllocationOutput, String> {
    run_allocation(&config, &volunteers).map_err(|errors| errors.join("\n"))
}

#[tauri::command]
fn mock_volunteers(config: AppConfig, seed: u64) -> Result<Vec<VolunteerInput>, String> {
    generate_mock(&config, seed).map_err(|errors| errors.join("\n"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_config,
            validate_input,
            allocate,
            mock_volunteers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
