use crate::hungarian::solve_assignment;
use crate::models::{
    AllocationOutput, AppConfig, PreferenceMode, ResultRow, SummaryRow, VolunteerInput, MAX_VOLUNTEERS,
};

const MOCK_NAMES: [&str; 20] = [
    "김민준", "이서연", "박지호", "최유나", "정도윤", "강하은", "조시우", "윤지민", "장준서", "임수아",
    "한예준", "오지우", "신채원", "권민재", "황서윤", "송태양", "배수빈", "류현우", "노다은", "문승호",
];

fn trim_pref(value: &str) -> String {
    let text = value.trim();
    if text.eq_ignore_ascii_case("nan")
        || text.eq_ignore_ascii_case("none")
        || text.eq_ignore_ascii_case("nat")
    {
        return String::new();
    }
    text.to_string()
}

fn preference_cost(rank: usize) -> i32 {
    match rank {
        1 => 0,
        2 => 1,
        3 => 2,
        _ => 10,
    }
}

pub fn validate_config(config: &AppConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if config.subjects.is_empty() {
        errors.push("프로그램을 1개 이상 입력하세요.".into());
        return errors;
    }

    let mut seen = std::collections::HashSet::new();
    for (idx, subject) in config.subjects.iter().enumerate() {
        let name = subject.name.trim();
        if name.is_empty() {
            errors.push(format!("프로그램 {}: 이름을 입력하세요.", idx + 1));
            continue;
        }
        if !seen.insert(name.to_string()) {
            errors.push(format!("프로그램 이름이 중복됩니다: {name}"));
        }
    }

    for subject in &config.subjects {
        if subject.capacity == 0 {
            errors.push(format!("{}: 정원은 1 이상이어야 합니다.", subject.name.trim()));
        }
    }

    let total = config.total_capacity();
    if total == 0 {
        errors.push("총 인원은 1명 이상이어야 합니다.".into());
    }
    if total > MAX_VOLUNTEERS {
        errors.push(format!(
            "총 인원은 최대 {MAX_VOLUNTEERS}명입니다. (현재 {total}명)"
        ));
    }

    errors
}

fn build_slots(config: &AppConfig) -> Vec<String> {
    let mut slots = Vec::new();
    for subject in &config.subjects {
        for _ in 0..subject.capacity {
            slots.push(subject.name.trim().to_string());
        }
    }
    slots
}

fn prefs_for(volunteer: &VolunteerInput, mode: PreferenceMode) -> Vec<String> {
    let mut prefs = vec![trim_pref(&volunteer.pref1), trim_pref(&volunteer.pref2)];
    if mode == PreferenceMode::Three {
        prefs.push(trim_pref(&volunteer.pref3));
    }
    prefs
}

fn pref_rank(volunteer: &VolunteerInput, program: &str, mode: PreferenceMode) -> usize {
    for (idx, pref) in prefs_for(volunteer, mode).iter().enumerate() {
        if pref == program {
            return idx + 1;
        }
    }
    99
}

pub fn validate_volunteers(
    config: &AppConfig,
    volunteers: &[VolunteerInput],
) -> Vec<String> {
    let mut errors = validate_config(config);
    if !errors.is_empty() {
        return errors;
    }

    let expected = config.total_capacity() as usize;
    if volunteers.len() != expected {
        errors.push(format!(
            "나누미는 {expected}명이어야 합니다. (현재 {}명)",
            volunteers.len()
        ));
    }

    let allowed: std::collections::HashSet<String> = config
        .subjects
        .iter()
        .map(|s| s.name.trim().to_string())
        .collect();

    for volunteer in volunteers {
        let name = volunteer.name.trim();
        if name.is_empty() {
            errors.push("이름은 모두 입력해야 합니다.".into());
            continue;
        }

        let prefs = prefs_for(volunteer, config.preference_mode);
        let filled: Vec<&String> = prefs.iter().filter(|p| !p.is_empty()).collect();

        if prefs[0].is_empty() {
            errors.push(format!("{name}: 1지망은 필수입니다."));
            continue;
        }

        for pref in &filled {
            if !allowed.contains(*pref) {
                errors.push(format!("{name}: 잘못된 지망 '{pref}'"));
            }
        }

        if config.preference_mode == PreferenceMode::Two {
            if !prefs[1].is_empty() && prefs[0].is_empty() {
                errors.push(format!("{name}: 2지망만 입력할 수 없습니다."));
            }
        }

        if config.preference_mode == PreferenceMode::Three {
            if (!prefs[1].is_empty() || !prefs[2].is_empty()) && prefs[0].is_empty() {
                errors.push(format!("{name}: 1지망 없이 2·3지망만 입력할 수 없습니다."));
            }
        }

        let mut uniq = std::collections::HashSet::new();
        for pref in filled {
            if !uniq.insert(pref.clone()) {
                errors.push(format!("{name}: 지망은 서로 달라야 합니다."));
                break;
            }
        }
    }

    errors
}

fn match_label(volunteer: &VolunteerInput, assigned: &str, mode: PreferenceMode) -> String {
    if assigned.is_empty() {
        return "—".into();
    }
    let rank = pref_rank(volunteer, assigned, mode);
    match rank {
        1 => "1지망".into(),
        2 => "2지망".into(),
        3 => "3지망".into(),
        _ => "미지망".into(),
    }
}

pub fn run_allocation(
    config: &AppConfig,
    volunteers: &[VolunteerInput],
) -> Result<AllocationOutput, Vec<String>> {
    let errors = validate_volunteers(config, volunteers);
    if !errors.is_empty() {
        return Err(errors);
    }

    let slots = build_slots(config);
    let n = volunteers.len();
    let mut cost = vec![vec![0i32; n]; n];

    for (i, volunteer) in volunteers.iter().enumerate() {
        for (j, slot) in slots.iter().enumerate() {
            cost[i][j] = preference_cost(pref_rank(volunteer, slot, config.preference_mode));
        }
    }

    let col_assign = solve_assignment(&cost);
    let mut rows = Vec::with_capacity(n);

    for (i, volunteer) in volunteers.iter().enumerate() {
        let assigned = slots[col_assign[i]].clone();
        rows.push(ResultRow {
            id: (i + 1) as u32,
            name: volunteer.name.trim().to_string(),
            pref1: trim_pref(&volunteer.pref1),
            pref2: trim_pref(&volunteer.pref2),
            pref3: trim_pref(&volunteer.pref3),
            assigned: assigned.clone(),
            match_label: match_label(volunteer, &assigned, config.preference_mode),
        });
    }

    let summary = config
        .subjects
        .iter()
        .map(|subject| {
            let subject_name = subject.name.trim().to_string();
            let members: Vec<String> = rows
                .iter()
                .filter(|row| row.assigned == subject_name)
                .map(|row| row.name.clone())
                .collect();
            SummaryRow {
                subject: subject_name.clone(),
                capacity: subject.capacity,
                count: members.len() as u32,
                names: if members.is_empty() {
                    "—".into()
                } else {
                    members.join(", ")
                },
            }
        })
        .collect();

    Ok(AllocationOutput { rows, summary })
}

pub fn generate_mock(config: &AppConfig, seed: u64) -> Result<Vec<VolunteerInput>, Vec<String>> {
    let errors = validate_config(config);
    if !errors.is_empty() {
        return Err(errors);
    }

    let count = config.total_capacity() as usize;
    let names: Vec<String> = config.subject_names();
    let mut rng = seed;
    let mut volunteers = Vec::with_capacity(count);

    for i in 0..count {
        let base = MOCK_NAMES[i % MOCK_NAMES.len()];
        let display_name = if i < MOCK_NAMES.len() {
            base.to_string()
        } else {
            format!("{base}{}", i / MOCK_NAMES.len() + 1)
        };

        let pref_count = config.preference_mode.pref_count().min(names.len());
        let mut shuffled = names.clone();
        shuffle(&mut shuffled, &mut rng);
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);

        volunteers.push(VolunteerInput {
            name: display_name,
            pref1: shuffled[0].clone(),
            pref2: if pref_count > 1 {
                shuffled[1].clone()
            } else {
                String::new()
            },
            pref3: if pref_count > 2 {
                shuffled[2].clone()
            } else {
                String::new()
            },
        });
    }

    Ok(volunteers)
}

fn shuffle(items: &mut [String], seed: &mut u64) {
    for i in (1..items.len()).rev() {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (*seed as usize) % (i + 1);
        items.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Subject;

    #[test]
    fn default_config_has_twenty_slots() {
        let config = AppConfig::default();
        assert_eq!(config.total_capacity(), 20);
    }

    #[test]
    fn rejects_over_capacity() {
        let config = AppConfig {
            subjects: vec![Subject {
                name: "국어".into(),
                capacity: 31,
            }],
            preference_mode: PreferenceMode::Two,
        };
        let errors = validate_config(&config);
        assert!(errors.iter().any(|e| e.contains("30")));
    }

    #[test]
    fn mock_allocation_prefers_first_choices() {
        let config = AppConfig::default();
        let volunteers = generate_mock(&config, 42).expect("mock");
        let output = run_allocation(&config, &volunteers).expect("allocate");

        let first_choice_matches = output
            .rows
            .iter()
            .filter(|row| row.match_label == "1지망")
            .count();

        assert!(
            first_choice_matches >= 10,
            "expected many 1st-choice matches, got {first_choice_matches}"
        );
    }
}
