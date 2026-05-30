export type PreferenceMode = "two" | "three";

export interface Subject {
  name: string;
  capacity: number;
}

export interface AppConfig {
  subjects: Subject[];
  preference_mode: PreferenceMode;
}

export interface VolunteerInput {
  name: string;
  pref1: string;
  pref2: string;
  pref3?: string;
}

export interface ResultRow {
  id: number;
  name: string;
  pref1: string;
  pref2: string;
  pref3: string;
  assigned: string;
  match_label: string;
}

export interface SummaryRow {
  subject: string;
  capacity: number;
  count: number;
  names: string;
}

export interface AllocationOutput {
  rows: ResultRow[];
  summary: SummaryRow[];
}

export const MAX_VOLUNTEERS = 30;

export function totalCapacity(config: AppConfig): number {
  return config.subjects.reduce((sum, s) => sum + s.capacity, 0);
}

export function emptyVolunteer(): VolunteerInput {
  return { name: "", pref1: "", pref2: "", pref3: "" };
}

export function emptyVolunteers(count: number): VolunteerInput[] {
  return Array.from({ length: count }, () => emptyVolunteer());
}
