run() {
    local root="$1"
    local file_name="$2"
    local dir=$(pwd)
    cd "$root"
    rm -rf output
    mkdir output
    cp -a src/main/java/. output
    cd output
    javac -source 1.8 -target 1.8 -nowarn "$file_name"
    java "$file_name"
    cd "$dir"
}

run GraphOut GraphAndRankTester.java
exit
