import { invoke } from "@tauri-apps/api/core";
import type {
  AllocationOutput,
  AppConfig,
  VolunteerInput,
} from "./types";

export function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("get_config");
}

export function setConfig(config: AppConfig): Promise<AppConfig> {
  return invoke<AppConfig>("set_config", { config });
}

export function allocate(
  config: AppConfig,
  volunteers: VolunteerInput[],
): Promise<AllocationOutput> {
  return invoke<AllocationOutput>("allocate", { config, volunteers });
}

export function mockVolunteers(
  config: AppConfig,
  seed = 42,
): Promise<VolunteerInput[]> {
  return invoke<VolunteerInput[]>("mock_volunteers", { config, seed });
}
