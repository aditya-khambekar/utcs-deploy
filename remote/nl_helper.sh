. constants.sh

name_log() {
  local path="$1"
  local dt=$(date +%FT%T%Z)
  echo "$path-$dt"
}


