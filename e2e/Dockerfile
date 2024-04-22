FROM azul/zulu-openjdk:21-latest

RUN apt-get update && apt-get install -y git openssh-client bats
RUN cd $HOME && mkdir -p $HOME/.ssh
RUN ssh-keyscan -t rsa github.com >> $HOME/.ssh/known_hosts
RUN git clone https://github.com/dotanuki-labs/android-archives-watchdog.git aaw

COPY e2e/e2e.bats /usr/e2e.bats
COPY target/release/gradle-wiper /usr/bin/gradle-wiper
RUN chmod +x /usr/bin/gradle-wiper

CMD ["bats", "/usr/e2e.bats"]
