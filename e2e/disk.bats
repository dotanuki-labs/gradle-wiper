#!/usr/bin/env bats
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

@test "should reject invalid arguments" {
    run gradle-wiper disk full

    [[ "$output" == *"possible values: evaluate, shallow, deep"* ]]
    [ ! "$status" -eq 0 ]
}

@test "should detect usages of disk" {
    run $HOME/IdeaProjects/aaw/gradlew tasks -q -p $HOME/IdeaProjects/aaw
    run gradle-wiper disk evaluate

    [[ "$output" == *"Total resources (disk space)"* ]]
    [ "$status" -eq 0 ]
}

@test "should perform disk shallow wiping" {
    run $HOME/IdeaProjects/aaw/gradlew shadowJar -q -p $HOME/IdeaProjects/aaw
    run gradle-wiper disk shallow

    echo "$output"
    [[ "$output" == *"Reclaimed disk space"* ]]
    [ "$status" -eq 0 ]

    [ ! -d $HOME/.gradle/caches ]
    [ ! -d $HOME/.gradle/configuration-cache ]
    [ ! -d $HOME/.gradle/daemon ]
    [ ! -d $HOME/.gradle/.tmp ]
    [ ! -d $HOME/.gradle/.m2 ]
    [ ! -d $HOME/IdeaProjects/aaw/build ]

    # Do not clean Gradle metadata on shallow wiping
    [ -d $HOME/IdeaProjects/aaw/.gradle ]
}

@test "should perform disk deep wiping" {
    run $HOME/IdeaProjects/gradlew shadowJar -q -p $HOME/IdeaProjects/aaw
    run gradle-wiper disk deep

    [[ "$output" == *"Reclaimed disk space"* ]]
    [ "$status" -eq 0 ]

    [ ! -d $HOME/.gradle/caches ]
    [ ! -d $HOME/.gradle/configuration-cache ]
    [ ! -d $HOME/.gradle/daemon ]
    [ ! -d $HOME/.gradle/.tmp ]
    [ ! -d $HOME/.gradle/jdks ]
    [ ! -d $HOME/.gradle/wrapper ]
    [ ! -d $HOME/.gradle/native ]
    [ ! -d $HOME/.gradle/build-scan-data ]
    [ ! -d $HOME/.m2 ]
    [ ! -d $HOME/IdeaProjects/aaw/build ]
    [ ! -d $HOME/IdeaProjects/aaw/.gradle ]
}
