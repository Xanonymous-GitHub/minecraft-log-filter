use crate::log_keys::*;
use regex::Regex;

pub enum ParticipateActionType {
    Join,
    Leave,
}

pub struct PlayerParticipateRecord {
    name: String,
    action: ParticipateActionType,
}

pub fn find_participate_records(raw_log: String) -> Vec<PlayerParticipateRecord> {
    let filtered_logs = raw_log
        .trim()
        .lines()
        .filter(|line| line.contains(MINECRAFT_USER_JOIN) || line.contains(MINECRAFT_USER_LEAVE))
        .collect::<Vec<&str>>();

    let mut player_actions: Vec<PlayerParticipateRecord> = Vec::new();

    let pattern = format!(r"(.+?) ({}|{})", MINECRAFT_USER_JOIN, MINECRAFT_USER_LEAVE);
    let re = Regex::new(&pattern).unwrap();

    for log in filtered_logs {
        if let Some(captured) = re.captures(log) {
            let name = captured[1].trim().to_string();
            let action = match &captured[2] {
                MINECRAFT_USER_JOIN => ParticipateActionType::Join,
                MINECRAFT_USER_LEAVE => ParticipateActionType::Leave,
                _ => panic!("#1 Unknown action type"),
            };

            player_actions.push(PlayerParticipateRecord { name, action });
        }
    }

    player_actions
}
