FROM jenkins/jenkins:2.516.1-jdk21
USER root
RUN apt-get update && apt-get install -y lsb-release && apt-get install -y build-essential
RUN curl -fsSLo /usr/share/keyrings/docker-archive-keyring.asc \
    https://download.docker.com/linux/debian/gpg
RUN echo "deb [arch=$(dpkg --print-architecture) \
    signed-by=/usr/share/keyrings/docker-archive-keyring.asc] \
    https://download.docker.com/linux/debian \
    $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list
RUN apt-get update && apt-get install -y docker-ce-cli cmake pkg-config libssl-dev

# Install Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo tools
RUN /root/.cargo/bin/cargo install cargo-audit cargo-license

USER jenkins

# Install Rust for jenkins user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/var/jenkins_home/.cargo/bin:${PATH}"

# Install cargo tools for jenkins user
RUN /var/jenkins_home/.cargo/bin/cargo install cargo-audit cargo-license

# Install Jenkins plugins
RUN jenkins-plugin-cli --plugins "blueocean docker-workflow json-path-api"
