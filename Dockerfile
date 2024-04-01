#Copied from the other repo

# Use the official Ubuntu image as the base image
FROM ubuntu:22.04

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install necessary dependencies for building and running Rust applications
RUN apt-get update && \
    apt-get install -y \
    curl \
    build-essential \
    openssh-server

# Install Rust using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y


RUN apt-get install -y \
libncurses5-dev \
libncursesw5-dev

# Add Rust binaries to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Just dumping the rust binary at the home dir 
COPY target/release/kill-rs .

# Expose SSH port
EXPOSE 22

# Configure SSH
RUN sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Start SSH server
CMD ["/usr/sbin/sshd", "-D"]
