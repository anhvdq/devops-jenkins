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

# Install Trivy
RUN curl -sfL https://aquasecurity.github.io/trivy-repo/deb/public.key | gpg --dearmor | tee /usr/share/keyrings/trivy.gpg > /dev/null \
    && echo "deb [signed-by=/usr/share/keyrings/trivy.gpg] https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main" | tee -a /etc/apt/sources.list.d/trivy.list \
    && apt-get update \
    && apt-get install -y trivy

# Install kubectl
ARG KUBECTL_VERSION=v1.29.1
RUN curl -LO "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl" \
    && curl -LO "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/amd64/kubectl.sha256" \
    && echo "$(cat kubectl.sha256)  kubectl" | sha256sum --check \
    && install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl \
    && rm kubectl kubectl.sha256 \
    && chmod +x /usr/local/bin/kubectl

RUN apt update && \
    apt install -y gettext-base && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

USER jenkins

# Install Rust for jenkins user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/var/jenkins_home/.cargo/bin:${PATH}"

# Install cargo tools for jenkins user
RUN /var/jenkins_home/.cargo/bin/cargo install cargo-audit cargo-license

# Install Jenkins plugins
RUN jenkins-plugin-cli --plugins "blueocean docker-workflow json-path-api"
