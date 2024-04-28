#!/usr/bin/env bats
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

@test "should reject invalid arguments" {
    run gradle-wiper ram full

    [[ "$output" == *"possible values: evaluate, shallow, deep"* ]]
    [ ! "$status" -eq 0 ]
}

@test "should detect usages of ram" {
    run $HOME/IdeaProjects/aaw/gradlew tasks -q -p $HOME/IdeaProjects//aaw
    run gradle-wiper ram evaluate

    [[ "$output" == *"Total resources (RAM memory)"* ]]
    [ "$status" -eq 0 ]
}

@test "should perform ram shallow wiping" {
    run $HOME/IdeaProjects/aaw/gradlew shadowJar -q -p $HOME/IdeaProjects/aaw
    run gradle-wiper ram shallow

    [[ "$output" == *"Reclaimed RAM memory"* ]]
    [ "$status" -eq 0 ]

    [ $(jps | grep Jps | wc -l) -eq 1 ]
    [ $(jps | grep Gradle | wc -l) -eq 0 ]
    [ $(jps | grep Kotlin | wc -l) -eq 0 ]
}

@test "should perform ram deep wiping" {
    run $HOME/IdeaProjects/aaw/gradlew tasks -q -p $HOME/IdeaProjects/aaw
    run gradle-wiper ram deep

    [[ "$output" == *"Reclaimed RAM memory"* ]]
    [ "$status" -eq 0 ]

    [ $(jps | grep Gradle | wc -l) -eq 0 ]
    [ $(jps | grep Kotlin | wc -l) -eq 0 ]
    [ $(jps | grep GradleWorkerMain | wc -l) -eq 0 ]
    [ $(jps | grep GradleWrapper | wc -l) -eq 0 ]
    [ $(jps | grep Main | wc -l) -eq 0 ]
}
