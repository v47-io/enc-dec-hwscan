#!/bin/bash

alias c="./gradlew clean"
alias b="./gradlew build"
alias bn="./gradlew build -Dquarkus.package.jar.enabled=false -Dquarkus.native.enabled=true"
alias run="./gradlew integration-test:quarkusRun"
alias native="./integration-test/build/integration-test-0.0.0-SNAPSHOT-runner"
