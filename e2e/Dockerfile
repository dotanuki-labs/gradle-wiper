# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM azul/zulu-openjdk-debian:21.0.6@sha256:929447effe5cd097b9f3cc03236194501fe6bdf7cff51f06471f5edaf2efe8e0

RUN apt-get update && apt-get install -y git openssh-client bats --no-install-recommends

RUN mkdir -p "$HOME/.ssh" &&\
    mkdir "$HOME/IdeaProjects" && \
    ssh-keyscan -t rsa github.com >> "$HOME/.ssh/known_hosts" && \
    git clone https://github.com/dotanuki-labs/android-archives-watchdog.git "$HOME/IdeaProjects/aaw"

COPY e2e/ /usr
COPY target/e2e/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

ENTRYPOINT ["/usr/run-tests.sh"]
