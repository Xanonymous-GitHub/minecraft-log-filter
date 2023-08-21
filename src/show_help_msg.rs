pub(crate) fn show_usage() {
    println!(
        "A very simple Minecraft log parser by Xanonymous TU
        \n
        Usage: mclf <command> [arguments]
        \n
        This program parses the given Minecraft server log to analyze player activities. 
        It provides a summary of players joining and leaving the game, along with their online status.
        \n
        Available Commands:
        \n
        -> ps - Show who is online.
            Usage: mclf ps \"$(cat [log_file])\" or
                   cat [log_file] | mclf ps
            For example: mclf ps \"$(cat server.log)\"
            Or if your using docker compose, you can use:
                docker compose logs --no-log-prefix | mclf ps
        -> msg - Show the messages sent by players. This feature is not available yet.
        -> help - Show this message."
    );
}
