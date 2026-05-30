use serde::{Deserialize, Serialize};

pub const MAX_VOLUNTEERS: u32 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreferenceMode {
    Two,
    Three,
}

impl PreferenceMode {
    pub fn pref_count(self) -> usize {
        match self {
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub subjects: Vec<Subject>,
    pub preference_mode: PreferenceMode,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            subjects: vec![
                Subject { name: "국어".into(), capacity: 4 },
                Subject { name: "수학".into(), capacity: 4 },
                Subject { name: "영어".into(), capacity: 4 },
                Subject { name: "사탐".into(), capacity: 4 },
                Subject { name: "과탐".into(), capacity: 4 },
            ],
            preference_mode: PreferenceMode::Two,
        }
    }
}

impl AppConfig {
    pub fn total_capacity(&self) -> u32 {
        self.subjects.iter().map(|s| s.capacity).sum()
    }

    pub fn subject_names(&self) -> Vec<String> {
        self.subjects.iter().map(|s| s.name.clone()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerInput {
    pub name: String,
    pub pref1: String,
    pub pref2: String,
    #[serde(default)]
    pub pref3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultRow {
    pub id: u32,
    pub name: String,
    pub pref1: String,
    pub pref2: String,
    pub pref3: String,
    pub assigned: String,
    pub match_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryRow {
    pub subject: String,
    pub capacity: u32,
    pub count: u32,
    pub names: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationOutput {
    pub rows: Vec<ResultRow>,
    pub summary: Vec<SummaryRow>,
}
