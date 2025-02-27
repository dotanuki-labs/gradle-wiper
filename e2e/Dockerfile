# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM azul/zulu-openjdk-debian:21.0.6@sha256:4e44628526a1fef8020c5aba6936552bdc30cb4ad9d9b54bd903fdfb6c6e4b8e

RUN apt-get update && apt-get install -y git openssh-client bats --no-install-recommends

RUN mkdir -p "$HOME/.ssh" &&\
    mkdir "$HOME/IdeaProjects" && \
    ssh-keyscan -t rsa github.com >> "$HOME/.ssh/known_hosts" && \
    git clone https://github.com/dotanuki-labs/android-archives-watchdog.git "$HOME/IdeaProjects/aaw"

COPY e2e/ /usr
COPY target/e2e/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

ENTRYPOINT ["/usr/run-tests.sh"]
