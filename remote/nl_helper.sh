name_log() {
  local nl_path="$1"
  local dt=$(date +%FT%T%Z)
  echo "$nl_path-$dt.log"
}


