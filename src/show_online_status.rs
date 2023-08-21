use crate::participate_records::*;
use std::collections::HashMap;

pub(crate) fn show_online_status(raw_log: String) {
    let participate_records = find_participate_records(raw_log);

    let mut player_online_status: HashMap<String, i8> = HashMap::new();

    for record in participate_records {
        let name = record.name;
        let action = record.action;

        let status = match action {
            ParticipateActionType::Join => 1,
            ParticipateActionType::Leave => -1,
        };

        let current_status = player_online_status.entry(name).or_insert(0);
        *current_status += status;
    }

    for (name, status) in player_online_status {
        if status > 0 {
            println!("{} is online", name);
        }
    }
}
