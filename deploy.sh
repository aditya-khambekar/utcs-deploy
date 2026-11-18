readonly path=$(DIR=$(pwd) ; cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd ; cd "$DIR")

source "$path/remote/constants.sh"
source "$path/remote/user_constants.sh"
source "$path/remote/nl_helper.sh"

echo "Scraper: $SCRAPER_LOCATION"
echo "User: $USER"

mkdir -p "$LOG_DIR"
readonly HOST=$(DIR=$(pwd) ; cd "$SCRAPER_LOCATION"; cargo run -q ; cd "$DIR")
echo "Host: $HOST"
scp -r "$CURRENT_DIR" "$USER@$HOST:$REMOTE_DIR"
readonly LOG_PATH="$(name_log "$LOG_DIR/$LOG_PREFIX")"
echo "Log path: $LOG_PATH"
(cat "$path/service/service.sh" | ssh "$USER@$HOST") > "$LOG_PATH"