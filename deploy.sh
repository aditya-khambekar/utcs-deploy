. remote/constants.sh
. remote/nl_helper.sh
mkdir "$LOG_FOLDER"
readonly HOST=$(DIR=pwd ; cd "$SCRAPER_LOCATION" ; cargo run -q ; cd DIR)
scp -r "$CURRENT_DIR" "$USER@$HOST:$REMOTE_DIR"
(cat remote/service.sh | ssh "$USER@$HOST") > "$(name_log "$LOG_FOLDER/java")"