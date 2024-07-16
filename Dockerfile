# Use an Ubuntu base image
FROM ubuntu:22.04

# Install dependency tools
RUN apt-get update && apt-get install -y \
    net-tools iptables iproute2 wget bash git curl \
    libc++1 libc++abi1 && \
    rm -rf /var/lib/apt/lists/*

# Install noirup
RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

# Add noirup directory to PATH and ensure it's sourced
ENV PATH="/root/.nargo/bin:${PATH}"
RUN echo 'export PATH="$HOME/.nargo/bin:$PATH"' >> ~/.bashrc && \
    bash -c "source ~/.profile && noirup"

# working directory
WORKDIR /app

# Download all necessary components and set permissions in a single RUN command to reduce layers
RUN wget -O supervisord http://public.artifacts.marlin.pro/projects/enclaves/supervisord_master_linux_amd64 && \
    chmod +x supervisord && \
    wget -O ip-to-vsock-transparent http://public.artifacts.marlin.pro/projects/enclaves/ip-to-vsock-transparent_v1.0.0_linux_amd64 && \
    chmod +x ip-to-vsock-transparent && \
    wget -O keygen http://public.artifacts.marlin.pro/projects/enclaves/keygen_v1.0.0_linux_amd64 && \
    chmod +x keygen && \
    wget -O attestation-server http://public.artifacts.marlin.pro/projects/enclaves/attestation-server_v2.0.0_linux_amd64 && \
    chmod +x attestation-server && \
    wget -O vsock-to-ip http://public.artifacts.marlin.pro/projects/enclaves/vsock-to-ip_v1.0.0_linux_amd64 && \
    chmod +x vsock-to-ip && \
    wget -O dnsproxy http://public.artifacts.marlin.pro/projects/enclaves/dnsproxy_v0.46.5_linux_amd64 && \
    chmod +x dnsproxy && \
    wget -O oyster-keygen http://public.artifacts.marlin.pro/projects/enclaves/keygen-secp256k1_v1.0.0_linux_amd64 && \
    chmod +x oyster-keygen 

# Provide the github link to your circuit repo 
RUN git clone https://github.com/marlinprotocol/noir_enclave_setup.git

# supervisord config
COPY supervisord.conf /etc/supervisord.conf

# Copy and set permissions for executables
COPY setup.sh generator-client kalypso-listener prover-executable ./
RUN chmod +x setup.sh generator-client kalypso-listener prover-executable

# Copy config file (The config file should contain the path to your github repo and project name)
COPY config.toml ./

COPY ./app/id.pub ./app/id.sec ./app/secp.pub ./app/secp.sec ./

# entry point
ENTRYPOINT [ "/app/setup.sh" ]