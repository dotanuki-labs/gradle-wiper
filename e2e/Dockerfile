# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM azul/zulu-openjdk:21-latest@sha256:8f5b4958587a3f6f320f4d15fe079ccdc7172f3f0c8b885521b79a1c24c8c395

RUN apt-get update && apt-get install -y git openssh-client bats --no-install-recommends

RUN mkdir -p "$HOME/.ssh" &&\
    mkdir "$HOME/IdeaProjects" && \
    ssh-keyscan -t rsa github.com >> "$HOME/.ssh/known_hosts" && \
    git clone https://github.com/dotanuki-labs/android-archives-watchdog.git "$HOME/IdeaProjects/aaw"

COPY e2e/ /usr
COPY target/e2e/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

ENTRYPOINT ["/usr/run-tests.sh"]
