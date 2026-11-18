readonly constants_path=$(DIR=$(pwd) ; cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd ; cd "$DIR")
readonly LOCATION="${constants_path%/*}"
readonly SCRAPER_LOCATION="$LOCATION/gethost"
