# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM azul/zulu-openjdk:21-latest@sha256:c41476854bd13fb24871e5ef25921264e3bd660cca12a2d349362605df65f17f

RUN apt-get update && apt-get install -y git openssh-client bats --no-install-recommends

RUN mkdir -p "$HOME/.ssh" &&\
    mkdir "$HOME/IdeaProjects" && \
    ssh-keyscan -t rsa github.com >> "$HOME/.ssh/known_hosts" && \
    git clone https://github.com/dotanuki-labs/android-archives-watchdog.git "$HOME/IdeaProjects/aaw"

COPY e2e/ /usr
COPY target/e2e/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

ENTRYPOINT ["/usr/run-tests.sh"]
