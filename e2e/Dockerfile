# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM azul/zulu-openjdk:21-latest@sha256:7c9251bd5cf7b758d2397bede4d6502dd4efaad5ff6c6d8211a96de49682b6ca

RUN apt-get update && apt-get install -y git openssh-client bats --no-install-recommends

RUN mkdir -p "$HOME/.ssh" &&\
    mkdir "$HOME/IdeaProjects" && \
    ssh-keyscan -t rsa github.com >> "$HOME/.ssh/known_hosts" && \
    git clone https://github.com/dotanuki-labs/android-archives-watchdog.git "$HOME/IdeaProjects/aaw"

COPY e2e/ /usr
COPY target/e2e/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

ENTRYPOINT ["/usr/run-tests.sh"]
