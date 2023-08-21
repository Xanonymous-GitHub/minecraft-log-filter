use crate::participate_records::*;

pub(crate) fn show_online_status(raw_log: String) {
    let participate_records = find_participate_records(raw_log);
    println!("Online players:");
    for record in participate_records {
        match record.action {
            ParticipateActionType::Join => println!("+ {}", record.name),
            ParticipateActionType::Leave => println!("- {}", record.name),
        }
    }
}
