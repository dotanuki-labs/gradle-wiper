#!/usr/bin/env bats

current_dir="$(cd "$(dirname "$BATS_TEST_FILENAME")" >/dev/null 2>&1 && pwd)"

@test "should detect usages of disk" {
    run aaw/gradlew tasks -q -p aaw
    run gradle-wiper disk evaluate

    echo "$output"
    [[ "$output" == *"Total resources (disk space) : 1.1GiB"* ]]
    [ "$status" -eq 0 ]
}

@test "should perform disk shallow wiping" {
    run aaw/gradlew shadowJar -q -p aaw
    run gradle-wiper disk evaluate
    run gradle-wiper disk shallow

    [[ "$output" == *"Reclaimed disk space : 642.1MiB"* ]]
    [ "$status" -eq 0 ]
}
